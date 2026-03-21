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
*/

use crate::cross_ref::{Generation, ObjectStatus};
use crate::{
    ArrayObject, BooleanObject, DictionaryObject, NameObject, NumberObject, PdfObject, StreamObject,
};

enum PdfObjectType {
    Array(ArrayObject),
    Boolean(BooleanObject),
    Dictionary(DictionaryObject),
    Name(NameObject),
    NameTree(),
    Number(NumberObject),
    Stream(StreamObject),
    String(StreamObject),
}

// The PDF spec identity of an __ indirect __ object (§7.3.10)
#[derive(Debug, Clone, PartialEq)]
pub struct ObjectId {
    pub number: usize,
    pub generation: Generation,
}

// __ Direct __ objects carry NO metadata at all
pub struct NameObject {
    pub value: Option<String>,
}

// Tracks where an object ended up after serialisation — not intrinsic to the object itself
#[derive(Debug, Clone, PartialEq, Default)]
pub struct SerialLocation {
    pub offset: usize,
    pub status: ObjectStatus, // free or inuse
}

// Indirect is a wrapper, not a peer variant
struct IndirectObject {
    pub id: ObjectId,
    pub location: Option<SerialLocation>,
    pub inner: PdfObjectType, // owns the direct object
}

// The offset/status truly belong only in the cross-reference table, not on the objects themselves
pub struct CrossRefEntry {
    pub id: ObjectId,
    pub location: SerialLocation,
}

pub struct PDF {
    pub objects: Vec<IndirectObject>,  // no metadata on inner objects
    pub cross_ref: Vec<CrossRefEntry>, // populated during write
}
