use std::fmt;

//---------------- ObjectStatus -----------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObjectStatus {
    Free,  // deleted or never used
    InUse, // normal, active object
}

impl ObjectStatus {
    pub fn as_char(&self) -> char {
        match self {
            ObjectStatus::Free => 'f',
            ObjectStatus::InUse => 'n',
        }
    }
}

impl fmt::Display for ObjectStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_char())
    }
}

impl Default for ObjectStatus {
    fn default() -> Self {
        ObjectStatus::InUse
    }
}
enum Generation {
    Zero,
    OnePlus,
}

//---------------- PdfMetadata -----------------

/// PDF generation number:
///     0 = original/current version (standard for all new objects)
/// 65535 = special value for the free object 0 (PDF spec requirement)
///     1+ = incremental updates (rarely used in modern PDFs)
/// Spec:
///     Any object in a PDF file may be labelled as an indirect object. This gives the object a
///         unique object identifier by which other objects can refer to it (for example, as an
///         element of an array or as the value of a dictionary entry).
///     Object identifier shall consist of two parts:
///     • A positive integer object number. Indirect objects may be numbered sequentially
///         within a PDF file, but this is not required; object numbers may be assigned in any
///         arbitrary order.
///     • A non-negative integer generation number. In a newly created file, all indirect
///         objects shall have generation numbers of 0. Nonzero generation numbers may be
///         introduced when the file is later updated; see sub-clauses 7.5.4, "Cross-Reference
///         Table" and 7.5.6, "Incremental Updates."
///     Together, the combination of an object number and a generation number shall uniquely
///         identify an indirect object.
#[derive(Debug, Clone, PartialEq)]
pub struct PdfMetadata {
    pub object_number: Option<usize>, // None for unassigned objects
    pub generation_number: u32,
    pub offset: usize, // used in xref table
    pub status: ObjectStatus,
}

impl PdfMetadata {
    pub fn new() -> Self {
        PdfMetadata {
            object_number: None,
            generation_number: 0,
            offset: 0,
            status: ObjectStatus::InUse,
        }
    }

    pub fn new_free() -> Self {
        let mut pdf = PdfMetadata::new();
        pdf.status = ObjectStatus::Free;

        pdf
    }

    /// Formats the metadata as a 19-character PDF xref entry string.
    /// Example: "0000000010 00000 n "
    pub fn format_xref_entry(&self) -> String {
        format!(
            "{:010} {:05} {} ",
            self.offset, self.generation_number, self.status
        )
    }
}

impl Default for PdfMetadata {
    fn default() -> Self {
        Self::new()
    }
}
