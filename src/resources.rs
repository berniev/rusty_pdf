use crate::{DictionaryObject, IndirectObject, PdfError, PdfResult};
use std::collections::HashMap;

pub const STANDARD_RESOURCE_CATEGORIES: &[&str] = &[
    "ColorSpace",
    "ExtGState",
    "Font",
    "Pattern",
    "Properties",
    "Shading",
    "XObject",
    "ProcSet",
];

#[derive(Clone)]
pub struct ResourceDictionary {
    categories: HashMap<String, HashMap<String, usize>>,
}

impl Default for ResourceDictionary {
    fn default() -> Self {
        Self::new()
    }
}

impl ResourceDictionary {
    pub fn new() -> Self {
        Self {
            categories: HashMap::new(),
        }
    }

    fn validate_resource_category(category: &str) -> PdfResult<()> {
        if STANDARD_RESOURCE_CATEGORIES.contains(&category) {
            Ok(())
        } else {
            Err(PdfError::StructureError(format!(
                "Invalid resource category: '{}'. Expected one of {:?}",
                category, STANDARD_RESOURCE_CATEGORIES
            )))
        }
    }

    pub fn implement(&mut self, category: &str) -> PdfResult<CategoryHandle<'_>> {
        Self::validate_resource_category(category)?;
        self.categories.entry(category.to_string()).or_default();

        Ok(CategoryHandle {
            dictionary: self,
            category: category.to_string(),
        })
    }

    pub fn add(&mut self, category: impl Into<String>, name: impl Into<String>, id: usize) {
        self.categories
            .entry(category.into())
            .or_default()
            .insert(name.into(), id);
    }

    /// Merges another ResourceDictionary into this one.
    ///
    /// Iterates through all categories in the 'other' dictionary. If a category
    /// exists in both, the resource maps are merged (with the 'other' values
    /// overwriting duplicates).
    pub fn merge(&mut self, other: &ResourceDictionary) {
        for (category, map) in &other.categories {
            self.categories
                .entry(category.clone())
                .or_default()
                .extend(map.clone());
        }
    }

    /// Transforms the logical resources into a physical DictionaryObject.
    pub fn to_dict(&self) -> DictionaryObject {
        let mut root = DictionaryObject::new(None);
        for (category, map) in &self.categories {
            let mut sub_dict = DictionaryObject::new(None);
            for (name, &id) in map {
                sub_dict.set(name, IndirectObject::build(id));
            }
            // Inlines the sub-dictionary directly into the Resources dictionary
            root.set(category, DictionaryObject::build(sub_dict.values));
        }

        root
    }

    pub fn is_empty(&self) -> bool {
        self.categories.is_empty()
    }

    pub fn clear(&mut self) {
        self.categories.clear();
    }

    pub fn category_count(&self, cat: &str) -> usize {
        self.categories.get(cat).map_or(0, |m| m.len())
    }
}

/// A "Base Structure" that captures context to provide a simple add(name, id) API.
pub struct CategoryHandle<'a> {
    dictionary: &'a mut ResourceDictionary,
    category: String,
}

impl<'a> CategoryHandle<'a> {
    pub fn add(&mut self, name: &str, id: usize) {
        self.dictionary.add(&self.category, name, id)
    }

    pub fn count(&self) -> usize {
        self.dictionary.category_count(&self.category)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_resources() {
        let resources = ResourceDictionary::new();
        assert!(resources.is_empty());
        assert_eq!(resources.to_dict().len(), 0);
    }

    #[test]
    fn test_add_resources() {
        let mut resources = ResourceDictionary::new();

        {
            let mut extgstate = resources.implement("ExtGState").unwrap();

            extgstate.add("GS0", 5);
            assert_eq!(extgstate.count(), 1);
        }
        {
            let mut pattern = resources.implement("Pattern").unwrap();
            pattern.add("P0", 8);
            assert_eq!(pattern.count(), 1);
        }

        assert!(!resources.is_empty());

        let dict = resources.to_dict();
        assert!(dict.contains_key("ExtGState"));
        assert!(dict.contains_key("Pattern"));
    }

    #[test]
    fn test_merge_resources() {
        let mut res1 = ResourceDictionary::new();
        {
            let mut extgstate = res1.implement("ExtGState").unwrap();
            extgstate.add("GS0", 5);
            assert_eq!(extgstate.count(), 1);
            assert_eq!(res1.category_count("ExtGState"), 1);
            assert_eq!(res1.category_count("Pattern"), 0);
        }

        let mut res2 = ResourceDictionary::new();
        {
            let mut pattern = res2.implement("Pattern").unwrap();
            pattern.add("P0", 8);
            assert_eq!(pattern.count(), 1);
        }

        res1.merge(&res2);
    }
}
