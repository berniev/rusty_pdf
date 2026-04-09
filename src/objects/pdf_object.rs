// Object:
//     a basic data structure from which PDF files are constructed and includes these types:
//     array, boolean, dictionary, integer, name, null, real, stream and string
//
// Object Reference:
//     an object value used to allow one object to refer to another: “<n> <m> R”
//     where <n> is an indirect object number, <m> is its version number and R is the uppercase R
//
// Object stream:
//     a stream that contains a sequence of PDF objects
//
/*
Direct Objects
==============
Direct objects are defined inline within the content stream or dictionary where they are used.

    Definition: Not surrounded by obj and endobj keywords.
    Storage: They are placed immediately within the parent object (e.g., in an array or dictionary).
    Usage: Used for small, unique, or ephemeral data that does not need to be referenced elsewhere.
    Limitation: They cannot be shared between multiple objects.
    Examples: Integers, real numbers, boolean values, nulls, names, and small arrays/dictionaries
    that are unique to a specific page or location.

Indirect Objects
================
Indirect objects are defined separately in the body of the PDF, outside the main content flow, and
are referenced by other objects.

    Definition: Enclosed within n m obj ... endobj keywords, where n is an object number and m is a
                generation number.
    Reference: Referred to via an "indirect reference" (e.g., 1 0 R).
    Sharing: Can be referenced multiple times by different objects, enabling data sharing (e.g.,
             using the same image on multiple pages).
    Lookup: Located via the cross-reference table (xref) at the end of the file, allowing viewers
            to access them directly without parsing the whole file.
    Examples: The Document Catalog, Pages tree, Font objects, Images, and large dictionaries.

Feature 	 Direct Object	           Indirect Object
===========  ========================  ==================
Location	 Inline (inside parent)	   Separately in body
Identifiers  None	                   Numbered (n m obj)
Shared       No                        Yes
Referenced	 Directly in place	       Via n m R
Access	     Sequential parsing	       Via xref table
Stream	     Rarely (always indirect)  Always

Stream Objects
==============
All streams (used for images, page content, etc.) are indirect objects. This allows them to be
large and shared efficiently, rather than being embedded directly within a parent dictionary.

Summary of Usage
================
A PDF file consists of a mix of both. An indirect object might have a direct object as its value,
or an indirect object might reference another indirect object.

    Direct Example: /Width 800 (The number 800 is a direct integer).
    Indirect Example: 5 0 obj << /Type /Font ... >> endobj (The dictionary is an indirect font
    object).

    // Tracks where an object ended up after serialisation — not intrinsic to the object itself
#[derive(Debug, Clone, PartialEq, Default)]
pub struct SerialLocation {
    pub offset: usize,
    pub status: ObjectStatus, // free or inuse
}

*/
use crate::cross_reference_table::{CrossRefTable, CrossReferenceEntry, ObjectStatus};
use crate::generation::Generation;
use crate::objects::number::PdfNumberObject;
use crate::objects::reference::PdfReferenceObject;
use crate::{
    NumberType, PdfArrayObject, PdfBooleanObject, PdfDictionaryObject, PdfError, PdfNameObject,
    PdfNullObject, PdfStreamObject, PdfStringObject,
};
use std::cmp::PartialEq;
use std::fs::File;
use std::io::{Seek, Write};
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
    pub fn serialise(&self, xref: &mut CrossRefTable, file: &mut File) -> Result<(), PdfError> {
        if matches!(self, PdfObject::Reference(_)) {
            return Ok(());
        }

        let object_number = self.get_object_number();
        if object_number == None {
            return Ok(()); // direct object (no object number)
        }

        let offset = file.stream_position()?;

        // indirect object
        let mut vec = vec![];
        //vec.push(b'\n');
        vec.extend(object_number.unwrap().to_string().as_bytes());
        vec.extend(b" 0 obj\n");
        vec.extend(match_pdf_object!(self, x => x.encode())?);
        vec.extend(b"endobj\n\n");
        file.write_all(&*vec)?;

        let xref_ent = CrossReferenceEntry::new(
            object_number.unwrap(),
            offset,
            ObjectStatus::InUse,
            Generation::Normal,
        );
        xref.add_entry(xref_ent);

        Ok(())
    }

    //------------------ constructor helpers --------------------------

    pub fn with_object_number(mut self, value: u64) -> Self {
        match_pdf_object!(&mut self, x => x.object_number = Some(value));
        self
    }

    pub fn with_generation_number(mut self, value: u16) -> Self {
        match_pdf_object!(&mut self, x => x.generation_number = Some(value));
        self
    }

    /// Encode this object as it should appear when used as a value inside a dictionary or array.
    /// If it's an indirect object, emit a reference (N 0 R); otherwise encode inline.
    pub fn encode_as_value(&self) -> Result<Vec<u8>, PdfError> {
        // References are always encoded inline — they ARE the reference
        if matches!(self, PdfObject::Reference(_)) {
            return self.encode();
        }
        if let Some(obj_num) = self.get_object_number() {
            PdfReferenceObject::new(obj_num).encode()
        } else {
            self.encode()
        }
    }

    pub fn encode(&self) -> Result<Vec<u8>, PdfError> {
        match_pdf_object!(&self, x => x.encode())
    }

    //------------------ getters and setters --------------------------

    pub fn get_object_number(&self) -> Option<u64> {
        match_pdf_object!(self, x => x.object_number)
    }

    pub fn set_object_number(&mut self, object_number: u64) {
        match_pdf_object!(self, x => x.object_number = Some(object_number));
    }

    pub fn get_generation_number(&self) -> Option<u16> {
        match_pdf_object!(self, x => x.generation_number)
    }

    pub fn set_generation_number(&mut self, generation_number: u16) {
        match_pdf_object!(self, x => x.generation_number = Some(generation_number));
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

//--------------------------- PdfObj -------------------------//

pub struct PdfObj {}

// Dictionary, Array, Stream, Number, Boolean are now automatically converted to PdfObject
impl PdfObj {
    pub fn make_reference_obj(value: u64) -> PdfObject {
        PdfObject::Reference(PdfReferenceObject::new(value))
    }

    pub fn make_null_obj() -> PdfObject {
        PdfObject::Null(PdfNullObject::new())
    }

    pub fn make_num_obj(value: impl Into<NumberType>) -> PdfObject {
        PdfObject::Number(PdfNumberObject::new(value.into()))
    }

    pub fn make_num_or_null_obj<T: Into<NumberType>>(value: Option<T>) -> PdfObject {
        match value {
            Some(v) => PdfObj::make_num_obj(v),
            None => PdfObj::make_null_obj(),
        }
    }

    // name and string are ambiguous so have to stay
    pub fn make_name_obj(value: &str) -> PdfObject {
        PdfObject::Name(PdfNameObject::new(value))
    }

    pub fn make_string_obj(value: &str) -> PdfObject {
        PdfObject::String(PdfStringObject::new(value))
    }
}

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
