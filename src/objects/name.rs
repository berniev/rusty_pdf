/// Spec:
/// name object:
///     an atomic symbol uniquely defined by a sequence of characters introduced by a SOLIDUS (/),
///     (2Fh) but the SOLIDUS is not considered to be part of the name
use crate::PdfError;

#[derive(Clone)]
pub struct PdfNameObject {
    pub(crate) value: String,
}

impl PdfNameObject {
    pub fn new(value: &str) -> Self {
        Self {
            value: value.to_string(),
        }
    }

    pub fn encode(&self) -> Result<Vec<u8>, PdfError> {
        Ok(format!("/{}", self.value).into_bytes())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_simple_name() {
        let obj = PdfNameObject::new("Type");
        assert_eq!(obj.encode().unwrap(), b"/Type");
    }

    #[test]
    fn encode_longer_name() {
        let obj = PdfNameObject::new("FlateDecode");
        assert_eq!(obj.encode().unwrap(), b"/FlateDecode");
    }

    #[test]
    fn encode_empty_name() {
        let obj = PdfNameObject::new("");
        assert_eq!(obj.encode().unwrap(), b"/");
    }
}
