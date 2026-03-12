use std::default::Default;
use std::rc::Rc;
use crate::NameObject;
use crate::objects::pdf_object::PdfObject;

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
pub struct DictionaryObject {
    pub values: Vec<(String, Rc<dyn PdfObject>)>,
}

impl DictionaryObject {

    pub fn new(values: Option<Vec<(String, Rc<dyn PdfObject>)>>) -> Self {
        Self {
            values: values.unwrap_or_default(),
        }
    }

    pub(crate) fn typed(name: &str) -> Self {
        Self::new(Some(vec![(
            "Type".to_string(),
            Rc::new(NameObject::new(name.to_string())),
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
        let ir = IndirectReference {metadata: Default::default(), id};
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

    pub fn get(&self, key: &str) -> Option<&Rc<dyn PdfObject>> {
        self.values.iter().find(|(k, _)| k == key).map(|(_, v)| v)
    }
}

impl PdfObject for DictionaryObject {

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

}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::NameObject;

    #[test]
    fn test_dictionary_methods() {
        let mut dict = DictionaryObject::new(None);
        assert!(dict.is_empty());
        assert_eq!(dict.len(), 0);

        dict.set("Key1", Rc::new(NameObject::new("Value1".to_string())));
        assert!(!dict.is_empty());
        assert_eq!(dict.len(), 1);
        assert!(dict.contains_key("Key1"));
        assert!(!dict.contains_key("Key2"));

        dict.set("Key2", Rc::new(NameObject::new("Value2".to_string())));
        assert_eq!(dict.len(), 2);
        assert!(dict.contains_key("Key2"));
    }
}
