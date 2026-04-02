use crate::cross_reference_table::ObjectStatus;
use crate::generation::Generation;
use crate::objects::number::PdfNumberObject;
use crate::objects::reference::PdfReferenceObject;
use crate::{
    NumberType, PdfArrayObject, PdfBooleanObject, PdfDictionaryObject, PdfError, PdfNameObject,
    PdfNullObject, PdfStreamObject, PdfStringObject,
};
use std::cmp::PartialEq;

//--------------------------- PdfObj -------------------------//

pub struct PdfObj {}

impl PdfObj {
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
            Some(v) => PdfObj::num(v),
            None => PdfObj::null(),
        }
    }

    pub fn reference(value: u64) -> PdfObject {
        PdfObject::Reference(PdfReferenceObject::new(value))
    }

    pub fn stream(value: PdfStreamObject) -> PdfObject {
        PdfObject::Stream(value)
    }

    pub fn string(value: &str) -> PdfObject {
        PdfObject::String(PdfStringObject::new(value))
    }
}
/*
Is it referenced from more than one place? → indirect (shared fonts, images, etc.)
Does anything need to refer to it by object number? → indirect (e.g. a page in the Kids array)
Is it a stream? → indirect (spec mandates it)
Everything else → direct
*/

// Tracks where an object ended up after serialisation — not intrinsic to the object itself
#[derive(Debug, Clone, PartialEq, Default)]
pub struct SerialLocation {
    pub offset: usize,
    pub status: ObjectStatus, // free or inuse
}

// The PDF spec identity of an __ indirect __ object (§7.3.10)
#[derive(Debug, Clone, PartialEq)]
pub struct ObjectId {
    pub number: usize,          // 0 is root. 1 is first object
    pub generation: Generation, // for obj#0 is 65535, else is 0 for new objects
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
    Reference(PdfReferenceObject),
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
            PdfObject::Reference(r) => r.serialise(),
            PdfObject::Stream(s) => s.serialise(),
            PdfObject::String(sg) => sg.serialise(),
        }
    }

    pub fn get_object_number(&self) -> Option<u64> {
        match self {
            PdfObject::Array(a) => a.object_number,
            PdfObject::Boolean(b) => b.object_number,
            PdfObject::Dictionary(d) => d.object_number,
            PdfObject::Name(na) => na.object_number,
            PdfObject::Null(nu) => nu.object_number,
            PdfObject::Number(m) => m.object_number,
            PdfObject::Reference(r) => r.object_number,
            PdfObject::Stream(s) => s.object_number,
            PdfObject::String(sg) => sg.object_number,
        }
    }

    pub fn set_object_number(&mut self, object_number: u64) {
        match self {
            PdfObject::Array(a) => a.object_number = Some(object_number),
            PdfObject::Boolean(b) => b.object_number = Some(object_number),
            PdfObject::Stream(s) => s.object_number = Some(object_number),
            PdfObject::String(sg) => sg.object_number = Some(object_number),
            PdfObject::Null(nu) => nu.object_number = Some(object_number),
            PdfObject::Number(m) => m.object_number = Some(object_number),
            PdfObject::Dictionary(d) => d.object_number = Some(object_number),
            PdfObject::Name(na) => na.object_number = Some(object_number),
            PdfObject::Reference(r) => r.object_number = Some(object_number),
        }
    }

    // todo: add to body
    // result of this gets added to body
    pub fn serialise_wrapper(&mut self) -> Result<Vec<u8>, PdfError> {
        if let Some(object_number) = self.get_object_number()
        {
            let mut vec = vec![];
            vec.extend(object_number.to_string().as_bytes());
            vec.extend(b" 0 obj\n");
            vec.extend(self.serialise()?);
            vec.extend(b"\nendobj\n");
            // todo: add obj num to xref table

            Ok(vec)
        } else {
            self.serialise()
        }
    }
}
