//! Annotation framework for interactive PDF features.
//!
//! Annotations are interactive elements that can be added to PDF pages, including
//! text notes, links, highlights, and form widgets.

use crate::color::Color;
use crate::color::RGB;
use crate::util::{Posn, Rect};
use crate::{ArrayObject, DictionaryObject, NameObject, NumberObject, PdfResult, StringObject};

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
    pub const fn or(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}

//-------------------Annotation Types ----------------------

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

//-------------------Annotation Types ----------------------

/// Base trait for all PDF annotations.
///
/// Annotations are interactive elements that can be attached to PDF pages.
pub trait Annotation {
    fn subtype(&self) -> &'static str;

    fn rect(&self) -> Rect;

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

    fn add_border_style_to_dict(&self, dict: &mut DictionaryObject) {
        if let Some(style) = self.border_style() {
            let mut bs = DictionaryObject::new(None);
            bs.set("S", NameObject::build(style.as_str()));
            dict.set_dict("BS", bs);
        }
    }

    fn to_dict(&self) -> PdfResult<DictionaryObject> {
        let mut dict = DictionaryObject::new(None);

        // Required entries
        dict.set("Type", NameObject::build("Annot"));
        dict.set("Subtype", NameObject::build(self.subtype()));
        dict.set_array("Rect", ArrayObject::from_rect(self.rect()));

        // Optional common entries
        let flags = self.flags();
        if flags.bits() != 0 {
            dict.set("F", NumberObject::build(flags.bits() as i64));
        }

        self.add_border_style_to_dict(&mut dict);

        if let Some(rgb) = self.color() {
            dict.set_array("C", ArrayObject::from_rgb(rgb));
        }

        if let Some(contents) = self.contents() {
            dict.set("Contents", StringObject::build(contents.to_string()));
        }

        Ok(dict)
    }
}

//-------------------TextIcon ----------------------

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

//-------------------TextAnnotation ----------------------

pub struct TextAnnotation {
    pub rect: Rect,
    pub contents: String,
    pub flags: AnnotationFlags,
    pub color: Option<RGB>,
    pub icon: TextIcon,
}

impl Default for TextAnnotation {
    fn default() -> Self {
        Self {
            rect: Rect {
                x1: 0.0,
                y1: 0.0,
                x2: 0.0,
                y2: 0.0,
            },
            contents: String::new(),
            flags: AnnotationFlags::PRINT,
            color: Some(RGB {
                // Default: yellow
                red: Color { color: 1.0 },
                green: Color { color: 1.0 },
                blue: Color { color: 0.0 },
            }),
            icon: TextIcon::Note,
        }
    }
}

impl TextAnnotation {
    pub fn new(rect: Rect, contents: String) -> Self {
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

    fn rect(&self) -> Rect {
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

    fn to_dict(&self) -> PdfResult<DictionaryObject> {
        let mut dict = DictionaryObject::new(None);

        // Required
        dict.set("Type", NameObject::build("Annot"));
        dict.set("Subtype", NameObject::build(self.subtype()));
        dict.set_array("Rect", ArrayObject::from_rect(self.rect));

        if self.flags.bits() != 0 {
            dict.set("F", NumberObject::build(self.flags.bits() as i64));
        }

        if let Some(rgb) = self.color {
            dict.set_array("C", ArrayObject::from_rgb(rgb));
        }

        dict.set("Contents", StringObject::build(self.contents.clone()));

        dict.set("Name", NameObject::build(self.icon.as_str()));

        Ok(dict)
    }
}

//-------------------LinkAction ----------------------

#[derive(Debug, Clone)]
pub enum LinkAction {
    Uri(String),
    GoTo {
        page: usize,
        position: Posn<f64>,
        zoom: Option<f64>,
    },
}

//-------------------LinkAnnotation ----------------------

pub struct LinkAnnotation {
    pub rect: Rect,
    pub flags: AnnotationFlags,
    pub border_style: Option<BorderStyle>,
    pub action: LinkAction,
}

impl LinkAnnotation {
    pub fn uri(rect: Rect, uri: String) -> Self {
        Self {
            rect,
            flags: AnnotationFlags::PRINT,
            border_style: None,
            action: LinkAction::Uri(uri),
        }
    }

    pub fn goto(rect: Rect, page: usize, position: Posn<f64>, zoom: Option<f64>) -> Self {
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

    fn rect(&self) -> Rect {
        self.rect
    }

    fn flags(&self) -> AnnotationFlags {
        self.flags
    }

    fn border_style(&self) -> Option<BorderStyle> {
        self.border_style
    }

    fn to_dict(&self) -> PdfResult<DictionaryObject> {
        let mut dict = DictionaryObject::new(None);

        // Required
        dict.set("Type", NameObject::build("Annot"));
        dict.set("Subtype", NameObject::build(self.subtype()));
        dict.set_array("Rect", ArrayObject::from_rect(self.rect()));

        // Optional
        let flags = self.flags();
        if flags.bits() != 0 {
            dict.set("F", NumberObject::build(flags.bits() as i64));
        }

        self.add_border_style_to_dict(&mut dict);

        // Link-specific action
        match &self.action {
            LinkAction::Uri(uri) => {
                let mut action_dict = DictionaryObject::new(None);
                action_dict.set("S", NameObject::build("URI"));
                action_dict.set("URI", StringObject::build(uri.clone()));
                dict.set_dict("A", action_dict);
            }
            LinkAction::GoTo {
                page,
                position,
                zoom,
            } => {
                // Create explicit destination array [page /XYZ x y zoom]
                let mut dest = ArrayObject::new(None);
                dest.push_number(*page as i64);
                dest.push_name("XYZ");
                dest.push_number(position.x);
                dest.push_number(position.y);
                if let Some(z) = zoom {
                    dest.push_number(*z);
                } else {
                    dest.push_name("null");
                }
                dict.set_array("Dest", dest);
            }
        }

        Ok(dict)
    }
}

//-------------------test ----------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rect_to_array() {
        let rect = Rect {
            x1: 10.0,
            y1: 20.0,
            x2: 100.0,
            y2: 200.0,
        };
        let arr = ArrayObject::from_rect(rect);
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
            Rect {
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
            Rect {
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
            Rect {
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
