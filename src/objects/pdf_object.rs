use crate::{NumberType, PdfArrayObject, PdfBooleanObject, PdfDictionaryObject, PdfError, PdfNameObject, PdfNullObject, PdfStreamObject, PdfStringObject};
use crate::objects::number::PdfNumberObject;
//--------------------------- Pdf -------------------------//

pub struct Pdf {}

impl Pdf {
    pub fn array(value: PdfArrayObject) -> PdfObject {
        PdfObject::Array(value)
    }

    pub fn bool(value: bool) -> PdfObject {
        PdfObject::Boolean(PdfBooleanObject::new(value))
    }

    pub fn dict(value: PdfDictionaryObject) -> PdfObject {
        PdfObject::Dictionary(value)
    }

    pub fn name(value: &str) -> PdfObject {
        PdfObject::Name(PdfNameObject::new(value))
    }

    pub fn null() -> PdfObject {
        PdfObject::Null(PdfNullObject::new())
    }

    pub fn num(value: impl Into<NumberType>) -> PdfObject {
        PdfObject::Number(PdfNumberObject::new(value.into()))
    }

    pub fn num_or_null<T: Into<NumberType>>(value: Option<T>) -> PdfObject {
        match value {
            Some(v) => Pdf::num(v),
            None => Pdf::null(),
        }
    }

    pub fn stream(value: PdfStreamObject) -> PdfObject {
        PdfObject::Stream(value)
    }

    pub fn string(value: &str) -> PdfObject {
        PdfObject::String(PdfStringObject::new(value))
    }
}

//--------------------------- PdfObject -------------------------//

#[derive(Clone)]
pub enum PdfObject {
    Array(PdfArrayObject),
    Boolean(PdfBooleanObject),
    Dictionary(PdfDictionaryObject),
    Name(PdfNameObject),
    Null(PdfNullObject),
    Number(PdfNumberObject),
    Stream(PdfStreamObject),
    String(PdfStringObject),
}

impl PdfObject {
    pub fn serialise(&mut self) -> Result<Vec<u8>, PdfError> {
        match self {
            PdfObject::Array(a) => a.serialise(),
            PdfObject::Boolean(b) => b.serialise(),
            PdfObject::Dictionary(d) => d.serialise(),
            PdfObject::Name(na) => na.serialise(),
            PdfObject::Null(nu) => nu.serialise(),
            PdfObject::Number(m) => m.serialise(),
            PdfObject::Stream(s) => s.serialise(),
            PdfObject::String(sg) => sg.serialise(),
        }
    }

    pub fn is_indirect_by_default(&self) -> bool {
        match self {
            PdfObject::Array(_) => true,
            PdfObject::Boolean(_) => false,
            PdfObject::Dictionary(_) => true,
            PdfObject::Name(_) => false,
            PdfObject::Number(_) => false,
            PdfObject::Null(_) => false,
            PdfObject::Stream(_) => true,
            PdfObject::String(_) => false,
        }
    }
}
