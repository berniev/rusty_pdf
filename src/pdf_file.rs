use crate::page::{ObjectId, PageObject, PageTree, PageTreeItem};
use std::io::Write;

use crate::body::Body;
use crate::cross_ref::CrossRefTable;
use crate::fonts::Fonts;
use crate::header::Header;
use crate::pdf_version::PdfVersion;
use crate::trailer::Trailer;
use crate::writer::{CompressedStrategy, LegacyStrategy, PdfWriter};
use crate::{PdfDictionaryObject, PdfObject, PdfStreamObject};
use crate::file_identifier::FileIdentifierMode;
use crate::objects::pdf_object::Pdf;

/// File Structure
///
/// =====================  =====================================================================
/// Header                 One line identifying pdf version
/// Body                   The objects that make up the document
/// Cross-Reference Table  Information about the __indirect__ objects in the file
/// Trailer                Location of the xref tbl and of certain special objects in the file body
/// ============================================================================================
///

/**
space lines are optional
```
%PDF-1.4                    ← header
%âãÏÓ                       ← comment in the body, not required nowadays but spec does say 'shall'
1 0 obj                     ← first actual body object
...
endobj
...

xref                        ← cross-reference table
0 9
0000000000 65535 f\r\n
...

trailer                     ← trailer
<<
  /Size 9
  /Root 1 0 R
>>
startxref
1234                        ← byte offset of xref
%%EOF
```
*/

//--------------------------- PDF -------------------------

pub struct PdfFile {
    header: Header,
    body: Body,
    xref: CrossRefTable,
    trailer: Trailer,

    xref_position: Option<usize>,


    root_page_tree: PageTree, // catalog /Pages entry must point to this
}

impl PdfFile {
    pub fn new() -> Self {
        PdfFile {
            header: Header::new(),
            body: Body::new(),
            xref: CrossRefTable::new(),
            trailer: Trailer::new(),
            
            xref_position: None,
            root_page_tree: PageTree::new(None),
        }
    }

    pub fn with_version(mut self, version: PdfVersion) -> Self {
        self.header.set_version(version);

        self
    }

    // todo: is this creating a direct or indirect object?
    pub fn add_object(&mut self, mut object: Box<dyn PdfObject>) -> usize {
        let number = self.allocate_object_id();
        object.metadata_mut().object_identifier = Some(number);
        self.indirect_pdf_objects.push(object);

        number
    }

    pub fn add_pdf_stream(&mut self, stream: PdfStreamObject) -> usize {
        // is a stream an indirect object ie needs wrapping in PdfIndirectObject ?
        let number = self.allocate_object_id();
        object.metadata_mut().object_identifier = Some(number);
        self.indirect_pdf_objects.push(object);

        number
    }

    pub fn add_page(&mut self, mut page: PageObject) {
        page.set_id((self.next_page_num() as u64).into());
        self.root_page_tree.add_page(page);
    }

    fn write_common(&mut self) {
        let resources_number = self.add_font_resources();
        self.initialize_page_tree(resources_number);
        self.initialize_catalog();
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
        let mut resources_dict = PdfDictionaryObject::new();
        resources_dict.add("Font", Pdf::dict(Fonts::get_standard_fonts_dict()));

        self.indirect_pdf_objects.push(resources_dict.boxed());

        let resources_number = self.allocate_object_id();
        resources_dict.metadata.object_identifier = Some(resources_number);

        resources_number
    }

    pub fn initialize_page_tree(&mut self, resources_number: usize) {
        if self.root_page_tree.metadata.object_identifier.is_some() {
            return;
        }

        // Ensure page tree has a MediaBox if no pages have one
        // every page must have MediaBox (direct or inherited)
        if self.root_page_tree.media_box.is_none() {
            let has_page_with_mediabox = self.root_page_tree.kids.iter().any(|kid| {
                if let PageTreeItem::Page(page) = kid {
                    page.media_box.is_some()
                } else {
                    false
                }
            });
            if !has_page_with_mediabox {
                // Set default A4 size
                self.root_page_tree.media_box = Some(crate::page::PageSize::A4);
            }
        }

        // Count pages and allocate IDs
        let num_pages = self.root_page_tree.kids.len();
        let mut page_ids = Vec::new();
        for _ in 0..num_pages {
            page_ids.push(self.allocate_object_id());
        }

        // Allocate ID for page tree itself (after all pages)
        let pages_number = self.allocate_object_id();
        self.root_page_tree.metadata.object_identifier = Some(pages_number);

        // Now assign IDs and clone pages
        let mut page_objects = Vec::new();
        let mut page_idx = 0;
        for kid in &mut self.root_page_tree.kids {
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
            self.indirect_pdf_objects.push(Box::new(page));
        }

        // Now add the page tree itself
        // Clone the page tree to add it to objects
        let page_tree_clone = PageTree {
            id: self.root_page_tree.id.clone(),
            parent_id: self.root_page_tree.parent_id.clone(),
            kids: self.root_page_tree.kids.clone(),
            media_box: self.root_page_tree.media_box,
            resources: self.root_page_tree.resources.clone(),
        };
        self.indirect_pdf_objects.push(Box::new(page_tree_clone));
    }

    pub fn initialize_catalog(&mut self) {
        if self.catalog.metadata.object_identifier.is_some() {
            return;
        }

        self.catalog.metadata.object_identifier = Some(self.allocate_object_id());

        // Add reference to page tree
        let pages_id = self.root_page_tree.metadata.object_identifier.unwrap();
        self.catalog.add_indirect_norm("Pages", pages_id);

        let catalog_copy = self.catalog.clone();
        self.indirect_pdf_objects.push(Box::new(catalog_copy));
    }
}
