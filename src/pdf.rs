use std::collections::HashMap;
use std::io::Write;

use crate::dictionary::Dictionary;
use crate::object::{BaseObject, ObjectStatus, PdfObject};
use crate::page::{PageSize, Page};
use crate::string::encode_pdf_string;

/// PDF file identifier mode.
///
/// Controls how the file identifier is generated in the PDF trailer.
pub enum Identifier {
    None,
    AutoMD5,
    Custom(Vec<u8>),
}

/// Main PDF document structure.
///
/// Represents a complete PDF document with objects, pages, and metadata.
pub struct PDF {
    pub objects: Vec<Box<dyn PdfObject>>,
    pub page_tree: Dictionary,
    pub info: Dictionary,
    pub catalog: Dictionary,
    pub current_position: usize,
    pub xref_position: Option<usize>,
    pub default_page_size: PageSize,
}

impl Default for PDF {
    fn default() -> Self {
        PDF {
            objects: Vec::new(),
            page_tree: Dictionary::new(None),
            info: Dictionary::new(None),
            catalog: Dictionary::new(None),
            current_position: 0,
            xref_position: None,
            default_page_size: PageSize::default(),
        }
    }
}

impl PDF {

    pub fn new(size: PageSize) -> Self {
        let mut pdf = PDF {
            default_page_size: size,
            ..Default::default()
        };

        let zero_object = BaseObject::sentinel();
        pdf.add_object(Box::new(zero_object));

        let mut pages_values = HashMap::new();
        pages_values.insert("Type".to_string(), b"/Pages".to_vec());
        pages_values.insert("Kids".to_string(), b"[]".to_vec());
        pages_values.insert("Count".to_string(), b"0".to_vec());
        pages_values.insert("MediaBox".to_string(), size.to_mediabox());
        pdf.page_tree = Dictionary::new(Some(pages_values));

        let mut catalog_values = HashMap::new();
        catalog_values.insert("Type".to_string(), b"/Catalog".to_vec());
        pdf.catalog = Dictionary::new(Some(catalog_values));

        pdf
    }

    /// Set the default page size for the document.
    pub fn with_default_page_size(mut self, size: PageSize) -> Self {
        self.default_page_size = size;
        self.page_tree.values.insert("MediaBox".to_string(), size.to_mediabox());
        self
    }

    /// Preferred API: add a strongly-typed Page
    pub fn add_page(&mut self, page: Page) {
        let count_bytes = self.page_tree.values.get("Count").unwrap();
        let count: i32 = String::from_utf8_lossy(count_bytes).trim().parse().unwrap();
        self.page_tree
            .values
            .insert("Count".to_string(), (count + 1).to_string().into_bytes());

        self.add_object(Box::new(page));

        let page_number = self.objects.len() - 1;
        let mut kids = self.page_tree.values.get("Kids").unwrap().clone();
        kids.pop();
        if kids.len() > 1 {
            kids.push(b' ');
        }
        kids.extend(format!("{} 0 R]", page_number).as_bytes());
        self.page_tree.values.insert("Kids".to_string(), kids);
    }

    pub fn add_page_simple(&mut self, size: Option<PageSize>, contents: &[u8]) {
        let mut page = Page::new();
        if let Some(s) = size { page.set_size(s); }
        page.set_contents(contents.to_vec());
        self.add_page(page);
    }

    pub fn add_object(&mut self, mut object: Box<dyn PdfObject>) -> usize {
        let number = self.objects.len();
        object.metadata_mut().number = Some(number);
        self.objects.push(object);
        number
    }

    pub fn page_references(&self) -> Vec<Vec<u8>> {
        let kids_str = self
            .page_tree
            .values
            .get("Kids")
            .map(|v| String::from_utf8_lossy(v).to_string())
            .unwrap_or_else(|| "[]".to_string());

        // Parse references in format "[1 0 R 2 0 R 3 0 R]"
        kids_str
            .trim_matches(|c| c == '[' || c == ']')
            .split_whitespace()
            .collect::<Vec<_>>()
            .chunks(3)
            .filter_map(|chunk| {
                if chunk.len() == 3 && chunk[2] == "R" {
                    Some(format!("{} {} {}", chunk[0], chunk[1], chunk[2]).into_bytes())
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn write_line<W: Write>(&mut self, content: &[u8], output: &mut W) -> std::io::Result<()> {
        self.current_position += content.len() + 1;
        output.write_all(content)?;
        output.write_all(b"\n")?;
        Ok(())
    }

    fn write_legacy_xref_and_trailer<W: Write>(
        &mut self,
        output: &mut W,
        identifier: Identifier,
    ) -> std::io::Result<()> {
        self.xref_position = Some(self.current_position);
        self.write_line(b"xref", output)?;
        self.write_line(format!("0 {}", self.objects.len()).as_bytes(), output)?;

        let xref_entries: Vec<String> = self
            .objects
            .iter()
            .map(|obj| obj.metadata().format_xref_entry())
            .collect();

        for entry in xref_entries {
            self.write_line(entry.as_bytes(), output)?;
        }

        self.write_line(b"trailer", output)?;
        self.write_line(b"<<", output)?;
        self.write_line(format!("/Size {}", self.objects.len()).as_bytes(), output)?;
        self.write_line(
            &format!("/Root {} 0 R", self.catalog.metadata().number.unwrap()).into_bytes(),
            output,
        )?;

        if !self.info.values.is_empty() {
            self.write_line(
                &format!("/Info {} 0 R", self.info.metadata().number.unwrap()).into_bytes(),
                output,
            )?;
        }

        if let Some(id_line) = Self::format_identifier(&self.objects, &identifier) {
            self.write_line(&id_line, output)?;
        }
        self.write_line(b">>", output)?;
        Ok(())
    }

    fn get_standard_fonts() -> Vec<u8> {
        let mut font_dict = String::from("<<");
        let fonts = [
            ("Helvetica", "Type1"),
            ("Helvetica-Bold", "Type1"),
            ("Courier", "Type1"),
        ];
        for (name, subtype) in fonts {
            font_dict.push_str(&format!(
                " /{} << /Type /Font /Subtype /{} /BaseFont /{} >>",
                name, subtype, name
            ));
        }
        font_dict.push_str(" >>");
        font_dict.into_bytes()
    }

    /// Generates the fully formatted PDF /ID line based on the identifier mode.
    fn format_identifier(
        objects: &[Box<dyn PdfObject>],
        identifier: &Identifier,
    ) -> Option<Vec<u8>> {
        match identifier {
            Identifier::None => None,
            Identifier::AutoMD5 | Identifier::Custom(_) => {
                // Calculate MD5 hash of all non-free objects
                let mut context = md5::Context::new();
                for obj in objects {
                    if obj.metadata().status != ObjectStatus::Free {
                        context.consume(obj.data());
                    }
                }
                let hash_result = context.finalize().0;

                // Convert hash to hex string for the permanent part of the ID
                let data_hash_hex: String =
                    hash_result.iter().map(|b| format!("{:02x}", b)).collect();
                let data_hash_bytes = data_hash_hex.as_bytes();

                // Select bytes for the first ID (Custom or Auto-MD5)
                let id_bytes = match identifier {
                    Identifier::AutoMD5 => data_hash_bytes,
                    Identifier::Custom(bytes) => bytes,
                    _ => unreachable!(),
                };

                // Wrap IDs in PDF string escaping (handling (), etc.)
                let s1 = encode_pdf_string(&String::from_utf8_lossy(id_bytes));
                let s2 = encode_pdf_string(&String::from_utf8_lossy(data_hash_bytes));

                Some(
                    format!(
                        "/ID [{} {}]",
                        String::from_utf8_lossy(&s1),
                        String::from_utf8_lossy(&s2)
                    )
                    .into_bytes(),
                )
            }
        }
    }

    fn write_legacy_objects<W: Write>(
        objects: &mut Vec<Box<dyn PdfObject>>,
        current_position: &mut usize,
        output: &mut W,
    ) -> std::io::Result<()> {
        let mut indirect_objects = Vec::new();
        for obj in objects.iter_mut() {
            if obj.metadata().status == ObjectStatus::Free {
                continue;
            }
            obj.metadata_mut().offset = *current_position;
            let indirect = obj.indirect();
            let len = indirect.len();
            indirect_objects.push(indirect);
            *current_position += len + 1;
        }

        for indirect_obj in &indirect_objects {
            output.write_all(indirect_obj)?;
            output.write_all(b"\n")?;
        }
        Ok(())
    }

    pub fn write<W: Write>(
        &mut self,
        output: &mut W,
        version: Option<&[u8]>,
        id_mode: Identifier,
        compress: bool,
    ) -> std::io::Result<()> {
        let version = version.unwrap_or(b"1.7");

        let font_resources = Self::get_standard_fonts();
        let resources_number = self.objects.len();
        let mut resources = Dictionary::new(None);
        resources.metadata.number = Some(resources_number);
        resources
            .values
            .insert("Font".to_string(), font_resources.clone());
        self.objects.push(Box::new(resources));

        if self.page_tree.metadata.number.is_none() {
            let pages_number = self.objects.len();
            self.page_tree.metadata.number = Some(pages_number);
            let pages_ref = format!("{} 0 R", pages_number).into_bytes();
            let res_ref = format!("{} 0 R", resources_number).into_bytes();

            for obj in &mut self.objects {
                if let Some(page) = obj.as_any_mut().downcast_mut::<Page>() {
                    // Set Parent on Page by injecting into its 'other' map
                    page.other.insert("Parent".to_string(), pages_ref.clone());
                    // Merge or set Resources
                    if let Some(resources) = page.resources.clone() {
                        let res_bytes = resources.data();
                        let res_str = String::from_utf8_lossy(&res_bytes);
                        if res_str.starts_with("<<") {
                            let mut merged = res_str.trim_end_matches(">>").to_string();
                            merged.push_str(" /Font ");
                            merged.push_str(&String::from_utf8_lossy(&font_resources));
                            merged.push_str(" >>");
                            page.other.insert("Resources".to_string(), merged.into_bytes());
                        }
                    } else {
                        page.other.insert("Resources".to_string(), res_ref.clone());
                    }
                }
            }
            let pages_copy = self.page_tree.clone();
            self.objects.push(Box::new(pages_copy));
        }

        if self.catalog.metadata.number.is_none() {
            let catalog_number = self.objects.len();
            self.catalog.metadata.number = Some(catalog_number);
            let pages_ref = self.page_tree.reference();
            self.catalog.values.insert("Pages".to_string(), pages_ref);
            let catalog_copy = self.catalog.clone();
            self.objects.push(Box::new(catalog_copy));
        }

        // Add Info if needed
        if !self.info.values.is_empty() && self.info.metadata.number.is_none() {
            self.info.metadata.number = Some(self.objects.len());
            let info_copy = self.info.clone();
            self.objects.push(Box::new(info_copy));
        }

        // --- SCOPE 2: PDF Printing Phase (The Writing Phase) ---
        self.write_line(
            &format!("%PDF-{}", String::from_utf8_lossy(version)).into_bytes(),
            output,
        )?;
        self.write_line(b"%\xf0\x9f\x96\xa4", output)?;

        if version >= b"1.5" && compress {
            self.write_compressed(output)?;
        } else {
            Self::write_legacy_objects(&mut self.objects, &mut self.current_position, output)?;

            // Re-calculate positions for Catalog and Info since they were pushed to objects
            // The original logic used self.catalog.reference(), but that only works if catalog.number is set.
            // In our refactor, we push a CLONE of self.catalog, so self.catalog itself might not have the ID.

            self.write_legacy_xref_and_trailer(output, id_mode)?;
        }

        self.write_line(b"startxref", output)?;
        self.write_line(
            self.xref_position.unwrap_or(0).to_string().as_bytes(),
            output,
        )?;
        self.write_line(b"%%EOF", output)?;

        Ok(())
    }

    /// Write compressed PDF using object streams and cross-reference streams (PDF 1.5+)
    fn write_compressed<W: Write>(&mut self, output: &mut W) -> std::io::Result<()> {
        use crate::array::Array;
        use crate::stream::Stream;

        // Separate compressible objects from non-compressible ones
        let mut compressed_data: Vec<(usize, Vec<u8>)> = Vec::new(); // (obj_num, data)
        let mut indirect_to_write = Vec::new();

        let catalog_num = self.catalog.metadata.number;
        let pages_num = self.page_tree.metadata.number;

        for obj in &mut self.objects {
            let meta = obj.metadata();
            if meta.status == ObjectStatus::Free {
                continue;
            }

            // Don't compress Catalog, Pages, or inherently non-compressible objects (streams)
            let is_catalog_or_pages = meta.number == catalog_num || meta.number == pages_num;

            if obj.is_compressible() && !is_catalog_or_pages {
                compressed_data.push((meta.number.unwrap_or(0), obj.data()));
            } else {
                // Write non-compressible objects directly
                obj.metadata_mut().offset = self.current_position;
                let indirect = obj.indirect();
                let len = indirect.len();
                indirect_to_write.push(indirect);
                self.current_position += len + 1;
            }
        }

        // Write non-compressible objects first
        for indirect_obj in &indirect_to_write {
            output.write_all(indirect_obj)?;
            output.write_all(b"\n")?;
        }

        // Build object stream from compressed objects
        let mut stream_parts: Vec<Vec<u8>> = vec![Vec::new()];
        let mut position = 0;
        let mut first_part_entries = Vec::new();

        for (obj_num, data) in &compressed_data {
            stream_parts.push(data.clone());
            first_part_entries.push(obj_num.to_string());
            first_part_entries.push(position.to_string());
            position += data.len() + 1;
        }

        stream_parts[0] = first_part_entries.join(" ").into_bytes();

        let mut extra = HashMap::new();
        extra.insert("Type".to_string(), b"/ObjStm".to_vec());
        extra.insert(
            "N".to_string(),
            compressed_data.len().to_string().into_bytes(),
        );
        extra.insert(
            "First".to_string(),
            (stream_parts[0].len() + 1).to_string().into_bytes(),
        );

        let obj_stream_number = self.objects.len();
        let mut object_stream = Stream::new_compressed().with_data(Some(stream_parts), Some(extra));
        object_stream.metadata.number = Some(obj_stream_number);
        object_stream.metadata.offset = self.current_position;

        let obj_stream_indirect = object_stream.indirect();
        let len = obj_stream_indirect.len();
        output.write_all(&obj_stream_indirect)?;
        output.write_all(b"\n")?;
        self.current_position += len + 1;

        let mut xref: Vec<(u8, usize, u32)> = Vec::new();

        for obj in &self.objects {
            let meta = obj.metadata();
            let obj_num = meta.number.unwrap_or(0);

            // Check if this object was actually compressed (not just compressible)
            if let Some(pos) = compressed_data.iter().position(|(n, _)| *n == obj_num) {
                // Type 2: compressed object
                xref.push((2, obj_stream_number, pos as u32));
            } else {
                let flag = if meta.status == ObjectStatus::Free {
                    0 // free
                } else {
                    1 // normal object
                };
                xref.push((flag, meta.offset, meta.generation));
            }
        }

        xref.push((1, object_stream.metadata.offset, 0)); // Add object stream itself as type 1
        xref.push((1, self.current_position, 0)); // Add xref stream itself as type 1

        let field2_size = ((self.current_position + 1) as f64).log(256.0).ceil() as usize;

        // field3 needs to handle max generation (65535 for free objects) and max index
        let max_generation = xref.iter().map(|(_, _, g)| *g).max().unwrap_or(0);
        let max_index = compressed_data.len().max(1) as u32;
        let max_field3 = max_generation.max(max_index);
        let field3_size = if max_field3 == 0 {
            1
        } else {
            (max_field3 as f64).log(256.0).ceil() as usize
        };

        let mut xref_stream_data = Vec::new();
        for (flag, field2, field3) in &xref {
            xref_stream_data.push(*flag);
            let field2_bytes = field2.to_be_bytes();
            xref_stream_data.extend(&field2_bytes[8 - field2_size..]);
            let field3_bytes = field3.to_be_bytes();
            xref_stream_data.extend(&field3_bytes[4 - field3_size..]);
        }

        let mut xref_extra = HashMap::new();
        xref_extra.insert("Type".to_string(), b"/XRef".to_vec());

        // objstm number is self.objects.len(), xref stream is self.objects.len() + 1
        // Size is highest object number + 1, so self.objects.len() + 2
        let xref_stream_number = self.objects.len() + 1;
        let total_size = self.objects.len() + 2;

        let index_array = Array::new(Some(vec![0.0, total_size as f64]));
        xref_extra.insert("Index".to_string(), index_array.data());

        let w_array = Array::new(Some(vec![1.0, field2_size as f64, field3_size as f64]));
        xref_extra.insert("W".to_string(), w_array.data());

        xref_extra.insert("Size".to_string(), total_size.to_string().into_bytes());
        xref_extra.insert("Root".to_string(), self.catalog.reference());

        let mut xref_stream =
            Stream::new_compressed().with_data(Some(vec![xref_stream_data]), Some(xref_extra));
        xref_stream.metadata.number = Some(xref_stream_number);
        self.xref_position = Some(self.current_position);
        xref_stream.metadata.offset = self.current_position;

        self.write_line(&xref_stream.indirect(), output)?;

        Ok(())
    }
}
