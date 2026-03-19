use crate::PdfMetadata;
use crate::PdfObject;
use crate::objects::metadata::Generation;

pub struct BooleanObject {
    pub metadata: PdfMetadata,
    pub value: Option<bool>,
}

impl BooleanObject {
    pub fn new(value: Option<bool>) -> Self {
        Self {
            metadata: PdfMetadata::default(),
            value,
        }
    }
    pub fn set(&mut self, value: bool) {
        self.value = Some(value);
    }

    pub fn make_pdf_obj(value: bool) -> std::rc::Rc<dyn PdfObject> {
        std::rc::Rc::new(Self::new(Some(value)))
    }
}

impl PdfObject for BooleanObject {
    fn data(&self) -> String {
        if self.value.unwrap_or(false) {
            "true".to_string()
        } else {
            "false".to_string()
        }
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn metadata(&self) -> &PdfMetadata {
        &self.metadata
    }

    fn metadata_mut(&mut self) -> &mut PdfMetadata {
        &mut self.metadata
    }

    fn is_compressible(&self) -> bool {
        self.metadata.generation_number == Generation::Normal
    }
}
