use crate::PdfStreamObject;
use crate::object_ops::{ObjectNumber, ObjectOps};
use crate::objects::pdf_object::PdfObj;
pub use crate::page_size::PageSize;
use crate::xref_ops::XRefOps;
use crate::{PdfArrayObject, PdfDictionaryObject, PdfError};
use std::cell::RefCell;
use std::fs::File;
use std::rc::Rc;

/// Page: (pdf dictionary)
///
/// The attributes of a page, organized into various categories (e.g., Font, ColorSpace, Pattern)
///
///     A page object may not have children.
///
/// I = Can be inherited from parent pageTree heirarchy, which satisfies R (Reqd).
///

///```
/// Page dict entries:
/// ==============================================================================
/// Name                  Ver  R  I  Type      Value
/// ====================  ===  =  =  ========  ===================================
/// Type                       R     name      "Page"
/// Parent                     R     dict      indirect reference
/// LastModified               *     date      * Reqd if PieceInfo
/// Resources                  R  I  dict      Reqd if not inherited
/// MediaBox                   R  I  rect      Reqd if not inherited
///
/// Annots                     O     array
/// Contents                   O     stream or array
/// CropBox                    O  I  rect
/// Rotate                     O  I  int
/// Thumb                      O     stream
/// Trans                      O     dict
///
/// B                     1.1  O     array
/// Dur                   1.1  O     number
///
/// AA                    1.2  O     dict
///
/// ArtBox                1.3  O     rect
/// BleedBox              1.3  O     rect
/// ID byte               1.3  O     string
/// PieceInfo             1.3  O     dict
/// PZ                    1.3  O     number
/// SeparationInfo        1.3  O     dict
/// StructParents         1.3  *     int         Reqd if struct content items
/// TrimBox               1.3  O     rect
///
/// BoxColorInfo          1.4  O     dict
/// Group                 1.4  O     dict
/// Metadata              1.4  O     stream
///
/// PresSteps             1.5  O     dict
/// Tabs                  1.5  O     name
/// TemplateInstantiated  1.5  O     name
///
/// UserUnit              1.6  O     numb
/// VP                    1.6  O     dict
/// ==============================================================================
///```

///```
/// PageTreeNode dict entries:
/// ========  ==========  =====  ===  ===========================================
/// Name      PdfObjType  Reqd   Inh  Value
/// ========  ==========  =====  ===  ===========================================
/// Type      Name        Reqd        "Pages"
/// Parent    Indirect    Reqd*       Parent PageTree. * Not allowed in root node
/// Kids      Array       Reqd        Indirect references to descendant pages
/// Count     Integer     Reqd        Number of descendant pages
///
/// Resources Dictionary  Opt    Inh
/// MediaBox  Rectangle   Opt    Inh
/// CropBox   Rectangle   Opt    Inh
/// Rotate    Integer     Opt    Inh
/// =============================================================================
///```
///

pub struct PageOps {
    pub root_tree: PageTree,
}

impl PageOps {
    pub fn new(object_ops: Rc<RefCell<ObjectOps>>) -> Result<Self, PdfError> {
        let mut root_tree = PageTree::new(
            object_ops.clone(),
            object_ops.borrow_mut().next_object_number(),
        )?;
        let mut resources = PdfDictionaryObject::new();
        resources.add("Font", crate::fonts::standard_fonts_dict()?)?;
        root_tree.dictionary.add("Resources", resources)?;
        root_tree
            .dictionary
            .add("MediaBox", PageSize::default().to_rect())?;

        Ok(PageOps { root_tree })
    }

    pub fn set_default_page_size(&mut self, page_size: PageSize) {
        self.root_tree.dictionary.update_or_add("MediaBox", page_size.to_rect());
    }

    pub fn root_tree(&mut self) -> &mut PageTree {
        &mut self.root_tree
    }

    pub fn serialise(&mut self, xref: &mut XRefOps, file: &mut File) -> Result<(), PdfError> {
        PageTree::update_counts(&mut self.root_tree.dictionary);
        self.root_tree.dictionary.serialise(xref, file)
    }
}

pub struct PageTree {
    dictionary: PdfDictionaryObject,
    object_ops: Rc<RefCell<ObjectOps>>,
}

impl PageTree {
    fn new(
        object_ops: Rc<RefCell<ObjectOps>>,
        object_number: ObjectNumber,
    ) -> Result<Self, PdfError> {
        let mut dict = PdfDictionaryObject::new()
            .typed("Pages")?
            .with_object_number(object_number);
        dict.add("Kids", PdfArrayObject::new())?;
        dict.add("Count", 0)?;

        Ok(PageTree {
            dictionary: dict,
            object_ops,
        })
    }

    pub fn with_page_size(mut self, page_size: PageSize) -> Self {
        self.dictionary
            .update_or_add("MediaBox", page_size.to_rect());
        self
    }

    pub fn make_page(&self, content: Vec<u8>) -> Result<Page, PdfError> {
        Page::new(self.object_ops.clone(), content)
    }

    pub fn make_tree(&self) -> Result<PageTree, PdfError> {
        PageTree::new(
            self.object_ops.clone(),
            self.object_ops.borrow_mut().next_object_number(),
        )
    }

    // NB: Tree must be added AFTER its constituent pages are defined
    pub fn add_tree(&mut self, mut tree: PageTree) -> Result<(), PdfError> {
        self.has_kids()?;

        tree.dictionary
            .add("Parent", PdfObj::make_reference_obj(self.object_number()))?;

        self.dictionary
            .add_kid_to_page_tree(Box::new(tree.dictionary))?;

        Ok(())
    }

    pub fn add_page(&mut self, mut page: Page) -> Result<(), PdfError> {
        page.dictionary
            .add("Parent", PdfObj::make_reference_obj(self.object_number()))?;
        self.dictionary.update_or_add(
            "Count",
            self.dictionary.get_integer("Count").unwrap_or(0) + 1,
        );
        self.dictionary
            .add_kid_to_page_tree(Box::new(page.dictionary))?;

        Ok(())
    }

    pub fn add_resources() {}

    fn has_kids(&self) -> Result<(), PdfError> {
        if !self.dictionary.contains_key("Kids") {
            return Err(PdfError::StructureError(
                "Parent page tree must have a Kids array".to_string(),
            ));
        }
        Ok(())
    }

    pub fn object_number(&self) -> ObjectNumber {
        self.dictionary.object_number.unwrap()
    }

    fn update_counts(dict: &mut PdfDictionaryObject) {
        let mut count = 0i64;
        for child in &mut dict.children {
            match child.get_name("Type") {
                Ok("Page") => count += 1,
                Ok("Pages") => {
                    Self::update_counts(child);
                    count += child.get_integer("Count").unwrap_or(0);
                }
                _ => {}
            }
        }
        dict.update_or_add("Count", count);
    }
}

pub struct Page {
    dictionary: PdfDictionaryObject,
}

impl Page {
    fn new(object_ops: Rc<RefCell<ObjectOps>>, content: Vec<u8>) -> Result<Self, PdfError> {
        let mut dict = PdfDictionaryObject::new()
            .typed("Page")?
            .with_object_number(object_ops.clone().borrow_mut().next_object_number());

        let stream = PdfStreamObject::new()
            .with_object_number(object_ops.borrow_mut().next_object_number())
            .with_data(content);
        dict.add("Contents", stream)?;

        Ok(Page { dictionary: dict })
    }

    pub fn with_page_size(mut self, page_size: PageSize) -> Self {
        self.dictionary
            .update_or_add("MediaBox", page_size.to_rect());
        self
    }
}
