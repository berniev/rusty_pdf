use crate::PdfObject;

pub struct PdfNullObject {}

impl PdfNullObject {
    pub fn new() -> Self {
        Self {}
    }
}

impl PdfObject for PdfNullObject {
    fn serialise(&mut self) -> Vec<u8> {
        vec![]
    }
}
