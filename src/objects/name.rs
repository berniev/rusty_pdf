/// Spec:
/// name object:
///     an atomic symbol uniquely defined by a sequence of characters introduced by a SOLIDUS (/),
///     (2Fh) but the SOLIDUS is not considered to be part of the name
///
/// name tree:
///     similar to a dictionary that associates keys and values but the keys in a name tree are
///     strings and are ordered
///
use crate::PdfError;

//--------------------------- PdfNameObject ----------------------//

#[derive(Clone)]
pub struct PdfNameObject {
    pub(crate) value: String,
    pub(crate) object_number: Option<u64>,
    pub(crate) generation_number: Option<u16>,
}

impl PdfNameObject {
    pub fn new(value: &str) -> Self {
        Self {
            value: value.to_string(),
            object_number: None,
            generation_number: None,
        }
    }

    pub fn encode(&self) -> Result<Vec<u8>, PdfError> {
        Ok(format!("/{}", self.value).into_bytes())
    }
}
