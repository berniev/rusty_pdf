use crate::objects::metadata::PdfMetadata;
use crate::objects::pdf_object::PdfObject;

pub struct DictionaryObject {
    pub metadata: PdfMetadata,
    pub values: Vec<(String, Box<dyn PdfObject>)>,
}

impl DictionaryObject {
    pub fn new(values: Vec<(String, Box<dyn PdfObject>)>) -> Self {
        Self {
            metadata: PdfMetadata::default(),
            values,
        }
    }
    pub fn reference(&self) -> Vec<u8> {
        let number = self.metadata.number.unwrap_or(0);
        format!("{} {} R", number, self.metadata.generation).into_bytes()
    }
}

impl PdfObject for DictionaryObject {
    fn metadata(&self) -> &PdfMetadata {
        &self.metadata
    }

    fn metadata_mut(&mut self) -> &mut PdfMetadata {
        &mut self.metadata
    }

    fn data(&self) -> Vec<u8> {
        let mut result = b"<<".to_vec();
        for (key, value) in &self.values {
            result.push(b'/');
            result.extend(key.as_bytes());
            result.push(b' ');
            result.extend(value.reference());
        }
        result.extend(b">>");
        result
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn is_compressible(&self) -> bool {
        self.metadata.generation == 0
    }
}
