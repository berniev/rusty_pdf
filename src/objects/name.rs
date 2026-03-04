use crate::objects::metadata::PdfMetadata;
use crate::PdfObject;

pub struct NameObject {
    pub metadata: PdfMetadata,
    pub value: String,
}

impl NameObject {
    pub fn new(value: String) -> Self {
        Self {
            metadata: PdfMetadata::default(),
            value,
        }
    }
}

impl PdfObject for NameObject {
    fn metadata(&self) -> &PdfMetadata {
        &self.metadata
    }

    fn metadata_mut(&mut self) -> &mut PdfMetadata {
        &mut self.metadata
    }

    fn data(&self) -> Vec<u8> {
        let mut result = b"/".to_vec();
        result.extend(self.value.as_bytes());
        result
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn is_compressible(&self) -> bool {
        self.metadata.generation == 0
    }
}
