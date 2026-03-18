use std::rc::Rc;

use crate::{
    BooleanObject, DictionaryObject, IndirectObject, NameObject, NumberObject, NumberType,
    PdfMetadata, PdfObject, StringObject,
    action::Destination,
    color::{CMYK, RGB, RGBA},
    util::{Matrix, Posn, Rect},
};

//-------------------ArrayObject ----------------------

/// Spec:
/// Array Objects:
///     An array object is a one-dimensional collection of objects arranged sequentially. Unlike
///     arrays in many other computer languages, PDF arrays may be heterogeneous; that is, an
///     array’s elements may be any combination of numbers, strings, dictionaries, or any other
///     objects, including other arrays. An array may have zero elements.
/// Construction:
///     An array shall be written as a sequence of objects enclosed in SQUARE BRACKETS.
///     EXAMPLE [ 549 3.14 false ( Ralph ) /SomeName ]
pub struct ArrayObject {
    pub values: Vec<Rc<dyn PdfObject>>,
    pub metadata: PdfMetadata,
}

impl ArrayObject {
    pub fn new(values: Option<Vec<Rc<dyn PdfObject>>>) -> Self {
        Self {
            values: values.unwrap_or_default(),
            metadata: PdfMetadata::default(),
        }
    }

    pub fn build(values: Vec<Rc<dyn PdfObject>>) -> Rc<dyn PdfObject> {
        Rc::new(Self::new(Some(values)))
    }

    pub fn push_object(&mut self, value: Rc<dyn PdfObject>) {
        self.values.push(value);
    }

    pub fn push_real(&mut self, value: f64) {
        self.push_number(NumberType::Real(value));
    }

    pub fn push_reals(&mut self, values: &[f64]) {
        for &value in values {
            self.push_real(value);
        }
    }

    pub fn push_optional_real(&mut self, value: Option<f64>) {
        if let Some(v) = value {
            self.push_real(v);
        } else {
            self.push_name("null");
        }
    }

    pub fn push_name(&mut self, name: &str) {
        self.push_object(NameObject::build(name));
    }

    pub fn push_string(&mut self, value: String) {
        self.push_object(StringObject::build(value));
    }

    pub fn push_number(&mut self, value: impl Into<NumberType>) {
        self.push_object(NumberObject::build(value));
    }

    pub fn push_bool(&mut self, value: bool) {
        self.push_object(BooleanObject::build(value));
    }

    pub fn push_indirect(&mut self, id: usize) {
        self.push_object(Rc::new(IndirectObject::new(Some(id))));
    }

    pub fn push_array(&mut self, array: ArrayObject) {
        self.push_object(Rc::new(array));
    }

    pub fn push_dict(&mut self, dict: DictionaryObject) {
        self.push_object(Rc::new(dict));
    }

    pub fn from_points(start: Posn<f64>, end: Posn<f64>) -> Self {
        let mut arr = Self::new(None);
        arr.push_real(start.x);
        arr.push_real(start.y);
        arr.push_real(end.x);
        arr.push_real(end.y);
        arr
    }

    pub fn from_rect(rect: Rect) -> Self {
        let mut arr = Self::new(None);
        arr.push_real(rect.x1);
        arr.push_real(rect.y1);
        arr.push_real(rect.x2);
        arr.push_real(rect.y2);
        arr
    }

    pub fn from_matrix(matrix: Matrix) -> Self {
        let mut arr = Self::new(None);
        arr.push_real(matrix.a);
        arr.push_real(matrix.b);
        arr.push_real(matrix.c);
        arr.push_real(matrix.d);
        arr.push_real(matrix.e);
        arr.push_real(matrix.f);
        arr
    }

    pub fn from_destination(dest: Destination) -> Self {
        dest.to_array()
    }

    pub fn from_destination_ref(dest: &Destination) -> Self {
        dest.to_array()
    }

    pub fn from_rgb_tuple(r: f64, g: f64, b: f64) -> Self {
        let mut arr = Self::new(None);
        arr.push_real(r);
        arr.push_real(g);
        arr.push_real(b);
        arr
    }

    pub fn from_rgb(rgb: RGB) -> Self {
        let mut arr = Self::new(None);
        arr.push_real(rgb.red.color as f64);
        arr.push_real(rgb.green.color as f64);
        arr.push_real(rgb.blue.color as f64);
        arr
    }

    pub fn from_rgba(rgba: RGBA) -> Self {
        let mut arr = Self::new(None);
        arr.push_real(rgba.red.color as f64);
        arr.push_real(rgba.green.color as f64);
        arr.push_real(rgba.blue.color as f64);
        arr.push_real(rgba.alpha.color as f64);
        arr
    }

    pub fn from_cmyk(cmyk: CMYK) -> Self {
        let mut arr = Self::new(None);
        arr.push_real(cmyk.cyan.color as f64);
        arr.push_real(cmyk.magenta.color as f64);
        arr.push_real(cmyk.yellow.color as f64);
        arr.push_real(cmyk.black.color as f64);
        arr
    }
}

impl PdfObject for ArrayObject {
    fn data(&self) -> String {
        format!(
            "[ {} ]",
            self.values
                .iter()
                .map(|item| item.data())
                .collect::<Vec<_>>()
                .join(" ")
        )
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn metadata(&self) -> &PdfMetadata {
        &self.metadata
    }

    fn metadata_mut(&mut self) -> &mut PdfMetadata {
        &mut self.metadata
    }
}

