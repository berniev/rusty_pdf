use crate::PdfArrayObject;
use crate::encoding::f_to_pdf_num;
use crate::objects::pdf_object::Pdf;

//------------------------- ToPdf -----------------------------

pub trait ToPdf {
    fn to_pdf(&self) -> String;
    fn as_string(&self) -> String;
}

impl ToPdf for f64 {
    fn to_pdf(&self) -> String {
        f_to_pdf_num(*self).to_string()
    }
    fn as_string(&self) -> String {
        format!("{}", *self)
    }
}

//------------------------ Posn -------------------------------

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Posn {
    pub x: f64,
    pub y: f64, // In pdf zero is at the bottom
}

impl Posn {
    pub fn as_pdf_array(&self) -> PdfArrayObject {
        let mut arr = PdfArrayObject::new();
        arr.push(Pdf::num(self.x));
        arr.push(Pdf::num(self.y));

        arr
    }

    fn as_vec(&self) -> Vec<f64> {
        vec![self.x.clone().into(), self.y.into()]
    }
}

impl ToPdf for Posn {
    fn to_pdf(&self) -> String {
        format!("{} {}", f_to_pdf_num(self.x), f_to_pdf_num(self.y))
    }

    fn as_string(&self) -> String {
        format!("({} x {})", self.x, self.y)
    }
}

//------------------------ Line -------------------------------

#[derive(Clone)]
pub struct Line {
    pub start: Posn,
    pub end: Posn,
}

impl Line {
    pub fn as_pdf_array(&self) -> PdfArrayObject {
        let mut arr = PdfArrayObject::new();
        arr.push(Pdf::array(self.start.as_pdf_array()));
        arr.push(Pdf::array(self.end.as_pdf_array()));

        arr
    }
}
//------------------------ Dims -------------------------------

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Dims {
    pub width: f64,
    pub height: f64,
}

impl ToPdf for Dims {
    fn to_pdf(&self) -> String {
        format!("{} {}", f_to_pdf_num(self.width), f_to_pdf_num(self.height),)
    }

    fn as_string(&self) -> String {
        format!("w:{} x h:{}", self.width, self.height,)
    }
}

//------------------------ Rect -------------------------------

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rect {
    pub x1: f64,
    pub y1: f64,
    pub x2: f64,
    pub y2: f64,
}

impl Rect {
    pub fn as_vec(&self) -> Vec<f64> {
        vec![self.x1, self.y1, self.x2, self.y2]
    }

    pub fn as_pdf_array(&self) -> PdfArrayObject {
        let mut arr = PdfArrayObject::new();
        arr.push(Pdf::num(self.x1));
        arr.push(Pdf::num(self.y1));
        arr.push(Pdf::num(self.x2));
        arr.push(Pdf::num(self.y2));

        arr
    }
}

impl ToPdf for Rect {
    fn to_pdf(&self) -> String {
        format!(
            "{} {} {} {}",
            f_to_pdf_num(self.x1),
            f_to_pdf_num(self.y1),
            f_to_pdf_num(self.x2),
            f_to_pdf_num(self.y2),
        )
    }
    fn as_string(&self) -> String {
        format!("[{} {} {} {}]", self.x1, self.y1, self.x2, self.y2)
    }
}

//------------------------ Matrix -------------------------------

/// PDF transformation matrix [a b c d e f]
/// | a  b  0 |
/// | c  d  0 |
/// | e  f  1 |
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Matrix {
    pub a: f64,
    pub b: f64,
    pub c: f64,
    pub d: f64,
    pub e: f64,
    pub f: f64,
}

impl Matrix {
    fn as_vec(&self) -> Vec<f64> {
        vec![self.a, self.b, self.c, self.d, self.e, self.f]
    }

    pub fn as_pdf_array(&self) -> PdfArrayObject {
        let mut arr = PdfArrayObject::new();
        arr.push(Pdf::num(self.a));
        arr.push(Pdf::num(self.b));
        arr.push(Pdf::num(self.c));
        arr.push(Pdf::num(self.d));
        arr.push(Pdf::num(self.e));
        arr.push(Pdf::num(self.f));

        arr
    }
}

impl ToPdf for Matrix {
    fn to_pdf(&self) -> String {
        format!(
            "{} {} {} {} {} {}",
            f_to_pdf_num(self.a),
            f_to_pdf_num(self.b),
            f_to_pdf_num(self.c),
            f_to_pdf_num(self.d),
            f_to_pdf_num(self.e),
            f_to_pdf_num(self.f),
        )
    }

    fn as_string(&self) -> String {
        format!(
            "{} {} {} {} {} {}",
            self.a, self.b, self.c, self.d, self.e, self.f,
        )
    }
}

//------------------------ EvenOdd -------------------------------

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WindingRule {
    NonZero,
    EvenOdd,
}

//------------------- CompressionMethod ----------------------------

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CompressionMethod {
    None,
    Flate,
}

impl CompressionMethod {
    pub fn to_string(&self) -> String {
        match self {
            CompressionMethod::Flate => "/A85 /Fl".to_string(),
            CompressionMethod::None => "/A85".to_string(),
        }
    }
}
//--------------------- StrokeOrFill -----------------------------

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum StrokeOrFill {
    Stroke,
    Fill,
}
