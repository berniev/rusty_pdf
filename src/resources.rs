use std::collections::HashMap;

/// Manages PDF resource dictionaries for pages.
///
/// Handles creation and organization of resource dictionaries that contain
/// ExtGState, Pattern, Font, and other resources needed by page content.
///
/// # Example
///
/// ```rust
/// use pydyf::resources::ResourceDictionary;
///
/// let mut resources = ResourceDictionary::new();
/// resources.add_extgstate("GS0".to_string(), 5);
/// resources.add_pattern("P0".to_string(), 8);
///
/// let dict = resources.build();
/// ```
pub struct ResourceDictionary {
    /// ExtGState resources (name -> object number)
    extgstates: HashMap<String, usize>,
    /// Pattern resources (name -> object number)
    patterns: HashMap<String, usize>,
    /// Font resources (name -> object number)
    fonts: HashMap<String, usize>,
    /// Shading resources (name -> object number)
    shadings: HashMap<String, usize>,
    /// XObject resources (name -> object number)
    xobjects: HashMap<String, usize>,
}

impl ResourceDictionary {
    /// Create a new empty resource dictionary.
    pub fn new() -> Self {
        ResourceDictionary {
            extgstates: HashMap::new(),
            patterns: HashMap::new(),
            fonts: HashMap::new(),
            shadings: HashMap::new(),
            xobjects: HashMap::new(),
        }
    }

    /// Add an ExtGState resource.
    ///
    /// # Arguments
    ///
    /// * `name` - Resource name (e.g., "GS0")
    /// * `obj_num` - Object number in the PDF
    pub fn add_extgstate(&mut self, name: String, obj_num: usize) {
        self.extgstates.insert(name, obj_num);
    }

    /// Add a Pattern resource.
    ///
    /// # Arguments
    ///
    /// * `name` - Resource name (e.g., "P0")
    /// * `obj_num` - Object number in the PDF
    pub fn add_pattern(&mut self, name: String, obj_num: usize) {
        self.patterns.insert(name, obj_num);
    }

    /// Add a Font resource.
    ///
    /// # Arguments
    ///
    /// * `name` - Resource name (e.g., "F0")
    /// * `obj_num` - Object number in the PDF
    pub fn add_font(&mut self, name: String, obj_num: usize) {
        self.fonts.insert(name, obj_num);
    }

    /// Add a Shading resource.
    ///
    /// # Arguments
    ///
    /// * `name` - Resource name (e.g., "Sh0")
    /// * `obj_num` - Object number in the PDF
    pub fn add_shading(&mut self, name: String, obj_num: usize) {
        self.shadings.insert(name, obj_num);
    }

    /// Add an XObject resource.
    ///
    /// # Arguments
    ///
    /// * `name` - Resource name (e.g., "Im0")
    /// * `obj_num` - Object number in the PDF
    pub fn add_xobject(&mut self, name: String, obj_num: usize) {
        self.xobjects.insert(name, obj_num);
    }

    /// Build the Resources dictionary as a HashMap.
    ///
    /// Returns a HashMap where keys are resource type names (e.g., "ExtGState", "Pattern")
    /// and values are the encoded dictionary content as bytes.
    ///
    /// This can be used to construct the Resources entry in a page dictionary.
    pub fn build(&self) -> HashMap<String, Vec<u8>> {
        let mut resources_dict = HashMap::new();

        // Build ExtGState dictionary
        if !self.extgstates.is_empty() {
            let mut extgstate_values = Vec::new();
            extgstate_values.push(b"<<".to_vec());
            for (name, obj_num) in &self.extgstates {
                extgstate_values.push(format!("/{} ", name).into_bytes());
                extgstate_values.push(format!("{} 0 R", obj_num).into_bytes());
                extgstate_values.push(b" ".to_vec());
            }
            extgstate_values.push(b">>".to_vec());
            resources_dict.insert("ExtGState".to_string(), extgstate_values.concat());
        }

        // Build Pattern dictionary
        if !self.patterns.is_empty() {
            let mut pattern_values = Vec::new();
            pattern_values.push(b"<<".to_vec());
            for (name, obj_num) in &self.patterns {
                pattern_values.push(format!("/{} ", name).into_bytes());
                pattern_values.push(format!("{} 0 R", obj_num).into_bytes());
                pattern_values.push(b" ".to_vec());
            }
            pattern_values.push(b">>".to_vec());
            resources_dict.insert("Pattern".to_string(), pattern_values.concat());
        }

        // Build Font dictionary
        if !self.fonts.is_empty() {
            let mut font_values = Vec::new();
            font_values.push(b"<<".to_vec());
            for (name, obj_num) in &self.fonts {
                font_values.push(format!("/{} ", name).into_bytes());
                font_values.push(format!("{} 0 R", obj_num).into_bytes());
                font_values.push(b" ".to_vec());
            }
            font_values.push(b">>".to_vec());
            resources_dict.insert("Font".to_string(), font_values.concat());
        }

        // Build Shading dictionary
        if !self.shadings.is_empty() {
            let mut shading_values = Vec::new();
            shading_values.push(b"<<".to_vec());
            for (name, obj_num) in &self.shadings {
                shading_values.push(format!("/{} ", name).into_bytes());
                shading_values.push(format!("{} 0 R", obj_num).into_bytes());
                shading_values.push(b" ".to_vec());
            }
            shading_values.push(b">>".to_vec());
            resources_dict.insert("Shading".to_string(), shading_values.concat());
        }

        // Build XObject dictionary
        if !self.xobjects.is_empty() {
            let mut xobject_values = Vec::new();
            xobject_values.push(b"<<".to_vec());
            for (name, obj_num) in &self.xobjects {
                xobject_values.push(format!("/{} ", name).into_bytes());
                xobject_values.push(format!("{} 0 R", obj_num).into_bytes());
                xobject_values.push(b" ".to_vec());
            }
            xobject_values.push(b">>".to_vec());
            resources_dict.insert("XObject".to_string(), xobject_values.concat());
        }

        resources_dict
    }

    /// Build the complete Resources dictionary entry as bytes.
    ///
    /// This creates the full `/Resources << ... >>` entry suitable for
    /// including in a page dictionary.
    pub fn build_bytes(&self) -> Vec<u8> {
        let dict = self.build();
        if dict.is_empty() {
            return Vec::new();
        }

        let mut result = Vec::new();
        result.extend(b"<<");

        for (key, value) in dict {
            result.push(b' ');
            result.push(b'/');
            result.extend(key.as_bytes());
            result.push(b' ');
            result.extend(value);
        }

        result.extend(b" >>");
        result
    }

    /// Check if the resource dictionary is empty.
    pub fn is_empty(&self) -> bool {
        self.extgstates.is_empty()
            && self.patterns.is_empty()
            && self.fonts.is_empty()
            && self.shadings.is_empty()
            && self.xobjects.is_empty()
    }

    /// Clear all resources.
    pub fn clear(&mut self) {
        self.extgstates.clear();
        self.patterns.clear();
        self.fonts.clear();
        self.shadings.clear();
        self.xobjects.clear();
    }

    /// Merge another resource dictionary into this one.
    ///
    /// If there are duplicate names, the other dictionary's values will overwrite this one's.
    pub fn merge(&mut self, other: &ResourceDictionary) {
        self.extgstates.extend(other.extgstates.clone());
        self.patterns.extend(other.patterns.clone());
        self.fonts.extend(other.fonts.clone());
        self.shadings.extend(other.shadings.clone());
        self.xobjects.extend(other.xobjects.clone());
    }

    /// Get the number of ExtGState resources.
    pub fn extgstate_count(&self) -> usize {
        self.extgstates.len()
    }

    /// Get the number of Pattern resources.
    pub fn pattern_count(&self) -> usize {
        self.patterns.len()
    }

    /// Get the number of Font resources.
    pub fn font_count(&self) -> usize {
        self.fonts.len()
    }
}

impl Default for ResourceDictionary {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_resources() {
        let resources = ResourceDictionary::new();
        assert!(resources.is_empty());
        assert_eq!(resources.build().len(), 0);
    }

    #[test]
    fn test_add_resources() {
        let mut resources = ResourceDictionary::new();
        resources.add_extgstate("GS0".to_string(), 5);
        resources.add_pattern("P0".to_string(), 8);

        assert!(!resources.is_empty());
        assert_eq!(resources.extgstate_count(), 1);
        assert_eq!(resources.pattern_count(), 1);

        let dict = resources.build();
        assert!(dict.contains_key("ExtGState"));
        assert!(dict.contains_key("Pattern"));
    }

    #[test]
    fn test_merge_resources() {
        let mut res1 = ResourceDictionary::new();
        res1.add_extgstate("GS0".to_string(), 5);

        let mut res2 = ResourceDictionary::new();
        res2.add_pattern("P0".to_string(), 8);

        res1.merge(&res2);
        assert_eq!(res1.extgstate_count(), 1);
        assert_eq!(res1.pattern_count(), 1);
    }
}
