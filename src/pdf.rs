use crate::catalog_ops::CatalogOps;
use crate::object_ops::ObjectOps;
use crate::page_ops::PageOps;
use crate::trailer::Trailer;
use crate::version::Version;
use crate::xref_ops::XRefOps;
use crate::{GraphicsOps, PageSize, PdfError, header};
use std::cell::RefCell;
use std::fs::File;
use std::rc::Rc;

pub struct Pdf {
    pub version: Version,
    pub object_ops: Rc<RefCell<ObjectOps>>,
    pub page_ops: PageOps,
    pub graphics_ops: GraphicsOps,
}

impl Pdf {
    pub fn new() -> Result<Self, PdfError> {
        let object_ops = Rc::new(RefCell::new(ObjectOps::new()));
        let page_ops = PageOps::new(Rc::clone(&object_ops))?;
        let graphics_ops = GraphicsOps::new(Rc::clone(&object_ops));

        let pdf = Pdf {
            version: Version::default(),
            object_ops,
            page_ops,
            graphics_ops,
        };

        Ok(pdf)
    }

    pub fn having_version(mut self, version: Version) -> Self {
        self.version = version;

        self
    }

    pub fn with_default_page_size(mut self, page_size: PageSize) -> Self {
        self.page_ops.set_default_page_size(page_size);

        self
    }

    pub fn finalize(&mut self, path: &str) -> Result<(), PdfError> {
        let mut build = || {
            let catalog_object_number = self.object_ops.borrow_mut().next_object_number();
            let mut catalog_ops =
                CatalogOps::new(catalog_object_number.clone(), &mut self.page_ops)?;
            let trailer = Trailer::new(
                self.object_ops.borrow().last_object_number(),
                catalog_object_number,
            )?;
            let mut xref_ops = XRefOps::new();
            let mut file = File::create(path)?;

            header::serialize(self.version, &mut file)?;
            catalog_ops.serialize(self.version, &mut xref_ops, &mut file)?;
            self.page_ops
                .serialize(self.version, &mut xref_ops, &mut file)?;
            xref_ops.serialize(&mut file)?;
            trailer.serialize(self.version, &mut xref_ops, &mut file)?;

            Ok(())
        };

        let res = build();
        if res.is_err() {
            let _ = std::fs::remove_file(path);
        }

        res
    }
}

pub enum Strategy {
    Legacy,
    Compressed,
}
