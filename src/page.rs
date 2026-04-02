/// Page: (pdf dictionary)
///
/// The attributes of a page, organized into various categories (e.g., Font, ColorSpace, Pattern)
///
///     A page object may not have children.
///
/// Inh = Can be inherited from parent pageTree heirarchy, which satisfies Reqd.
///
/// ====================  ===  ====  ===  ================  ===================================
/// Entry Key             Ver  Reqd  Inh  Type              Value
/// ====================  ===  ====  ===  ================  ===================================
/// Type                       Reqd       name              "Page"
/// Parent                     Reqd       dictionary        indirect reference
/// LastModified               *          date              * Reqd if PieceInfo
/// Resources                  Reqd  Inh  dictionary        Reqd if not inherited
/// MediaBox                   Reqd  Inh  rectangle         Reqd if not inherited
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

//==============================================================================================//

/// PageTree: (pdf dictionary)
///
/// Nodes:
///
/// ========  ==========  =====  ===  ===============================================
/// Name      PdfObjType  Reqd   Inh  Value
/// ========  ==========  =====  ===  ===============================================
/// Type      Name        Reqd        "Pages"
/// Parent    Indirect    Reqd*       Parent PageTree. * Not allowed in root node.
/// Kids      Array       Reqd        Indirect references to descendant pages
/// Count     Integer     Reqd        Number of descendant pages
///
/// Resources Dictionary  Opt    Inh
/// MediaBox  Rectangle   Opt    Inh
/// CropBox   Rectangle   Opt    Inh
/// Rotate    Integer     Opt    Inh
use crate::objects::pdf_object::PdfObj;
pub use crate::page_size::PageSize;
use crate::{PdfArrayObject, PdfDictionaryObject};

//--------------------------- Page ---------------------------//

pub fn make_page(object_number: u64) -> PdfDictionaryObject {
    PdfDictionaryObject::new().typed("Page").with_object_number(object_number)
 }

//--------------------------- PageTree -------------------------//

pub fn make_page_tree(object_number: u64) -> PdfDictionaryObject {
    let mut tree = PdfDictionaryObject::new().typed("Pages").with_object_number(object_number);
    tree.add("Kids", PdfObj::array(PdfArrayObject::new()));
    tree.add("Count", PdfObj::num(0));

    tree
}

//--------------------------- add_page_to_tree -------------------------//

#[allow(unused_variables)]
#[allow(dead_code)]
fn add_page_to_tree(mut page_dict: PdfDictionaryObject, mut tree_dict: PdfDictionaryObject) {
   page_dict.add("Parent", PdfObj::num(0)); // indirect ref to tree
    if !page_dict.contains_key("Resources") && !tree_dict.contains_key("Resources") {}
    if !page_dict.contains_key("MediaBox") && !tree_dict.contains_key("MediaBox") {}
    let curr_count = tree_dict.get_integer("Count").unwrap_or(0);
    //tree_dict.add_ref(page_dict);
    tree_dict.update("Count", PdfObj::num(curr_count + 1));

    /*if let Some(obj) = tree_dict.get_mut("Kids") {
        if let Some(array) = obj.as_any_mut().downcast_mut::<PdfArrayObject>() {
            array.push(Pdf::dict(page_dict));
        }
    }*/
}
