use std::rc::Rc;
use crate::objects::base::BaseObject;
use crate::{ArrayObject, DictionaryObject, NameObject, NumberObject, NumberType, PdfObject};

use crate::page::Page;
use crate::page_size::PageSize;

pub const DEFAULT_VERSION: TargetVersion = TargetVersion::Auto;
pub const DEFAULT_PAGE_SIZE: PageSize = PageSize::A4;

//--------------------------- Catalog -------------------------

pub struct DocumentCatalog {
    pub dict: DictionaryObject,
}

impl DocumentCatalog {
    pub fn new() -> Self {
        Self { dict: DictionaryObject::typed("Catalog")}
    }
}

//--------------------------- Page Tree -------------------------

pub struct PageTree {
    pub dict: DictionaryObject,
}

impl PageTree {
    pub fn new() -> Self {
        let mut dict = DictionaryObject::typed("Pages");
        dict.set("Kids", Rc::new(ArrayObject::new(None)));
        dict.set("Count", Rc::new(NumberObject::new(NumberType::from(0.0))));
        Self { dict }
    }
}

//--------------------------- Version -------------------------

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum TargetVersion {
    Auto, // Use the lowest working version
    V1_4,
    V1_5,
}

impl TargetVersion {
    pub fn as_str(&self) -> &str {
        match self {
            TargetVersion::Auto => DEFAULT_VERSION.as_str(), // Default floor
            TargetVersion::V1_4 => "1.4",
            TargetVersion::V1_5 => "1.5",
        }
    }
}

//----------------------- Identifier -----------------------

/// for trailer
pub enum FileIdentifierMode {
    None,
    AutoMD5,
    Custom(Vec<u8>),
}

//--------------------------- PDF -------------------------

pub struct PDF {
    pub version: TargetVersion,
    pub objects: Vec<Box<dyn PdfObject>>,
    pub info: DictionaryObject,
    pub catalog: DocumentCatalog,
    pub page_tree: DictionaryObject,
    pub page_ids: Vec<usize>,
    pub xref_position: Option<usize>,
}

impl Default for PDF {
    fn default() -> Self {
        PDF {
            version: DEFAULT_VERSION,
            info: DictionaryObject::new(None),
            catalog: DocumentCatalog::new(),
            page_tree: DictionaryObject::new(None),
            objects: Vec::new(),
            page_ids: vec![],
            xref_position: None,
        }
    }
}

impl PDF {
    pub fn new() -> Self {
        let mut pdf = PDF {
            ..Default::default()
        };

        pdf.add_object(Box::new(BaseObject::sentinel()));

        pdf.page_tree
            .set("MediaBox", Rc::new([0]));

        pdf
    }

    pub fn with_version(mut self, version: TargetVersion) -> Self {
        self.version = version;
        self
    }

    pub fn add_object(&mut self, mut object: Box<dyn PdfObject>) -> usize {
        let number = self.objects.len();
        object.metadata_mut().number = Some(number);
        self.objects.push(object);

        number
    }

    pub fn add_page(&mut self, page: Page) {
    let dict = page.into_dictionary();
    let id = self.add_object(Box::new(dict));
    self.page_ids.push(id);
}

    pub fn add_page_simple(&mut self, size: PageSize, contents: &[u8]) {
        let mut page = Page::new(size);
        page.set_contents(contents.to_vec());
        self.add_page(page);
    }

    pub fn page_references(&self) -> Vec<Vec<u8>> {
        let kids_str = self
            .page_tree
            .values
            .get("Kids")
            .map(|v| String::from_utf8_lossy(v).to_string())
            .unwrap_or_else(|| "[]".to_string());

        // Parse references in format "[1 0 R 2 0 R 3 0 R]"
        kids_str
            .trim_matches(|c| c == '[' || c == ']')
            .split_whitespace()
            .collect::<Vec<_>>()
            .chunks(3)
            .filter_map(|chunk| {
                if chunk.len() == 3 && chunk[2] == "R" {
                    Some(format!("{} {} {}", chunk[0], chunk[1], chunk[2]).into_bytes())
                } else {
                    None
                }
            })
            .collect()
    }

    fn get_standard_fonts_dict() -> DictionaryObject {
        let mut font_dict = DictionaryObject::new(None);
        let fonts = [
            ("Helvetica", "Type1"),
            ("Helvetica-Bold", "Type1"),
            ("Courier", "Type1"),
        ];

        for (name, subtype) in fonts {
            let mut f = DictionaryObject::typed("Font");
            f.set("Subtype", Rc::new(NameObject::new(subtype.to_string())));
            f.set("BaseFont", Rc::new(NameObject::new(name.to_string())));
            font_dict.set(name, Rc::new(f));
        }
        font_dict
    }

    fn get_standard_fonts() -> Vec<u8> {
        let mut font_dict = String::from("<<");
        let fonts = [
            ("Helvetica", "Type1"),
            ("Helvetica-Bold", "Type1"),
            ("Courier", "Type1"),
        ];
        for (name, subtype) in fonts {
            font_dict.push_str(&format!(
                " /{} << /Type /Font /Subtype /{} /BaseFont /{} >>",
                name, subtype, name
            ));
        }
        font_dict.push_str(" >>");
        font_dict.into_bytes()
    }
}
