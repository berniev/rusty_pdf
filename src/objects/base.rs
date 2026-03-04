pub(crate) use crate::objects::metadata::PdfMetadata;
use crate::objects::pdf_object::PdfObject;
pub(crate) use crate::objects::status::ObjectStatus;

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
        self.metadata.generation == 0
    }
}

