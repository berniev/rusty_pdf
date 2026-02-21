use std::collections::HashMap;
use crate::object::{PdfObject, PdfMetadata};

#[derive(Clone, Debug)]
pub struct Dictionary {
    pub metadata: PdfMetadata,
    pub values: HashMap<String, Vec<u8>>,
}

impl Dictionary {
    pub fn new(values: Option<HashMap<String, Vec<u8>>>) -> Self {
        Dictionary {
            metadata: PdfMetadata::default(),
            values: values.unwrap_or_default(),
        }
    }

    pub fn reference(&self) -> Vec<u8> {
        let number = self.metadata.number.unwrap_or(0);
        format!("{} {} R", number, self.metadata.generation).into_bytes()
    }
}

impl PdfObject for Dictionary {
    fn metadata(&self) -> &PdfMetadata {
        &self.metadata
    }

    fn metadata_mut(&mut self) -> &mut PdfMetadata {
        &mut self.metadata
    }

    fn data(&self) -> Vec<u8> {
        let mut result = Vec::new();
        result.extend(b"<<");
        for (key, value) in &self.values {
            result.push(b'/');
            result.extend(key.as_bytes());
            result.push(b' ');
            result.extend(value);
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
