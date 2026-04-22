/*
 Tree

 NodeTypes: Root, Int (intermediate), Leaf

 Tree must have at least one root node with a single entry (either Kids or Names)

 ===========================================================================================
 Dict Key  Root  Int   Leaf  Value (flat array)
 ========  ====  ====  ====  ===============================================================
 Kids      Cond  Reqd  None  Cond = no Data. Indirect references to Int or Leaf nodes
 *         Cond  None  Reqd  Cond = no Kids. Key** /Value(PdfObject) pairs, sorted*** on key
 Limits    None  Reqd  Reqd  Least and Greatest keys****
============================================================================================

 ===================================================================================
        NameTree                             NumberTree
 ====   ==================================   =======================================
 *      "Names"                              "Nums"
 **     string                               integer
 ***    Shortest first                       Alphabetically sorted, lowest first
 ****   Strings, lexical sort, short first   Integers, numerical sort, lowest first
====================================================================================

 Object References
 ===================================== Stream?
 Indirect: Dict, Array, String
 Direct:   Null, Number, Boolean, Name
 =====================================
 */
use crate::objects::pdf_object::PdfObj;
use crate::{PdfArrayObject, PdfDictionaryObject, PdfError, PdfObject};

///Usage:
/// ```
///  let mut name_node: NameTreeNode = TreeNode::new(1);
///  node.set_entries(vec![("key".to_string(), val)]);
///  node.set_limits("a".to_string(), "z".to_string());
///
///  let mut num_node: NumTreeNode = TreeNode::new(2);
///  node.set_entries(vec![(42, val)]);
///  node.set_limits(1, 99);
/// ```

//------------------------ Tree -----------------------------//

pub struct Tree {
    pub(crate) dict: PdfDictionaryObject,
}

impl Tree {
    pub fn new(object_number: u64) -> Self {
        Self {
            dict: PdfDictionaryObject::new().with_object_number(object_number),
        }
    }

    pub fn set_kids(&mut self, kids: Vec<u64>) -> Result<(), PdfError> {
        let mut arr = PdfArrayObject::new();
        for kid in kids {
            arr.push(PdfObj::make_reference_obj(kid));
        }
        self.dict.add("Kids", arr)?;

        Ok(())
    }

    pub fn set_entries<K: TreeKey>(
        &mut self,
        entries: Vec<(K, PdfObject)>,
    ) -> Result<(), PdfError> {
        let mut arr = PdfArrayObject::new();
        for (key, val) in entries {
            arr.push(key.to_pdf_obj());
            arr.push(val);
        }
        self.dict.add(K::entry_key_name(), arr)?;

        Ok(())
    }

    pub fn set_limits<K: TreeKey>(&mut self, least: K, greatest: K) -> Result<(), PdfError> {
        let mut arr = PdfArrayObject::new();
        arr.push(least.to_pdf_obj());
        arr.push(greatest.to_pdf_obj());
        self.dict.add("Limits", arr)?;

        Ok(())
    }
}

//------------------------ TreeKey -----------------------------//

pub trait TreeKey: Sized {
    fn to_pdf_obj(self) -> PdfObject;
    fn entry_key_name() -> &'static str;
}

impl TreeKey for String {
    fn to_pdf_obj(self) -> PdfObject {
        PdfObj::make_string_obj(&self)
    }
    fn entry_key_name() -> &'static str {
        "Names"
    }
}

impl TreeKey for i64 {
    fn to_pdf_obj(self) -> PdfObject {
        PdfObj::make_num_obj(self)
    }
    fn entry_key_name() -> &'static str {
        "Nums"
    }
}

