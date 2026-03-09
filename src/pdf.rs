use crate::page::Page;
use crate::page::PageSize;

use crate::objects::base::BaseObject;
use crate::{ArrayObject, DictionaryObject, NameObject, NumberObject, NumberType, PdfObject};
use std::rc::Rc;

pub struct StreamObject {
    data: Vec<u8>,
    metadata: DictionaryObject,
}

pub const DEFAULT_VERSION: TargetVersion = TargetVersion::Auto;
pub const DEFAULT_PAGE_SIZE: PageSize = PageSize::A4;

//--------------------------- Catalog -------------------------

/// Spec:
/// Document Catalog:
///     The primary dictionary object containing references directly or indirectly to all other 
///     objects in the document with the exception that there may be objects in the trailer that 
///     are not referred to by the catalog
/// 
///  Catalog 
///          Page Tree
///                           Page
///                                          Content Stream
///                                          Thumbnail Image
///                                          Annotations
///                                    ...
///                           Page
///          Outline Hierachy
///                           Outline Entry
///                                ...
///                           Outline Entry
///          Article Threads
///                           Thread
///                                          Bead <--> Bead
///                               ...
///                           Thread
///          Named Destinations
///          Interactive form
/// Entries:
///     Type               name           Reqd          "Catalog"
///     Version            name           Opt     1.4   
///     Extensions         dictionary     Opt
///     Pages              dictionary     Reqd          shall be indirect ref
///     PageLabels         number tree    Opt     1.3
///     Names              dictionary     Opt     1.2
///     Dests              dictionary     Opt     1.1   indirect reference
///     ViewerPreferences  dictionary     Opt     1.2
///     PageLayout         name           Opt 
///         SinglePage (def)
///         OneColumn
///         TwoColumnLeft
///         TwoColumnRight
///         TwoPageLeft
///         TwoPageRight
///     PageMode           name           Opt     
///          UseNone (def)
///          UseOutlines
///          UseThumbs
///          FullScreen
///          UseOC
///          UseAttachments
///     Outlines            dictionary     Opt         indirect reference
///     Threads             array          Opt    1.1  indirect reference
///     OpenAction          array or dict  Opt    1.1   
///     AA                  dictionary     Opt    1.4
///     URI                 dictionary     Opt    1.1
///     AcroForm            dictionary     Opt    1.2
///     Metadata            dictionary     Opt    1.4
///     StructTreeRoot      dictionary     Opt    1.3
///     MarkInfo            dictionary     Opt    1.4
///     Lang                text string    Opt    1.4
///     SpiderInfo          dictionary     Opt    1.3
///     OutputIntents       array          Opt    1.4
///     PieceInfo           dictionary     Opt    1.4
///     OCProperties        dictionary     Opt    1.5
///     Perms               dictionary     Opt    1.5
///     Legal               dictionary     Opt    1.5
///     Requirements        array          Opt    1.7
///     Collection          dictionary     Opt    1.7
///     NeedsRendering      boolean        Opt    1.7
/// 
pub struct DocumentCatalog {
    pub catalog: DictionaryObject,
}

impl DocumentCatalog {
    pub fn new() -> Self {
        Self {
            catalog: DictionaryObject::typed("Catalog"),
        }
    }
}

//--------------------------- Page Tree -------------------------

/// Spec:
/// Page Tree Nodes:
///     Type    name        "Pages"    Reqd
///     Parent  dictionary             Prohibited in root, else Reqd indirect reference  
///     Kids    array                  Reqd  indirect references
///     Count   integer                Reqd  Number of descendant leaf nodes (page objects)
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

/// Spec:
/// Object:
///     a basic data structure from which PDF files are constructed and includes these types:
///     array, Boolean, dictionary, integer, name, null, real, stream and string
/// Object Reference:
///     an object value used to allow one object to refer to another; that has the form “<n> <m> R”
///     where <n> is an indirect object number, <m> is its version number and R is the uppercase
///     letter R
/// Object stream:
///     a stream that contains a sequence of PDF objects
/// File Structure:
///     Header: One line identifying pdf version
///     Body: containing the objects that make up the document
///     Cross-Reference Table: (xreft) information about the indirect objects in the file
///     Trailer: location of the xreft and of certain special objects within the body of the file
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

        pdf.add_object(Box::new(BaseObject::sentinel())); // object zero

        pdf.page_tree.set("MediaBox", Rc::new([0]));

        pdf
    }

    pub fn with_version(mut self, version: TargetVersion) -> Self {
        self.version = version;
        self
    }

    pub fn add_object(&mut self, mut object: Box<dyn PdfObject>) -> usize {
        let number = self.objects.len();
        object.metadata_mut().object_number = Some(number);
        self.objects.push(object);

        number
    }

    pub fn add_page(&mut self, page: Page) {
        let dict = page.into_dictionary();
        let id = self.add_object(Box::new(dict));
        self.page_ids.push(id);
    }

    pub fn page_references(&self) -> Vec<Vec<u8>> {
        let kids_str = self
            .page_tree
            .get("Kids")
            .map(|v| String::from_utf8_lossy(&v.data()).to_string())
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
