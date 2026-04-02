//! Array Objects:
///
/// An array object is a one-dimensional collection of pdf objects arranged sequentially.
///
/// Unlike arrays in many other computer languages, PDF arrays may be heterogeneous; that is, an
/// array’s elements may be any combination of numbers, strings, dictionaries, or any other pdf
/// objects, including other arrays. An array may have zero elements.
///
/// Construction:
///
/// An array shall be written as a sequence of objects enclosed in SQUARE BRACKETS.
/// EXAMPLE [ 549 3.14 false ( Ralph ) /SomeName ]
///
use crate::{PdfError, PdfObject};

//--------------------------- PdfArrayObject --------------------------//

#[derive(Clone)]
pub struct PdfArrayObject {
    pub values: Vec<PdfObject>,
    pub object_number: Option<u64>,
}

impl PdfArrayObject {
    pub fn new() -> Self {
        Self {
            values: vec![],
            object_number: None,
        }
    }

    pub fn with_object_number(mut self, value: u64) -> Self {
        self.object_number = Some(value);
        self
    }
    
    pub fn push(&mut self, value: PdfObject) {
        self.values.push(value);
    }

    pub fn serialise(&mut self) -> Result<Vec<u8>, PdfError> {
        let mut arr = vec![];
        arr.push(b'[');
        arr.push(b' ');
        for pdf_object in &mut self.values {
            arr.extend(pdf_object.serialise()?);
            arr.push(b' ');
        }
        arr.push(b']');

        Ok(arr)
    }
}
