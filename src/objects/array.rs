use std::rc::Rc;

use crate::PdfMetadata;
use crate::PdfObject;

//-------------------ArrayObject ----------------------

/// Spec:
/// Array Objects:
///     An array object is a one-dimensional collection of objects arranged sequentially. Unlike
///     arrays in many other computer languages, PDF arrays may be heterogeneous; that is, an
///     array’s elements may be any combination of numbers, strings, dictionaries, or any other
///     objects, including other arrays. An array may have zero elements.
/// Construction:
///     An array shall be written as a sequence of objects enclosed in SQUARE BRACKETS.
///     EXAMPLE [ 549 3.14 false ( Ralph ) /SomeName ]
pub struct ArrayObject {
    pub values: Vec<Rc<dyn PdfObject>>,
    pub metadata: PdfMetadata,
}

impl ArrayObject {
    pub fn new(values: Option<Vec<Rc<dyn PdfObject>>>) -> Self {
        Self {
            values: values.unwrap_or_default(),
            metadata: PdfMetadata::default(),
        }
    }

    pub fn push_object(&mut self, value: Rc<dyn PdfObject>) {
        self.values.push(value);
    }
}

impl PdfObject for ArrayObject {
    fn data(&self) -> String {
        format!(
            "[ {} ]",
            self.values
                .iter()
                .map(|item| item.data())
                .collect::<Vec<_>>()
                .join(" ")
        )
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn metadata(&self) -> &PdfMetadata {
        &self.metadata
    }

    fn metadata_mut(&mut self) -> &mut PdfMetadata {
        &mut self.metadata
    }
}
