/*
 Node: TreeNode
 NodeTypes: Root, Int (intermediate), Leaf

 Tree
 Must have at least one root node with a single entry (either Kids or Names)

 =================================================================================================
 Dict Key  Root  Int   Leaf  Value (flat array)
 ========  ====  ====  ====  =====================================================================
 Kids      Cond  Reqd  None  Cond = no Data. Indirect references to Int or Leaf nodes
 Title*    Cond  None  Reqd  Cond = no Kids. Key** /Value(PdfObject) pairs, sorted*** on key
 Limits    None  Reqd  Reqd  Least and Greatest keys****
===================================================================================================
        Name Tree                            Number Tree
 ====   ==================================   ==============================
 *      "Names"                              "Nums"
 **     string                               integer
 ***    Shortest first                       Alphabetically sorted, lowest first
 ****   Strings, lexical sort, short first   Integers, numerical sort, lowest first
===================================================================================================

 Object References
 ===================================== Stream?
 Indirect: Dict, Array, String
 Direct:   Null, Number, Boolean, Name
 =====================================
 */
use crate::objects::pdf_object::PdfObj;
use crate::{PdfArrayObject, PdfDictionaryObject, PdfObject};

///Usage:
/// ```
///  let mut node: NameTreeNode = TreeNode::new(1);
///  node.set_entries(vec![("key".to_string(), val)]);
///  node.set_limits("a".to_string(), "z".to_string());
///
///  let mut node: NumTreeNode = TreeNode::new(2);
///  node.set_entries(vec![(42, val)]);
///  node.set_limits(1, 99);
/// ```

//------------------------ TreeNode -----------------------------//

pub trait TreeKey: Sized {
    fn to_pdf_obj(self) -> PdfObject;
    fn entry_key_name() -> &'static str;
}

impl TreeKey for String {
    fn to_pdf_obj(self) -> PdfObject { PdfObj::string(&self) }
    fn entry_key_name() -> &'static str { "Names" }
}

impl TreeKey for i64 {
    fn to_pdf_obj(self) -> PdfObject { PdfObj::num(self) }
    fn entry_key_name() -> &'static str { "Nums" }
}

pub struct TreeNode {
    pub(crate) dict: PdfDictionaryObject,
}

impl TreeNode {
    pub fn new(object_number: u64) -> Self {
        Self {
            dict: PdfDictionaryObject::new().with_object_number(object_number),
        }
    }

    pub fn set_kids(&mut self, kids: Vec<u64>) {
        let mut arr = PdfArrayObject::new();
        for n in kids { arr.push(PdfObj::reference(n)); }
        self.dict.add("Kids", PdfObj::array(arr));
    }

    pub fn set_entries<K: TreeKey>(&mut self, entries: Vec<(K, PdfObject)>) {
        let mut arr = PdfArrayObject::new();
        for (k, v) in entries {
            arr.push(k.to_pdf_obj());
            arr.push(v);
        }
        self.dict.add(K::entry_key_name(), PdfObj::array(arr));
    }

    pub fn set_limits<K: TreeKey>(&mut self, least: K, greatest: K) {
        let mut arr = PdfArrayObject::new();
        arr.push(least.to_pdf_obj());
        arr.push(greatest.to_pdf_obj());
        self.dict.add("Limits", PdfObj::array(arr));
    }
}
