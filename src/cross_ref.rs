pub use crate::objects::metadata::{Generation, ObjectStatus};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CrossRefError {
    EmptyTable,
    InvalidRootEntry,
}

/// 7.5.4 Cross-Reference Table
/// The cross-reference table contains information that permits random access to indirect objects
/// within the file so that the entire file need not be read to locate any particular object. The
/// table shall contain a one-line entry for each indirect object, specifying the byte offset of
/// that object within the body of the file. (Beginning with PDF 1.5, some or all of the
/// cross-reference information may alternatively be contained in cross-reference streams.
///
/// The table comprises one or more cross-reference sections. Initially, the entire table consists
/// of a single section (or two sections if the file is linearized; see Annex F). One additional
/// section shall be added each time the file is incrementally updated.
///
/// Each cross-reference section shall begin with a line containing the keyword xref. Following
/// this line shall be one or more cross-reference subsections, which may appear in any order.
///
/// For a file that has never been incrementally updated, the cross-reference section shall contain
/// only one subsection, whose object numbering begins at 0.
///
/// We are not designing for modification.
pub struct Entry {
    object_number: u32,
    object_status: ObjectStatus, // determines treatment of offset
    offset_or_next_free: u64,    // InUse: offset in stream. Free: next free object number
    generation: Generation,      // 65535 for root entry, otherwise 0
}

impl Entry {
    pub fn new(number: u32, in_use: ObjectStatus, offset: u64, generation: Generation) -> Self {
        Entry {
            object_number: number,
            object_status: in_use,
            offset_or_next_free: offset,
            generation,
        }
    }

    /// number: 10-digit number padded with leading zeros
    /// generation: 5-digit number padded with leading zeros
    /// status: n
    /// eol: 2-character end-of-line sequence
    pub fn as_pdf(&self) -> String {
        format!(
            "{:010} {:05} {} \r\n",
            self.offset_or_next_free,
            self.generation.as_u16(),
            self.object_status.as_char()
        )
    }
}

pub struct CrossRefTable {
    entries: Vec<Entry>, // contiguous, order by object number
}

impl Default for CrossRefTable {
    fn default() -> Self {
        Self::new()
    }
}

impl CrossRefTable {
    pub fn new() -> Self {
        let mut table = CrossRefTable {
            entries: Vec::new(),
        };
        let root = Entry::new(0, ObjectStatus::Free, 0, Generation::Root);
        table.add_entry(root);

        table
    }

    pub fn add_entry(&mut self, entry: Entry) {
        self.entries.push(entry);
    }

    pub fn as_pdf(&self) -> Result<String, CrossRefError> {
        if self.entries.is_empty() {
            return Err(CrossRefError::EmptyTable);
        }

        let first = self.entries.first().unwrap();

        if first.generation != Generation::Root || first.object_status != ObjectStatus::Free {
            return Err(CrossRefError::InvalidRootEntry);
        }

        let head = format!("xref\r\n{} {}\r\n", first.object_number, self.entries.len());

        Ok(head
            + &self
                .entries
                .iter()
                .map(|entry| entry.as_pdf())
                .collect::<String>())
    }
}

/// Cross-Reference Streams
/// Beginning with PDF 1.5, cross-reference information may be stored in a cross-reference stream
/// instead of in a cross-reference table. Cross-reference streams provide the following advantages:
/// • More compact representation of cross-reference information
/// • Ability to access compressed objects that are stored in object streams (see 7.5.7,
///   "Object Streams") and to allow new cross-reference entry types to be added in the future.
/// 
/// Cross-reference streams are stream objects (see 7.3.8, "Stream Objects"), and contain a
/// dictionary and a data stream. 
/// 
/// Each cross-reference stream contains the information equivalent
/// to the cross-reference table (see 7.5.4, "Cross-Reference Table") and trailer (see 7.5.5, "File
/// Trailer") for one cross-reference section.
///
/// Entry types:
///  Type                Num  Field2         Field3
///  Free object           0  next free obj  generation (65535 for object 0)
///  Uncompressed          1  byte offset    generation
///  Compressed in objstm  2  objstm number  index within objstm
///

#[derive(Clone)]
pub enum CrossRefEntry {
    FreeObject {
        next_free_obj: usize,
        generation: u16,
    },
    Uncompressed {
        byte_offset: usize,
        generation: u16,
    },
    CompressedInObjstm {
        objstm_number: usize,
        index_within_objstm: u16,
    },
}

impl CrossRefEntry {
    /// Construct a CrossRefEntry from object metadata and optional compression info
    pub fn from_object_metadata(
        status: ObjectStatus,
        offset: usize,
        generation: u16,
        compression_info: Option<(usize, usize)>,
    ) -> Self {
        if status == ObjectStatus::Free {
            CrossRefEntry::FreeObject {
                next_free_obj: 0,
                generation: 65535,
            }
        } else if let Some((objstm_num, index)) = compression_info {
            CrossRefEntry::CompressedInObjstm {
                objstm_number: objstm_num,
                index_within_objstm: index as u16,
            }
        } else {
            CrossRefEntry::Uncompressed {
                byte_offset: offset,
                generation,
            }
        }
    }

    /// Calculate minimum bytes needed to represent a usize value
    fn bytes_needed_usize(value: usize) -> usize {
        if value == 0 {
            return 1;
        }
        // Calculate ceil(log256(val)) = number of bytes needed
        let bits = usize::BITS - value.leading_zeros();
        ((bits + 7) / 8) as usize
    }

    /// Calculate minimum bytes needed to represent a u16 value
    fn bytes_needed_u16(value: u16) -> usize {
        if value == 0 {
            return 1;
        }
        // Calculate ceil(log256(val)) = number of bytes needed
        let bits = 16 - value.leading_zeros();
        ((bits + 7) / 8) as usize
    }

    /// Returns the type byte for this cross-reference entry according to PDF spec
    pub fn type_byte(&self) -> u8 {
        match self {
            CrossRefEntry::FreeObject { .. } => 0,
            CrossRefEntry::Uncompressed { .. } => 1,
            CrossRefEntry::CompressedInObjstm { .. } => 2,
        }
    }

    /// Returns minimum (field2_width, field3_width) needed for this entry
    pub fn required_widths(&self) -> (usize, usize) {
        match self {
            CrossRefEntry::FreeObject { next_free_obj, generation } => {
                (Self::bytes_needed_usize(*next_free_obj), Self::bytes_needed_u16(*generation))
            }
            CrossRefEntry::Uncompressed { byte_offset, generation } => {
                (Self::bytes_needed_usize(*byte_offset), Self::bytes_needed_u16(*generation))
            }
            CrossRefEntry::CompressedInObjstm { objstm_number, index_within_objstm } => {
                (Self::bytes_needed_usize(*objstm_number), Self::bytes_needed_u16(*index_within_objstm))
            }
        }
    }
}

pub(crate) struct CrossRefStream {
    entries: Vec<CrossRefEntry>,
}
impl CrossRefStream {
    pub fn new() -> Self {
        let mut stream = CrossRefStream {
            entries: Vec::new(),
        };
        stream.entries.push(CrossRefEntry::FreeObject {
            next_free_obj: 0,
            generation: 65535,
        });
        stream
    }


    pub fn add_entry(&mut self, entry: CrossRefEntry) {
        self.entries.push(entry);
    }

    /// Calculate optimal field widths based on actual entry data
    /// Returns (field2_width, field3_width)
    pub fn calculate_optimal_widths(&self) -> (usize, usize) {
        self.entries
            .iter()
            .map(|entry| entry.required_widths())
            .fold((1, 1), |(max2, max3), (w2, w3)| {
                (max2.max(w2), max3.max(w3))
            })
    }

    pub fn build_binary_data(&self, field2_width: usize, field3_width: usize) -> Vec<u8> {
        let mut data = Vec::new();
        for entry in &self.entries {
            data.push(entry.type_byte());

            let (field2, field3) = match entry {
                CrossRefEntry::FreeObject { next_free_obj, generation } => {
                    (*next_free_obj, *generation)
                }
                CrossRefEntry::Uncompressed { byte_offset, generation } => {
                    (*byte_offset, *generation)
                }
                CrossRefEntry::CompressedInObjstm { objstm_number, index_within_objstm } => {
                    (*objstm_number, *index_within_objstm)
                }
            };

            // Encode field2 in big-endian
            let field2_bytes = field2.to_be_bytes();
            data.extend_from_slice(&field2_bytes[8 - field2_width..]);

            // Encode field3 in big-endian
            let field3_bytes = field3.to_be_bytes();
            data.extend_from_slice(&field3_bytes[2 - field3_width..]);
        }
        data
    }

    pub fn entry_count(&self) -> usize {
        self.entries.len()
    }

    /// Build the complete xref stream object with dictionary and binary data
    /// Returns (stream_bytes, xref_data_length) for writing to PDF
    pub fn build_stream_object(
        &self,
        stream_num: usize,
        root_obj_id: usize,
        info_obj_id: Option<usize>,
    ) -> Vec<u8> {
        let (field2_width, field3_width) = self.calculate_optimal_widths();
        let xref_data = self.build_binary_data(field2_width, field3_width);

        let w_array_str = format!("[ 1 {} {} ]", field2_width, field3_width);
        let total_entries = self.entry_count();

        let mut dict_entries = vec![
            "/Type /XRef".to_string(),
            format!("/Size {}", total_entries),
            format!("/Root {} 0 R", root_obj_id),
            format!("/W {}", w_array_str),
        ];

        if let Some(info_id) = info_obj_id {
            dict_entries.push(format!("/Info {} 0 R", info_id));
        }

        dict_entries.push(format!("/Length {}", xref_data.len()));

        let dict_str = format!("<< {} >>", dict_entries.join(" "));
        let mut stream_bytes = Vec::new();
        stream_bytes.extend_from_slice(
            format!("{} 0 obj\n{}\nstream\n", stream_num, dict_str).as_bytes()
        );
        stream_bytes.extend_from_slice(&xref_data);
        stream_bytes.extend_from_slice(b"\nendstream\nendobj");

        stream_bytes
    }
}
