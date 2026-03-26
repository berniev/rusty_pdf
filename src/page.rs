/// Page:
/// A dictionary specifying the attributes of a single page of the document, organized into
/// various categories (e.g., Font, ColorSpace, Pattern)
/// A page object can not have children.
/// Inh = Can be inherited from parent pageTree entry.
///
/// ====================  ===  ====  ===  ================  ===================================
/// Entry Key             Ver  Reqd  Inh  Type              Value
/// ====================  ===  ====  ===  ================  ===================================
/// Type                       Reqd       name              "Page"
/// Parent                     Reqd       dictionary        indirect reference
/// LastModified               *          date              * Reqd if PieceInfo
/// Resources                  Reqd  Inh  dictionary
/// MediaBox                   Reqd  Inh  rectangle
///
/// Annots                     Opt        array
/// Contents                   Opt        stream or array
/// CropBox                    Opt   Inh  rectangle
/// Rotate                     Opt   Inh  integer
/// Thumb                      Opt        stream
/// Trans                      Opt        dictionary
///
/// B                     1.1  Opt        array
/// Dur                   1.1  Opt        number
///
/// AA                    1.2  Opt        dictionary
///
/// ArtBox                1.3  Opt        rectangle
/// BleedBox              1.3  Opt        rectangle
/// ID byte               1.3  Opt        string
/// PieceInfo             1.3  Opt        dictionary
/// PZ                    1.3  Opt        number
/// SeparationInfo        1.3  Opt        dictionary
/// StructParents         1.3  *          integer          Reqd if struct content items
/// TrimBox               1.3  Opt        rectangle
///
/// BoxColorInfo          1.4  Opt        dictionary
/// Group                 1.4  Opt        dictionary
/// Metadata              1.4  Opt        stream
///
/// PresSteps             1.5  Opt        dictionary
/// Tabs                  1.5  Opt        name
/// TemplateInstantiated  1.5  Opt        name
///
/// UserUnit              1.6  Opt        number
/// VP                    1.6  Opt        dictionary

/// PageTree Nodes:
/// 
/// ======  ==========  =====  =================================================================
/// Name    PdfObjType  Reqd   Value
/// ======  ==========  =====  =================================================================
/// Type    Name        Reqd   "Pages" 
/// Parent  Indirect    Reqd*  Parent PageTree. * Not allowed in root node.
/// Kids    Array       Reqd   Indirect references to descendant leaf nodes (pages)
/// Count   Integer     Reqd   Number of descendant leaf nodes (pages)
///
use std::fmt;
use std::iter::Sum;

use crate::{PdfMetadata, PdfObject, ResourceMap};
pub use crate::page_size::PageSize;
//--------------------------- ObjectId ---------------------------//

#[derive(Clone, Debug, Default)]
pub struct ObjectId(u64);

impl From<u64> for ObjectId {
    fn from(value: u64) -> Self {
        ObjectId(value)
    }
}

impl From<usize> for ObjectId {
    fn from(value: usize) -> Self {
        ObjectId(value as u64)
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

//--------------------------- Page ---------------------------//


#[derive(Clone)]
pub struct PageObject {
    pub(crate) id: ObjectId,
    pub(crate) parent: ObjectId,
    pub(crate) resources: Option<ResourceMap>,
    pub(crate) resources_id: Option<usize>,
    pub media_box: Option<PageSize>,
    pub(crate) contents: Vec<usize>, // Content stream object IDs
    pub(crate) metadata: PdfMetadata,
}

impl PageObject {
    pub fn new(parent: ObjectId) -> Self {
        Self {
            id: ObjectId(0),
            parent,
            resources: None,
            resources_id: None,
            media_box: None,
            contents: Vec::new(),
            metadata: PdfMetadata::default(),
        }
    }

    pub fn set_id(&mut self, id: ObjectId) {
        self.id = id;
    }

    pub fn set_media_box(&mut self, size: PageSize) {
        self.media_box = Some(size);
    }

    pub fn set_resources(&mut self, resources: ResourceMap) {
        self.resources = Some(resources);
    }

    pub fn add_content(&mut self, content_id: usize) {
        self.contents.push(content_id);
    }

    pub fn set_contents(&mut self, content_ids: Vec<usize>) {
        self.contents = content_ids;
    }
}

impl PdfObject for PageObject {
    fn data(&mut self) -> Vec<u8> {
        let mut entries = vec!["/Type /Page".to_string()];

        entries.push(format!("/Parent {} 0 R", u64::from(self.parent.clone())));

        if !self.contents.is_empty() {
            let refs: Vec<String> = self
                .contents
                .iter()
                .map(|id| format!("{} 0 R", id))
                .collect();
            entries.push(format!("/Contents [{}]", refs.join(" ")));
        }

        // MediaBox (optional if parent provides - inheritance)
        if let Some(size) = &self.media_box {
            let dims = size.dims();
            entries.push(format!("/MediaBox [0 0 {} {}]", dims.width, dims.height));
        }

        // Resources (optional if parent provides - inheritance)
        if let Some(resources_id) = self.resources_id {
            entries.push(format!("/Resources {} 0 R", resources_id));
        }

        format!("<< {} >>", entries.join(" ")).into_bytes()
    }
/*
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn metadata(&self) -> &PdfMetadata {
        &self.metadata
    }

    fn metadata_mut(&mut self) -> &mut PdfMetadata {
        &mut self.metadata
    }*/
}

//--------------------------- Page Tree -------------------------
#[derive(Clone)]
pub enum PageTreeItemType {
    Page(PageObject),
    Node(PageTree),
}

impl PageTreeItemType {
    pub fn id(&self) -> ObjectId {
        match self {
            PageTreeItemType::Page(page) => {
                ObjectId::from(page.metadata.object_identifier.unwrap_or(0))
            }
            PageTreeItemType::Node(node) => {
                ObjectId::from(node.metadata.object_identifier.unwrap_or(0))
            }
        }
    }
}

#[derive(Clone)]
pub struct PageTree {
    pub(crate) id: ObjectId,
    pub(crate) parent_id: Option<ObjectId>, // root is None
    pub(crate) kids: Vec<PageTreeItemType>,
    pub(crate) media_box: Option<PageSize>, // Shared dimensions
    pub(crate) resources: Option<ResourceMap>, // Shared fonts, etc.
    pub metadata: PdfMetadata,
}

impl PageTree {
    pub fn new(parent_id: Option<ObjectId>) -> Self {
        Self {
            id: ObjectId(0),
            parent_id,
            kids: Vec::new(),
            media_box: None,
            resources: None,
            metadata: PdfMetadata::default(),
        }
    }

    pub fn id(&self) -> ObjectId {
        self.id.clone()
    }

    pub fn count(&self) -> ObjectId {
        self.kids
            .iter()
            .map(|kid| match kid {
                PageTreeItemType::Page(_) => ObjectId(1),
                PageTreeItemType::Node(node) => node.count(),
            })
            .sum()
    }

    pub fn add_page(&mut self, page: PageObject) {
        self.kids.push(PageTreeItemType::Page(page));
    }

    pub fn add_node(&mut self, page_tree_node: PageTree) {
        self.kids.push(PageTreeItemType::Node(page_tree_node));
    }

    pub fn kids_array(&self) -> String {
        format!(
            "[{}]",
            self.kids
                .iter()
                .map(|kid| format!("{} 0 R", kid.id()))
                .collect::<Vec<_>>()
                .join(" ")
        )
    }

    pub fn set_media_box(&mut self, size: PageSize) {
        self.media_box = Some(size);
    }

    pub fn set_resources(&mut self, resources: ResourceMap) {
        self.resources = Some(resources);
    }

    pub fn reference(&self) -> String {
        format!("{} 0 R", self.metadata.object_identifier.unwrap_or(0))
    }
}

impl PdfObject for PageTree {
    fn data(&mut self) -> Vec<u8> {
        let mut entries = vec!["/Type /Pages".to_string()];

        entries.push(format!("/Kids {}", &self.kids_array()));

        entries.push(format!("/Count {}", self.count()));

        // MediaBox (optional, inherited)
        if let Some(size) = &self.media_box {
            let dims = size.dims();
            entries.push(format!("/MediaBox [0 0 {} {}]", dims.width, dims.height));
        }

        // Resources (optional, inherited)
        if let Some(_resources) = &self.resources {
            // TODO: Serialize resources
        }

        format!("<< {} >>", entries.join(" ")).into_bytes()
    }
/*
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn metadata(&self) -> &PdfMetadata {
        &self.metadata
    }

    fn metadata_mut(&mut self) -> &mut PdfMetadata {
        &mut self.metadata
    }*/
}
