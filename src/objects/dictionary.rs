use std::rc::Rc;

use crate::NameObject;
use crate::objects::metadata::PdfMetadata;
use crate::objects::pdf_object::PdfObject;
use crate::objects::base::IndirectReference;
//--------------------------- DictionaryObject----------------------//

/// Spec:
/// Dictionary:
///     An associative table containing pairs of objects, the first object being a name object
///     serving as the key and the second object serving as the value and may be any kind of object
///     including another dictionary.
/// Entrees:
///     The entries in a dictionary represent an associative table and as such shall be unordered
///     even though an arbitrary order may be imposed upon them when written in a file. That
///     ordering shall be ignored.
///     Multiple entries in the same dictionary shall not have the same key.
///     A dictionary shall be written as a sequence of key-value pairs enclosed in double angle
///     brackets (<< … >>) (using LESS-THAN SIGNs (3Ch) and GREATER-THAN SIGNs (3Eh)).
/// Resource Dictionary:
///     Associates resource names, used in content streams, with the resource objects themselves and
///     organized into various categories (e.g., Font, ColorSpace, Pattern)
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
        let number = self.metadata.object_number.unwrap_or(0);
        format!("{} {} R", number, self.metadata.generation_number).into_bytes()
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
        self.metadata.generation_number == 0
    }
}
