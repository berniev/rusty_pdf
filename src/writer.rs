use std::collections::HashMap;
use std::io::Write;

use crate::objects::string::encode_pdf_string;
use crate::{ArrayObject, FileIdentifierMode, ObjectStatus, PDF, PdfObject, StreamObject};

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
        self.pos += bytes.len() + 1;

        Ok(())
    }
}

//---------------------------- WriteStrategy -----------------

pub trait WriteStrategy {

    //////////////////////////////////////////////////////////
    // These MUST be supplied in every strategy implementation
    //////////////////////////////////////////////////////////

    const VERSION: &[u8];
    fn get_version(&self) -> &[u8];

    fn write_body<W: Write>(&self, pdf: &mut PDF, stream: &mut PdfStream<W>)
                            -> std::io::Result<()>;

    fn write_index<W: Write>(
        &self,
        pdf: &mut PDF,
        stream: &mut PdfStream<W>,
    ) -> std::io::Result<()>;

    ///////////////////
    // shared functions
    ///////////////////

    fn write_line<W: Write>(&self, stream: &mut PdfStream<W>, bytes: &[u8]) -> std::io::Result<()> {
        stream.write_line(bytes);
        Ok(())
    }

    /// Generates the fully formatted PDF /ID line based on the identifier mode.
    /// in trailer
    fn format_identifier(
        objects: &[Box<dyn PdfObject>],
        identifier_mode: &FileIdentifierMode,
    ) -> Option<Vec<u8>> {
        match identifier_mode {
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
                let id_bytes = match identifier_mode {
                    FileIdentifierMode::AutoMD5 => data_hash_bytes,
                    FileIdentifierMode::Custom(bytes) => bytes,
                    _ => unreachable!(),
                };

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

    fn write_header<W: Write>(&self, stream: &mut PdfStream<W>) -> std::io::Result<()> {
        let mut header = b"%PDF-".to_vec();
        header.extend_from_slice(self.get_version());
        stream.write_line(&header)?;
        stream.write_line(b"%\xf0\x9f\x96\xa4") // Binary marker
    }
    // Writing a single, uncompressed object
    // Both Legacy and Compressed writers need this for Streams/Images.
    fn write_single_object<W: Write>(
        &self,
        obj: &mut dyn PdfObject,
        stream: &mut PdfStream<W>,
    ) -> std::io::Result<()> {
        obj.metadata_mut().offset = stream.pos;
        stream.write_line(&obj.indirect())
    }

    // 4. ORCHESTRATOR: The shared "Skeleton" of the performance
    fn perform<W: Write>(&self, pdf: &mut PDF, stream: &mut PdfStream<W>) -> std::io::Result<()> {
        self.write_header(stream)?;
        self.write_body(pdf, stream)?;
        self.write_index(pdf, stream)?;

        stream.write_line(b"startxref")?;
        stream.write_line(pdf.xref_position.unwrap_or(0).to_string().as_bytes())?;
        stream.write_line(b"%%EOF")
    }

    fn write_trailer<W: Write>(&mut self, stream: &mut PdfStream<W>) -> std::io::Result<()> {
        stream.write_line(b"trailer")?;
        stream.write_line(b"<<")?;
        stream.write_line(format!("/Size {}", self.objects.len()).as_bytes())?;
        stream.write_line(
            format!("/Root {} 0 R", self.catalog.metadata().number.unwrap()).into_bytes(),
        )?;
        stream.write_line(b">>")?;
        Ok(())
    }
}

//------------------------------ Legacy Strategy -----------------

pub struct LegacyStrategy;

impl LegacyStrategy {
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

        Ok(())
    }

}

impl WriteStrategy for LegacyStrategy {
    const VERSION: &[u8] = b"1.4";

    fn get_version(&self) -> &[u8] {
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


}

//------------------------ Compressed Strategy -----------------

pub struct CompressedStrategy;

impl CompressedStrategy {
    //If a file has object streams, the cross-reference table is replaced with a cross-reference stream,
    // which has a binary format and is therefore harder to read
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
            StreamObject::new().compressed().with_data(Some(stream_parts), Some(extra));
        object_stream.metadata.number = Some(obj_stream_number);
        object_stream.metadata.offset = self.current_position;

        let obj_stream_indirect = object_stream.indirect();
        let len = obj_stream_indirect.len();
        self.write_line(obj_stream_indirect)?;
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

        let index_array = ArrayObject::new(Some(vec![0.0, total_size as f64]));
        xref_extra.insert("Index".to_string(), index_array.data());

        let w_array = ArrayObject::new(Some(vec![1.0, field2_size as f64, field3_size as f64]));
        xref_extra.insert("W".to_string(), w_array.data());

        xref_extra.insert("Size".to_string(), total_size.to_string().into_bytes());
        xref_extra.insert("Root".to_string(), self.catalog.reference());

        let mut xref_stream = StreamObject::compressed()
            .with_data(Some(vec![xref_stream_data]), Some(xref_extra));
        xref_stream.metadata.number = Some(xref_stream_number);
        self.xref_position = Some(self.current_position);
        xref_stream.metadata.offset = self.current_position;

        self.write_line(&xref_stream.indirect(), output)?;

        Ok(())
    }
}

impl WriteStrategy for CompressedStrategy {
    const VERSION: &[u8] = b"1.7";

    fn get_version(&self) -> &[u8] {
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
}

