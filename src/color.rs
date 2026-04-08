use std::cmp::Ordering;
use std::fmt::{self, Display};

use crate::encoding::f_to_pdf_num;
use crate::util::ToPdf;
use crate::{PdfArrayObject, PdfError, PdfResult};

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
    color: f32,
}

impl Color {
    pub fn new(color: f32) -> Self {
        let instance = Self { color };
        instance
            .validate()
            .expect("color must be in range 0.0..=1.0");
        instance
    }

    pub fn to_f32(&self) -> f32 {
        self.color
    }

    pub fn to_f64(&self) -> f64 {
        self.color as f64
    }

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
    red: Color,
    green: Color,
    blue: Color,
}

impl RGB {
    pub fn new(red: Color, green: Color, blue: Color) -> Self {
        Self { red, green, blue }
    }

    pub(crate) fn as_pdf_array(&self) -> PdfArrayObject {
        let mut arr = PdfArrayObject::new();
        arr.push(self.red.to_f64());
        arr.push(self.green.to_f64());
        arr.push(self.blue.to_f64());

        arr
    }
    pub fn as_vec(&self) -> [Color; 3] {
        [self.red, self.green, self.blue]
    }

    pub fn r(&self) -> Color {
        self.red
    }

    pub fn g(&self) -> Color {
        self.green
    }

    pub fn b(&self) -> Color {
        self.blue
    }
}

impl_color_logic!(RGB, InvalidRGB, rgb, red: "r", green: "g", blue: "b");

//------------------------ RGBA -------------------------------

#[derive(Debug, Clone, Copy)]
pub struct RGBA {
    red: Color,
    green: Color,
    blue: Color,
    alpha: Color,
}

impl RGBA {
    pub fn new(red: Color, green: Color, blue: Color, alpha: Color) -> Self {
        Self {
            red,
            green,
            blue,
            alpha,
        }
    }

    pub fn as_vec(&self) -> [Color; 4] {
        [self.red, self.green, self.blue, self.alpha]
    }

    pub fn as_vec_64(&self) -> [f64; 4] {
        [
            self.red.to_f64(),
            self.green.to_f64(),
            self.blue.to_f64(),
            self.alpha.to_f64(),
        ]
    }

    pub fn has_transparency(&self) -> bool {
        self.alpha.color < 1.0
    }

    pub fn r(&self) -> Color {
        self.red
    }

    pub fn g(&self) -> Color {
        self.green
    }

    pub fn b(&self) -> Color {
        self.blue
    }

    pub fn a(&self) -> Color {
        self.alpha
    }
}

impl_color_logic!(RGBA, InvalidRGBA, rgb, red: "r", green: "g", blue: "b", alpha: "a");

//------------------------ CMYK -------------------------------

#[derive(Debug, Clone, Copy)]
pub struct CMYK {
    cyan: Color,
    magenta: Color,
    yellow: Color,
    black: Color,
}

impl CMYK {
    pub fn new(cyan: Color, magenta: Color, yellow: Color, black: Color) -> Self {
        Self {
            cyan,
            magenta,
            yellow,
            black,
        }
    }

    pub fn as_vec(&self) -> [Color; 4] {
        [self.cyan, self.magenta, self.yellow, self.black]
    }

    pub fn c(&self) -> Color {
        self.cyan
    }

    pub fn m(&self) -> Color {
        self.magenta
    }

    pub fn y(&self) -> Color {
        self.yellow
    }

    pub fn k(&self) -> Color {
        self.black
    }
}

impl_color_logic!(CMYK, InvalidCMYK, cmyk, cyan: "c", magenta: "m", yellow: "y", black: "k");
