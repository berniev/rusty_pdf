//! Annotation framework for interactive PDF features.
//!
//! Annotations are interactive elements that can be added to PDF pages, including
//! text notes, links, highlights, and form widgets.

use crate::{DictionaryObject, NameObject, NumberObject, NumberType, PdfResult, ArrayObject};
use std::rc::Rc;

/// Rectangle defining the annotation's location on the page.
///
/// Coordinates are in PDF default user space (points, from bottom-left).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rect {
    pub x1: f64,
    pub y1: f64,
    pub x2: f64,
    pub y2: f64,
}

impl Rect {
    /// Create a new rectangle.
    pub fn new(x1: f64, y1: f64, x2: f64, y2: f64) -> Self {
        Self { x1, y1, x2, y2 }
    }

    /// Convert to a PDF array object [x1 y1 x2 y2].
    pub fn to_array(&self) -> ArrayObject {
        let mut arr = ArrayObject::new(None);
        arr.push_object(Rc::new(NumberObject::new(NumberType::Real(self.x1))));
        arr.push_object(Rc::new(NumberObject::new(NumberType::Real(self.y1))));
        arr.push_object(Rc::new(NumberObject::new(NumberType::Real(self.x2))));
        arr.push_object(Rc::new(NumberObject::new(NumberType::Real(self.y2))));
        arr
    }
}

/// Annotation flags as defined in PDF specification.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AnnotationFlags(u32);

impl AnnotationFlags {
    /// No flags set.
    pub const NONE: Self = Self(0);

    /// Annotation is invisible (don't display, don't print).
    pub const INVISIBLE: Self = Self(1 << 0);

    /// Annotation is hidden (don't display, don't print).
    pub const HIDDEN: Self = Self(1 << 1);

    /// Print annotation when page is printed.
    pub const PRINT: Self = Self(1 << 2);

    /// Don't zoom annotation to fit screen.
    pub const NO_ZOOM: Self = Self(1 << 3);

    /// Don't rotate annotation with page.
    pub const NO_ROTATE: Self = Self(1 << 4);

    /// Hide annotation from view but allow printing.
    pub const NO_VIEW: Self = Self(1 << 5);

    /// Annotation is read-only (no interaction).
    pub const READ_ONLY: Self = Self(1 << 6);

    /// Annotation is locked (can't be deleted/modified).
    pub const LOCKED: Self = Self(1 << 7);

    /// Create flags from raw value.
    pub const fn from_bits(bits: u32) -> Self {
        Self(bits)
    }

    /// Get raw flag value.
    pub const fn bits(&self) -> u32 {
        self.0
    }

    /// Combine flags.
    pub const fn or(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}

/// Border style for annotations.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BorderStyle {
    /// Solid border.
    Solid,
    /// Dashed border.
    Dashed,
    /// Beveled (3D raised) border.
    Beveled,
    /// Inset (3D sunken) border.
    Inset,
    /// Underline only.
    Underline,
}

impl BorderStyle {
    /// Get the PDF name for this border style.
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
    /// Get the annotation subtype (Text, Link, Highlight, etc.)
    fn subtype(&self) -> &'static str;

    /// Get the annotation's rectangle on the page.
    fn rect(&self) -> Rect;

    /// Get annotation flags.
    fn flags(&self) -> AnnotationFlags {
        AnnotationFlags::NONE
    }

    /// Get the border style, if any.
    fn border_style(&self) -> Option<BorderStyle> {
        None
    }

    /// Get the annotation's color (RGB), if any.
    fn color(&self) -> Option<(f64, f64, f64)> {
        None
    }

    /// Get the annotation's contents (text description).
    fn contents(&self) -> Option<&str> {
        None
    }

    /// Convert this annotation to a PDF dictionary object.
    fn to_dict(&self) -> PdfResult<DictionaryObject> {
        let mut dict = DictionaryObject::new(None);

        // Required entries
        dict.set("Type", Rc::new(NameObject::new(Some("Annot".to_string()))));
        dict.set("Subtype", Rc::new(NameObject::new(Some(self.subtype().to_string()))));
        dict.set("Rect", Rc::new(self.rect().to_array()));

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

/// A text annotation (sticky note).
pub struct TextAnnotation {
    pub rect: Rect,
    pub contents: String,
    pub flags: AnnotationFlags,
    pub color: Option<(f64, f64, f64)>,
    pub icon: TextIcon,
}

/// Icon style for text annotations.
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
    /// Create a new text annotation.
    pub fn new(rect: Rect, contents: String) -> Self {
        Self {
            rect,
            contents,
            flags: AnnotationFlags::PRINT,
            color: Some((1.0, 1.0, 0.0)), // Default: yellow
            icon: TextIcon::Note,
        }
    }

    /// Set the icon style.
    pub fn with_icon(mut self, icon: TextIcon) -> Self {
        self.icon = icon;
        self
    }

    /// Set the color.
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

        // Required entries
        dict.set("Type", Rc::new(NameObject::new(Some("Annot".to_string()))));
        dict.set("Subtype", Rc::new(NameObject::new(Some(self.subtype().to_string()))));
        dict.set("Rect", Rc::new(self.rect().to_array()));

        // Optional common entries
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

/// A link annotation (hyperlink or internal jump).
pub struct LinkAnnotation {
    pub rect: Rect,
    pub flags: AnnotationFlags,
    pub border_style: Option<BorderStyle>,
    pub action: LinkAction,
}

/// Action for a link annotation.
#[derive(Debug, Clone)]
pub enum LinkAction {
    /// URI (external web link).
    Uri(String),
    /// Go to a destination (will be expanded in destination system).
    GoTo { page: usize, x: f64, y: f64, zoom: Option<f64> },
}

impl LinkAnnotation {
    /// Create a new link to a URI.
    pub fn uri(rect: Rect, uri: String) -> Self {
        Self {
            rect,
            flags: AnnotationFlags::PRINT,
            border_style: None,
            action: LinkAction::Uri(uri),
        }
    }

    /// Create a new link to a page.
    pub fn goto(rect: Rect, page: usize, x: f64, y: f64, zoom: Option<f64>) -> Self {
        Self {
            rect,
            flags: AnnotationFlags::PRINT,
            border_style: None,
            action: LinkAction::GoTo { page, x, y, zoom },
        }
    }

    /// Set border style.
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

        // Required entries
        dict.set("Type", Rc::new(NameObject::new(Some("Annot".to_string()))));
        dict.set("Subtype", Rc::new(NameObject::new(Some(self.subtype().to_string()))));
        dict.set("Rect", Rc::new(self.rect().to_array()));

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
        let rect = Rect::new(10.0, 20.0, 100.0, 200.0);
        let arr = rect.to_array();
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
            Rect::new(100.0, 100.0, 120.0, 120.0),
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
            Rect::new(10.0, 10.0, 100.0, 30.0),
            "https://example.com".to_string(),
        );

        let dict = annot.to_dict().unwrap();
        assert!(dict.contains_key("A")); // Action dictionary
    }

    #[test]
    fn test_link_annotation_goto() {
        let annot = LinkAnnotation::goto(Rect::new(10.0, 10.0, 100.0, 30.0), 5, 0.0, 0.0, Some(1.0));

        let dict = annot.to_dict().unwrap();
        assert!(dict.contains_key("Dest")); // Destination array
    }
}
