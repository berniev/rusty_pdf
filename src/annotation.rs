//! Annotation framework for interactive PDF features.
//!
//! Annotations are interactive elements that can be added to PDF pages, including
//! text notes, links, highlights, and form widgets.

use crate::color::Color;
use crate::color::RGB;
use crate::objects::pdf_object::PdfObj;
use crate::util::{Posn, Rectangle};
use crate::{PdfArrayObject, PdfDictionaryObject, PdfResult};
//-------------------AnnotationFlags ----------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AnnotationFlags(u32);

impl AnnotationFlags {
    pub const NONE: Self = Self(0);
    pub const INVISIBLE: Self = Self(1 << 0);
    pub const HIDDEN: Self = Self(1 << 1);
    pub const PRINT: Self = Self(1 << 2);
    pub const NO_ZOOM: Self = Self(1 << 3);
    pub const NO_ROTATE: Self = Self(1 << 4);
    pub const NO_VIEW: Self = Self(1 << 5);
    pub const READ_ONLY: Self = Self(1 << 6);
    pub const LOCKED: Self = Self(1 << 7);

    pub const fn from_bits(bits: u32) -> Self {
        Self(bits)
    }

    pub const fn bits(&self) -> u32 {
        self.0
    }

    pub fn is_empty(&self) -> bool {
        self.bits() == 0
    }

    pub const fn or(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}

//------------------- Annotation Types ----------------------//

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BorderStyle {
    Solid,
    Dashed,
    Beveled,
    Inset,
    Underline,
}

impl BorderStyle {
    pub fn as_str(&self) -> &'static str {
        match self {
            BorderStyle::Solid => "S",
            BorderStyle::Dashed => "D",
            BorderStyle::Beveled => "B",
            BorderStyle::Inset => "I",
            BorderStyle::Underline => "U",
        }
    }
}

//------------------- Annotation ----------------------//

/// Base trait for all PDF annotations.
///
/// Annotations are interactive elements that can be attached to PDF pages.
pub trait Annotation {
    fn subtype(&self) -> &'static str;

    fn rect(&self) -> Rectangle;

    fn flags(&self) -> AnnotationFlags {
        AnnotationFlags::NONE
    }

    fn border_style(&self) -> Option<BorderStyle> {
        None
    }

    fn color(&self) -> Option<RGB> {
        None
    }

    fn contents(&self) -> Option<&str> {
        None
    }

    fn add_border_style_to_dict(&self, dest_dict: &mut PdfDictionaryObject) {
        if let Some(style) = self.border_style() {
            let mut bs_dict = PdfDictionaryObject::new();
            bs_dict.add("S", PdfObj::string(style.as_str()));
            dest_dict.add("BS", bs_dict);
        }
    }

    fn to_dict(&self) -> PdfResult<PdfDictionaryObject> {
        let mut dest_dict = PdfDictionaryObject::new();

        // Required entries
        dest_dict.add("Type", PdfObj::string("Annot"));
        dest_dict.add("Subtype", PdfObj::string(self.subtype()));
        dest_dict.add("Rect", self.rect().as_pdf_array());

        // Optional common entries
        let flags = self.flags();
        if flags.bits() != 0 {
            dest_dict.add("F", flags.bits() as i64);
        }

        self.add_border_style_to_dict(&mut dest_dict);

        if let Some(rgb) = self.color() {
            dest_dict.add("C", rgb.as_pdf_array());
        }

        if let Some(contents) = self.contents() {
            dest_dict.add("Contents", PdfObj::string(contents));
        }

        Ok(dest_dict)
    }
}

//------------------- TextIcon ----------------------//

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextIcon {
    Comment,
    Key,
    Note,
    Help,
    NewParagraph,
    Paragraph,
    Insert,
}

impl TextIcon {
    pub fn as_str(&self) -> &'static str {
        match self {
            TextIcon::Comment => "Comment",
            TextIcon::Key => "Key",
            TextIcon::Note => "Note",
            TextIcon::Help => "Help",
            TextIcon::NewParagraph => "NewParagraph",
            TextIcon::Paragraph => "Paragraph",
            TextIcon::Insert => "Insert",
        }
    }
}

//------------------- TextAnnotation ----------------------//

pub struct TextAnnotation {
    pub rect: Rectangle,
    pub contents: String,
    pub flags: AnnotationFlags,
    pub color: Option<RGB>,
    pub icon: TextIcon,
}

impl Default for TextAnnotation {
    fn default() -> Self {
        Self {
            rect: Rectangle {
                x1: 0.0,
                y1: 0.0,
                x2: 0.0,
                y2: 0.0,
            },
            contents: String::new(),
            flags: AnnotationFlags::PRINT,
            color: Some(RGB::new(
                // Default: yellow
                Color::new(1.0),
                Color::new(1.0),
                Color::new(0.0),
            )),
            icon: TextIcon::Note,
        }
    }
}

impl TextAnnotation {
    pub fn new(rect: Rectangle, contents: String) -> Self {
        Self {
            rect,
            contents,
            ..Default::default()
        }
    }

    pub fn with_icon(mut self, icon: TextIcon) -> Self {
        self.icon = icon;
        self
    }

    pub fn with_color(mut self, rgb: RGB) -> Self {
        self.color = Some(rgb);
        self
    }
}

impl Annotation for TextAnnotation {
    fn subtype(&self) -> &'static str {
        "Text"
    }

    fn rect(&self) -> Rectangle {
        self.rect
    }

    fn flags(&self) -> AnnotationFlags {
        self.flags
    }

    fn color(&self) -> Option<RGB> {
        self.color
    }

    fn contents(&self) -> Option<&str> {
        Some(&self.contents)
    }

    fn to_dict(&self) -> PdfResult<PdfDictionaryObject> {
        let mut dict = PdfDictionaryObject::new().typed("Annot");
        dict.add("Subtype", PdfObj::name(self.subtype()));
        dict.add("Rect", self.rect.as_pdf_array());
        if !self.flags.is_empty() {
            dict.add("F", self.flags.bits() as i64);
        }
        if let Some(rgb) = self.color {
            dict.add("C", rgb.as_pdf_array());
        }
        dict.add("Contents", PdfObj::string(self.contents.as_str()));
        dict.add("Name", PdfObj::name(self.icon.as_str()));

        Ok(dict)
    }
}

//-------------------LinkAction ----------------------

#[derive(Debug, Clone)]
pub enum LinkAction {
    Uri(String),
    GoTo {
        page: usize,
        position: Posn,
        zoom: Option<f64>,
    },
}

//-------------------LinkAnnotation ----------------------

pub struct LinkAnnotation {
    pub rect: Rectangle,
    pub flags: AnnotationFlags,
    pub border_style: Option<BorderStyle>,
    pub action: LinkAction,
}

impl LinkAnnotation {
    pub fn uri(rect: Rectangle, uri: String) -> Self {
        Self {
            rect,
            flags: AnnotationFlags::PRINT,
            border_style: None,
            action: LinkAction::Uri(uri),
        }
    }

    pub fn goto(rect: Rectangle, page: usize, position: Posn, zoom: Option<f64>) -> Self {
        Self {
            rect,
            flags: AnnotationFlags::PRINT,
            border_style: None,
            action: LinkAction::GoTo {
                page,
                position,
                zoom,
            },
        }
    }

    pub fn with_border(mut self, style: BorderStyle) -> Self {
        self.border_style = Some(style);
        self
    }
}

impl Annotation for LinkAnnotation {
    fn subtype(&self) -> &'static str {
        "Link"
    }

    fn rect(&self) -> Rectangle {
        self.rect
    }

    fn flags(&self) -> AnnotationFlags {
        self.flags
    }

    fn border_style(&self) -> Option<BorderStyle> {
        self.border_style
    }

    fn to_dict(&self) -> PdfResult<PdfDictionaryObject> {
        let mut dict = PdfDictionaryObject::new().typed("Annot");
        dict.add("Subtype", PdfObj::name(self.subtype()));
        dict.add("Rect", self.rect().as_pdf_array());

        let flags = self.flags();
        if flags.bits() != 0 {
            dict.add("F", flags.bits() as i64);
        }

        self.add_border_style_to_dict(&mut dict);

        match &self.action {
            LinkAction::Uri(uri) => {
                let mut action_dict = PdfDictionaryObject::new();
                action_dict.add("S", PdfObj::name("URI"));
                action_dict.add("URI", PdfObj::string(uri));
                dict.add("A", action_dict);
            }
            LinkAction::GoTo {
                page,
                position,
                zoom,
            } => {
                let mut dest = PdfArrayObject::new();
                dest.push(*page as i64);
                dest.push(PdfObj::name("XYZ"));
                dest.push(position.x);
                dest.push(position.y);
                if let Some(z) = zoom {
                    dest.push(*z);
                } else {
                    dest.push(PdfObj::name("null"));
                }
                
                dict.add("Dest", dest);
            }
        }

        Ok(dict)
    }
}

//------------------- test ----------------------//

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rect_to_array() {
        let rect = Rectangle {
            x1: 10.0,
            y1: 20.0,
            x2: 100.0,
            y2: 200.0,
        };
        let arr = rect.as_pdf_array();
        assert_eq!(arr.values.len(), 4);
    }

    #[test]
    fn test_annotation_flags() {
        let flags = AnnotationFlags::PRINT.or(AnnotationFlags::READ_ONLY);
        assert_eq!(flags.bits(), (1 << 2) | (1 << 6));
    }

    #[test]
    fn test_text_annotation() {
        let annot = TextAnnotation::new(
            Rectangle {
                x1: 100.0,
                y1: 100.0,
                x2: 120.0,
                y2: 120.0,
            },
            "This is a note".to_string(),
        );

        let dict = annot.to_dict().unwrap();
        assert!(dict.contains_key("Type"));
        assert!(dict.contains_key("Subtype"));
        assert!(dict.contains_key("Rect"));
        assert!(dict.contains_key("Contents"));
    }

    #[test]
    fn test_link_annotation_uri() {
        let annot = LinkAnnotation::uri(
            Rectangle {
                x1: 10.0,
                y1: 10.0,
                x2: 100.0,
                y2: 30.0,
            },
            "https://example.com".to_string(),
        );

        let dict = annot.to_dict().unwrap();
        assert!(dict.contains_key("A")); // Action dictionary
    }

    #[test]
    fn test_link_annotation_goto() {
        let annot = LinkAnnotation::goto(
            Rectangle {
                x1: 10.0,
                y1: 10.0,
                x2: 100.0,
                y2: 30.0,
            },
            5,
            Posn { x: 0.0, y: 0.0 },
            Some(1.0),
        );

        let dict = annot.to_dict().unwrap();
        assert!(dict.contains_key("Dest")); // Destination array
    }
}
