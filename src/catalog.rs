use std::collections::HashMap;

use crate::objects::dictionary::Dictionary;
use crate::error::{PdfError, Result};
use crate::objects::metadata::PdfMetadata;
use crate::objects::pdf_object::PdfObject;

#[derive(Debug, Clone)]
pub struct Catalog {
    metadata: PdfMetadata,
    pages_ref: Option<Vec<u8>>,
    other: HashMap<String, Vec<u8>>,
}

impl Catalog {
    pub fn new() -> Self {
        Catalog {
            metadata: PdfMetadata::default(),
            pages_ref: None,
            other: HashMap::from([("Type".to_string(), b"/Catalog".to_vec())]),
        }
    }

    pub fn set_pages_ref(&mut self, pages_ref: Vec<u8>) {
        self.pages_ref = Some(pages_ref);
    }

    pub fn set_other(&mut self, key: String, value: Vec<u8>) -> Result<()> {
        if self.other.insert(key, value).is_some() {
            return Err(PdfError::StructureError("Duplicate key in Catalog".to_string()));
        }
        Ok(())
    }

    pub fn to_dictionary(&self) -> Dictionary {
        let mut values = self.other.clone();
        if let Some(pages_ref) = &self.pages_ref {
            values.insert("Pages".to_string(), pages_ref.clone());
        }
        let mut dict = Dictionary::new(Some(values));
        dict.metadata = self.metadata.clone();
        dict
    }
}

impl Default for Catalog {
    fn default() -> Self {
        Self::new()
    }
}

impl PdfObject for Catalog {
    fn metadata(&self) -> &PdfMetadata {
        &self.metadata
    }

    fn metadata_mut(&mut self) -> &mut PdfMetadata {
        &mut self.metadata
    }

    fn data(&self) -> Vec<u8> {
        self.to_dictionary().data()
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn is_compressible(&self) -> bool {
        self.metadata.generation == 0
    }
}
