use crate::cross_reference_table::CrossRefTable;
use crate::header::Header;
use crate::object_ops::ObjectOps;
use crate::objects::pdf_object::PdfObj;
use crate::page_ops::PageOps;
use crate::trailer::Trailer;
use crate::version::Version;
use crate::{GraphicsOps, PdfDictionaryObject, PdfError, PdfObject};
use std::cell::RefCell;
use std::fs::File;
use std::rc::Rc;
//--------------------------- Pdf -------------------------//

pub struct Pdf {
    pub header: Header,
    catalog_dict: PdfDictionaryObject,
    pub root_page_tree_dict: PdfDictionaryObject,
    pub xref_table: CrossRefTable,
    pub object_ops: Rc<RefCell<ObjectOps>>,
    pub page_ops: PageOps,
    pub graphics_ops: GraphicsOps,
}

impl Pdf {
    pub fn new() -> Result<Self, PdfError> {
        let object_ops = Rc::new(RefCell::new(ObjectOps::new()));
        let page_ops = PageOps::new(Rc::clone(&object_ops));
        let graphics_ops = GraphicsOps::new(Rc::clone(&object_ops));

        let mut pdf = Pdf {
            header: Header::new(),
            catalog_dict: PdfDictionaryObject::new().typed("Catalog")?, // serialises into body
            root_page_tree_dict: PdfDictionaryObject::new(),
            xref_table: CrossRefTable::new(), // buffers xref until body is complete, then appended
            object_ops,
            page_ops,
            graphics_ops,
        };
        pdf.root_page_tree_dict = pdf.page_ops.new_tree()?;

        Ok(pdf)
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

        self.header.serialise(&mut file)?;

        // Give catalog an object number and link it to the page tree
        let catalog_obj_num = self.object_ops.borrow_mut().next_object_number();
        self.catalog_dict = PdfDictionaryObject::new()
            .typed("Catalog")?
            .with_object_number(catalog_obj_num);
        self.catalog_dict.add(
            "Pages",
            PdfObj::make_reference_obj(self.root_page_tree_dict.object_number.unwrap()),
        )?;

        // Serialise catalog
        let catalog_obj = PdfObject::from(self.catalog_dict.clone());
        catalog_obj.serialise(&mut self.xref_table, &mut file)?;

        self.root_page_tree_dict
            .serialise(&mut self.xref_table, &mut file)?;

        self.xref_table_ref().serialise(&mut file)?;

        let o_siz = self.object_ops.borrow_mut().last_object_number() + 1;

        let trailer = Trailer::new().with_size(o_siz)?.with_root(catalog_obj_num)?;
        trailer.serialise(&mut self.xref_table, &mut file)?;

        Ok(())
    }
}

pub enum Strategy {
    Legacy,
    Compressed,
}
