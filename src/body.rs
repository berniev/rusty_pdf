/*
body

The second block in a pdf file.

Body contains all the objects in the document
 */
use crate::PdfObject;

pub struct Body {
    objects: Vec<Box<dyn PdfObject>>,
}

impl Body {
    pub fn new() -> Self {
        Body {
            objects: Vec::new(),
        }
    }

    pub fn add_object(&mut self, object: Box<dyn PdfObject>) {
        self.objects.push(object);
    }

    pub fn serialise(&mut self) -> Vec<u8> {
        let mut serialised = vec![];
        for object in &mut self.objects {
            serialised.extend(object.serialise());
        }
        serialised
    }
}
