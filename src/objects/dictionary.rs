use std::rc::Rc;
use crate::objects::metadata::PdfMetadata;
use crate::objects::pdf_object::PdfObject;
use crate::{ArrayObject, NameObject, NumberObject, NumberType};
use crate::objects::base::IndirectReference;
//--------------------------- DictionaryObject----------------------//

pub struct DictionaryObject {
    pub metadata: PdfMetadata,
    pub values: Vec<(String, Rc<dyn PdfObject>)>,
}

impl DictionaryObject {
    pub fn new(values: Option<Vec<(String, Rc<dyn PdfObject>)>>) -> Self {
        Self {
            metadata: PdfMetadata::default(),
            values: values.unwrap_or_default(),
        }
    }

    pub(crate) fn typed(name: &str) -> Self {
        Self::new(Some(vec![(
            "Type".to_string(),
            Rc::new(NameObject::new(name.to_string())),
        )]))
    }

    pub fn reference(&self) -> Vec<u8> {
        let number = self.metadata.number.unwrap_or(0);
        format!("{} {} R", number, self.metadata.generation).into_bytes()
    }
    
    pub fn set(&mut self, key: &str, value: Rc<dyn PdfObject>) {
        if let Some(pos) = self.values.iter().position(|(k, _)| k == key) {
            self.values[pos].1 = value;
        } else {
            self.values.push((key.to_string(), value));
        }
    }

    pub fn set_indirect(&mut self, key: &str, id: usize) {
        self.set(key, Rc::new(IndirectReference {
            metadata: Default::default(),
            id,
        }));
    }

    pub fn get(&self, key: &str) -> Option<&Rc<dyn PdfObject>> {
        self.values.iter().find(|(k, _)| k == key).map(|(_, v)| v)
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
