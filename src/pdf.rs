use crate::page::{ObjectId, PageObject, PageTree, PageTreeItemType};
use std::io::Write;

use crate::cross_ref::CrossRefTable;
use crate::fonts::Fonts;
use crate::pdf_version::PdfVersion;
use crate::writer::{CompressedStrategy, LegacyStrategy, PdfWriter};
use crate::{PdfDictionaryObject, PdfIndirectObject, PdfObject};

/// File Structure
///
/// =====================  =====================================================================
/// Header                 One line identifying pdf version
/// Body                   The objects that make up the document
/// Cross-Reference Table  Information about the __indirect__ objects in the file
/// Trailer                Location of the xreft and of certain special objects in the file body
/// ============================================================================================
///

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
    pub catalog: PdfDictionaryObject,
    pub page_tree: PageTree,
    pub cross_ref_table: CrossRefTable,
    pub info: PdfDictionaryObject,
    pub xref_position: Option<usize>,
    next_object_id: usize, // Single source of truth for object ID allocation.
    last_num: usize,       // todo: what's this one for??
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
            objects: vec![],
            catalog: PdfDictionaryObject::new().typed("Catalog"),
            page_tree: PageTree::new(None),
            cross_ref_table: CrossRefTable::new(),
            info: PdfDictionaryObject::new(),
            xref_position: None,
            next_object_id: 1, // Start at 1 (0 is reserved)
            last_num: 0,
        }
    }

    pub fn with_version(mut self, version: PdfVersion) -> Self {
        self.version = version;

        self
    }

    fn next_page_num(&mut self) -> usize {
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
        page.set_id((self.next_page_num() as u64).into());
        self.page_tree.add_page(page);
    }

    fn write_common(&mut self) {
        let resources_number = self.add_font_resources();
        self.initialize_page_tree(resources_number);
        self.initialize_catalog();
        self.initialize_info();
    }

    pub fn write_legacy<W: Write>(
        &mut self,
        output: W,
        id_mode: FileIdentifierMode,
    ) -> std::io::Result<()> {
        self.write_common();
        PdfWriter::new(output, LegacyStrategy::default(), id_mode).perform(self)
    }

    pub fn write_compressed<W: Write>(
        &mut self,
        output: W,
        id_mode: FileIdentifierMode,
    ) -> std::io::Result<()> {
        self.write_common();
        PdfWriter::new(output, CompressedStrategy::default(), id_mode).perform(self)
    }

    pub fn add_font_resources(&mut self) -> usize {
        let mut resources = PdfDictionaryObject::new();
        resources.set("Font", Fonts::get_standard_fonts_dict().boxed());
        
        self.objects.push(resources.boxed());

        let resources_number = self.allocate_object_id();
        resources.metadata.object_identifier = Some(resources_number);

        resources_number
    }

    pub fn initialize_page_tree(&mut self, resources_number: usize) {
        if self.page_tree.metadata.object_identifier.is_some() {
            return;
        }

        // Ensure page tree has a MediaBox if no pages have one
        // every page must have MediaBox (direct or inherited)
        if self.page_tree.media_box.is_none() {
            let has_page_with_mediabox = self.page_tree.kids.iter().any(|kid| {
                if let PageTreeItemType::Page(page) = kid {
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
            if let PageTreeItemType::Page(page) = kid {
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
        let page_tree_clone = PageTree {
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
            .set("Pages", PdfIndirectObject::new(pages_id));

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
