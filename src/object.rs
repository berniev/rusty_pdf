use std::fmt;

/// PDF object status as specified in the xref table.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObjectStatus {
    Free,  // deleted or never used
    InUse, // normal, active object
}

impl ObjectStatus {
    /// Returns the PDF character representation ('f' or 'n') for xref table
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

#[derive(Debug, Clone)]
pub struct PdfMetadata {
    pub number: Option<usize>, // None for unassigned objects
    pub offset: usize, // used in xref table
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

pub trait PdfObject {
    fn metadata(&self) -> &PdfMetadata;
    fn metadata_mut(&mut self) -> &mut PdfMetadata;
    fn data(&self) -> Vec<u8>;
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any; // Downcast to Any for type checking

    fn indirect(&self) -> Vec<u8> {
        let meta = self.metadata();
        let number = meta.number.unwrap_or(0);
        let header = format!("{} {} obj\n", number, meta.generation);
        let mut result = header.into_bytes();
        result.extend(self.data());
        result.extend(b"\nendobj");
        result
    }

    fn reference(&self) -> Vec<u8> {
        let meta = self.metadata();
        let number = meta.number.unwrap_or(0);
        format!("{} {} R", number, meta.generation).into_bytes()
    }

    /// Whether the object can be included in an object stream (PDF 1.5+).
    ///
    /// PDF spec: Only generation 0 objects can be compressed in object streams.
    /// Objects with generation > 0 (incremental updates) must be written directly.
    ///
    /// Note: Some object types (like Stream) override this to always return false.
    fn is_compressible(&self) -> bool {
        self.metadata().generation == 0
    }
}

#[derive(Debug, Clone)]
pub struct BaseObject {
    pub metadata: PdfMetadata,
}

impl BaseObject {
    /// Creates the specific sentinel object required for PDF object 0.
    /// This ensures Object 0 is 'Free' and has the '65535' generation number.
    pub fn sentinel() -> Self {
        Self {
            metadata: PdfMetadata {
                generation: 65535,
                status: ObjectStatus::Free,
                ..PdfMetadata::default()
            }
        }
    }
}

impl PdfObject for BaseObject {
    fn metadata(&self) -> &PdfMetadata {
        &self.metadata
    }

    fn metadata_mut(&mut self) -> &mut PdfMetadata {
        &mut self.metadata
    }

    fn data(&self) -> Vec<u8> {
        Vec::new() // Base Object has no data - used for free/placeholder objects
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn is_compressible(&self) -> bool {
        self.metadata.generation == 0
    }
}

