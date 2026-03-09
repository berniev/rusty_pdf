use std::rc::Rc;

use crate::page_size::PageSize;
use crate::{DictionaryObject, NameObject, PdfObject};

//--------------------------- Page ---------------------------//

pub struct Page {
    pub size: PageSize,
    pub contents: Option<Rc<dyn PdfObject>>,
    pub resources: DictionaryObject,
    pub custom_dict: DictionaryObject, // For any other /Page entries
}

impl Page {
    pub fn new(size: PageSize) -> Self {
        Self {
            size,
            contents: None,
            resources: DictionaryObject::new(None),
            custom_dict: DictionaryObject::new(None),
        }
    }

    pub fn set_parent(&mut self, parent_id: usize) {
        self.custom_dict.set_indirect("Parent", parent_id);
    }

    pub fn set_contents(&mut self, contents: Option<Rc<dyn PdfObject>>) {
        self.contents = contents;
    }

    pub fn into_dictionary(self) -> DictionaryObject {
        let mut dict = self.custom_dict;
        dict.set("Type", Rc::new(NameObject::new("Page".to_string())));
        dict.set("MediaBox", Rc::new(self.size.as_array()));

        if let Some(contents) = self.contents {
            dict.set("Contents", contents);
        }

        if !self.resources.values.is_empty() {
            dict.set("Resources", Rc::new(self.resources));
        }

        dict
    }
}
