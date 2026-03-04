use crate::objects::status::ObjectStatus;

#[derive(Debug, Clone, PartialEq)]
pub struct PdfMetadata {
    pub number: Option<usize>, // None for unassigned objects
    pub offset: usize,         // used in xref table
    pub status: ObjectStatus,

    /// PDF generation number.
    ///     0 = original/current version (standard for all new objects)
    /// 65535 = special value for the free object 0 (PDF spec requirement)
    ///     1+ = incremental updates (rarely used in modern PDFs)
    pub generation: u32,
}

impl Default for PdfMetadata {
    fn default() -> Self {
        PdfMetadata {
            number: None,
            offset: 0,
            status: ObjectStatus::InUse,
            generation: 0,
        }
    }
}

impl PdfMetadata {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn new_free() -> Self {
        PdfMetadata {
            status: ObjectStatus::Free,
            ..Default::default()
        }
    }

    /// Formats the metadata as a 19-character PDF xref entry string.
    /// Example: "0000000010 00000 n "
    pub fn format_xref_entry(&self) -> String {
        format!(
            "{:010} {:05} {} ",
            self.offset, self.generation, self.status
        )
    }
}
