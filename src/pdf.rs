use std::collections::HashMap;
use std::io::Write;

use crate::dictionary::Dictionary;
use crate::object::{Object, ObjectStatus, PdfObject};
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
///
/// # Example
///
/// ```rust
/// use pydyf::PDF;
///
/// let mut pdf = PDF::new();
/// // Add objects and pages...
/// ```
pub struct PDF {
    pub objects: Vec<Box<dyn PdfObject>>,
    pub pages: Dictionary,
    pub info: Dictionary,
    pub catalog: Dictionary,
    pub current_position: usize,
    pub xref_position: Option<usize>,
}

impl PDF {
    /// Create a new PDF document.
    ///
    /// Initializes the document with required structures including the catalog,
    /// pages tree, and object 0 sentinel.
    pub fn new() -> Self {
        let mut pdf = PDF {
            objects: Vec::new(),
            pages: Dictionary::new(None),
            info: Dictionary::new(None),
            catalog: Dictionary::new(None),
            current_position: 0,
            xref_position: None,
        };

        // PDF spec requires object 0 to be free with generation 65535
        // This is a sentinel value marking the head of the free object list
        let mut zero_object = Object::new();
        zero_object.metadata.generation = 65535;
        zero_object.metadata.status = ObjectStatus::Free;
        pdf.add_object(Box::new(zero_object));

        let mut pages_values = HashMap::new();
        pages_values.insert("Type".to_string(), b"/Pages".to_vec());
        pages_values.insert("Kids".to_string(), b"[]".to_vec());
        pages_values.insert("Count".to_string(), b"0".to_vec());
        pdf.pages = Dictionary::new(Some(pages_values));

        let mut catalog_values = HashMap::new();
        catalog_values.insert("Type".to_string(), b"/Catalog".to_vec());
        pdf.catalog = Dictionary::new(Some(catalog_values));

        pdf
    }

    /// Add a page to the document.
    ///
    /// The page dictionary should contain at minimum:
    /// - `Type`: `/Page`
    /// - `MediaBox`: Page dimensions (e.g., `[0 0 612 792]`)
    /// - `Contents`: Reference to content stream
    ///
    /// # Example
    ///
    /// ```rust
    /// use pydyf::{PDF, Dictionary};
    /// use std::collections::HashMap;
    ///
    /// let mut pdf = PDF::new();
    /// let mut page_values = HashMap::new();
    /// page_values.insert("Type".to_string(), b"/Page".to_vec());
    /// page_values.insert("MediaBox".to_string(), b"[0 0 612 792]".to_vec());
    /// let page = Dictionary::new(Some(page_values));
    /// pdf.add_page(page);
    /// ```
    pub fn add_page(&mut self, page: Dictionary) {
        let count_bytes = self.pages.values.get("Count").unwrap();
        let count: i32 = String::from_utf8_lossy(count_bytes).trim().parse().unwrap();
        self.pages
            .values
            .insert("Count".to_string(), (count + 1).to_string().into_bytes());

        // Parent reference will be set at write time
        self.add_object(Box::new(page));

        // Note: In Python this is simpler: self.pages['Kids'].extend([page.number, 0, 'R'])
        // In Rust, we need to reconstruct the Kids array as bytes
        let page_number = self.objects.len() - 1;
        let mut kids = self.pages.values.get("Kids").unwrap().clone();
        kids.pop(); // Remove ']'
        if kids.len() > 1 {
            kids.push(b' ');
        }
        kids.extend(format!("{} 0 R]", page_number).as_bytes());
        self.pages.values.insert("Kids".to_string(), kids);
    }

    pub fn add_object(&mut self, mut object: Box<dyn PdfObject>) -> usize {
        let number = self.objects.len();
        object.metadata_mut().number = Some(number);
        self.objects.push(object);
        number
    }

    pub fn page_references(&self) -> Vec<Vec<u8>> {
        let kids_str = self
            .pages
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

    /// Write a line to output and track byte position.
    pub fn write_line<W: Write>(&mut self, content: &[u8], output: &mut W) -> std::io::Result<()> {
        self.current_position += content.len() + 1;
        output.write_all(content)?;
        output.write_all(b"\n")?;
        Ok(())
    }

    /// Write the PDF document to output.
    ///
    /// # Arguments
    ///
    /// * `output` - Output writer (e.g., File)
    /// * `version` - PDF version (e.g., `Some(b"1.7")` or `None` for default 1.3)
    /// * `identifier` - File identifier mode
    /// * `compress` - Enable stream compression with flate
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use pydyf::{PDF, Identifier};
    /// use std::fs::File;
    ///
    /// let mut pdf = PDF::new();
    /// let mut file = File::create("output.pdf").unwrap();
    /// pdf.write(&mut file, Some(b"1.7"), Identifier::AutoMD5, false).unwrap();
    /// ```
    ///
    /// # Arguments
    /// * `output` - Output stream
    /// * `version` - PDF version (default: b"1.7")
    /// * `identifier` - PDF file identifier. None to exclude, Auto to generate, or Custom bytes
    /// * `compress` - Whether the PDF uses a compressed object stream
    pub fn write<W: Write>(
        &mut self,
        output: &mut W,
        version: Option<&[u8]>,
        identifier: Identifier,
        compress: bool,
    ) -> std::io::Result<()> {
        // Convert version to bytes, default to "1.7"
        let version = version.unwrap_or(b"1.7");

        // Create and add Resources dictionary with standard fonts
        let resources_number = self.objects.len();
        let mut resources = Dictionary::new(None);
        resources.metadata.number = Some(resources_number);

        let mut font_dict_values = HashMap::new();
        font_dict_values.insert(
            "Helvetica".to_string(),
            b"<</Type /Font/Subtype /Type1/BaseFont /Helvetica>>".to_vec(),
        );
        font_dict_values.insert(
            "Helvetica-Bold".to_string(),
            b"<</Type /Font/Subtype /Type1/BaseFont /Helvetica-Bold>>".to_vec(),
        );
        font_dict_values.insert(
            "Courier".to_string(),
            b"<</Type /Font/Subtype /Type1/BaseFont /Courier>>".to_vec(),
        );

        let mut font_dict_bytes = b"<<".to_vec();
        for (name, def) in &font_dict_values {
            font_dict_bytes.extend(b"/");
            font_dict_bytes.extend(name.as_bytes());
            font_dict_bytes.extend(b" ");
            font_dict_bytes.extend(def);
        }
        font_dict_bytes.extend(b">>");

        resources
            .values
            .insert("Font".to_string(), font_dict_bytes.clone());
        self.objects.push(Box::new(resources));

        let resources_ref = format!("{} 0 R", resources_number).into_bytes();

        // Add Pages object if not already added
        if self.pages.metadata.number.is_none() {
            let pages_number = self.objects.len();
            self.pages.metadata.number = Some(pages_number);

            // Update all page objects to point to this Pages object and add Resources
            let pages_ref = format!("{} 0 R", pages_number).into_bytes();
            for obj in &mut self.objects {
                // Check if this is a page object (has /Type /Page)
                if let Some(dict) = obj.as_any_mut().downcast_mut::<Dictionary>() {
                    if dict.values.get("Type").map(|v| v.as_slice()) == Some(b"/Page") {
                        dict.values.insert("Parent".to_string(), pages_ref.clone());

                        // Merge resources instead of overwriting
                        if let Some(existing_resources) = dict.values.get("Resources").cloned() {
                            // Parse existing resources and merge with font resources
                            let existing_str = String::from_utf8_lossy(&existing_resources);
                            if existing_str.starts_with("<<") && existing_str.ends_with(">>") {
                                // Existing resources is inline dictionary - merge it with fonts
                                let mut merged = existing_str.trim_end_matches(">>").to_string();
                                // Add space before /Font if needed
                                if !merged.ends_with(' ') {
                                    merged.push(' ');
                                }
                                merged.push_str("/Font ");
                                merged.push_str(&String::from_utf8_lossy(&font_dict_bytes));
                                merged.push_str(" >>");
                                dict.values
                                    .insert("Resources".to_string(), merged.into_bytes());
                            } else {
                                // Existing resources is likely a reference - keep it
                                // (this shouldn't happen with current usage but handle it gracefully)
                            }
                        } else {
                            // No existing resources - use font-only resources
                            dict.values
                                .insert("Resources".to_string(), resources_ref.clone());
                        }
                    }
                }
            }

            let pages_copy = self.pages.clone();
            self.objects.push(Box::new(pages_copy));
        }

        // Add Catalog object if not already added
        if self.catalog.metadata.number.is_none() {
            let catalog_number = self.objects.len();
            self.catalog.metadata.number = Some(catalog_number);

            // Set catalog to point to pages
            let pages_ref = self.pages.reference();
            self.catalog.values.insert("Pages".to_string(), pages_ref);

            let catalog_copy = self.catalog.clone();
            self.objects.push(Box::new(catalog_copy));
        }

        // Add info object if needed
        if !self.info.values.is_empty() && self.info.metadata.number.is_none() {
            self.info.metadata.number = Some(self.objects.len());
            let info_copy = self.info.clone();
            self.objects.push(Box::new(info_copy));
        }

        // Write header
        let mut header = b"%PDF-".to_vec();
        header.extend(version);
        self.write_line(&header, output)?;
        self.write_line(b"%\xf0\x9f\x96\xa4", output)?;

        if version >= b"1.5" && compress {
            self.write_compressed(output)?; // using object streams and xref streams
        } else {
            // First pass: set offsets and collect indirect data
            let mut indirect_objects = Vec::new();
            for obj in &mut self.objects {
                if obj.metadata().status == ObjectStatus::Free {
                    continue; // don't write free objects
                }
                obj.metadata_mut().offset = self.current_position;
                let indirect = obj.indirect();
                let len = indirect.len();
                indirect_objects.push(indirect);
                self.current_position += len + 1; // +1 for newline
            }

            // Second pass: write the objects
            for indirect_obj in &indirect_objects {
                output.write_all(indirect_obj)?;
                output.write_all(b"\n")?;
            }

            // Write cross-reference table
            self.xref_position = Some(self.current_position);
            self.write_line(b"xref", output)?;

            let xref_header = format!("0 {}", self.objects.len());
            self.write_line(xref_header.as_bytes(), output)?;

            let xref_entries: Vec<String> = self
                .objects
                .iter()
                .map(|obj| {
                    let meta = obj.metadata();
                    format!(
                        "{:010} {:05} {} ",
                        meta.offset, meta.generation, meta.status
                    )
                })
                .collect();

            for xref_entry in &xref_entries {
                self.write_line(xref_entry.as_bytes(), output)?;
            }

            // Write trailer
            self.write_line(b"trailer", output)?;
            self.write_line(b"<<", output)?;

            let size_line = format!("/Size {}", self.objects.len());
            self.write_line(size_line.as_bytes(), output)?;

            let mut root_line = b"/Root ".to_vec();
            root_line.extend(self.catalog.reference());
            self.write_line(&root_line, output)?;

            if !self.info.values.is_empty() {
                let mut info_line = b"/Info ".to_vec();
                info_line.extend(self.info.reference());
                self.write_line(&info_line, output)?;
            }

            match identifier {
                Identifier::None => {}
                Identifier::AutoMD5 | Identifier::Custom(_) => {
                    // Collect all non-free object data
                    let mut all_data = Vec::new();
                    for obj in &self.objects {
                        if obj.metadata().status != ObjectStatus::Free {
                            all_data.extend(obj.data());
                        }
                    }

                    // Calculate MD5 hash
                    let hash_result = md5::compute(&all_data);
                    let data_hash = format!("{:x}", hash_result).into_bytes();

                    let id_value = match identifier {
                        Identifier::AutoMD5 => &data_hash,
                        Identifier::Custom(ref bytes) => bytes,
                        _ => unreachable!(),
                    };

                    let string1 = encode_pdf_string(&String::from_utf8_lossy(id_value));
                    let string2 = encode_pdf_string(&String::from_utf8_lossy(&data_hash));

                    let mut id_line = b"/ID [".to_vec();
                    id_line.extend(&string1);
                    id_line.push(b' ');
                    id_line.extend(&string2);
                    id_line.push(b']');
                    self.write_line(&id_line, output)?;
                }
            }

            self.write_line(b">>", output)?;
        }

        // Write footer
        self.write_line(b"startxref", output)?;
        let xref_pos = format!("{}", self.xref_position.unwrap_or(0));
        self.write_line(xref_pos.as_bytes(), output)?;
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
        let pages_num = self.pages.metadata.number;

        for obj in &mut self.objects {
            let meta = obj.metadata();
            if meta.status == ObjectStatus::Free {
                continue;
            }

            // Don't compress Catalog, Pages, or inherently non-compressible objects (streams)
            let is_catalog_or_pages = meta.number == catalog_num || meta.number == pages_num;

            if obj.is_compressible() && !is_catalog_or_pages {
                // Collect data for compression
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

        // Build cross-reference stream
        let mut xref: Vec<(u8, usize, u32)> = Vec::new();

        for obj in &self.objects {
            let meta = obj.metadata();
            let obj_num = meta.number.unwrap_or(0);

            // Check if this object was actually compressed (not just compressible)
            if let Some(pos) = compressed_data.iter().position(|(n, _)| *n == obj_num) {
                // Type 2: compressed object
                xref.push((2, obj_stream_number, pos as u32));
            } else {
                // Type 1: normal object or Type 0: free
                let flag = if meta.status == ObjectStatus::Free {
                    0
                } else {
                    1
                };
                xref.push((flag, meta.offset, meta.generation));
            }
        }

        // Add object stream itself as type 1
        xref.push((1, object_stream.metadata.offset, 0));

        // Add xref stream itself as type 1
        xref.push((1, self.current_position, 0));

        // Calculate field sizes (bytes needed to store values)
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

        // Build xref stream data
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
