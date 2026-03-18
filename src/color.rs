use std::cmp::Ordering;
use std::fmt::{self, Display};
use std::rc::Rc;

use crate::encoding::f_to_pdf_num;
use crate::util::ToPdf;
use crate::{ArrayObject, PdfError, PdfObject, PdfResult};

//------------------------ ColorSpace -------------------------------

pub enum ColorSpace {
    CMYK,
    Gray,
    RGB,
}

impl Display for ColorSpace {
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

macro_rules! impl_color_logic {
    ($ty:ident, $err_var:ident, $err_field:ident, $($field:ident: $label:expr),+) => {
        impl $ty {
            pub fn validate(&self) -> PdfResult<()> {
                $( self.$field.validate().map_err(|_| PdfError::$err_var { $err_field: *self })?; )+
                Ok(())
            }
        }

        impl ToPdf for $ty {
            fn to_pdf(&self) -> String {
                [$(self.$field.to_string()),+].join(" ")
            }

            fn as_string(&self) -> String {
                [$(format!("{}:{}", $label, self.$field)),+].join(" ")
            }
        }
    };
}

#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub color: f32,
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

impl Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", f_to_pdf_num(self.color))
    }
}

impl ToPdf for Color {
    fn to_pdf(&self) -> String {
        f_to_pdf_num(self.color).to_string()
    }
    fn as_string(&self) -> String {
        format!("{}", self.color)
    }
}

impl PartialEq<f32> for Color {
    fn eq(&self, other: &f32) -> bool {
        self.color == *other
    }
}

impl PartialOrd<f32> for Color {
    fn partial_cmp(&self, other: &f32) -> Option<Ordering> {
        self.color.partial_cmp(other)
    }
}

//------------------------- RGB -------------------------------

#[derive(Debug, Clone, Copy)]
pub struct RGB {
    pub red: Color,
    pub green: Color,
    pub blue: Color,
}

impl RGB {
    pub fn build(self) -> Rc<dyn PdfObject> {
        Rc::new(ArrayObject::from_rgb(self))
    }
}

impl_color_logic!(RGB, InvalidRGB, rgb, red: "r", green: "g", blue: "b");

//------------------------ RGBA -------------------------------

#[derive(Debug, Clone, Copy)]
pub struct RGBA {
    pub red: Color,
    pub green: Color,
    pub blue: Color,
    pub alpha: Color,
}

impl RGBA {
    pub fn build(self) -> Rc<dyn PdfObject> {
        Rc::new(ArrayObject::from_rgba(self))
    }
}

impl_color_logic!(RGBA, InvalidRGBA, rgb, red: "r", green: "g", blue: "b", alpha: "a");

//------------------------ CMYK -------------------------------

#[derive(Debug, Clone, Copy)]
pub struct CMYK {
    pub cyan: Color,
    pub magenta: Color,
    pub yellow: Color,
    pub black: Color,
}

impl CMYK {
    pub fn build(self) -> Rc<dyn PdfObject> {
        Rc::new(ArrayObject::from_cmyk(self))
    }
}

impl_color_logic!(CMYK, InvalidCMYK, cmyk, cyan: "c", magenta: "m", yellow: "y", black: "k");

