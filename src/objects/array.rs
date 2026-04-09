//! Array Objects:

///
/// An array object is a one-dimensional collection of pdf objects arranged sequentially.
///
/// Unlike arrays in many other computer languages, PDF arrays may be heterogeneous; that is, an
/// array’s elements may be any combination of numbers, strings, dictionaries, or any other pdf
/// objects, including other arrays. An array may have zero elements.
///
/// An array shall be written as a sequence of objects enclosed in SQUARE BRACKETS.
/// EXAMPLE [ 549 3.14 false ( Ralph ) /SomeName ]
///
use crate::{PdfError, PdfObject};

//--------------------------- PdfArrayObject --------------------------//

#[derive(Clone)]
pub struct PdfArrayObject {
    pub(crate) values: Vec<PdfObject>,
    pub(crate) object_number: Option<u64>,
    pub(crate) generation_number: Option<u16>,
}

impl PdfArrayObject {
    pub fn new() -> Self {
        Self {
            values: vec![],
            object_number: None,
            generation_number: None,
        }
    }

    pub fn push(&mut self, value: impl Into<PdfObject>) {
        self.values.push(value.into());
    }

    pub fn encode(&self) -> Result<Vec<u8>, PdfError> {
        let mut arr = vec![];
        arr.push(b'[');
        arr.push(b' ');
        for pdf_object in &self.values {
            arr.extend(pdf_object.encode_as_value()?);
            arr.push(b' ');
        }
        arr.push(b']');

        Ok(arr)
    }
}
