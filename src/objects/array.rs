//! Array Objects:
///     An array object is a one-dimensional collection of pdf objects arranged sequentially. Unlike
///     arrays in many other computer languages, PDF arrays may be heterogeneous; that is, an
///     array’s elements may be any combination of numbers, strings, dictionaries, or any other pdf
///     objects, including other arrays. An array may have zero elements.
///
/// Construction:
///     An array shall be written as a sequence of objects enclosed in SQUARE BRACKETS.
///     EXAMPLE [ 549 3.14 false ( Ralph ) /SomeName ]
///
use crate::{
    NumberType, PdfBooleanObject, PdfDictionaryObject, PdfIndirectObject, PdfNameObject,
    PdfNumberObject, PdfObject, PdfStringObject, action::FitDestination,
};

//-------------------PdfArrayObject ----------------------

pub struct PdfArrayObject {
    pub values: Vec<Box<dyn PdfObject>>,
}

impl PdfArrayObject {
    pub fn new() -> Self {
        Self { values: vec![] }
    }

    pub fn push_object(&mut self, value: Box<dyn PdfObject>) {
        self.values.push(value);
    }

    pub fn push_real(&mut self, value: f64) {
        self.push_number(NumberType::Real(value));
    }

    pub fn push_optional_real(&mut self, value: Option<f64>) {
        if let Some(v) = value {
            self.push_real(v);
        } else {
            self.push_name("null");
        }
    }

    pub fn push_bool(&mut self, value: bool) {
        self.push_object(PdfBooleanObject::new(value).boxed());
    }

    pub fn push_indirect(&mut self, id: usize) {
        self.push_object(PdfIndirectObject::new(id).boxed());
    }

    pub fn push_name(&mut self, name: &str) {
        self.push_object(PdfNameObject::new(name).boxed());
    }

    pub fn push_number(&mut self, value: impl Into<NumberType>) {
        self.push_object(PdfNumberObject::new(value.into()).boxed());
    }

    pub fn push_string(&mut self, value: String) {
        self.push_object(PdfStringObject::new(value).boxed());
    }

    pub fn push_pdf_dict(&mut self, dict: PdfDictionaryObject) {
        self.push_object(dict.boxed());
    }

    pub fn push_pdf_array(&mut self, array: PdfArrayObject) {
        self.push_object(array.boxed());
    }

    pub fn from_destination(dest: FitDestination) -> Self {
        dest.to_pdf_array()
    }
}

impl PdfObject for PdfArrayObject {
    fn data(&mut self) -> Vec<u8> {
        let mut arr = vec![];
        arr.push(b'[');
        arr.push(b' ');
        for pdf_object in &mut self.values {
            arr.extend(pdf_object.data());
            arr.push(b' ');
        }
        arr.push(b']');

        arr
    }
}
