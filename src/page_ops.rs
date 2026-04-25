use crate::object_ops::{ObjectNumber, ObjectOps};
use crate::objects::pdf_object::PdfObj;
pub use crate::page_size::PageSize;
use crate::xref_ops::XRefOps;
use crate::{PdfArrayObject, PdfDictionaryObject, PdfError};
use crate::PdfStreamObject;
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
        let root_tree = PageTree::new(
            object_ops.clone(),
            object_ops.borrow_mut().next_object_number(),
        )?;
        Ok(PageOps {
            root_tree,
        })
    }

    pub fn root_tree(&mut self) -> &mut PageTree {
        &mut self.root_tree
    }

    pub fn serialise(&self, xref: &mut XRefOps, file: &mut File) -> Result<(), PdfError> {
        self.root_tree.serialise(xref, file)
    }
}

pub struct PageTree {
    dictionary: PdfDictionaryObject,
    object_ops: Rc<RefCell<ObjectOps>>,
    pages: Vec<Page>,
    subtrees: Vec<PageTree>,
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
            pages: vec![],
            subtrees: vec![],
        })
    }

    pub fn make_page(&self, page_size: PageSize, content: Vec<u8>) -> Result<Page, PdfError> {
        Page::new(self.object_ops.clone(), page_size, content)
    }

    pub fn make_tree(&self) -> Result<PageTree, PdfError> {
        PageTree::new(
            self.object_ops.clone(),
            self.object_ops.borrow_mut().next_object_number(),
        )
    }

    pub fn add_tree(&mut self, tree: PageTree) -> Result<(), PdfError> {
        self.has_kids()?;

        self.dictionary
            .push_to_array("Kids", PdfObj::make_reference_obj(tree.object_number()))?;
       self.subtrees.push(tree);
 
        Ok(())
    }

    pub fn add_page(&mut self, mut page: Page) -> Result<(), PdfError> {
        for expected in vec!["Resources", "MediaBox"] {
            if !page.dictionary.contains_key(expected) && !self.dictionary.contains_key(expected) {
                return Err(PdfError::StructureError(format!(
                    "Page must have, or inherit, a {expected} dictionary"
                )));
            }
        }
        let tree_dict_num = self.dictionary.object_number.unwrap();
        page.dictionary
            .add("Parent", PdfObj::make_reference_obj(tree_dict_num))?;
        self.dictionary.update_or_add(
            "Count",
            self.dictionary.get_integer("Count").unwrap_or(0) + 1,
        );
        self.dictionary.push_to_array(
            "Kids",
            PdfObj::make_reference_obj(page.dictionary.object_number.unwrap()),
        )?;
        self.pages.push(page);

        Ok(())
    }

    pub fn add_resources() {}

    pub fn has_kids(&self) -> Result<(), PdfError> {
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

    pub fn serialise(&self, xref: &mut XRefOps, file: &mut File) -> Result<(), PdfError> {
        self.dictionary.serialise(xref, file)
    }
}

pub struct Page {
    dictionary: PdfDictionaryObject,
}

impl Page {
    fn new(
        object_ops: Rc<RefCell<ObjectOps>>,
        page_size: PageSize,
        content: Vec<u8>,
    ) -> Result<Self, PdfError> {
        let mut dict = PdfDictionaryObject::new()
            .typed("Page")?
            .with_object_number(object_ops.clone().borrow_mut().next_object_number());

        dict.add("MediaBox", page_size.to_rect())?;
        dict.add("Resources", PdfDictionaryObject::new())?;

        let stream = PdfStreamObject::new()
            .with_object_number(object_ops.borrow_mut().next_object_number())
            .with_data(content);
        dict.add("Contents", stream)?;

        Ok(Page { dictionary: dict })
    }

    pub fn serialiase(&self, xref: &mut XRefOps, file: &mut File) -> Result<(), PdfError> {
        self.dictionary.serialise(xref, file)
    }
}
