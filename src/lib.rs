#![allow(dead_code)]

//! # PyDyf - PDF Library for Rust
//!
//! A low-level PDF generation library ported from Python's pydyf.
//!
//! ## Quick Start
//!
//! ```rust
//! use pydyf::{PDF, Stream, Dictionary};
//! use std::collections::HashMap;
//! use std::fs::File;
//!
//! // Create a new PDF document
//! let mut pdf = PDF::new();
//!
//! // Create a content stream
//! let mut stream = Stream::new();
//!
//! // Draw a red rectangle
//! stream.set_color_rgb(1.0, 0.0, 0.0, false).unwrap();
//! stream.rectangle(100.0, 100.0, 200.0, 150.0);
//! stream.fill(false);
//!
//! // Add text
//! stream.begin_text();
//! stream.set_font_size("Helvetica", 24.0);
//! stream.set_text_matrix(1.0, 0.0, 0.0, 1.0, 100.0, 300.0);
//! stream.show_text_string("Hello, PDF!");
//! stream.end_text();
//!
//! // Add stream to PDF
//! pdf.add_object(Box::new(stream));
//!
//! // Create page
//! let content_ref = format!("{} 0 R", pdf.objects.len() - 1).into_bytes();
//! let mut page_values = HashMap::new();
//! page_values.insert("Type".to_string(), b"/Page".to_vec());
//! page_values.insert("MediaBox".to_string(), b"[0 0 612 792]".to_vec());
//! page_values.insert("Contents".to_string(), content_ref);
//! let page = Dictionary::new(Some(page_values));
//!
//! pdf.add_page(page);
//!
//! // Write to file
//! let mut file = File::create("output.pdf").unwrap();
//! pdf.write(&mut file, Some(b"1.7"), pydyf::Identifier::AutoMD5, false).unwrap();
//! ```
//!
//! ## Features
//!
//! - **Graphics**: Rectangles, lines, curves, paths
//! - **Text**: Multiple fonts, positioning, transformations
//! - **Colors**: RGB, CMYK, and grayscale color spaces
//! - **Images**: Inline images and external image file loading (PNG, JPEG)
//! - **Compression**: Optional stream compression with flate
//! - **Error Handling**: Comprehensive validation with custom error types
//!
//! ## Color Spaces
//!
//! ```rust
//! # use pydyf::Stream;
//! # let mut stream = Stream::new();
//! // RGB colors (values 0.0-1.0)
//! stream.set_color_rgb(1.0, 0.0, 0.0, false).unwrap();
//!
//! // CMYK colors (values 0.0-1.0)
//! stream.set_color_cmyk(0.0, 1.0, 1.0, 0.0, false).unwrap();
//!
//! // Grayscale (value 0.0-1.0)
//! stream.set_color_gray(0.5, false).unwrap();
//! ```
//!
//! ## Images
//!
//! ```rust,no_run
//! # use pydyf::Stream;
//! # let mut stream = Stream::new();
//! // Load from file (PNG or JPEG)
//! stream.push_state();
//! stream.set_matrix(200.0, 0.0, 0.0, 200.0, 50.0, 500.0); // Scale and position
//! stream.inline_image_from_file("image.png").unwrap();
//! stream.pop_state();
//! ```

pub mod array;
pub mod dictionary;
pub mod encoding;
pub mod error;
pub mod gradient;
pub mod graphics_state;
pub mod object;
pub mod pdf;
pub mod resources;
pub mod stream;
pub mod string;
pub mod text;

// Re-export main types for convenience
pub use array::Array;
pub use dictionary::Dictionary;
pub use error::{PdfError, Result};
pub use gradient::{ColorStop, LinearGradient, RadialGradient};
pub use graphics_state::GraphicsStateManager;
pub use object::{BaseObject, PdfObject, PdfMetadata, ObjectStatus};
pub use pdf::{Identifier, PDF};
pub use resources::ResourceDictionary;
pub use stream::Stream;
pub use string::encode_pdf_string;
pub use text::{wrap_text, StandardFont, WrapMode};
