//! Annotation framework for interactive PDF features.
//!
//! Annotations are interactive elements that can be added to PDF pages, including
//! text notes, links, highlights, and form widgets.

use std::rc::Rc;

use crate::util::Rect;
use crate::{DictionaryObject, NameObject, NumberObject, NumberType, PdfResult, ArrayObject};

/// Rectangle defining the annotation's location on the page.
///
/// Coordinates are in PDF default user space (points, from bottom-left).
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

    fn color(&self) -> Option<(f64, f64, f64)> {
        None
    }

    fn contents(&self) -> Option<&str> {
        None
    }

    fn to_dict(&self) -> PdfResult<DictionaryObject> {
        let mut dict = DictionaryObject::new(None);

        // Required entries
        dict.set("Type", Rc::new(NameObject::new(Some("Annot".to_string()))));
        dict.set("Subtype", Rc::new(NameObject::new(Some(self.subtype().to_string()))));
        dict.set("Rect", Rc::new(ArrayObject::from_rect(self.rect())));

        // Optional common entries
        let flags = self.flags();
        if flags.bits() != 0 {
            dict.set("F", Rc::new(NumberObject::new(NumberType::Integer(flags.bits() as i64))));
        }

        if let Some(style) = self.border_style() {
            let mut bs = DictionaryObject::new(None);
            bs.set("S", Rc::new(NameObject::new(Some(style.as_str().to_string()))));
            dict.set("BS", Rc::new(bs));
        }

        if let Some((r, g, b)) = self.color() {
            let mut arr = ArrayObject::new(None);
            arr.push_object(Rc::new(NumberObject::new(NumberType::Real(r))));
            arr.push_object(Rc::new(NumberObject::new(NumberType::Real(g))));
            arr.push_object(Rc::new(NumberObject::new(NumberType::Real(b))));
            dict.set("C", Rc::new(arr));
        }

        if let Some(contents) = self.contents() {
            dict.set("Contents", Rc::new(crate::StringObject::new(Some(contents.to_string()))));
        }

        Ok(dict)
    }
}

pub struct TextAnnotation {
    pub rect: Rect,
    pub contents: String,
    pub flags: AnnotationFlags,
    pub color: Option<(f64, f64, f64)>,
    pub icon: TextIcon,
}

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

impl TextAnnotation {
    pub fn new(rect: Rect, contents: String) -> Self {
        Self {
            rect,
            contents,
            flags: AnnotationFlags::PRINT,
            color: Some((1.0, 1.0, 0.0)), // Default: yellow
            icon: TextIcon::Note,
        }
    }

    pub fn with_icon(mut self, icon: TextIcon) -> Self {
        self.icon = icon;
        self
    }

    pub fn with_color(mut self, r: f64, g: f64, b: f64) -> Self {
        self.color = Some((r, g, b));
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

    fn color(&self) -> Option<(f64, f64, f64)> {
        self.color
    }

    fn contents(&self) -> Option<&str> {
        Some(&self.contents)
    }

    fn to_dict(&self) -> PdfResult<DictionaryObject> {
        let mut dict = DictionaryObject::new(None);

        // Required
        dict.set("Type", Rc::new(NameObject::new(Some("Annot".to_string()))));
        dict.set("Subtype", Rc::new(NameObject::new(Some(self.subtype().to_string()))));
        dict.set("Rect", Rc::new(ArrayObject::from_rect(self.rect())));

        // Optional
        let flags = self.flags();
        if flags.bits() != 0 {
            dict.set("F", Rc::new(NumberObject::new(NumberType::Integer(flags.bits() as i64))));
        }

        if let Some((r, g, b)) = self.color() {
            let mut arr = ArrayObject::new(None);
            arr.push_object(Rc::new(NumberObject::new(NumberType::Real(r))));
            arr.push_object(Rc::new(NumberObject::new(NumberType::Real(g))));
            arr.push_object(Rc::new(NumberObject::new(NumberType::Real(b))));
            dict.set("C", Rc::new(arr));
        }

        if let Some(contents) = self.contents() {
            dict.set("Contents", Rc::new(crate::StringObject::new(Some(contents.to_string()))));
        }

        // Text annotation specific
        dict.set("Name", Rc::new(NameObject::new(Some(self.icon.as_str().to_string()))));
        Ok(dict)
    }
}

pub struct LinkAnnotation {
    pub rect: Rect,
    pub flags: AnnotationFlags,
    pub border_style: Option<BorderStyle>,
    pub action: LinkAction,
}

#[derive(Debug, Clone)]
pub enum LinkAction {
    Uri(String),
    GoTo { page: usize, x: f64, y: f64, zoom: Option<f64> },
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

    pub fn goto(rect: Rect, page: usize, x: f64, y: f64, zoom: Option<f64>) -> Self {
        Self {
            rect,
            flags: AnnotationFlags::PRINT,
            border_style: None,
            action: LinkAction::GoTo { page, x, y, zoom },
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
        dict.set("Type", Rc::new(NameObject::new(Some("Annot".to_string()))));
        dict.set("Subtype", Rc::new(NameObject::new(Some(self.subtype().to_string()))));
        dict.set("Rect", Rc::new(ArrayObject::from_rect(self.rect())));

        // Optional
        let flags = self.flags();
        if flags.bits() != 0 {
            dict.set("F", Rc::new(NumberObject::new(NumberType::Integer(flags.bits() as i64))));
        }

        if let Some(style) = self.border_style() {
            let mut bs = DictionaryObject::new(None);
            bs.set("S", Rc::new(NameObject::new(Some(style.as_str().to_string()))));
            dict.set("BS", Rc::new(bs));
        }

        // Link-specific action
        match &self.action {
            LinkAction::Uri(uri) => {
                let mut action_dict = DictionaryObject::new(None);
                action_dict.set("S", Rc::new(NameObject::new(Some("URI".to_string()))));
                action_dict.set("URI", Rc::new(crate::StringObject::new(Some(uri.clone()))));
                dict.set("A", Rc::new(action_dict));
            }
            LinkAction::GoTo { page, x, y, zoom } => {
                // Create explicit destination array [page /XYZ x y zoom]
                let mut dest = ArrayObject::new(None);
                dest.push_object(Rc::new(NumberObject::new(NumberType::Integer(*page as i64))));
                dest.push_object(Rc::new(NameObject::new(Some("XYZ".to_string()))));
                dest.push_object(Rc::new(NumberObject::new(NumberType::Real(*x))));
                dest.push_object(Rc::new(NumberObject::new(NumberType::Real(*y))));
                if let Some(z) = zoom {
                    dest.push_object(Rc::new(NumberObject::new(NumberType::Real(*z))));
                } else {
                    dest.push_object(Rc::new(NameObject::new(Some("null".to_string()))));
                }
                dict.set("Dest", Rc::new(dest));
            }
        }

        Ok(dict)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rect_to_array() {
        let rect = Rect { x1: 10.0, y1: 20.0, x2: 100.0, y2: 200.0 };
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
            Rect { x1: 100.0, y1: 100.0, x2: 120.0, y2: 120.0 },
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
            Rect { x1: 10.0, y1: 10.0, x2: 100.0, y2: 30.0 },
            "https://example.com".to_string(),
        );

        let dict = annot.to_dict().unwrap();
        assert!(dict.contains_key("A")); // Action dictionary
    }

    #[test]
    fn test_link_annotation_goto() {
        let annot = LinkAnnotation::goto(Rect { x1: 10.0, y1: 10.0, x2: 100.0, y2: 30.0 }, 5, 0.0, 0.0, Some(1.0));

        let dict = annot.to_dict().unwrap();
        assert!(dict.contains_key("Dest")); // Destination array
    }
}
