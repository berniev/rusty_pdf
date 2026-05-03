use crate::object_ops::{ObjectNumber, PdfObject};
use crate::{PdfError};

#[derive(Clone)]
pub struct PdfArrayObject {
    pub(crate) object_number: Option<ObjectNumber>,
    pub(crate) generation_number: Option<u16>,
    pub(crate) values: Vec<PdfObject>,
}

impl PdfArrayObject {
    pub fn new() -> Self {
        Self {
            object_number: None,
            generation_number: None,
            values: vec![],
        }
    }

    pub fn from_vec(values: Vec<PdfObject>) -> PdfArrayObject {
        PdfArrayObject {
            object_number: None,
            generation_number: None,
            values,
        }
    }

    pub fn from_vec_u32(values: Vec<u32>) -> PdfArrayObject {
        PdfArrayObject {
            object_number: None,
            generation_number: None,
            values: values.into_iter().map(|v| v.into()).collect(),
        }
    }

    pub fn from_vec_f32(values: Vec<f32>) -> PdfArrayObject {
        PdfArrayObject {
            object_number: None,
            generation_number: None,
            values: values.into_iter().map(|v| v.into()).collect(),
        }
    }
    
    pub fn from_vec_f64(values: Vec<f64>) -> PdfArrayObject {
        PdfArrayObject {
            object_number: None,
            generation_number: None,
            values: values.into_iter().map(|v| v.into()).collect(),
        }
    }
    
    pub fn to_vec_f64(&self) -> Result<Vec<f64>, PdfError> {
        self.values.iter().map(|v| v.as_f64()).collect()
    }

    pub fn push(&mut self, value: impl Into<PdfObject>) {
        self.values.push(value.into());
    }

    pub fn encode(&self) -> Result<Vec<u8>, PdfError> {
        let mut arr = vec![];
        arr.push(b'[');
        arr.push(b' ');
        for pdf_object in &self.values {
            arr.extend(pdf_object.encode()?);
            arr.push(b' ');
        }
        arr.push(b']');

        Ok(arr)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::NumberType;
    use crate::objects::pdf_boolean::PdfBooleanObject;
    use crate::objects::pdf_name::PdfNameObject;
    use crate::objects::pdf_number::PdfNumberObject;
    use crate::objects::pdf_reference::PdfReferenceObject;

    #[test]
    fn encode_empty_array() {
        let arr = PdfArrayObject::new();
        assert_eq!(arr.encode().unwrap(), b"[ ]");
    }

    #[test]
    fn encode_single_element() {
        let mut arr = PdfArrayObject::new();
        arr.push(42);
        assert_eq!(arr.encode().unwrap(), b"[ 42 ]");
    }

    #[test]
    fn encode_mixed_elements() {
        let mut arr = PdfArrayObject::new();
        arr.push(549);
        arr.push(3.14);
        arr.push(false);
        assert_eq!(arr.encode().unwrap(), b"[ 549 3.14 false ]");
    }

    #[test]
    fn encode_with_name() {
        let mut arr = PdfArrayObject::new();
        arr.push(PdfObject::name_obj("SomeName"));
        assert_eq!(arr.encode().unwrap(), b"[ /SomeName ]");
    }

    #[test]
    fn encode_with_indirect_reference() {
        let mut arr = PdfArrayObject::new();
        arr.push(PdfReferenceObject::new(ObjectNumber::new(10)));
        assert_eq!(arr.encode().unwrap(), b"[ 10 0 R  ]");
    }
}
