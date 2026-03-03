use std::collections::HashMap;
use crate::dictionary::Dictionary;
use crate::object::{PdfMetadata, PdfObject};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PageSize {
    A4,
    Letter,
    Legal,
    A3,
    Custom(f64, f64), // width, height in points
}

impl Default for PageSize {
    fn default() -> Self {
        PageSize::A4
    }
}

impl PageSize {
    /// Returns the [width, height] in PDF points (1 PDF point = 1/72 inch).
    /// Returns 0.0 for negative custom dimensions.
    pub fn dimensions(&self) -> (f64, f64) {
        match self {
            PageSize::A4 => (595.0, 842.0),
            PageSize::Letter => (612.0, 792.0),
            PageSize::Legal => (612.0, 1008.0),
            PageSize::A3 => (842.0, 1191.0),
            PageSize::Custom(w, h) => (w.max(0.0), h.max(0.0)),
        }
    }

    pub fn to_mediabox(&self) -> Vec<u8> {
        let (w, h) = self.dimensions();
        format!("[0 0 {w} {h}]").into_bytes()
    }
}

#[derive(Debug, Clone)]
pub struct Page {
    pub metadata: PdfMetadata,
    pub size: Option<PageSize>,
    pub contents: Vec<u8>,
    pub resources: Option<Dictionary>,
    pub other: HashMap<String, Vec<u8>>,
}

impl Page {

    pub fn new() -> Self {
        Page {
            metadata: PdfMetadata::default(),
            size: None,
            contents: Vec::new(),
            resources: None,
            other: HashMap::new(),
        }
    }

    pub fn set_size(&mut self, size: PageSize) {
        self.size = Some(size);
    }

    pub fn set_contents(&mut self, contents: Vec<u8>) {
        self.contents = contents;
    }

    pub fn set_resources(&mut self, resources: Dictionary) {
        self.resources = Some(resources);
    }

    pub fn to_dictionary(&self) -> Dictionary {
        let mut values = self.other.clone();
        values.insert("Type".to_string(), b"/Page".to_vec());
        if let Some(size) = self.size {
            values.insert("MediaBox".to_string(), size.to_mediabox());
        }
        if !self.contents.is_empty() {
            values.insert("Contents".to_string(), self.contents.clone());
        }
        if let Some(resources) = &self.resources {
            values.insert("Resources".to_string(), resources.data());
        }

        let mut dict = Dictionary::new(Some(values));
        dict.metadata = self.metadata.clone();
        dict
    }
}

impl Default for Page {
    fn default() -> Self {
        Self::new()
    }
}

impl PdfObject for Page {
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
