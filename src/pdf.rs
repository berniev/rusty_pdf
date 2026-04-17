use crate::header::Header;
use crate::object_ops::ObjectOps;
use crate::objects::pdf_object::PdfObj;
use crate::page_ops::PageOps;
use crate::trailer::Trailer;
use crate::version::Version;
use crate::xref_ops::XRefOps;
use crate::{GraphicsOps, PdfDictionaryObject, PdfError, PdfObject};
use std::cell::RefCell;
use std::fs::File;
use std::rc::Rc;

//--------------------------- Pdf -------------------------//

pub struct Pdf {
    pub version: Option<Version>,
    pub object_ops: Rc<RefCell<ObjectOps>>,
    pub page_ops: PageOps,
    pub graphics_ops: GraphicsOps,
}

impl Pdf {
    pub fn new() -> Result<Self, PdfError> {
        let object_ops = Rc::new(RefCell::new(ObjectOps::new()));
        let page_ops = PageOps::new(Rc::clone(&object_ops));
        let graphics_ops = GraphicsOps::new(Rc::clone(&object_ops));

        let pdf = Pdf {
            version: None,
            object_ops,
            page_ops: page_ops?,
            graphics_ops,
        };

        Ok(pdf)
    }

    pub fn having_version(mut self, version: Version) -> Self {
        self.version = Some(version);

        self
    }

    pub fn finalise(&mut self, path: &str) -> Result<(), PdfError> {
        let header = Header::new().with_version(self.version.unwrap_or_default());

        let next_num = self.object_ops.borrow_mut().next_object_number();
        let mut catalog_dict: PdfDictionaryObject = PdfDictionaryObject::new()
            .typed("Catalog")?
            .with_object_number(next_num);
        catalog_dict.add(
            "Pages",
            PdfObj::make_reference_obj(self.page_ops.root_page_tree_dict.object_number.unwrap()),
        )?;
        let catalog_obj = PdfObject::from(catalog_dict.clone());

        let o_siz = self.object_ops.borrow().last_object_number() + 1;
        let trailer = Trailer::new().with_size(o_siz)?.with_root(next_num)?;

        //

        let mut xref_ops = XRefOps::new();
        let mut file = File::create(path)?;
        header.serialise(&mut file)?;
        catalog_obj.serialise(&mut xref_ops, &mut file)?;
        self.page_ops.serialise(&mut xref_ops, &mut file)?;
        xref_ops.serialise(&mut file)?;
        trailer.serialise(&mut xref_ops, &mut file)?;

        Ok(())
    }
}

pub enum Strategy {
    Legacy,
    Compressed,
}
