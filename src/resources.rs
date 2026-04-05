use crate::resource_category::STANDARD_RESOURCE_CATEGORIES;
use crate::{PdfDictionaryObject, PdfError, PdfResult};
use std::collections::HashMap;

//--------------------------- ResourcesMap -------------------------------//

#[derive(Clone)]
pub struct ResourceMap {
    categories: HashMap<String, HashMap<String, usize>>,
}

impl ResourceMap {
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

    /// Transforms the logical resources into a physical DictionaryObject.
    pub fn to_dict(&self) -> PdfDictionaryObject {
        let mut root_dict = PdfDictionaryObject::new();
        for (name, map) in &self.categories {
            let mut sub_dict = PdfDictionaryObject::new();
            for (name, &id) in map {
                sub_dict.add(name, id);
            }
            // Inlines the sub-dictionary directly into the Resources dictionary
            root_dict.add(name, sub_dict);
        }

        root_dict
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

//--------------------------- CategoryHandle -------------------------------//

/// A "Base Structure" that captures context to provide a simple add(name, id) API.
pub struct CategoryHandle<'a> {
    dictionary: &'a mut ResourceMap,
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

//--------------------------- Tests ---------------------------------//

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_resources() {
        let resources = ResourceMap::new();
        assert!(resources.is_empty());
        assert_eq!(resources.to_dict().len(), 0);
    }

    #[test]
    fn test_add_resources() {
        let mut resources = ResourceMap::new();

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
}
