use crate::{PdfMetadata, PdfObject};

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
pub struct IndirectObject {
    value: Option<usize>,
    generation_number: u16,
    metadata: PdfMetadata,
}

impl IndirectObject {
    pub fn new(obj: Option<usize>) -> Self {
        IndirectObject {
            value: obj,
            generation_number: 0,
            metadata: Default::default(),
        }
    }

    pub fn make_pdf_obj(id: usize) -> std::rc::Rc<dyn PdfObject> {
        std::rc::Rc::new(Self::new(Some(id)))
    }
}

impl PdfObject for IndirectObject {
    fn data(&self) -> String {
        format!("{} {} R", self.value.unwrap_or(0), self.generation_number)
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self as &mut dyn std::any::Any
    }

    fn metadata(&self) -> &PdfMetadata {
        &self.metadata
    }

    fn metadata_mut(&mut self) -> &mut PdfMetadata {
        &mut self.metadata
    }
}
