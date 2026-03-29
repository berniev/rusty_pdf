/// Spec:
/// name object:
///     an atomic symbol uniquely defined by a sequence of characters introduced by a SOLIDUS (/),
///     (2Fh) but the SOLIDUS is not considered to be part of the name
///
/// name tree:
///     similar to a dictionary that associates keys and values but the keys in a name tree are
///     strings and are ordered
///
use crate::PdfObject;

//--------------------------- PdfNameObject ----------------------//

pub struct PdfNameObject {
    pub value: String,
}

impl PdfNameObject {
    pub fn new(value: &str) -> Self {
        Self {
            value: value.to_string(),
        }
    }
}

impl PdfObject for PdfNameObject {
    fn serialise(&mut self) -> Vec<u8> {
        format!("/{}", self.value).into_bytes()
    }
}
