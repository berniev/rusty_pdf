use crate::cross_reference_table::CrossRefTable;
use crate::header::Header;
use crate::object_ops::ObjectOps;
use crate::page_ops::PageOps;
//use crate::trailer::Trailer;
use crate::version::Version;
use crate::{PdfDictionaryObject, PdfError};
use std::cell::RefCell;
use std::fs::File;
use std::rc::Rc;
//--------------------------- Pdf -------------------------//

pub struct Pdf {
    pub header: Header,
    catalog_dict: PdfDictionaryObject,
    root_page_tree_dict: PdfDictionaryObject,
    pub xref_table: CrossRefTable,
    //object_ops: Rc<RefCell<ObjectOps>>,
    page_ops: PageOps,
}

impl Pdf {
    pub fn new() -> Self {
        let object_ops = Rc::new(RefCell::new(ObjectOps::new()));
        let page_ops = PageOps::new(Rc::clone(&object_ops));

        let mut pdf = Pdf {
            header: Header::new(),
            catalog_dict: PdfDictionaryObject::new().typed("Catalog"), // serialises into body
            root_page_tree_dict: PdfDictionaryObject::new(),
            xref_table: CrossRefTable::new(), // buffers xref until body is complete, then appended
            //object_ops,
            page_ops,
        };
        pdf.root_page_tree_dict = pdf.page_ops.new_tree();

        pdf
    }

    pub fn version(mut self, version: Version) -> Self {
        self.header.set_version(version);

        self
    }

    pub fn catalog_dict_ref(&mut self) -> &mut PdfDictionaryObject {
        &mut self.catalog_dict
    }

    pub fn root_page_tree_dict_ref(&mut self) -> &mut PdfDictionaryObject {
        &mut self.root_page_tree_dict
    }

    pub fn xref_table_ref(&mut self) -> &mut CrossRefTable {
        &mut self.xref_table
    }

    pub fn finalise(&mut self, path: &str) -> Result<(), PdfError> {
        let mut file = File::create(path)?;

        self.header.serialise(&mut self.xref_table, &mut file)?;
        //let osiz = self.object_ops.borrow_mut().last_object_number();
        //let onum = self.root_page_tree_dict_ref().object_number.unwrap();

        self.xref_table_ref().serialise(&mut file)?;

        //let trailer = Trailer::new().with_size(osiz).with_root(onum);

        //trailer.serialise(&mut self.xref_table, &mut file)?;

        Ok(())
    }
}

pub enum Strategy {
    Legacy,
    Compressed,
}
