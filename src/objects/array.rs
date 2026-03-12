use std::rc::Rc;
use crate::NameObject;
use crate::objects::pdf_object::PdfObject;

/// Spec:
/// Array Objects:
///     An array object is a one-dimensional collection of objects arranged sequentially. Unlike
///     arrays in many other computer languages, PDF arrays may be heterogeneous; that is, an
///     array’s elements may be any combination of numbers, strings, dictionaries, or any other
///     objects, including other arrays. An array may have zero elements.
/// Construction:
///     An array shall be written as a sequence of objects enclosed in SQUARE BRACKETS (using LEFT
///     SQUARE BRACKET (5Bh) and RIGHT SQUARE BRACKET (5Dh)).
///     EXAMPLE [ 549 3.14 false ( Ralph ) /SomeName ]
///     PDF directly supports only one-dimensional arrays. Arrays of higher dimension can be
///     constructed by using arrays as elements of arrays, nested to any depth
pub struct ArrayObject {
    pub values: Vec<Rc<dyn PdfObject>>,
}

impl ArrayObject {

    pub fn new(values: Option<Vec<Rc<dyn PdfObject>>>) -> Self {
        Self {
            values: values.unwrap_or_default(),
        }
    }

    pub fn push_indirect(&mut self, id: usize) {
        self.values.push(Rc::new(IndirectReference {
            metadata: Default::default(),
            id,
        }));
    }
}

impl PdfObject for ArrayObject {

    fn data(&self) -> Vec<u8> {
        let mut result = b"[".to_vec();
        for (i, item) in self.values.iter().enumerate() {
            if i > 0 {
                result.push(b' ');
            }
            result.extend(item.reference());
        }
        result.push(b']');
        result
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
