use std::rc::Rc;
use crate::{ArrayObject, NumberObject, PdfObject};

//--------------------------- Page Size ---------------------------//

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PageSize {
    A4,
    Letter,
    Legal,
    A3,
    Custom(f64, f64), // width, height in points
}

impl Default for PageSize {
    fn default() -> Self {
        PageSize::A4
    }
}

impl PageSize {
    /// Returns the [width, height] in PDF points (1 PDF point = 1/72 inch).
    /// Returns 0.0 for negative custom dimensions.
    pub fn dimensions(&self) -> (f64, f64) {
        match self {
            PageSize::A4 => (595.0, 842.0),
            PageSize::Letter => (612.0, 792.0),
            PageSize::Legal => (612.0, 1008.0),
            PageSize::A3 => (842.0, 1191.0),
            PageSize::Custom(w, h) => (w.max(0.0), h.max(0.0)),
        }
    }

    pub fn as_array(&self) -> ArrayObject {
        let (width, height) = self.dimensions();
        ArrayObject::new(Some(vec![
            Rc::new(NumberObject::from(0.0)) as Rc<dyn PdfObject>,
            Rc::new(NumberObject::from(0.0)),
            Rc::new(NumberObject::from(width)),
            Rc::new(NumberObject::from(height)),
        ]))
    }
}
