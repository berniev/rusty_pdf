use crate::encoding::to_pdf_num;
use crate::{PdfError, PdfResult};

pub trait ToPdf {
    fn to_pdf(&self) -> String;
}

impl ToPdf for f64 {
    fn to_pdf(&self) -> String {
        format!("{}", to_pdf_num(*self))
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum StrokeOrFill {
    Stroke,
    Fill,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EvenOdd {
    Even,
    Odd,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CompressionMethod {
    None,
    Flate,
}

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
        format!("{}", to_pdf_num(self.color))
    }
}

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
            to_pdf_num(self.red),
            to_pdf_num(self.green),
            to_pdf_num(self.blue),
        )
    }
}

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
            to_pdf_num(self.red),
            to_pdf_num(self.green),
            to_pdf_num(self.blue),
            to_pdf_num(self.alpha)
        )
    }
}

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
            to_pdf_num(self.cyan),
            to_pdf_num(self.magenta),
            to_pdf_num(self.yellow),
            to_pdf_num(self.black)
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PosnXY {
    pub x: f64,
    pub y: f64,
}

impl ToPdf for PosnXY {
    fn to_pdf(&self) -> String {
        format!("{} {}", to_pdf_num(self.x), to_pdf_num(self.y),)
    }
}

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
            to_pdf_num(self.a),
            to_pdf_num(self.b),
            to_pdf_num(self.c),
            to_pdf_num(self.d)
            to_pdf_num(self.e)
            to_pdf_num(self.f)
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Size {
    pub width: f64,
    pub height: f64,
}

impl ToPdf for Size {
    fn to_pdf(&self) -> String {
        format!("{} {}", to_pdf_num(self.width), to_pdf_num(self.height),)
    }
}

