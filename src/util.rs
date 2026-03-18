use std::fmt::Display;

use crate::{ArrayObject, PdfObject};
use crate::encoding::f_to_pdf_num;

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
pub struct Posn<T> {
    pub x: T,
    pub y: T, // In pdf zero is at the bottom
}

impl<T> ToPdf for Posn<T>
where
    T: Display + Copy + Into<f64>,
{
    fn to_pdf(&self) -> String {
        format!(
            "{} {}",
            f_to_pdf_num(self.x.into()),
            f_to_pdf_num(self.y.into())
        )
    }

    fn as_string(&self) -> String {
        format!("({} x {})", self.x, self.y)
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
    pub fn make_pdf_obj(self) -> std::rc::Rc<dyn PdfObject> {
        std::rc::Rc::new(ArrayObject::from_rect(self))
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
    pub fn make_pdf_obj(self) -> std::rc::Rc<dyn PdfObject> {
        std::rc::Rc::new(ArrayObject::from_matrix(self))
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

