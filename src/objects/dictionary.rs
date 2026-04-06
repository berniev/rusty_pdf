use crate::objects::pdf_object::PdfObj;
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
use crate::{PdfError, PdfNameObject, PdfObject};

//--------------------------- PdfDictionaryObject ----------------------//

#[derive(Clone)]
pub struct PdfDictionaryObject {
    pub(crate) values: Vec<(PdfNameObject, PdfObject)>,
    pub(crate) object_number: Option<u64>,
    pub(crate) generation_number: Option<u16>,
}

impl PdfDictionaryObject {
    pub fn new() -> Self {
        Self {
            values: vec![],
            object_number: None,
            generation_number: None,
        }
    }

    pub(crate) fn typed(mut self, name: &str) -> Self {
        self.add("Type", PdfObj::name(name));

        self
    }

    pub fn with_object_number(mut self, value: u64) -> Self {
        self.object_number = Some(value);
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

    pub fn get(&self, key: &str) -> Option<&PdfObject> {
        self.values
            .iter()
            .find_map(|(k, v)| if k.value == key { Some(v) } else { None })
    }

    pub fn get_integer(&self, key: &str) -> Option<i64> {
        match self.get(key) {
            Some(PdfObject::Number(n)) => Some(n.as_int()),
            _ => None,
        }
    }

    pub fn update(&mut self, key: &str, object: PdfObject) {
        if let Some((_, value)) = self.values.iter_mut().find(|(k, _)| k.value == key) {
            *value = object;
        } else {
            self.values.push((PdfNameObject::new(key), object));
        }
    }

    pub fn get_mut(&mut self, key: &str) -> Option<&mut PdfObject> {
        self.values
            .iter_mut()
            .find_map(|(k, v)| if k.value == key { Some(v) } else { None })
    }

    pub fn add(&mut self, key: &str, object: impl Into<PdfObject>){
        if self.contains_key(key) {
            PdfError::StructureError(format!("add: Duplicate key {} in dictionary", key));
        }
        self.values.push((PdfNameObject::new(key), object.into()));
    }

    pub fn serialise(&self) -> Result<Vec<u8>, PdfError> {
        let mut arr = vec![];
        arr.extend(b"<<");
        for (pdf_name_obj, pdf_object) in & self.values {
            arr.extend(pdf_name_obj.serialise()?);
            arr.push(b' ');
            arr.extend(pdf_object.serialise()?);
            arr.extend(b"\n");
        }
        arr.extend(b">>");

        Ok(arr)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::objects::pdf_object::PdfObj;

    #[test]
    fn test_dictionary_methods() {
        let mut dict = PdfDictionaryObject::new();
        assert!(dict.is_empty());
        assert_eq!(dict.len(), 0);

        dict.add("Key1", *Box::from(PdfObj::name("Value1")));
        assert!(!dict.is_empty());
        assert_eq!(dict.len(), 1);
        assert!(dict.contains_key("Key1"));
        assert!(!dict.contains_key("Key2"));

        dict.add("Key2", *Box::from(PdfObj::name("Value2")));
        assert_eq!(dict.len(), 2);
        assert!(dict.contains_key("Key2"));
    }
}
