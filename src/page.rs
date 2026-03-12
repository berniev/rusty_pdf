use std::fmt;
use std::iter::Sum;

use crate::ResourceDictionary;
use crate::util::Dims;

//--------------------------- Offset ---------------------------//

///// Usage: let object_num: ObjectNum = 100u64.into();
#[derive(Clone, Debug)]
#[derive(Default)]
pub struct ObjectId(u64);

impl From<u64> for ObjectId {
    fn from(value: u64) -> Self {
        ObjectId(value)
    }
}

impl From<ObjectId> for u64 {
    fn from(object_num: ObjectId) -> u64 {
        object_num.0
    }
}

impl Sum for ObjectId {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        ObjectId(iter.map(|id| id.0).sum())
    }
}

impl fmt::Display for ObjectId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

//--------------------------- Page Size ---------------------------//

pub const DEFAULT_PAGE_SIZE: PageSize = PageSize::A4;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PageSize {
    A4,
    Letter,
    Legal,
    A3,
    Custom(Dims), // width, height in points
}

impl Default for PageSize {
    fn default() -> Self {
        PageSize::A4
    }
}

impl PageSize {
    /// Returns the Dims in PDF points (1 PDF point = 1/72 inch).
    /// Returns 0.0 for negative custom dimensions.
    pub fn dimensions(&self) -> Dims {
        match self {
            PageSize::A4 => Dims {
                width: 595.0,
                height: 842.0,
            },
            PageSize::Letter => Dims {
                width: 612.0,
                height: 792.0,
            },
            PageSize::Legal => Dims {
                width: 612.0,
                height: 1008.0,
            },
            PageSize::A3 => Dims {
                width: 842.0,
                height: 1191.0,
            },
            PageSize::Custom(dims) => Dims {
                width: dims.width.max(0.0),
                height: dims.height.max(0.0),
            },
        }
    }
}

//--------------------------- Page ---------------------------//

/// Spec:
/// Page:
///     a dictionary specifying the attributes of a single page of the document.
///     organized into various categories (e.g., Font, ColorSpace, Pattern)
///     A page object cannot have children.
/// Entries:
/// Key                   Ver             Type              Value
/// Type                       Reqd       name              "Page"
/// Parent                     Reqd       dictionary        indirect reference
/// LastModified               *          date              Reqd if PieceInfo
/// Resources                  Reqd  Inh  dictionary
/// MediaBox                   Reqd  Inh  rectangle
/// CropBox                    Opt   Inh  rectangle
/// BleedBox              1.3  Opt        rectangle
/// TrimBox               1.3  Opt        rectangle
/// ArtBox                1.3  Opt        rectangle
/// BoxColorInfo          1.4  Opt        dictionary
/// Contents                   Opt        stream or array
/// Rotate                     Opt   Inh  integer
/// Group                 1.4  Opt        dictionary
/// Thumb                      Opt        stream
/// B                     1.1  Opt        array
/// Dur                   1.1  Opt        number
/// Trans                      Opt        dictionary
/// Annots                     Opt        array
/// AA                    1.2  Opt        dictionary
/// Metadata              1.4  Opt        stream
/// PieceInfo             1.3  Opt        dictionary
/// StructParents         1.3  *          integer          Reqd if struct content items
/// ID byte               1.3  Opt        string
/// PZ                    1.3  Opt        number
/// SeparationInfo        1.3  Opt        dictionary
/// Tabs                  1.5  Opt        name
/// TemplateInstantiated  1.5  Opt        name
/// PresSteps             1.5  Opt        dictionary
/// UserUnit              1.6  Opt        number
/// VP                    1.6  Opt        dictionary

pub struct PageObject {
    id: ObjectId,
    parent: ObjectId,
    resources: Option<ResourceDictionary>,
    pub media_box: Option<PageSize>,
}

impl PageObject {
    pub fn new(parent: ObjectId) -> Self {
        Self {
            id: 0.into(),
            parent,
            resources: None,
            media_box: None,
        }
    }

    pub fn set_id(&mut self, id: ObjectId) {
        self.id = id;
    }

    /// If None, the page will later try to inherit from its parent.
    pub fn set_media_box(&mut self, size: PageSize) {
        self.media_box = Some(size);
    }

    /// If None, the page will later try to inherit from its parent.
    pub fn set_resources(&mut self, resources: ResourceDictionary) {
        self.resources = Some(resources);
    }
}

//--------------------------- Page Tree -------------------------

/// Spec:
/// Page Tree Nodes:
///     Type    name        "Pages"    Reqd
///     Parent  dictionary             Prohibited in root, else Reqd indirect ref to pagetree entry
///     Kids    array                  Reqd  indirect references to descendant leaf nodes (pages)
///     Count   integer                Reqd  Number of descendant leaf nodes (pages)
pub enum PageTreeItem {
    Page(PageObject),
    Node(PageTreeNode),
}

impl PageTreeItem {
    pub fn id(&self) -> ObjectId {
        match self {
            PageTreeItem::Page(page) => page.id.clone(),
            PageTreeItem::Node(node) => node.id.clone(),
        }
    }
}

pub struct PageTreeNode {
    id: ObjectId,
    parent_id: Option<ObjectId>, // root is None
    kids: Vec<PageTreeItem>,
    media_box: Option<PageSize>,           // Shared dimensions
    resources: Option<ResourceDictionary>, // Shared fonts, etc.
}

impl PageTreeNode {
    pub fn new(parent: Option<ObjectId>) -> Self {
        Self {
            id: 0.into(),
            parent_id: parent,
            kids: Vec::new(),
            media_box: None,
            resources: None,
        }
    }

    pub fn id(&self) -> ObjectId {
        self.id.clone()
    }

    pub fn count(&self) -> ObjectId {
        self.kids
            .iter()
            .map(|kid| match kid {
                PageTreeItem::Page(_) => ObjectId(1),
                PageTreeItem::Node(node) => node.count().into(),
            })
            .sum()
    }

    pub fn add_page(&mut self, page: PageObject) {
        self.kids.push(PageTreeItem::Page(page));
    }

    pub fn add_node(&mut self, page_tree_node: PageTreeNode) {
        self.kids.push(PageTreeItem::Node(page_tree_node));
    }

    pub fn kids_array(&self) -> Vec<u8> {
        let mut items: Vec<String> = Vec::new();
        for kid in &self.kids {
            items.push(format!("{} 0 R", kid.id()));
        }
        format!("[{}]", items.join(" ")).into_bytes()
    }

    pub fn set_media_box(&mut self, size: PageSize) {
        self.media_box = Some(size);
    }

    pub fn set_resources(&mut self, resources: ResourceDictionary) {
        self.resources = Some(resources);
    }
}
