use crate::encoding::f_to_pdf_num;
use crate::{PdfError, PdfResult};
use std::fmt;
use std::fmt::Display;
//------------------------- ToPdf -----------------------------

pub trait ToPdf {
    fn to_pdf(&self) -> String;
    fn as_string(&self) -> String;
}

impl ToPdf for f64 {
    fn to_pdf(&self) -> String {
        format!("{}", f_to_pdf_num(*self))
    }
    fn as_string(&self) -> String {
        format!("{}", *self)
    }
}

//--------------------- StrokeOrFill -----------------------------

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum StrokeOrFill {
    Stroke,
    Fill,
}

//------------------------ EvenOdd -------------------------------

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EvenOdd {
    Even,
    Odd,
}

//------------------- CompressionMethod ----------------------------

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CompressionMethod {
    None,
    Flate,
}

//------------------------ Posn -------------------------------

/// Position is X:Y. Positive Y moves up.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Posn<T> {
    pub x: T,
    pub y: T,
}

impl<T> ToPdf for Posn<T> 
where 
    T: Display + Copy + Into<f64> 
{
    fn to_pdf(&self) -> String {
        format!("{} {}", f_to_pdf_num(self.x.into()), f_to_pdf_num(self.y.into()))
    }
    
    fn as_string(&self) -> String {
        format!("({} x {})", self.x, self.y)
    }
}
//------------------------ Dims -------------------------------

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DimsPoints {
    pub width: f64,
    pub height: f64,
}

impl ToPdf for DimsPoints {
    fn to_pdf(&self) -> String {
        format!("{} {}", f_to_pdf_num(self.width), f_to_pdf_num(self.height),)
    }

    fn as_string(&self) -> String {
        format!("w:{} x h:{}", self.width, self.height,)
    }
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

impl Matrix {
    pub fn new(a: f64, b: f64, c: f64, d: f64, e: f64, f: f64) -> Self {
        Matrix { a, b, c, d, e, f }
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
