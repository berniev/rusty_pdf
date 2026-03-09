use crate::objects::metadata::PdfMetadata;
use crate::PdfObject;

/// Spec:
/// null object:
///     The null object has a type and value that are unequal to those of any other object.
///     A single object of type null, denoted by the keyword 'null', and having a type and value
///     that are unequal to those of any other object
pub struct NullObject {
    pub metadata: PdfMetadata,
}

impl NullObject {
    pub fn new() -> Self {
        Self {
            metadata: PdfMetadata::default(),
        }
    }
}

impl PdfObject for NullObject {
    fn metadata(&self) -> &PdfMetadata {
        &self.metadata
    }

    fn metadata_mut(&mut self) -> &mut PdfMetadata {
        &mut self.metadata
    }

    fn data(&self) -> Vec<u8> {
        b"null".to_vec()
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn is_compressible(&self) -> bool {
        self.metadata.generation_number == 0
    }
}
