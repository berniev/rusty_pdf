use crate::page::{ObjectId, PageObject, PageTreeItem, PageTreeNode};
use std::io::Write;

use crate::cross_ref::CrossRefTable;
use crate::{DictionaryObject, IndirectObject, NameObject, PdfObject};

//--------------------------- PDF -------------------------

/// Spec:
/// Object:
///     a basic data structure from which PDF files are constructed and includes these types:
///     array, boolean, dictionary, integer, name, null, real, stream and string
/// Object Reference:
///     an object value used to allow one object to refer to another: “<n> <m> R”
///     where <n> is an indirect object number, <m> is its version number and R is the uppercase R
/// Object stream:
///     a stream that contains a sequence of PDF objects
/// File Structure:
///     Header: One line identifying pdf version
///     Body: containing the objects that make up the document
///     Cross-Reference Table: (xreft) information about the indirect objects in the file
///     Trailer: location of the xreft and of certain special objects within the body of the file

//--------------------------- Version -------------------------

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub enum PdfVersion {
    #[default]
    Auto,
    V1_4,
    V1_5,
}

impl PdfVersion {
    pub fn as_str(&self) -> &str {
        match self {
            PdfVersion::Auto => "Auto",
            PdfVersion::V1_4 => "1.4",
            PdfVersion::V1_5 => "1.5",
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
    pub version: PdfVersion,
    pub objects: Vec<Box<dyn PdfObject>>,
    pub catalog: DictionaryObject,
    pub page_tree: PageTreeNode,
    pub cross_ref_table: CrossRefTable,
    pub last_num: usize,
    pub info: DictionaryObject,
    pub current_position: usize,
    pub xref_position: Option<usize>,
    next_object_id: usize, // Single source of truth for object ID allocation.
}

impl Default for PDF {
    fn default() -> Self {
        Self::new()
    }
}

impl PDF {
    pub fn new() -> Self {
        PDF {
            version: PdfVersion::Auto,
            objects: Vec::new(),
            catalog: DictionaryObject::typed("Catalog"),
            next_object_id: 1, // Start at 1 (0 is reserved)
            page_tree: PageTreeNode::new(None),
            cross_ref_table: CrossRefTable::new(),
            last_num: 0,
            info: DictionaryObject::new(None),
            current_position: 0,
            xref_position: None,
        }
    }

    pub fn with_version(mut self, version: PdfVersion) -> Self {
        self.version = version;

        self
    }

    fn next_num(&mut self) -> usize {
        self.last_num += 1;

        self.last_num
    }

    pub(crate) fn allocate_object_id(&mut self) -> usize {
        let id = self.next_object_id;
        self.next_object_id += 1;
        id
    }

    pub(crate) fn object_count(&self) -> usize {
        self.next_object_id
    }

    pub fn add_object(&mut self, mut object: Box<dyn PdfObject>) -> usize {
        let number = self.allocate_object_id();
        object.metadata_mut().object_identifier = Some(number);
        self.objects.push(object);

        number
    }

    pub fn add_page(&mut self, mut page: PageObject) {
        page.set_id((self.next_num() as u64).into());
        self.page_tree.add_page(page);
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
            f.set("Subtype", NameObject::make_pdf_obj(subtype));
            f.set("BaseFont", NameObject::make_pdf_obj(name));
            font_dict.set(name, DictionaryObject::make_pdf_obj(f.values));
        }

        font_dict
    }

    #[allow(dead_code)]
    fn get_standard_fonts() -> String {
        let fonts = [
            ("Helvetica", "Type1"),
            ("Helvetica-Bold", "Type1"),
            ("Courier", "Type1"),
        ];
        format!(
            "<<{}>>",
            fonts
                .into_iter()
                .map(|(name, subtype)| format!(
                    " /{name} << /Type /Font /Subtype /{subtype} /BaseFont /{name} >>"
                ))
                .collect::<Vec<String>>()
                .join(" ")
        )
    }

    pub fn write<W: Write>(
        &mut self,
        output: W,
        id_mode: FileIdentifierMode,
    ) -> std::io::Result<()> {
        // Initialize required PDF structures
        let resources_number = self.add_font_resources();
        self.initialize_page_tree(resources_number);
        self.initialize_catalog();
        self.initialize_info();

        // Delegate to writer based on version
        use crate::writer::{LegacyStrategy, PdfWriter};

        let mut writer = PdfWriter::new(output, LegacyStrategy, id_mode);
        writer.perform(self)
    }

    /// Write PDF to output using compressed format (PDF 1.5+)
    /// Uses object streams and cross-reference streams for smaller file size
    pub fn write_compressed<W: Write>(
        &mut self,
        output: &mut W,
        id_mode: FileIdentifierMode,
    ) -> std::io::Result<()> {
        // Initialize required PDF structures
        let resources_number = self.add_font_resources();
        self.initialize_page_tree(resources_number);
        self.initialize_catalog();
        self.initialize_info();

        // Delegate to compressed writer
        use crate::writer::{CompressedStrategy, PdfWriter};

        let mut writer = PdfWriter::new(output, CompressedStrategy::new(), id_mode);
        writer.perform(self)
    }

    pub fn add_font_resources(&mut self) -> usize {
        let font_dict = Self::get_standard_fonts_dict();
        let resources_number = self.allocate_object_id();
        let mut resources = DictionaryObject::new(None);
        resources.metadata.object_identifier = Some(resources_number);
        resources.set("Font", DictionaryObject::make_pdf_obj(font_dict.values));
        self.objects.push(Box::new(resources));
        resources_number
    }

    pub fn initialize_page_tree(&mut self, resources_number: usize) {
        if self.page_tree.metadata.object_identifier.is_some() {
            return;
        }

        // Ensure page tree has a MediaBox if no pages have one
        // This is required by PDF spec - every page must have MediaBox (direct or inherited)
        if self.page_tree.media_box.is_none() {
            let has_page_with_mediabox = self.page_tree.kids.iter().any(|kid| {
                if let PageTreeItem::Page(page) = kid {
                    page.media_box.is_some()
                } else {
                    false
                }
            });
            if !has_page_with_mediabox {
                // Set default A4 size
                self.page_tree.media_box = Some(crate::page::PageSize::A4);
            }
        }

        // Count pages and allocate IDs
        let num_pages = self.page_tree.kids.len();
        let mut page_ids = Vec::new();
        for _ in 0..num_pages {
            page_ids.push(self.allocate_object_id());
        }

        // Allocate ID for page tree itself (after all pages)
        let pages_number = self.allocate_object_id();
        self.page_tree.metadata.object_identifier = Some(pages_number);

        // Now assign IDs and clone pages
        let mut page_objects = Vec::new();
        let mut page_idx = 0;
        for kid in &mut self.page_tree.kids {
            if let PageTreeItem::Page(page) = kid {
                let page_id = page_ids[page_idx];
                page_idx += 1;
                page.metadata.object_identifier = Some(page_id);
                page_objects.push((page_id, page.clone()));
            }
        }

        // Add all pages to objects with correct parent reference
        for (page_id, mut page) in page_objects {
            page.parent = ObjectId::from(pages_number);
            page.resources_id = Some(resources_number);
            page.metadata.object_identifier = Some(page_id);
            self.objects.push(Box::new(page));
        }

        // Now add the page tree itself
        // Clone the page tree to add it to objects
        let page_tree_clone = PageTreeNode {
            id: self.page_tree.id.clone(),
            parent_id: self.page_tree.parent_id.clone(),
            kids: self.page_tree.kids.clone(),
            media_box: self.page_tree.media_box,
            resources: self.page_tree.resources.clone(),
            metadata: self.page_tree.metadata.clone(),
        };
        self.objects.push(Box::new(page_tree_clone));
    }

    pub fn initialize_catalog(&mut self) {
        if self.catalog.metadata.object_identifier.is_some() {
            return;
        }

        let catalog_number = self.allocate_object_id();
        self.catalog.metadata.object_identifier = Some(catalog_number);

        // Add reference to page tree
        let pages_id = self.page_tree.metadata.object_identifier.unwrap();
        self.catalog
            .set("Pages", IndirectObject::make_pdf_obj(pages_id));

        let catalog_copy = self.catalog.clone();
        self.objects.push(Box::new(catalog_copy));
    }

    pub fn initialize_info(&mut self) {
        if !self.info.values.is_empty() && self.info.metadata.object_identifier.is_none() {
            self.info.metadata.object_identifier = Some(self.allocate_object_id());
            let info_copy = self.info.clone();
            self.objects.push(Box::new(info_copy));
        }
    }
}
