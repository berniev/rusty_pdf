use crate::catalog::CatalogOps;
use crate::header::Header;
use crate::object_ops::ObjectOps;
use crate::page_ops::PageOps;
use crate::trailer::Trailer;
use crate::version::Version;
use crate::xref_ops::XRefOps;
use crate::{GraphicsOps, PageSize, PdfError};
use std::cell::RefCell;
use std::fs::File;
use std::rc::Rc;

pub struct Pdf {
    pub version: Option<Version>,
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
            version: None,
            object_ops,
            page_ops,
            graphics_ops,
        };

        Ok(pdf)
    }

    pub fn having_version(mut self, version: Version) -> Self {
        self.version = Some(version);

        self
    }

    pub fn with_page_size(mut self, page_size: PageSize) -> Self {
        self.page_ops.set_default_page_size(page_size);
        
        self
    }
    pub fn finalise(&mut self, path: &str) -> Result<(), PdfError> {
        let mut build = || {
            let header = Header::new().with_version(self.version.unwrap_or_default());
            let mut catalog_ops = CatalogOps::new(Rc::clone(&self.object_ops), &mut self.page_ops)?;
            let trailer = Trailer::new(Rc::clone(&self.object_ops), &catalog_ops)?;
            let mut xref_ops = XRefOps::new();
            let mut file = File::create(path)?;

            header.serialise(&mut file)?;
            catalog_ops.serialise(&mut xref_ops, &mut file)?;
            self.page_ops.serialise(&mut xref_ops, &mut file)?;
            xref_ops.serialise(&mut file)?;
            trailer.serialise(&mut xref_ops, &mut file)?;

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
