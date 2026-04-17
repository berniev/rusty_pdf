use crate::{PdfDictionaryObject, ResourceCategory};

#[derive(Clone)]
pub struct ResourcesDict {
    dictionary: PdfDictionaryObject,
}

impl ResourcesDict {
    pub fn new() -> Self {
        Self {
            dictionary: PdfDictionaryObject::new(),
        }
    }

    // returns generated resource name
    pub fn add(&mut self, cat: ResourceCategory, object_id: u64) -> String {
        let cat_str = cat.as_str();
        let name = format!("{}{}", cat.prefix(), self.category_count(cat));
        let mut cat_dict = self
            .dictionary
            .get_dict(cat_str)
            .ok()
            .cloned()
            .unwrap_or_else(PdfDictionaryObject::new);

        cat_dict.update_or_add(&*name, object_id);
        self.dictionary.update_or_add(cat_str, cat_dict);

        name
    }

    pub fn get(&self, cat: ResourceCategory) -> Option<&PdfDictionaryObject> {
        self.dictionary.get_dict(cat.as_str()).ok()
    }

    pub fn contains(&self, cat: ResourceCategory) -> bool {
        self.dictionary.contains_key(cat.as_str())
    }

    pub fn category_count(&self, cat: ResourceCategory) -> usize {
        self.dictionary
            .get_dict(cat.as_str())
            .map_or(0, |d| d.len()) 
        // todo bad to rely on len (eg an entry is deleted) use HashMap<ResourceCategory, usize> ?
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn len(&self) -> usize {
        self.dictionary.len()
    }
}

//--------------------------- Tests ---------------------------------//

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_resources() {
        let resources = ResourcesDict::new();
        assert!(resources.is_empty());
        assert_eq!(resources.len(), 0);
    }

    #[test]
    fn test_add_resources() {
        let mut res_dict = ResourcesDict::new();
        let name = res_dict.add(ResourceCategory::Font, 1);
        assert_eq!(name, "F0");

        let name = res_dict.add(ResourceCategory::ExtGState, 5);
        assert_eq!(name, "GS0");
        assert_eq!(res_dict.get(ResourceCategory::ExtGState).unwrap().len(), 1);

        let name = res_dict.add(ResourceCategory::Pattern, 8);
        assert_eq!(name, "P0");
        let name = res_dict.add(ResourceCategory::Pattern, 18);
        assert_eq!(name, "P1");
        assert_eq!(res_dict.get(ResourceCategory::Pattern).unwrap().len(), 2);

        assert!(!res_dict.is_empty());
        assert!(res_dict.contains(ResourceCategory::ExtGState));
        assert!(res_dict.contains(ResourceCategory::Pattern));
    }
}
