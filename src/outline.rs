//! Document outline (bookmarks) system for PDF navigation.
//!
//! The outline provides a hierarchical table of contents that allows users
//! to navigate through the document.

use crate::{action::Destination, DictionaryObject, NameObject, PdfResult, StringObject, NumberObject, NumberType};
use std::rc::Rc;

/// A bookmark item in the document outline.
///
/// Bookmarks form a tree structure where each item can have children.
#[derive(Clone)]
pub struct OutlineItem {
    /// Display title for the bookmark.
    pub title: String,

    /// Destination to jump to when clicked.
    pub destination: Option<Destination>,

    /// Child bookmarks under this item.
    pub children: Vec<OutlineItem>,

    /// Whether this item is initially open (showing children).
    pub is_open: bool,

    /// Text color (RGB), if specified.
    pub color: Option<(f64, f64, f64)>,

    /// Text style flags.
    pub flags: OutlineItemFlags,
}

/// Text style flags for outline items.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OutlineItemFlags(u32);

impl OutlineItemFlags {
    /// Normal text.
    pub const NORMAL: Self = Self(0);

    /// Italic text.
    pub const ITALIC: Self = Self(1 << 0);

    /// Bold text.
    pub const BOLD: Self = Self(1 << 1);

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

impl OutlineItem {
    /// Create a new outline item.
    pub fn new(title: String, destination: Option<Destination>) -> Self {
        Self {
            title,
            destination,
            children: Vec::new(),
            is_open: true,
            color: None,
            flags: OutlineItemFlags::NORMAL,
        }
    }

    /// Add a child bookmark.
    pub fn add_child(&mut self, child: OutlineItem) {
        self.children.push(child);
    }

    /// Set whether the item is initially open.
    pub fn with_open(mut self, is_open: bool) -> Self {
        self.is_open = is_open;
        self
    }

    /// Set the text color.
    pub fn with_color(mut self, r: f64, g: f64, b: f64) -> Self {
        self.color = Some((r, g, b));
        self
    }

    /// Set text style flags.
    pub fn with_flags(mut self, flags: OutlineItemFlags) -> Self {
        self.flags = flags;
        self
    }

    /// Count total descendants (children + all nested children).
    pub fn count_descendants(&self) -> i32 {
        let mut count = self.children.len() as i32;
        for child in &self.children {
            count += child.count_descendants();
        }
        count
    }
}

/// The document outline (bookmark tree).
///
/// Represents the entire bookmark structure of the PDF document.
pub struct DocumentOutline {
    /// Root-level outline items.
    pub items: Vec<OutlineItem>,
}

impl DocumentOutline {
    /// Create a new empty outline.
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
        }
    }

    /// Add a root-level bookmark.
    pub fn add_item(&mut self, item: OutlineItem) {
        self.items.push(item);
    }

    /// Check if the outline is empty.
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Get total number of bookmarks at all levels.
    pub fn total_count(&self) -> usize {
        let mut count = self.items.len();
        for item in &self.items {
            count += item.count_descendants() as usize;
        }
        count
    }

    /// Convert the outline to PDF dictionary objects.
    ///
    /// Returns a tuple of (outline_dict, all_item_dicts) where object IDs need to be assigned.
    pub fn to_dicts(&self, allocate_id: &mut dyn FnMut() -> usize) -> PdfResult<OutlineDictionaries> {
        if self.items.is_empty() {
            return Ok(OutlineDictionaries {
                outline_dict: None,
                item_dicts: Vec::new(),
            });
        }

        let outline_id = allocate_id();
        let mut item_dicts = Vec::new();

        // Allocate IDs for all items first
        let mut item_ids = Vec::new();
        self.allocate_item_ids(&self.items, allocate_id, &mut item_ids);

        // Build item dictionaries
        let mut idx = 0;
        for (i, item) in self.items.iter().enumerate() {
            self.build_item_dict(
                item,
                &mut item_dicts,
                &item_ids,
                &mut idx,
                outline_id,
                if i > 0 { Some(item_ids[i - 1]) } else { None },
                if i < self.items.len() - 1 { Some(item_ids[i + 1]) } else { None },
            )?;
        }

        // Build outline dictionary
        let mut outline_dict = DictionaryObject::new(None);
        outline_dict.set("Type", Rc::new(NameObject::new(Some("Outlines".to_string()))));

        if !self.items.is_empty() {
            outline_dict.set_indirect("First", item_ids[0]);
            outline_dict.set_indirect("Last", item_ids[self.items.len() - 1]);

            let count = self.total_count() as i64;
            outline_dict.set("Count", Rc::new(NumberObject::new(NumberType::Integer(count))));
        }

        Ok(OutlineDictionaries {
            outline_dict: Some((outline_id, outline_dict)),
            item_dicts,
        })
    }

    fn allocate_item_ids(
        &self,
        items: &[OutlineItem],
        allocate_id: &mut dyn FnMut() -> usize,
        ids: &mut Vec<usize>,
    ) {
        for item in items {
            ids.push(allocate_id());
            if !item.children.is_empty() {
                self.allocate_item_ids(&item.children, allocate_id, ids);
            }
        }
    }

    fn build_item_dict(
        &self,
        item: &OutlineItem,
        dicts: &mut Vec<(usize, DictionaryObject)>,
        all_ids: &[usize],
        idx: &mut usize,
        parent_id: usize,
        prev_id: Option<usize>,
        next_id: Option<usize>,
    ) -> PdfResult<()> {
        let current_id = all_ids[*idx];
        *idx += 1;

        let mut dict = DictionaryObject::new(None);

        // Title
        dict.set("Title", Rc::new(StringObject::new(Some(item.title.clone()))));

        // Parent
        dict.set_indirect("Parent", parent_id);

        // Prev/Next siblings
        if let Some(prev) = prev_id {
            dict.set_indirect("Prev", prev);
        }
        if let Some(next) = next_id {
            dict.set_indirect("Next", next);
        }

        // Destination
        if let Some(ref dest) = item.destination {
            dict.set("Dest", Rc::new(dest.to_array()));
        }

        // Children
        if !item.children.is_empty() {
            let first_child_idx = *idx;
            let first_child_id = all_ids[first_child_idx];

            // Build all children
            for (i, child) in item.children.iter().enumerate() {
                let child_prev = if i > 0 { Some(all_ids[first_child_idx + i - 1]) } else { None };
                let child_next = if i < item.children.len() - 1 {
                    Some(all_ids[first_child_idx + i + 1])
                } else {
                    None
                };

                self.build_item_dict(
                    child,
                    dicts,
                    all_ids,
                    idx,
                    current_id,
                    child_prev,
                    child_next,
                )?;
            }

            dict.set_indirect("First", first_child_id);
            dict.set_indirect("Last", all_ids[first_child_idx + item.children.len() - 1]);

            // Count: positive if open, negative if closed
            let count = item.count_descendants();
            let count_val = if item.is_open { count } else { -count };
            dict.set("Count", Rc::new(NumberObject::new(NumberType::Integer(count_val as i64))));
        }

        // Color
        if let Some((r, g, b)) = item.color {
            let mut color_arr = crate::ArrayObject::new(None);
            color_arr.push_object(Rc::new(NumberObject::new(NumberType::Real(r))));
            color_arr.push_object(Rc::new(NumberObject::new(NumberType::Real(g))));
            color_arr.push_object(Rc::new(NumberObject::new(NumberType::Real(b))));
            dict.set("C", Rc::new(color_arr));
        }

        // Flags
        if item.flags.bits() != 0 {
            dict.set("F", Rc::new(NumberObject::new(NumberType::Integer(item.flags.bits() as i64))));
        }

        dicts.push((current_id, dict));
        Ok(())
    }
}

impl Default for DocumentOutline {
    fn default() -> Self {
        Self::new()
    }
}

/// Result of converting outline to dictionaries.
pub struct OutlineDictionaries {
    /// The main outline dictionary (Type /Outlines), if any.
    pub outline_dict: Option<(usize, DictionaryObject)>,

    /// All outline item dictionaries with their object IDs.
    pub item_dicts: Vec<(usize, DictionaryObject)>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::action::Destination;

    #[test]
    fn test_outline_item_creation() {
        let item = OutlineItem::new(
            "Chapter 1".to_string(),
            Some(Destination::fit(0)),
        );

        assert_eq!(item.title, "Chapter 1");
        assert!(item.destination.is_some());
        assert!(item.children.is_empty());
    }

    #[test]
    fn test_outline_item_with_children() {
        let mut parent = OutlineItem::new(
            "Part 1".to_string(),
            Some(Destination::fit(0)),
        );

        parent.add_child(OutlineItem::new(
            "Chapter 1".to_string(),
            Some(Destination::fit(1)),
        ));

        parent.add_child(OutlineItem::new(
            "Chapter 2".to_string(),
            Some(Destination::fit(2)),
        ));

        assert_eq!(parent.children.len(), 2);
        assert_eq!(parent.count_descendants(), 2);
    }

    #[test]
    fn test_document_outline() {
        let mut outline = DocumentOutline::new();

        outline.add_item(OutlineItem::new(
            "Introduction".to_string(),
            Some(Destination::fit(0)),
        ));

        outline.add_item(OutlineItem::new(
            "Conclusion".to_string(),
            Some(Destination::fit(10)),
        ));

        assert_eq!(outline.items.len(), 2);
        assert_eq!(outline.total_count(), 2);
    }

    #[test]
    fn test_outline_flags() {
        let flags = OutlineItemFlags::BOLD.or(OutlineItemFlags::ITALIC);
        assert_eq!(flags.bits(), 3);
    }

    #[test]
    fn test_empty_outline() {
        let outline = DocumentOutline::new();
        assert!(outline.is_empty());
        assert_eq!(outline.total_count(), 0);
    }

    #[test]
    fn test_nested_bookmarks() {
        let mut root = OutlineItem::new(
            "Part 1".to_string(),
            Some(Destination::fit(0)),
        );

        let mut chapter = OutlineItem::new(
            "Chapter 1".to_string(),
            Some(Destination::fit(1)),
        );

        chapter.add_child(OutlineItem::new(
            "Section 1.1".to_string(),
            Some(Destination::fit(2)),
        ));

        root.add_child(chapter);

        assert_eq!(root.count_descendants(), 2); // chapter + section
    }
}
