use std::ops::Deref;
use crate::objects::pdf_object::Pdf;
/// Spec:
/// Dictionary:
///     An associative table containing pairs of objects, the first object being a name object
///     serving as the key and the second object serving as the value and may be any kind of object
///     including another dictionary.
/// Entries:
///     The entries in a dictionary represent an associative table and as such shall be unordered
///     even though an arbitrary order may be imposed upon them when written in a file. That
///     ordering shall be ignored.
///
///     Multiple entries in the same dictionary shall not have the same key.
///     A dictionary shall be written as a sequence of key-value pairs enclosed in double angle
///     brackets (<< … >>) (using LESS-THAN SIGNs (3Ch) and GREATER-THAN SIGNs (3Eh)).
///     The value of a Type entry shall be either defined in this standard or a registered name.
///         name "Type"    Opt
///         name "Subtype" Opt (requires Type)
///
///
use crate::{PdfNameObject, PdfObject};

//--------------------------- PdfDictionaryObject ----------------------//

pub struct PdfDictionaryObject {
    pub(crate) values: Vec<(PdfNameObject, Box<dyn PdfObject>)>,
}

impl PdfDictionaryObject {
    pub fn new() -> Self {
        Self { values: vec![] }
    }

    pub(crate) fn typed(mut self, name: &str) -> Self {
        self.add("Type", Pdf::name(name));

        self
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    pub fn contains_key(&self, key: &str) -> bool {
        self.values.iter().any(|(k, _)| k.value == key)
    }

    pub fn add(&mut self, key: &str, object: Box<dyn PdfObject>) {
        // todo: check key is not duplicate
        self.values.push((PdfNameObject::new(key), object));
    }
}

impl PdfObject for PdfDictionaryObject {
    fn serialise(&mut self) -> Vec<u8> {
        let mut arr = vec![];
        arr.extend(b"<<");
        for (pdf_name_obj, pdf_object) in &mut self.values {
            arr.extend(pdf_name_obj.serialise());
            arr.push(b' ');
            arr.extend(pdf_object.serialise());
        }
        arr.extend(b">>");

        arr
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::objects::pdf_object::Pdf;

    #[test]
    fn test_dictionary_methods() {
        let mut dict = PdfDictionaryObject::new();
        assert!(dict.is_empty());
        assert_eq!(dict.len(), 0);

        dict.add("Key1", Pdf::name("Value1"));
        assert!(!dict.is_empty());
        assert_eq!(dict.len(), 1);
        assert!(dict.contains_key("Key1"));
        assert!(!dict.contains_key("Key2"));

        dict.add("Key2", Pdf::name("Value2"));
        assert_eq!(dict.len(), 2);
        assert!(dict.contains_key("Key2"));
    }
}
