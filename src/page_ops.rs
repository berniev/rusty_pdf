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
use crate::object_ops::ObjectOps;
use crate::objects::pdf_object::PdfObj;
pub use crate::page_size::PageSize;
use crate::{PdfArrayObject, PdfDictionaryObject, PdfError};
use std::cell::RefCell;
use std::rc::Rc;

pub struct PageOps {
    obj_ops: Rc<RefCell<ObjectOps>>,
}

impl PageOps {
    pub fn new(obj_ops: Rc<RefCell<ObjectOps>>) -> Self {
        Self { obj_ops }
    }

    pub fn new_page(&self) -> Result<PdfDictionaryObject, PdfError> {
        Ok(PdfDictionaryObject::new()
            .typed("Page")?
            .with_object_number(self.obj_ops.borrow_mut().next_object_number()))
        
    }

    pub fn new_tree(&self) -> Result<PdfDictionaryObject, PdfError> {
        let mut tree = PdfDictionaryObject::new()
            .typed("Pages")?
            .with_object_number(self.obj_ops.borrow_mut().next_object_number());
        tree.add("Kids", PdfArrayObject::new())?;
        tree.add("Count", 0)?;

        Ok(tree)
    }

    pub fn add_page_to_tree(
        &self,
        mut page_dict: PdfDictionaryObject,
        tree_dict: &mut PdfDictionaryObject,
    ) -> Result<(), PdfError> {
        self.either_dict_has(&page_dict, tree_dict, vec!["Resources", "MediaBox"])?;

        let tree_dict_num = tree_dict.object_number.unwrap();
        page_dict.add("Parent", PdfObj::make_reference_obj(tree_dict_num))?;
        tree_dict.update_or_add("Count", tree_dict.get_integer("Count").unwrap_or(0) + 1);
        tree_dict.add_kid_to_page_tree(Box::new(page_dict))?;

        Ok(())
    }

    pub fn add_tree_to_tree(
        &self,
        child_tree_dict: &PdfDictionaryObject,
        parent_tree_dict: &mut PdfDictionaryObject,
    ) -> Result<(), PdfError> {
        self.dict_has_kids(&parent_tree_dict)?;

        parent_tree_dict.push_to_array(
            "Kids",
            PdfObj::make_reference_obj(child_tree_dict.object_number.unwrap()),
        )?;

        Ok(())
    }

    fn either_dict_has(
        &self,
        page_dict: &PdfDictionaryObject,
        tree_dict: &PdfDictionaryObject,
        expecteds: Vec<&str>,
    ) -> Result<(), PdfError> {
        for expected in expecteds {
            if !page_dict.contains_key(expected) && !tree_dict.contains_key(expected) {
                return Err(PdfError::StructureError(format!(
                    "Page must have, or inherit, a {expected} dictionary"
                )));
            }
        }
        Ok(())
    }

    fn dict_has_kids(&self, dict: &PdfDictionaryObject) -> Result<(), PdfError> {
        if !dict.contains_key("Kids") {
            return Err(PdfError::StructureError(
                "Parent page tree must have a Kids array".to_string(),
            ));
        }
        Ok(())
    }
}
