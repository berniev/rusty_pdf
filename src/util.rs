use crate::encoding::f_to_pdf_num;
use crate::{PdfError, PdfResult};
use std::fmt;

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

/// Position is X:Y. Positivee Y moves up.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PosnPoints {
    pub x: f64,
    pub y: f64,
}

impl ToPdf for PosnPoints {
    fn to_pdf(&self) -> String {
        format!("{} {}", f_to_pdf_num(self.x), f_to_pdf_num(self.y),)
    }
    fn as_string(&self) -> String {
        format!("({} x {})", self.x, self.y,)
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

//------------------------ ColorSpace -------------------------------

pub enum ColorSpace {
    CMYK,
    Gray,
    RGB,
}

impl fmt::Display for ColorSpace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ColorSpace::RGB => f.write_str("RGB"),
            ColorSpace::CMYK => f.write_str("CMYK"),
            ColorSpace::Gray => f.write_str("Gray"),
        }
    }
}

impl ColorSpace {
    pub fn from_string(s: &str) -> Option<ColorSpace> {
        match s {
            "RGB" => Some(ColorSpace::RGB),
            "CMYK" => Some(ColorSpace::CMYK),
            "Gray" => Some(ColorSpace::Gray),
            _ => None,
        }
    }
}

//------------------------ Color -------------------------------

#[derive(Debug)]
pub struct Color {
    pub color: f64,
}

impl Color {
    pub fn validate(&self) -> PdfResult<()> {
        if !(0.0..=1.0).contains(&self.color) {
            return Err(PdfError::InvalidColorChannel {
                color: { Color { color: self.color } },
            });
        }

        Ok(())
    }
}

impl ToPdf for Color {
    fn to_pdf(&self) -> String {
        format!("{}", f_to_pdf_num(self.color))
    }
    fn as_string(&self) -> String {
        format!("{}", self.color)
    }
}

//------------------------ RGB -------------------------------

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RGB {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}

impl RGB {
    pub fn validate(&self) -> PdfResult<()> {
        for &v in &[self.red, self.green, self.blue] {
            if !(0.0..=1.0).contains(&v) {
                return Err(PdfError::InvalidRGB {
                    rgb: RGB {
                        red: self.red,
                        green: self.green,
                        blue: self.blue,
                    },
                });
            }
        }
        Ok(())
    }
}

impl ToPdf for RGB {
    fn to_pdf(&self) -> String {
        format!(
            "{} {} {}",
            f_to_pdf_num(self.red),
            f_to_pdf_num(self.green),
            f_to_pdf_num(self.blue),
        )
    }

    fn as_string(&self) -> String {
        format!("r:{} g:{} b:{}", self.red, self.green, self.blue,)
    }
}

//------------------------ RGBA -------------------------------

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RGBA {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
    pub alpha: f64,
}

impl RGBA {
    pub fn validate(&self) -> PdfResult<()> {
        for &v in &[self.red, self.green, self.blue, self.alpha] {
            if !(0.0..=1.0).contains(&v) {
                return Err(PdfError::InvalidRGBA {
                    rgb: RGBA {
                        red: self.red,
                        green: self.green,
                        blue: self.blue,
                        alpha: self.alpha,
                    },
                });
            }
        }
        Ok(())
    }
}

impl ToPdf for RGBA {
    fn to_pdf(&self) -> String {
        format!(
            "{} {} {} {}",
            f_to_pdf_num(self.red),
            f_to_pdf_num(self.green),
            f_to_pdf_num(self.blue),
            f_to_pdf_num(self.alpha)
        )
    }
    fn as_string(&self) -> String {
        format!(
            "r:{} g:{} b:{} a:{}",
            self.red, self.green, self.blue, self.alpha
        )
    }
}

//------------------------ CMYK -------------------------------

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CMYK {
    pub cyan: f64,
    pub magenta: f64,
    pub yellow: f64,
    pub black: f64,
}

impl CMYK {
    pub fn validate(&self) -> PdfResult<()> {
        for &v in &[self.cyan, self.magenta, self.yellow, self.black] {
            if !(0.0..=1.0).contains(&v) {
                return Err(PdfError::InvalidCMYK {
                    cmyk: CMYK {
                        cyan: self.cyan,
                        magenta: self.magenta,
                        yellow: self.yellow,
                        black: self.black,
                    },
                });
            }
        }
        Ok(())
    }
}

impl ToPdf for CMYK {
    fn to_pdf(&self) -> String {
        format!(
            "{} {} {} {}",
            f_to_pdf_num(self.cyan),
            f_to_pdf_num(self.magenta),
            f_to_pdf_num(self.yellow),
            f_to_pdf_num(self.black)
        )
    }
    fn as_string(&self) -> String {
        format!(
            "c:{} m:{} y:{} k:{}",
            self.cyan, self.magenta, self.yellow, self.black
        )
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
