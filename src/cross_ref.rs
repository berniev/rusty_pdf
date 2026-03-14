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
pub(crate) enum CrossRefEntry {
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

    pub fn add_free_object(&mut self, next_free_obj: usize, generation: u16) {
        self.entries.push(CrossRefEntry::FreeObject {
            next_free_obj,
            generation,
        });
    }

    pub fn add_uncompressed(&mut self, byte_offset: usize, generation: u16) {
        self.entries.push(CrossRefEntry::Uncompressed {
            byte_offset,
            generation,
        });
    }

    pub fn add_compressed(&mut self, objstm_number: usize, index_within_objstm: u16) {
        self.entries.push(CrossRefEntry::CompressedInObjstm {
            objstm_number,
            index_within_objstm,
        });
    }

    pub fn build_binary_data(&self, field2_width: usize, field3_width: usize) -> Vec<u8> {
        let mut data = Vec::new();
        for entry in &self.entries {
            let (type_byte, field2, field3) = match entry {
                CrossRefEntry::FreeObject { next_free_obj, generation } => {
                    (0u8, *next_free_obj, *generation)
                }
                CrossRefEntry::Uncompressed { byte_offset, generation } => {
                    (1u8, *byte_offset, *generation)
                }
                CrossRefEntry::CompressedInObjstm { objstm_number, index_within_objstm } => {
                    (2u8, *objstm_number, *index_within_objstm)
                }
            };

            data.push(type_byte);

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
}
