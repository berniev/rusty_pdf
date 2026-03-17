//! Document outline (bookmarks) system for PDF navigation.
//!
//! The outline provides a hierarchical table of contents that allows users
//! to navigate through the document.

use crate::{
    ArrayObject, DictionaryObject, NameObject, NumberType, PdfResult, StringObject,
    action::Destination, color::RGB,
};
use std::rc::Rc;

//------------------ OutlineItemFlags -----------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OutlineItemFlags(u32);

impl OutlineItemFlags {
    pub const NORMAL: Self = Self(0);
    pub const ITALIC: Self = Self(1 << 0);
    pub const BOLD: Self = Self(1 << 1);

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

//------------------ OutlineItem -----------------------

#[derive(Clone)]
pub struct OutlineItem {
    pub title: String,
    pub destination: Option<Destination>,
    pub children: Vec<OutlineItem>,
    pub is_open: bool,
    pub color: Option<RGB>,
    pub flags: OutlineItemFlags,
}

impl OutlineItem {
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

    pub fn add_child(&mut self, child: OutlineItem) {
        self.children.push(child);
    }

    pub fn with_open(mut self, is_open: bool) -> Self {
        self.is_open = is_open;
        self
    }

    pub fn with_color(mut self, rgb: RGB) -> Self {
        self.color = Some(rgb);
        self
    }

    pub fn with_flags(mut self, flags: OutlineItemFlags) -> Self {
        self.flags = flags;
        self
    }

    pub fn count_descendants(&self) -> i32 {
        let mut count = self.children.len() as i32;
        for child in &self.children {
            count += child.count_descendants();
        }
        count
    }
}

//------------------ DocumentOutline -----------------------

pub struct DocumentOutline {
    pub items: Vec<OutlineItem>, // Root-level outline items.
}

impl Default for DocumentOutline {
    fn default() -> Self {
        Self::new()
    }
}

impl DocumentOutline {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn add_item(&mut self, item: OutlineItem) {
        self.items.push(item);
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn total_count(&self) -> usize {
        let mut count = self.items.len();
        for item in &self.items {
            count += item.count_descendants() as usize;
        }
        count
    }

    /// Returns a tuple of (outline_dict, all_item_dicts) where object IDs need to be assigned.
    pub fn to_dicts(
        &self,
        allocate_id: &mut dyn FnMut() -> usize,
    ) -> PdfResult<OutlineDictionaries> {
        if self.items.is_empty() {
            return Ok(OutlineDictionaries {
                outline_dict: None,
                item_dicts: Vec::new(),
            });
        }

        let outline_id = allocate_id();
        let mut item_dicts = Vec::new();

        let mut item_ids = Vec::new();
        self.allocate_item_ids(&self.items, allocate_id, &mut item_ids);

        let mut idx = 0;
        for (i, item) in self.items.iter().enumerate() {
            self.build_item_dict(
                item,
                &mut item_dicts,
                &item_ids,
                &mut idx,
                outline_id,
                if i > 0 { Some(item_ids[i - 1]) } else { None },
                if i < self.items.len() - 1 {
                    Some(item_ids[i + 1])
                } else {
                    None
                },
            )?;
        }

        let mut outline_dict = DictionaryObject::new(None);
        outline_dict.set(
            "Type",
            Rc::new(NameObject::new(Some("Outlines".to_string()))),
        );

        if !self.items.is_empty() {
            outline_dict.set_indirect("First", item_ids[0]);
            outline_dict.set_indirect("Last", item_ids[self.items.len() - 1]);

            outline_dict.set_number("Count", NumberType::Integer(self.total_count() as i64));
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

        dict.set(
            "Title",
            Rc::new(StringObject::new(Some(item.title.clone()))),
        );

        dict.set_indirect("Parent", parent_id);

        if let Some(prev) = prev_id {
            dict.set_indirect("Prev", prev);
        }
        if let Some(next) = next_id {
            dict.set_indirect("Next", next);
        }

        if let Some(ref dest) = item.destination {
            dict.set_array("Dest", ArrayObject::from_destination_ref(dest));
        }

        if !item.children.is_empty() {
            let first_child_idx = *idx;
            let first_child_id = all_ids[first_child_idx];

            for (i, child) in item.children.iter().enumerate() {
                let child_prev = if i > 0 {
                    Some(all_ids[first_child_idx + i - 1])
                } else {
                    None
                };
                let child_next = if i < item.children.len() - 1 {
                    Some(all_ids[first_child_idx + i + 1])
                } else {
                    None
                };

                self.build_item_dict(
                    child, dicts, all_ids, idx, current_id, child_prev, child_next,
                )?;
            }

            dict.set_indirect("First", first_child_id);
            dict.set_indirect("Last", all_ids[first_child_idx + item.children.len() - 1]);

            // Count: positive if open, negative if closed
            let count = item.count_descendants();
            let count_val = if item.is_open { count } else { -count };
            dict.set_number("Count", NumberType::Integer(count_val as i64));
        }

        if let Some(rgb) = item.color {
            dict.set_array("C", ArrayObject::from_rgb(rgb));
        }

        if item.flags.bits() != 0 {
            dict.set_number("F", NumberType::Integer(item.flags.bits() as i64));
        }

        dicts.push((current_id, dict));
        Ok(())
    }
}

//------------------ OutlineDictionaries -----------------------

pub struct OutlineDictionaries {
    pub outline_dict: Option<(usize, DictionaryObject)>,
    pub item_dicts: Vec<(usize, DictionaryObject)>,
}

//------------------ test -----------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::action::Destination;

    #[test]
    fn test_outline_item_creation() {
        let item = OutlineItem::new("Chapter 1".to_string(), Some(Destination::fit(0)));

        assert_eq!(item.title, "Chapter 1");
        assert!(item.destination.is_some());
        assert!(item.children.is_empty());
    }

    #[test]
    fn test_outline_item_with_children() {
        let mut parent = OutlineItem::new("Part 1".to_string(), Some(Destination::fit(0)));

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
        let mut root = OutlineItem::new("Part 1".to_string(), Some(Destination::fit(0)));

        let mut chapter = OutlineItem::new("Chapter 1".to_string(), Some(Destination::fit(1)));

        chapter.add_child(OutlineItem::new(
            "Section 1.1".to_string(),
            Some(Destination::fit(2)),
        ));

        root.add_child(chapter);

        assert_eq!(root.count_descendants(), 2); // chapter + section
    }
}
