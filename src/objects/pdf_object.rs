/**
 a direct object must always be owned by something, and that something ultimately traces back to an indirect object.
The containment hierarchy is:
```
body
└── indirect object  (5 0 obj ... endobj)
    └── dictionary
        ├── /MediaBox [ 0 0 612 792 ]   ← direct array
        │   ├── 0                        ← direct integer
        │   ├── 0                        ← direct integer
        │   └── ...
        └── /Rotate 0                    ← direct integer
```
```
6 0 obj
42
endobj

7 0 obj
[ 1 2 3 ]
endobj

8 0 obj
<< /Length 12 >>
stream
Hello World!
endstream
endobj
```
A direct object can be nested arbitrarily deep — a dictionary inside an array inside a dictionary
inside an indirect object — but there's always an indirect object at the root of the tree.
The spec puts it plainly: "a direct object is any object that is not an indirect object" — meaning
it has no obj/endobj wrapper, no object number, and cannot exist free-standing in the body.
Where objects live:
    Indirect: in the body (referenced by the XRef)
    Direct:   inside indirect objects (or inside other direct objs that are inside indirect objs)
*/
use crate::{
    NumberType, PdfArrayObject, PdfBooleanObject, PdfDictionaryObject, PdfIndirectObject,
    PdfNameObject, PdfNullObject, PdfNumberObject, PdfStringObject,
};

//--------------------------- Pdf -------------------------//

pub struct Pdf {}

impl Pdf {
    pub fn bool(value: bool) -> Box<dyn PdfObject> {
        Box::new(PdfBooleanObject::new(value))
    }

    pub fn name(value: &str) -> Box<dyn PdfObject> {
        Box::new(PdfNameObject::new(value))
    }

    pub fn num(value: impl Into<NumberType>) -> Box<dyn PdfObject> {
        Box::new(PdfNumberObject::new(value.into()))
    }

    pub fn num_or_null<T: Into<NumberType>>(value: Option<T>) -> Box<dyn PdfObject> {
        match value {
            Some(v) => Pdf::num(v),
            None => Pdf::null(),
        }
    }

    pub fn null() -> Box<dyn PdfObject> {
        Box::new(PdfNullObject::new())
    }

    pub fn string(value: &str) -> Box<dyn PdfObject> {
        Box::new(PdfStringObject::new(value))
    }

    pub fn array(value: PdfArrayObject) -> Box<dyn PdfObject> {
        Box::new(value)
    }

    pub fn dict(value: PdfDictionaryObject) -> Box<dyn PdfObject> {
        Box::new(value)
    }

    pub fn indirect(value: usize) -> Box<dyn PdfObject> {
        Box::new(PdfIndirectObject::new_standard(value))
    }
}

//--------------------------- PdfObject -------------------------//

pub trait PdfObject: 'static {
    fn serialise(&mut self) -> Vec<u8>;

    fn boxed(self) -> Box<dyn PdfObject>
    where
        Self: Sized,
    {
        Box::new(self)
    }
}
