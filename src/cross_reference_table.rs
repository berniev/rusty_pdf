use crate::PdfError;
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
///
pub use crate::generation::Generation;
pub use crate::objects::object_status::ObjectStatus;
use std::fs::File;
use std::io::{Seek, Write};
//--------------------------- CrossRefError -------------------------//

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CrossRefError {
    EmptyTable,
    InvalidRootEntry,
}

//--------------------------- Entry -------------------------//

pub struct CrossReferenceEntry {
    pub object_number: u64,
    pub object_status: ObjectStatus, // determines treatment of offset
    pub offset_or_next_free: u64,    // InUse: offset in stream. Free: next free object number
    pub generation: Generation,      // 65535 for root entry, otherwise 0
}

impl CrossReferenceEntry {
    pub fn new(number: u64, offset: u64, status: ObjectStatus, generation: Generation) -> Self {
        CrossReferenceEntry {
            object_number: number,
            object_status: status,
            offset_or_next_free: offset,
            generation,
        }
    }

    /// number: 10-digit number padded with leading zeros
    /// generation: 5-digit number padded with leading zeros
    /// status: n
    /// eol: 2-character end-of-line sequence
    pub fn serialise(&self) -> Vec<u8> {
        format!(
            "{:010} {:05} {}\r\n",
            self.offset_or_next_free, self.generation.as_u16(), self.object_status
        )
        .as_bytes()
        .to_vec()
    }
}

impl Default for CrossReferenceEntry {
    fn default() -> Self {
        Self::new(0, 0, ObjectStatus::Free, Generation::Root)
    }
}

//--------------------------- CrossRefTable -------------------------//

pub struct CrossRefTable {
    entries: Vec<CrossReferenceEntry>, // contiguous, ordered by object number
    pub(crate) xref_position: u64,
}

impl CrossRefTable {
    pub fn new() -> Self {
        let mut table = CrossRefTable {
            entries: Vec::new(),
            xref_position: 0,
        };
        table.add_entry(CrossReferenceEntry::default());

        table
    }

    pub fn add_entry(&mut self, entry: CrossReferenceEntry) {
        let idx = entry.object_number as usize;
        // Grow the table if needed to ensure slot exists
        while self.entries.len() <= entry.object_number as usize {
            self.entries.push(CrossReferenceEntry::default());
        }
        self.entries[idx] = entry;
    }

    pub fn serialise(&mut self, file: &mut File) -> Result<(), PdfError> {
        if self.entries.is_empty() {
            return Err(CrossRefError::EmptyTable.into());
        }

        self.entries.sort_by_key(|e| e.object_number);

        let first = self.entries.first().unwrap();
        if first.generation != Generation::Root || first.object_status != ObjectStatus::Free {
            return Err(CrossRefError::InvalidRootEntry.into());
        }

        self.xref_position = file.stream_position()?;

        let mut vec = format!("xref\r\n0 {}\r\n", self.entries.len())
            .as_bytes()
            .to_vec();

        for entry in &self.entries {
            vec.extend(entry.serialise());
        }

        file.write_all(&vec)?;

        Ok(())
    }
}
