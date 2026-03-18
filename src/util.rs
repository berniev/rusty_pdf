use crate::encoding::f_to_pdf_num;
use std::fmt::Display;

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

/// Position is X:Y. In pdf positive Y moves up.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Posn<T> {
    pub x: T,
    pub y: T,
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

//------------------------ Matrix -------------------------------

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Matrix {
    pub a: f64,
    pub b: f64,
    pub c: f64,
    pub d: f64,
    pub e: f64,
    pub f: f64,
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

