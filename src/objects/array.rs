use std::rc::Rc;
use crate::objects::base::IndirectReference;
use crate::objects::metadata::PdfMetadata;
use crate::objects::pdf_object::PdfObject;

pub struct ArrayObject {
    pub metadata: PdfMetadata,
    pub values: Vec<Rc<dyn PdfObject>>,
}

impl ArrayObject {
    pub fn new(values: Option<Vec<Rc<dyn PdfObject>>>) -> Self {
        Self {
            metadata: PdfMetadata::default(),
            values: values.unwrap_or_default(),
        }
    }

    pub fn push_indirect(&mut self, id: usize) {
        self.values.push(Rc::new(IndirectReference {
            metadata: PdfMetadata::default(),
            id,
        }));
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
            if i > 0 {
                result.push(b' ');
            }
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
