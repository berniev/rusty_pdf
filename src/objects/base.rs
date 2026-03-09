use crate::objects::metadata::ObjectStatus;
pub(crate) use crate::objects::metadata::PdfMetadata;
use crate::objects::pdf_object::PdfObject;

//---------------- IndirectReference -----------------

/// Spec:
/// Indirect object:
///     an object that is labeled with a positive integer object number followed by a non-negative
///     integer generation number followed by 'obj' and having 'endobj' after it
/// Direct object:
///     any object that has not been made into an indirect object
pub struct IndirectReference {
    pub metadata: PdfMetadata,
    pub id: usize,
}

impl PdfObject for IndirectReference {
    fn metadata(&self) -> &PdfMetadata {
        &self.metadata
    }

    fn metadata_mut(&mut self) -> &mut PdfMetadata {
        &mut self.metadata
    }

    fn data(&self) -> Vec<u8> {
        format!("{} 0 R", self.id).into_bytes()
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

//------------------- BaseObject ---------------------

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
                generation_number: 65535,
                status: ObjectStatus::Free,
                ..PdfMetadata::default()
            },
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
        self.metadata.generation_number == 0
    }
}
