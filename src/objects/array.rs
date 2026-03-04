use crate::encoding::to_bytes_num;
use crate::objects::metadata::PdfMetadata;
use crate::objects::pdf_object::PdfObject;


pub struct ArrayObject {
    pub metadata: PdfMetadata,
    pub values: Vec<Box<dyn PdfObject>>,
}

impl ArrayObject {
    pub fn new(values: Vec<Box<dyn PdfObject>>) -> Self {
        Self {
            metadata: PdfMetadata::default(),
            values,
        }
    }
}

impl PdfObject for ArrayObject {
    fn metadata(&self) -> &PdfMetadata {
        &self.metadata
    }

    fn metadata_mut(&mut self) -> &mut PdfMetadata {
        &mut self.metadata
    }

    fn data(&self) -> Vec<u8> {
        let mut result = b"[".to_vec();
        for (i, item) in self.values.iter().enumerate() {
            if i > 0 { result.push(b' '); }
            result.extend(item.reference());
        }
        result.push(b']');
        result
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn is_compressible(&self) -> bool {
        self.metadata.generation == 0
    }
}

#[derive(Debug, Clone)]
pub struct Array {
    pub metadata: PdfMetadata,
    pub items: Vec<f64>,
}

impl Array {
    pub fn new(array: Option<Vec<f64>>) -> Self {
        Array {
            metadata: PdfMetadata::default(),
            items: array.unwrap_or_default(),
        }
    }
}

impl PdfObject for Array {
    fn metadata(&self) -> &PdfMetadata {
        &self.metadata
    }

    fn metadata_mut(&mut self) -> &mut PdfMetadata {
        &mut self.metadata
    }

    fn data(&self) -> Vec<u8> {
        let parts: Vec<Vec<u8>> = self.items.iter().map(|&item| to_bytes_num(item)).collect();
        let mut result = b"[".to_vec();
        result.extend(parts.join(&b' '));
        result.push(b']');
        result
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn is_compressible(&self) -> bool {
        self.metadata.generation == 0
    }
}
