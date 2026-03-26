/// PDF Spec:
/// Any object in a PDF file may be labelled as an indirect object. This gives the object a
/// unique object identifier by which other objects can refer to it (for example, as an
/// element of an array or as the value of a dictionary entry).
///
/// An object identifier shall consist of two parts:
/// - A positive integer object number. Indirect objects may be numbered sequentially
///   within a PDF file, but this is not required; object numbers may be assigned in any
///   arbitrary order.
/// - A non-negative integer generation number. In a newly created file, all indirect
///   objects shall have generation numbers of 0. Nonzero generation numbers will be
///   introduced when the file is later updated.
///
/// Together, the combination of an object number and a generation number shall uniquely
/// identify an indirect object.
///
use crate::{PdfObject};

//-------------------------- PdfIndirectObject ----------------------//

pub struct PdfIndirectObject {
    id: usize,
    generation_number: u16,
    location: usize,
}

impl PdfIndirectObject {
    pub fn new(obj: usize) -> Self {
        PdfIndirectObject {
            id: obj,
            generation_number: 0,
            location: 0,
        }
    }

    fn data(&self) -> String {
        format!("{} {} R", self.id, self.generation_number)
    }
}

impl PdfObject for PdfIndirectObject {
    fn data(&mut self) -> Vec<u8> {
        self.data()
    }   
}
