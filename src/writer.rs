use std::io::Write;

use crate::cross_ref::{ObjectStatus, CrossRefEntry};
use crate::objects::string::encode_pdf_string;
use crate::{FileIdentifierMode, PDF, PdfObject};

//------------------------------ PdfStream ------------------

pub(crate) struct PdfStream<W: Write> {
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

    /// Write a String that may contain Latin-1 encoded binary data.
    /// Converts Latin-1 back to raw bytes to preserve binary data.
    fn write_line_latin1(&mut self, s: &str) -> std::io::Result<()> {
        let bytes: Vec<u8> = s.chars().map(|c| c as u8).collect();
        self.write_line(&bytes)
    }
}

//---------------------------- PdfWriter ------------------

pub(crate) struct PdfWriter<W: Write, S: WriteStrategy> {
    stream: PdfStream<W>,
    strategy: S,
    id_mode: FileIdentifierMode,
}

impl<W: Write, S: WriteStrategy> PdfWriter<W, S> {
    pub fn new(output: W, strategy: S, id_mode: FileIdentifierMode) -> Self {
        Self {
            stream: PdfStream { output, pos: 0 },
            strategy,
            id_mode,
        }
    }

    pub fn perform(&mut self, pdf: &mut PDF) -> std::io::Result<()> {
        self.strategy.write_header(&mut self.stream)?;
        self.strategy.write_body(pdf, &mut self.stream)?;
        self.strategy
            .write_index(pdf, &mut self.stream, &self.id_mode)?;
        self.stream.write_line(b"startxref")?;
        self.stream
            .write_line(pdf.xref_position.unwrap_or(0).to_string().as_bytes())?;
        self.stream.write_line(b"%%EOF")
    }
}

//---------------------------- WriteStrategy -----------------

pub(crate) trait WriteStrategy {
    //////////////////////////////////////////////////////////
    // These MUST be supplied in every strategy implementation
    //////////////////////////////////////////////////////////
    const VERSION: &[u8];
    fn get_version(&self) -> &[u8];

    // These have default implementations but can be overridden
    fn write_body<W: Write>(
        &self,
        pdf: &mut PDF,
        stream: &mut PdfStream<W>,
    ) -> std::io::Result<()> {
        // Default: Write all objects individually (uncompressed)
        for obj in &mut pdf.objects {
            if obj.metadata().status == ObjectStatus::Free {
                continue;
            }
            obj.metadata_mut().offset = stream.pos;
            stream.write_line_latin1(&obj.indirect())?;
        }
        Ok(())
    }

    fn write_index<W: Write>(
        &self,
        pdf: &mut PDF,
        stream: &mut PdfStream<W>,
        id_mode: &FileIdentifierMode,
    ) -> std::io::Result<()> {
        // Default: Write legacy xref table
        pdf.xref_position = Some(stream.pos);
        stream.write_line(b"xref")?;
        stream.write_line(format!("0 {}", pdf.object_count()).as_bytes())?;

        // Per PDF spec, object 0 is always free (head of free list)
        stream.write_line(b"0000000000 65535 f ")?;

        // Write entries for actual objects (1 through N-1)
        let xref_entries: Vec<String> = pdf
            .objects
            .iter()
            .map(|obj| obj.metadata().format_xref_entry())
            .collect();

        for entry in xref_entries {
            stream.write_line(entry.as_bytes())?;
        }

        stream.write_line(b"trailer")?;
        stream.write_line(b"<<")?;
        stream.write_line(format!("/Size {}", pdf.object_count()).as_bytes())?;
        stream.write_line(
            format!(
                "/Root {} 0 R",
                pdf.catalog.metadata.object_identifier.unwrap()
            )
            .as_bytes(),
        )?;

        if !pdf.info.values.is_empty() {
            stream.write_line(
                &format!("/Info {} 0 R", pdf.info.metadata.object_identifier.unwrap()).into_bytes(),
            )?;
        }

        if let Some(id_line) = Self::format_identifier(&pdf.objects, id_mode) {
            stream.write_line(&id_line)?;
        }

        stream.write_line(b">>")?;
        Ok(())
    }

    ///////////////////
    // shared functions
    ///////////////////

    /// Formats two byte arrays into a PDF ID array string.
    fn format_id_array(first_id: &[u8], second_id: &[u8]) -> Vec<u8> {
        let s1 = encode_pdf_string(&String::from_utf8_lossy(first_id));
        let s2 = encode_pdf_string(&String::from_utf8_lossy(second_id));
        format!("/ID [{} {}]", s1, s2).into_bytes()
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
                let data_hash_hex: String =
                    hash_result.iter().map(|b| format!("{:02x}", b)).collect();
                let data_hash_bytes = data_hash_hex.as_bytes();

                let id_bytes = match identifier_mode {
                    FileIdentifierMode::Custom(bytes) => bytes.as_slice(),
                    _ => data_hash_bytes,
                };

                Some(Self::format_id_array(id_bytes, data_hash_bytes))
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
    #[allow(dead_code)]
    fn write_single_object<W: Write>(
        &self,
        obj: &mut dyn PdfObject,
        stream: &mut PdfStream<W>,
    ) -> std::io::Result<()> {
        obj.metadata_mut().offset = stream.pos;
        stream.write_line_latin1(&obj.indirect())
    }
}
//------------------------------ Legacy Strategy -----------------

pub(crate) struct LegacyStrategy;

impl WriteStrategy for LegacyStrategy {
    const VERSION: &[u8] = b"1.4";

    fn get_version(&self) -> &[u8] {
        Self::VERSION
    }

    // Uses default trait implementations for write_body and write_index
}

//------------------------ Compressed Strategy -----------------

use std::cell::RefCell;
use std::collections::HashMap;

pub(crate) struct CompressedStrategy {
    // Track which objects are in object streams: object_id -> (objstm_num, index)
    compression_map: RefCell<HashMap<usize, (usize, usize)>>,
    // Track object stream: (objstm_num, offset)
    objstm_info: RefCell<Option<(usize, usize)>>,
}

impl CompressedStrategy {
    pub fn new() -> Self {
        Self {
            compression_map: RefCell::new(HashMap::new()),
            objstm_info: RefCell::new(None),
        }
    }

    /// Write objects using object streams (PDF 1.5+)
    /// Compressible objects are grouped into an object stream
    /// Returns a map of (object_id -> (objstm_number, index_in_stream))
    fn write_body_compressed<W: Write>(
        &self,
        pdf: &mut PDF,
        stream: &mut PdfStream<W>,
    ) -> std::io::Result<HashMap<usize, (usize, usize)>> {
        use std::collections::HashMap;
        use std::rc::Rc;

        // Track which objects are compressed: object_id -> (objstm_num, index)
        let mut compression_map: HashMap<usize, (usize, usize)> = HashMap::new();

        // Collect object data for compression decision
        let catalog_id = pdf.catalog.metadata.object_identifier;

        let mut compressed_objects: Vec<(usize, String)> = Vec::new();

        // First pass: write non-compressible objects and collect compressible ones
        for obj in &mut pdf.objects {
            if obj.metadata().status == ObjectStatus::Free {
                continue;
            }

            let obj_id = obj.metadata().object_identifier;
            let is_catalog = obj_id == catalog_id;
            let is_object_zero = obj_id == Some(0);

            // Don't compress: object 0 (PDF spec), catalog, streams, or non-compressible objects
            if is_object_zero || is_catalog || !obj.is_compressible() {
                obj.metadata_mut().offset = stream.pos;
                stream.write_line_latin1(&obj.indirect())?;
            } else {
                // Save for compression
                compressed_objects.push((obj_id.unwrap_or(0), obj.data()));
            }
        }

        // If no objects to compress, return empty map
        if compressed_objects.is_empty() {
            return Ok(compression_map);
        }

        // Build object stream content
        // Format: N pairs of (obj_num offset) followed by actual object data
        let mut index_pairs = Vec::new();
        let mut content_parts = Vec::new();
        let mut current_offset = 0;

        // Allocate object ID for the object stream itself
        let obj_stream_num = pdf.allocate_object_id();

        for (index, (obj_num, data)) in compressed_objects.iter().enumerate() {
            index_pairs.push(format!("{} {}", obj_num, current_offset));
            content_parts.push(data.clone());
            current_offset += data.len() + 1; // +1 for newline separator

            // Track this compressed object
            compression_map.insert(*obj_num, (obj_stream_num, index));
        }

        let index_section = index_pairs.join(" ");
        let first_offset = index_section.len() + 1; // +1 for newline after index

        // Combine index and content
        let mut full_content = index_section;
        full_content.push('\n');
        for part in content_parts {
            full_content.push_str(&part);
            full_content.push('\n');
        }

        // Create object stream (reuse obj_stream_num allocated above)

        let extra_entries = vec![
            (
                "Type".to_string(),
                Rc::new(crate::NameObject::new(Some("ObjStm".to_string())))
                    as Rc<dyn PdfObject>,
            ),
            (
                "N".to_string(),
                Rc::new(crate::NumberObject::new(crate::NumberType::Integer(
                    compressed_objects.len() as i64,
                ))) as Rc<dyn PdfObject>,
            ),
            (
                "First".to_string(),
                Rc::new(crate::NumberObject::new(crate::NumberType::Integer(
                    first_offset as i64,
                ))) as Rc<dyn PdfObject>,
            ),
        ];

        let mut obj_stream = crate::StreamObject::compressed()
            .with_data(Some(vec![full_content.into_bytes()]), Some(extra_entries));

        obj_stream.metadata_mut().object_identifier = Some(obj_stream_num);
        let objstm_offset = stream.pos;
        obj_stream.metadata_mut().offset = objstm_offset;

        // Write the object stream
        stream.write_line_latin1(&obj_stream.indirect())?;

        // Store object stream info for xref
        *self.objstm_info.borrow_mut() = Some((obj_stream_num, objstm_offset));

        Ok(compression_map)
    }
}

impl Default for CompressedStrategy {
    fn default() -> Self {
        Self::new()
    }
}

impl WriteStrategy for CompressedStrategy {
    const VERSION: &[u8] = b"1.5";

    fn get_version(&self) -> &[u8] {
        Self::VERSION
    }

    fn write_body<W: Write>(
        &self,
        pdf: &mut PDF,
        stream: &mut PdfStream<W>,
    ) -> std::io::Result<()> {
        // Call the compressed version and store the compression map
        let map = self.write_body_compressed(pdf, stream)?;
        *self.compression_map.borrow_mut() = map;
        Ok(())
    }

    /// Write cross-reference stream instead of traditional xref table (PDF 1.5+)
    fn write_index<W: Write>(
        &self,
        pdf: &mut PDF,
        stream: &mut PdfStream<W>,
        _id_mode: &FileIdentifierMode,
    ) -> std::io::Result<()> {
        use crate::cross_ref::CrossRefStream;
        use std::collections::HashMap;

        pdf.xref_position = Some(stream.pos);

        let mut xref_stream = CrossRefStream::new();
        let mut entry_map: HashMap<usize, CrossRefEntry> = HashMap::new();

        let compression_map = self.compression_map.borrow();

        // Process all objects in pdf.objects
        for obj in &pdf.objects {
            let meta = obj.metadata();
            let obj_id = meta.object_identifier.unwrap_or(0);

            if obj_id == 0 {
                continue;
            }

            entry_map.insert(
                obj_id,
                CrossRefEntry::from_object_metadata(
                    meta.status,
                    meta.offset,
                    meta.generation_number.as_u16(),
                    compression_map.get(&obj_id).copied(),
                ),
            );
        }

        // Entry for object stream (if present)
        if let Some((num, offset)) = *self.objstm_info.borrow() {
            entry_map.insert(num, CrossRefEntry::Uncompressed { byte_offset: offset, generation: 0 });
        }

        // Allocate object number for the xref stream and add its entry
        let xref_stream_num = pdf.allocate_object_id();
        let xref_stream_offset = stream.pos;
        entry_map.insert(xref_stream_num, CrossRefEntry::Uncompressed { byte_offset: xref_stream_offset, generation: 0 });

        // Build xref_entries array in order: xref_entries[N] = entry for object N
        let max_obj_num = entry_map.keys().max().copied().unwrap_or(0);
        for obj_num in 1..=max_obj_num {
            if let Some(entry) = entry_map.get(&obj_num) {
                xref_stream.add_entry(entry.clone());
            } else {
                xref_stream.add_entry(CrossRefEntry::FreeObject { next_free_obj: 0, generation: 65535 });
            }
        }
        let info_obj_id = if !pdf.info.values.is_empty() {
            pdf.info.metadata.object_identifier
        } else {
            None
        };

        let xref_stream_bytes = xref_stream.build_stream_object(
            xref_stream_num,
            pdf.catalog.metadata.object_identifier.unwrap(),
            info_obj_id,
        );

        stream.output.write_all(&xref_stream_bytes)?;
        stream.output.write_all(b"\n")?;
        stream.pos += xref_stream_bytes.len() + 1;

        Ok(())
    }
}
