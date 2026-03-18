use std::default::Default;
use std::rc::Rc;

use crate::objects::metadata::PdfMetadata;
use crate::{IndirectObject, NameObject, PdfObject};

//--------------------------- DictionaryObject----------------------//

/// Spec:
/// Dictionary:
///     An associative table containing pairs of objects, the first object being a name object
///     serving as the key and the second object serving as the value and may be any kind of object
///     including another dictionary.
/// Entries:
///     The entries in a dictionary represent an associative table and as such shall be unordered
///     even though an arbitrary order may be imposed upon them when written in a file. That
///     ordering shall be ignored.
///     Multiple entries in the same dictionary shall not have the same key.
///     A dictionary shall be written as a sequence of key-value pairs enclosed in double angle
///     brackets (<< … >>) (using LESS-THAN SIGNs (3Ch) and GREATER-THAN SIGNs (3Eh)).
///     The value of a Type entry shall be either defined in this standard or a registered name.
///         name "Type"    Opt
///         name "Subtype" Opt (requires Type)
#[derive(Clone)]
pub struct DictionaryObject {
    pub(crate) metadata: PdfMetadata,
    pub values: Vec<(String, Rc<dyn PdfObject>)>,
}

impl DictionaryObject {
    pub fn new(values: Option<Vec<(String, Rc<dyn PdfObject>)>>) -> Self {
        Self {
            metadata: Default::default(),
            values: values.unwrap_or_default(),
        }
    }

    pub(crate) fn typed(name: &str) -> Self {
        Self::new(Some(vec![(
            "Type".to_string(),
            NameObject::build(name),
        )]))
    }

    pub fn set(&mut self, key: &str, value: Rc<dyn PdfObject>) {
        if let Some(pos) = self.values.iter().position(|(k, _)| k == key) {
            self.values[pos].1 = value;
        } else {
            self.values.push((key.to_string(), value));
        }
    }

    pub fn set_indirect(&mut self, key: &str, id: usize) {
        let ir = IndirectObject::new(Some(id));
        self.set(key, Rc::new(ir));
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    pub fn contains_key(&self, key: &str) -> bool {
        self.values.iter().any(|(k, _)| k == key)
    }

    pub fn set_name(&mut self, key: &str, name: &str) {
        self.set(key, Rc::new(crate::NameObject::new(Some(name.to_string()))));
    }

    pub fn set_string(&mut self, key: &str, value: String) {
        self.set(key, Rc::new(crate::StringObject::new(Some(value))));
    }

    pub fn set_number(&mut self, key: &str, value: impl Into<crate::NumberType>) {
        self.set(key, Rc::new(crate::NumberObject::new(value.into())));
    }

    pub fn set_array(&mut self, key: &str, array: crate::ArrayObject) {
        self.set(key, Rc::new(array));
    }

    pub fn set_dict(&mut self, key: &str, dict: DictionaryObject) {
        self.set(key, Rc::new(dict));
    }

    pub fn set_bool(&mut self, key: &str, value: bool) {
        self.set(key, Rc::new(crate::BooleanObject::new(Some(value))));
    }

    pub fn get(&self, key: &str) -> Option<&Rc<dyn PdfObject>> {
        self.values.iter().find(|(k, _)| k == key).map(|(_, v)| v)
    }
}

impl PdfObject for DictionaryObject {
    fn data(&self) -> String {
        format!(
            "<<{}>>",
            self.values
                .iter()
                .map(|(k, v)| {
                    // For objects with no identifier, embed them directly
                    // For objects with an identifier, use an indirect reference
                    if v.metadata().object_identifier.is_none() {
                        format!("/{} {}", k, v.data())
                    } else {
                        format!("/{} {}", k, v.reference())
                    }
                })
                .collect::<Vec<_>>()
                .join(" ")
        )
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dictionary_methods() {
        let mut dict = DictionaryObject::new(None);
        assert!(dict.is_empty());
        assert_eq!(dict.len(), 0);

        dict.set(
            "Key1",
            NameObject::build("Value1"),
        );
        assert!(!dict.is_empty());
        assert_eq!(dict.len(), 1);
        assert!(dict.contains_key("Key1"));
        assert!(!dict.contains_key("Key2"));

        dict.set(
            "Key2",
            NameObject::build("Value2"),
        );
        assert_eq!(dict.len(), 2);
        assert!(dict.contains_key("Key2"));
    }
}

