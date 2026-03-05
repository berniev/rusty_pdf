use crate::objects::metadata::PdfMetadata;
use crate::objects::pdf_object::PdfObject;
use crate::{ArrayObject, NameObject, NumberObject, NumberType};

//--------------------------- DictionaryObject----------------------//

pub struct DictionaryObject {
    pub metadata: PdfMetadata,
    pub values: Vec<(String, Box<dyn PdfObject>)>,
}

impl DictionaryObject {
    pub fn new(values: Option<Vec<(String, Box<dyn PdfObject>)>>) -> Self {
        Self {
            metadata: PdfMetadata::default(),
            values: values.unwrap_or_default(),
        }
    }

    fn typed(name: &str) -> Self {
        Self::new(Some(vec![(
            "Type".to_string(),
            Box::new(NameObject::new(name.to_string())),
        )]))
    }
    pub fn catalog() -> Self {
        Self::typed("Catalog")
    }

    pub fn pages_tree() -> Self {
        let mut dict = Self::typed("Pages");
        dict.values
            .push(("Kids".to_string(), Box::new(ArrayObject::new(None))));
        dict.values
            .push(("Count".to_string(), Box::new(NumberObject::new(NumberType::from(0.0)))));
        dict
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
