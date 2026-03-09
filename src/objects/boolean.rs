use crate::objects::metadata::PdfMetadata;
use crate::objects::pdf_object::PdfObject;

pub struct BooleanObject {
    pub metadata: PdfMetadata,
    pub value: bool,
}

impl BooleanObject {
    pub fn new(value: bool) -> Self {
        Self {
            metadata: PdfMetadata::default(),
            value,
        }
    }
}

impl PdfObject for BooleanObject {
    fn metadata(&self) -> &PdfMetadata {
        &self.metadata
    }

    fn metadata_mut(&mut self) -> &mut PdfMetadata {
        &mut self.metadata
    }

    fn data(&self) -> Vec<u8> {
        if self.value { b"true".to_vec() } else { b"false".to_vec() }
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn is_compressible(&self) -> bool {
        self.metadata.generation_number == 0
    }
}

