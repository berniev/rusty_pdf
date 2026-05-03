use std::fs::File;
use std::io::{Seek, Write};
use image::EncodableLayout;
use crate::{NumberType, PdfArrayObject, PdfBooleanObject, PdfDictionaryObject, PdfError, PdfNameObject, PdfNullObject, PdfReferenceObject, PdfStreamObject, PdfStringObject};
use crate::generation::Generation;
use crate::objects::pdf_number::PdfNumberObject;
use crate::version::Version;
use crate::xref_ops::{ObjectStatus, XRefEntry, XRefOps};

pub struct ObjectOps {
    version: Version,
    last_object_number: ObjectNumber,
}

impl ObjectOps {
    pub fn new(version: Version) -> Self {
        Self {
            version,
            // 0 is in xref table as 'free'. is gen# 65535, else 0 for new
            last_object_number: ObjectNumber::new(0),
        }
    }

    pub fn last_object_number(&self) -> ObjectNumber {
        self.last_object_number
    }

    pub fn next_object_number(&mut self) -> ObjectNumber {
        self.last_object_number.object_number += 1;

        self.last_object_number
    }
}

#[derive(Clone, Copy, Debug)]
pub struct ObjectNumber {
    object_number: u64,
}

impl ObjectNumber {
    pub fn new(value: u64) -> Self {
        Self {
            object_number: value,
        }
    }

    pub fn value(self) -> u64 {
        self.object_number
    }

    pub fn to_string(&self) -> String {
        self.object_number.to_string()
    }
}

impl PartialEq for ObjectNumber {
    fn eq(&self, other: &Self) -> bool {
        self.value() == other.value()
    }
}

impl Eq for ObjectNumber {}

impl PartialOrd for ObjectNumber {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ObjectNumber {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.value().cmp(&other.value())
    }
}

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

macro_rules! match_pdf_object {
    ($self:expr, $x:ident => $body:expr) => {
        match $self {
            PdfObject::Array($x) => $body,
            PdfObject::Boolean($x) => $body,
            PdfObject::Dictionary($x) => $body,
            PdfObject::Name($x) => $body,
            PdfObject::Null($x) => $body,
            PdfObject::Number($x) => $body,
            PdfObject::Reference($x) => $body,
            PdfObject::Stream($x) => $body,
            PdfObject::String($x) => $body,
        }
    };
}

impl PdfObject {
    pub fn type_name(&self) -> &'static str {
        match self {
            PdfObject::Array(_) => "Array",
            PdfObject::Boolean(_) => "Boolean",
            PdfObject::Dictionary(_) => "Dictionary",
            PdfObject::Name(_) => "Name",
            PdfObject::Null(_) => "Null",
            PdfObject::Number(_) => "Number",
            PdfObject::Reference(_) => "Reference",
            PdfObject::Stream(_) => "Stream",
            PdfObject::String(_) => "String",
        }
    }

    pub fn as_integer(&self) -> Result<i64, PdfError> {
        match self {
            PdfObject::Number(n) => Ok(n.as_int()),
            other => Err(Self::unexpected_type(other)),
        }
    }

    pub fn as_f64(&self) -> Result<f64, PdfError> {
        match self {
            PdfObject::Number(n) => Ok(n.as_real()),
            other => Err(Self::unexpected_type(other)),
        }
    }

    pub fn as_vec_f64(&self) -> Result<Vec<f64>, PdfError> {
        match self {
            PdfObject::Array(a) => a.to_vec_f64(),
            other => Err(Self::unexpected_type(other)),
        }
    }

    pub fn as_string(&self) -> Result<&str, PdfError> {
        match self {
            PdfObject::String(s) => Ok(s.value.as_str()),
            other => Err(Self::unexpected_type(other)),
        }
    }

    pub fn as_name(&self) -> Result<Vec<u8>, PdfError> {
        match self {
            PdfObject::Name(n) => Ok(n.value.clone()),
            other => Err(Self::unexpected_type(other)),
        }
    }

    pub fn as_dict(&self) -> Result<&PdfDictionaryObject, PdfError> {
        match self {
            PdfObject::Dictionary(d) => Ok(d),
            other => Err(Self::unexpected_type(other)),
        }
    }

    fn unexpected_type(&self) -> PdfError {
        PdfError::StructureError(format!("Unexpected type: {}", self.type_name()))
    }

    pub fn serialise(&self, xref: &mut XRefOps, file: &mut File) -> Result<(), PdfError> {
        if self.is_reference() || self.is_direct() {
            return Ok(());
        }

        // indirect object

        let object_number = self.get_object_number().unwrap();

        let mut vec = vec![];
        vec.extend(object_number.to_string().as_bytes());
        vec.extend(b" 0 obj\n");
        vec.extend(match_pdf_object!(self, x => x.encode())?);
        vec.extend(b"endobj\n\n");
        file.write_all(&*vec)?;

        let xref_ent = XRefEntry::new(
            object_number,
            file.stream_position()?,
            ObjectStatus::InUse,
            Generation::Normal,
        );
        xref.add_entry(xref_ent);

        Ok(())
    }

    pub fn encode(&self) -> Result<Vec<u8>, PdfError> {
        if self.is_indirect() && !self.is_reference() {
            return PdfReferenceObject::new(self.get_object_number().unwrap()).encode();
        }
        
        match_pdf_object!(&self, x => x.encode())
    }

    pub fn get_object_number(&self) -> Option<ObjectNumber> {
        match self {
            PdfObject::Array(x) => x.object_number,
            PdfObject::Dictionary(x) => x.object_number,
            PdfObject::Stream(x) => x.object_number,
            PdfObject::Reference(x) => x.object_number,
            _ => None,
        }
    }

    pub fn is_indirect(&self) -> bool {
        self.get_object_number().is_some()
    }

    pub fn is_reference(&self) -> bool {
        matches!(self, PdfObject::Reference(_))
    }

    pub fn is_direct(&self) -> bool {
        !self.is_indirect()
    }
    pub fn reference_obj(value: ObjectNumber) -> PdfObject {
        PdfObject::Reference(PdfReferenceObject::new(value))
    }

    pub fn null_obj() -> PdfObject {
        PdfObject::Null(PdfNullObject::new())
    }

    pub fn num_obj(value: impl Into<NumberType>) -> PdfObject {
        PdfObject::Number(PdfNumberObject::new(value.into()))
    }

    pub fn num_or_null_obj<T: Into<NumberType>>(value: Option<T>) -> PdfObject {
        match value {
            Some(v) => Self::num_obj(v),
            None => Self::null_obj(),
        }
    }

    // disambiguate name from string
    pub fn name_obj(value: &str) -> PdfObject {
        PdfObject::Name(PdfNameObject::new(value))
    }

    pub fn string_obj(value: &str) -> PdfObject {
        PdfObject::String(PdfStringObject::new(value))
    }

    pub fn string_text_obj(value: &str) -> PdfObject {
        PdfObject::String(PdfStringObject::new(value))
    }
}

//--------------------------- From impl -------------------------//

impl From<PdfArrayObject> for PdfObject {
    fn from(v: PdfArrayObject) -> Self {
        PdfObject::Array(v)
    }
}

impl From<PdfBooleanObject> for PdfObject {
    fn from(v: PdfBooleanObject) -> Self {
        PdfObject::Boolean(v)
    }
}

impl From<PdfDictionaryObject> for PdfObject {
    fn from(v: PdfDictionaryObject) -> Self {
        PdfObject::Dictionary(v)
    }
}

impl From<PdfNameObject> for PdfObject {
    fn from(v: PdfNameObject) -> Self {
        PdfObject::Name(v)
    }
}

impl From<PdfNullObject> for PdfObject {
    fn from(v: PdfNullObject) -> Self {
        PdfObject::Null(v)
    }
}

impl From<PdfNumberObject> for PdfObject {
    fn from(v: PdfNumberObject) -> Self {
        PdfObject::Number(v)
    }
}

impl From<PdfReferenceObject> for PdfObject {
    fn from(v: PdfReferenceObject) -> Self {
        PdfObject::Reference(v)
    }
}

impl From<PdfStreamObject> for PdfObject {
    fn from(v: PdfStreamObject) -> Self {
        PdfObject::Stream(v)
    }
}

impl From<PdfStringObject> for PdfObject {
    fn from(v: PdfStringObject) -> Self {
        PdfObject::String(v)
    }
}

impl From<String> for PdfObject {
    fn from(v: String) -> Self {
        PdfObject::String(PdfStringObject::new(&v))
    }
}

impl From<&str> for PdfObject {
    fn from(v: &str) -> Self {
        PdfObject::String(PdfStringObject::new(&v))
    }
}

impl From<Vec<u32>> for PdfObject {
    fn from(v: Vec<u32>) -> Self {
        PdfObject::Array(PdfArrayObject::from_vec_u32(v))
    }
}

impl From<bool> for PdfObject {
    fn from(v: bool) -> Self {
        PdfObject::Boolean(PdfBooleanObject::new(v))
    }
}

impl From<NumberType> for PdfObject {
    fn from(v: NumberType) -> Self {
        PdfObject::Number(PdfNumberObject::new(v))
    }
}

impl From<u8> for PdfObject {
    fn from(v: u8) -> Self {
        PdfObject::from(NumberType::from(v))
    }
}

impl From<u32> for PdfObject {
    fn from(v: u32) -> Self {
        PdfObject::from(NumberType::from(v))
    }
}

impl From<usize> for PdfObject {
    fn from(v: usize) -> Self {
        PdfObject::from(NumberType::from(v))
    }
}

impl From<u64> for PdfObject {
    fn from(v: u64) -> Self {
        PdfObject::from(NumberType::from(v))
    }
}

impl From<i32> for PdfObject {
    fn from(v: i32) -> Self {
        PdfObject::from(NumberType::from(v))
    }
}

impl From<i64> for PdfObject {
    fn from(v: i64) -> Self {
        PdfObject::from(NumberType::from(v))
    }
}

impl From<f32> for PdfObject {
    fn from(v: f32) -> Self {
        PdfObject::from(NumberType::from(v))
    }
}

impl From<f64> for PdfObject {
    fn from(v: f64) -> Self {
        PdfObject::from(NumberType::from(v))
    }
}

// Array, Dictionary, Reference, Stream
macro_rules! impl_obj_num_gen {
    ($ty:ty) => {
        impl $ty {
            pub fn with_object_number(mut self, value: ObjectNumber) -> Self {
                self.object_number = Some(value);
                self
            }
            pub fn with_generation_number(mut self, value: u16) -> Self {
                self.generation_number = Some(value);
                self
            }
        }
    };
}

impl_obj_num_gen!(PdfArrayObject);
impl_obj_num_gen!(PdfDictionaryObject);
impl_obj_num_gen!(PdfReferenceObject);
impl_obj_num_gen!(PdfStreamObject);

