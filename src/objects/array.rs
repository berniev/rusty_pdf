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
use crate::PdfObject;

//--------------------------- PdfArrayObject --------------------------//

pub struct PdfArrayObject {
    pub values: Vec<Box<dyn PdfObject>>,
}

impl PdfArrayObject {
    pub fn new() -> Self {
        Self { values: vec![] }
    }

    pub fn push(&mut self, value: Box<dyn PdfObject>) {
        self.values.push(value);
    }

    pub fn push_vec(&mut self, vect: Vec<Box<dyn PdfObject>>) {
        self.values.extend(vect);
    }
}

impl PdfObject for PdfArrayObject {
    fn serialise(&mut self) -> Vec<u8> {
        let mut arr = vec![];
        arr.push(b'[');
        arr.push(b' ');
        for pdf_object in &mut self.values {
            arr.extend(pdf_object.serialise());
            arr.push(b' ');
        }
        arr.push(b']');

        arr
    }
}
