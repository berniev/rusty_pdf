use crate::cross_reference_table::CrossRefTable;
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
use std::fs::File;

#[derive(Clone)]
pub struct PdfDictionaryObject {
    pub(crate) values: Vec<(PdfNameObject, PdfObject)>,
    pub(crate) object_number: Option<u64>,
    pub(crate) generation_number: Option<u16>,
    pub(crate) children: Vec<Box<PdfDictionaryObject>>, // for page tree
}

impl PdfDictionaryObject {
    pub fn new() -> Self {
        Self {
            values: vec![],
            object_number: None,
            generation_number: None,
            children: vec![],
        }
    }

    pub(crate) fn typed(mut self, name: &str) -> Result<Self,PdfError>  {
        self.add("Type", PdfObj::make_name_obj(name))?;

        Ok(self)
    }

    pub fn with_object_number(mut self, value: u64) -> Self {
        self.object_number = Some(value);
        self
    }

    pub fn with_generation_number(mut self, value: u16) -> Self {
        self.generation_number = Some(value);
        self
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    pub fn contains_key(&self, key: &str) -> bool {
        self.get(key).is_some()
    }

    pub fn get(&self, key: &str) -> Option<&PdfObject> {
        self.values
            .iter()
            .find_map(|(k, v)| if k.value == key { Some(v) } else { None })
    }

    pub fn get_dict(&self, key: &str) -> Option<&PdfDictionaryObject> {
        match self.get(key) {
            Some(PdfObject::Dictionary(d)) => Some(d),
            _ => None,
        }
    }

    fn get_mut(&mut self, key: &str) -> Option<&mut PdfObject> {
        self.values
            .iter_mut()
            .find_map(|(k, v)| if k.value == key { Some(v) } else { None })
    }

    // special case for page tree todo: move to page_tree.rs ??
    pub fn add_kid_to_page_tree(
        &mut self,
        kid_obj: Box<PdfDictionaryObject>,
    ) -> Result<(), PdfError> {
        let reference = PdfObj::make_reference_obj(kid_obj.object_number.unwrap());
        self.children.push(kid_obj);

        if let Some(PdfObject::Array(arr)) = self.get_mut("Kids") {
            arr.push(reference);
            Ok(())
        } else {
            Err(PdfError::StructureError("Missing `Kids` array".to_string()))
        }
    }

    pub fn push_to_array(
        &mut self,
        key: &str,
        object: impl Into<PdfObject>,
    ) -> Result<(), PdfError> {
        if let Some(PdfObject::Array(arr)) = self.get_mut(key) {
            arr.push(object);
            Ok(())
        } else {
            Err(PdfError::StructureError(format!(
                "Key '{}' is not an array",
                key
            )))
        }
    }

    pub fn get_integer(&self, key: &str) -> Option<i64> {
        match self.get(key) {
            Some(PdfObject::Number(n)) => Some(n.as_int()),
            _ => None,
        }
    }

    pub fn update_or_add(&mut self, key: &str, object: impl Into<PdfObject>) {
        if let Some((_, value)) = self.values.iter_mut().find(|(k, _)| k.value == key) {
            *value = object.into();
        } else {
            self.values.push((PdfNameObject::new(key), object.into()));
        }
    }

    pub fn add(&mut self, key: &str, object: impl Into<PdfObject>) -> Result<(), PdfError> {
        if self.contains_key(key) {
            return Err(PdfError::StructureError(format!(
                "add: Attempt to make duplicate key {} in dictionary",
                key
            )));
        }
        self.values.push((PdfNameObject::new(key), object.into()));

        Ok(())
    }

    pub fn serialise(&self, xref: &mut CrossRefTable, file: &mut File) -> Result<(), PdfError> {
        let tree_obj = PdfObject::from(self.clone());
        tree_obj.serialise(xref, file)?;

        // serialise any indirect values (e.g. streams embedded in this dict)
        for (_name, value) in &self.values {
            if value.get_object_number().is_some() {
                value.serialise(xref, file)?;
            }
        }

        for child in &self.children {
            child.serialise(xref, file)?;
        }

        Ok(())
    }

    pub fn encode(&self) -> Result<Vec<u8>, PdfError> {
        let mut arr = vec![];
        arr.extend(b"<<\n");
        for (pdf_name_obj, pdf_object) in &self.values {
            arr.extend(pdf_name_obj.encode()?);
            arr.push(b' ');
            arr.extend(pdf_object.encode_as_value()?);
            arr.extend(b"\n");
        }
        arr.extend(b">>\n");

        Ok(arr)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::PdfBooleanObject;
    use crate::objects::pdf_object::PdfObj;

    #[test]
    fn test_dictionary_methods() {
        let mut dict = PdfDictionaryObject::new();
        assert!(dict.is_empty());
        assert_eq!(dict.len(), 0);

        dict.add("Key1", *Box::from(PdfObj::make_name_obj("Value1"))).expect("fail");
        assert!(!dict.is_empty());
        assert_eq!(dict.len(), 1);
        assert!(dict.contains_key("Key1"));
        assert!(!dict.contains_key("Key2"));

        dict.add("Key2", *Box::from(PdfObj::make_name_obj("Value2"))).expect("fail");
        assert_eq!(dict.len(), 2);
        assert!(dict.contains_key("Key2"));
    }

    #[test]
    fn encode_empty_dictionary() {
        let dict = PdfDictionaryObject::new();
        assert_eq!(dict.encode().unwrap(), b"<<\n>>\n");
    }

    #[test]
    fn encode_single_entry() {
        let mut dict = PdfDictionaryObject::new();
        dict.add("Type", PdfObj::make_name_obj("Catalog")).expect("fail");
        let output = String::from_utf8(dict.encode().unwrap()).unwrap();
        assert!(output.starts_with("<<\n"));
        assert!(output.contains("/Type /Catalog"));
        assert!(output.ends_with(">>\n"));
    }

    #[test]
    fn encode_multiple_entries() {
        let mut dict = PdfDictionaryObject::new();
        dict.add("Type", PdfObj::make_name_obj("Page")).expect("fail");
        dict.add("Count", PdfObj::make_num_obj(3i64)).expect("fail");
        let output = String::from_utf8(dict.encode().unwrap()).unwrap();
        assert!(output.contains("/Type /Page"));
        assert!(output.contains("/Count 3"));
    }

    #[test]
    fn encode_with_boolean_value() {
        let mut dict = PdfDictionaryObject::new();
        dict.add("Visible", PdfBooleanObject::new(true)).expect("fail");
        let output = String::from_utf8(dict.encode().unwrap()).unwrap();
        assert!(output.contains("/Visible true"));
    }

    #[test]
    fn encode_with_indirect_reference() {
        let mut dict = PdfDictionaryObject::new();
        dict.add("Pages", PdfObj::make_reference_obj(2)).expect("fail");
        let output = String::from_utf8(dict.encode().unwrap()).unwrap();
        assert!(output.contains("/Pages 2 0 R"));
    }
}
