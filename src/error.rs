use std::fmt;
use std::io;

use crate::color::{CMYK, Color, RGB, RGBA};

/// Errors that can occur during PDF generation
#[derive(Debug)]
pub enum PdfError {
    /// I/O error during file operations
    Io(io::Error),

    /// Invalid object reference (object number doesn't exist)
    InvalidObjectReference(usize),

    /// Invalid compression state
    CompressionError(String),

    /// Invalid font name or configuration
    InvalidFont(String),

    InvalidColorChannel {
        color: Color,
    },
    InvalidRGB {
        rgb: RGB,
    },
    InvalidRGBA {
        rgb: RGBA,
    },
    InvalidCMYK {
        cmyk: CMYK,
    },
    /// Invalid image data
    InvalidImage(String),

    /// PDF structure error
    StructureError(String),
}

impl fmt::Display for PdfError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PdfError::Io(err) => write!(f, "I/O error: {}", err),
            PdfError::InvalidObjectReference(num) => {
                write!(f, "Invalid object reference: {}", num)
            }
            PdfError::CompressionError(msg) => write!(f, "Compression error: {}", msg),
            PdfError::InvalidFont(msg) => write!(f, "Invalid font: {}", msg),
            PdfError::InvalidColorChannel { color } => {
                write!(f, "Invalid color channel: {:?}", color)
            }
            PdfError::InvalidRGB { rgb } => {
                write!(
                    f,
                    "Invalid color values: r={}, g={}, b={} (must be 0.0-1.0)",
                    rgb.r(),
                    rgb.g(),
                    rgb.b()
                )
            }
            PdfError::InvalidRGBA { rgb } => {
                write!(
                    f,
                    "Invalid color values: r={}, g={}, b={}, a={} (must be 0.0-1.0)",
                    rgb.r(),
                    rgb.g(),
                    rgb.b(),
                    rgb.a()
                )
            }
            PdfError::InvalidCMYK { cmyk } => {
                write!(
                    f,
                    "Invalid color values: c={}, m={}, y={}, k={} (must be 0.0-1.0)",
                    cmyk.c(),
                    cmyk.m(),
                    cmyk.y(),
                    cmyk.k()
                )
            }
            PdfError::InvalidImage(msg) => write!(f, "Invalid image: {}", msg),
            PdfError::StructureError(msg) => write!(f, "PDF structure error: {}", msg),
        }
    }
}

impl std::error::Error for PdfError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            PdfError::Io(err) => Some(err),
            _ => None,
        }
    }
}

impl From<io::Error> for PdfError {
    fn from(err: io::Error) -> Self {
        PdfError::Io(err)
    }
}

pub type PdfResult<T> = std::result::Result<T, PdfError>;
