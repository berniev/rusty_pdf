use crate::objects::string::encode_pdf_string;
use crate::{
    Array, DictionaryObject, FileIdentifierMode, ObjectStatus, PDF, Page, PdfObject, StreamObject,
};
use std::collections::HashMap;
use std::io::Write;
use std::sync::Arc;
//---------------------------- PdfWriter ------------------

pub struct PdfWriter<W: Write, S: WriteStrategy> {
    stream: PdfStream<W>,
    strategy: S,
}

impl<W: Write, S: WriteStrategy> PdfWriter<W, S> {
    pub fn perform(&mut self, pdf: &mut PDF) -> std::io::Result<()> {
        self.strategy.write_header(&mut self.stream)?;
        self.strategy.perform(pdf, &mut self.stream)?;

        Ok(())
    }
}

//------------------------------ PdfStream ------------------

struct PdfStream<W: Write> {
    output: W,
    pos: usize,
}

impl<W: Write> PdfStream<W> {
    fn write_line(&mut self, bytes: &[u8]) -> std::io::Result<()> {
        self.output.write_all(bytes)?;
        self.output.write_all(b"\n")?;
        self.pos += bytes.len() + 1; // if both writes were successful

        Ok(())
    }
}
/*pub fn write_line<W: Write>(&mut self, content: &[u8], output: &mut W) -> std::io::Result<()> {
    self.current_position += content.len() + 1;
    output.write_all(content)?;
    output.write_all(b"\n")?;
    Ok(())
}
*/
//---------------------------- WriteStrategy -----------------

pub trait WriteStrategy {
    const VERSION: &[u8];

    fn version(&self) -> &[u8];

    fn write_header<W: Write>(&self, stream: &mut PdfStream<W>) -> std::io::Result<()> {
        let mut header = b"%PDF-".to_vec();
        header.extend_from_slice(self.version());
        stream.write_line(&header)?;
        stream.write_line(b"%\xf0\x9f\x96\xa4") // Binary marker
    }

    // 2. SHARED HELPER: Writing a single, uncompressed object
    // Both Legacy and Compressed writers need this for Streams/Images.
    fn write_single_object<W: Write>(
        &self,
        obj: &mut dyn PdfObject,
        stream: &mut PdfStream<W>,
    ) -> std::io::Result<()> {
        obj.metadata_mut().offset = stream.pos;
        stream.write_line(&obj.indirect())
    }

    // 3. REQUIRED: These are the parts that MUST be different
    fn write_body<W: Write>(&self, pdf: &mut PDF, stream: &mut PdfStream<W>)
    -> std::io::Result<()>;

    fn write_index<W: Write>(
        &self,
        pdf: &mut PDF,
        stream: &mut PdfStream<W>,
    ) -> std::io::Result<()>;

    // 4. ORCHESTRATOR: The shared "Skeleton" of the performance
    fn perform<W: Write>(&self, pdf: &mut PDF, stream: &mut PdfStream<W>) -> std::io::Result<()> {
        self.write_header(stream)?;
        self.write_body(pdf, stream)?;
        self.write_index(pdf, stream)?;
        stream.write_line(b"startxref")?;
        stream.write_line(pdf.xref_position.unwrap_or(0).to_string().as_bytes())?;
        stream.write_line(b"%%EOF")
    }
}

/*pub fn write<W: Write>(
    &mut self,
    output: &mut W,
    version: Option<&[u8]>,
    id_mode: FileIdentifierMode,
    compress: bool,
) -> std::io::Result<()> {
    let version = version.unwrap_or(b"1.7");

    let standard_fonts = Arc::new(Self::get_standard_fonts_dict());

    for obj in &mut self.objects {
        if let Some(page) = obj.as_any_mut().downcast_mut::<Page>() {
            // Modern merge: 'resources' is now a DictionaryObject in Page
            page.resources.set("Font", standard_fonts.clone());

            // Use IndirectReference for the Parent link
            let pages_id = self.page_tree.metadata().number.unwrap_or(0);
            page.set_parent(pages_id);
        }
    }

    if self.page_tree.metadata.number.is_none() {
        let pages_number = self.objects.len();
        self.page_tree.metadata.number = Some(pages_number);
        let pages_ref = format!("{} 0 R", pages_number).into_bytes();
        let res_ref = format!("{} 0 R", resources_number).into_bytes();

        for obj in &mut self.objects {
            if let Some(page) = obj.as_any_mut().downcast_mut::<Page>() {
                // 1. Get or build the Resources dictionary
                // If it doesn't exist, 'dict.get' will return None, so we set it.
                let mut res_dict = page
                    .dict
                    .get("Resources")
                    .and_then(|r| r.as_any().downcast_ref::<DictionaryObject>())
                    .cloned() // Requires Arc to be easy
                    .unwrap_or_else(|| DictionaryObject::new(None));

                // 2. Set the Font entry (This now works perfectly)
                res_dict.set("Font", standard_fonts.clone());
                page.dict.set("Resources", Arc::new(res_dict));

                // 3. Set the Parent link (Using the new Page method)
                let pages_id = self.page_tree.metadata().number.unwrap_or(0);
                page.set_parent(pages_id);
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
*/
//------------------------------ Legacy Strategy -----------------
pub struct LegacyStrategy;
impl WriteStrategy for LegacyStrategy {
    const VERSION: &[u8] = b"1.4";

    fn version(&self) -> &[u8] {
        Self::VERSION
    }

    fn write_body<W: Write>(
        &self,
        pdf: &mut PDF,
        stream: &mut PdfStream<W>,
    ) -> std::io::Result<()> {
        todo!()
    }

    fn write_index<W: Write>(
        &self,
        pdf: &mut PDF,
        stream: &mut PdfStream<W>,
    ) -> std::io::Result<()> {
        todo!()
    }

    fn perform<W: Write>(&self, pdf: &mut PDF, stream: &mut PdfStream<W>) -> std::io::Result<()> {
        stream.write_line(b"1.4")?;

        // 2. Objects
        for obj in &mut pdf.objects {
            obj.metadata_mut().offset = stream.pos;
            stream.write_line(&obj.indirect())?;
        }

        // 3. Footer (XRef + Trailer)
        let xref_pos = stream.pos;
        // ... (write standard xref and trailer) ...
        Ok(())
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

    fn write_legacy_xref<W: Write>(&mut self, output: &mut W) -> std::io::Result<()> {
        self.write_line(format!("0 {}", self.objects.len()).as_bytes(), output)?;

        let xref_entries: Vec<String> = self
            .objects
            .iter()
            .map(|obj| obj.metadata().format_xref_entry())
            .collect();

        for entry in xref_entries {
            self.write_line(entry.as_bytes(), output)?;
        }

        Ok(())
    }

    /// Generates the fully formatted PDF /ID line based on the identifier mode.
    fn format_identifier(
        objects: &[Box<dyn PdfObject>],
        identifier: &FileIdentifierMode,
    ) -> Option<Vec<u8>> {
        match identifier {
            FileIdentifierMode::None => None,
            FileIdentifierMode::AutoMD5 | FileIdentifierMode::Custom(_) => {
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
                    FileIdentifierMode::AutoMD5 => data_hash_bytes,
                    FileIdentifierMode::Custom(bytes) => bytes,
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

    fn write_legacy_xref_and_trailer<W: Write>(
        &mut self,
        output: &mut W,
        identifier: FileIdentifierMode,
    ) -> std::io::Result<()> {
        self.xref_position = Some(self.current_position);
        self.write_line(b"xref", output)?;
        self.write_line(format!("0 {}", self.objects.len()).as_bytes(), output)?;

        self.write_legacy_xref(output)?;
        self.write_trailer(output)?;

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
}

//------------------------ Compressed Strategy -----------------

pub struct CompressedStrategy;
impl WriteStrategy for CompressedStrategy {
    const VERSION: &[u8] = b"1.7";

    fn version(&self) -> &[u8] {
        Self::VERSION
    }

    fn write_body<W: Write>(
        &self,
        pdf: &mut PDF,
        stream: &mut PdfStream<W>,
    ) -> std::io::Result<()> {
        todo!()
    }

    fn write_index<W: Write>(
        &self,
        pdf: &mut PDF,
        stream: &mut PdfStream<W>,
    ) -> std::io::Result<()> {
        todo!()
    }

    fn perform<W: Write>(&self, pdf: &mut PDF, stream: &mut PdfStream<W>) -> std::io::Result<()> {
        // ... implements Object Streams and XRef Streams ...

        Ok(())
    }

    /// Write compressed PDF using object streams and cross-reference streams (PDF 1.5+)
    fn write_compressed<W: Write>(&mut self, output: &mut W) -> std::io::Result<()> {
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
        let mut object_stream =
            StreamObject::new_compressed().with_data(Some(stream_parts), Some(extra));
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

        let mut xref_stream = StreamObject::new_compressed()
            .with_data(Some(vec![xref_stream_data]), Some(xref_extra));
        xref_stream.metadata.number = Some(xref_stream_number);
        self.xref_position = Some(self.current_position);
        xref_stream.metadata.offset = self.current_position;

        self.write_line(&xref_stream.indirect(), output)?;

        Ok(())
    }

    fn write_trailer<W: Write>(&mut self, output: &mut W) -> std::io::Result<()> {
        self.write_line(b"trailer", output)?;
        self.write_line(b"<<", output)?;
        self.write_line(format!("/Size {}", self.objects.len()).as_bytes(), output)?;
        self.write_line(
            &format!("/Root {} 0 R", self.catalog.metadata().number.unwrap()).into_bytes(),
            output,
        )?;

        Ok(())
    }
}
