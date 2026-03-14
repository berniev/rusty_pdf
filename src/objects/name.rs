use crate::PdfMetadata;
use crate::PdfObject;

/// Spec:
/// name object:
///     an atomic symbol uniquely defined by a sequence of characters introduced by a SOLIDUS (/),
///     (2Fh) but the SOLIDUS is not considered to be part of the name
///
/// name tree:
///     similar to a dictionary that associates keys and values but the keys in a name tree are
///     strings and are ordered
pub struct NameObject {
    metadata: PdfMetadata,
    pub value: Option<String>,
}

impl NameObject {
    pub fn new(value: Option<String>) -> Self {
        Self {
            metadata: Default::default(),
            value,
        }
    }

    pub fn set(&mut self, value: String) {
        self.value = Some(value);
    }
}

impl PdfObject for NameObject {
    fn data(&self) -> String {
        format!("/{}", self.value.clone().unwrap_or("".to_string()))
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
