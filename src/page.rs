use crate::objects::base::IndirectReference;
use crate::objects::metadata::PdfMetadata;
use crate::{DictionaryObject, PdfObject};
use std::sync::Arc;

//--------------------------- Page Size ---------------------------//

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

//--------------------------- Page ---------------------------//

pub struct Page {
    pub metadata: PdfMetadata,
    pub page_size: PageSize,
    pub dict: DictionaryObject, // The core dictionary (/Type /Page, etc)
    pub contents: Option<Arc<dyn PdfObject>>, // The page's actual drawing commands
}

impl Default for Page {
    fn default() -> Self {
        Page {
            metadata: PdfMetadata::default(),
            page_size: PageSize::default(),
            dict: DictionaryObject::typed("Page"),
            contents: None,
        }
    }
}

impl Page {
    pub fn new(size: PageSize) -> Self {
        Self {
            page_size: size,
            ..Default::default()
        }
    }

    pub fn set_parent(&mut self, parent_id: usize) {
        self.dict.set(
            "Parent",
            Arc::new(IndirectReference {
                metadata: PdfMetadata::default(),
                id: parent_id,
            }),
        );
    }

    pub fn set_size(&mut self, size: PageSize) {
        self.size = Some(size);
    }

    pub fn set_contents(&mut self, contents: Vec<u8>) {
        self.contents = contents;
    }

    pub fn set_resources(&mut self, resources: DictionaryObject) {
        self.resources = Some(resources);
    }

    pub fn to_dictionary(&self) -> DictionaryObject {
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

        let mut dict = DictionaryObject::new(Some(values));
        dict.metadata = self.metadata.clone();
        dict
    }
}

