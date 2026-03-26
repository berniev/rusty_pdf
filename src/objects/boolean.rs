use crate::PdfObject;

//--------------------------- PdfBooleanObject----------------------//

pub struct PdfBooleanObject {
    pub value: bool,
}

impl PdfBooleanObject {
    pub fn new(value: bool) -> Self {
        Self { value }
    }

    pub fn set(&mut self, value: bool) {
        self.value = value;
    }
}

impl PdfObject for PdfBooleanObject {
    fn data(&mut self) -> Vec<u8> {
        let value = if self.value { "true" } else { "false" };

        Vec::from(value)
    }
}
