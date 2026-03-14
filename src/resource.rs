//! Resource management system for PDF objects.
//!
//! Resources are objects that can be referenced from content streams (fonts, images,
//! patterns, graphics states, etc.). This module provides a unified framework for
//! managing all resource types.

use crate::{PdfObject, PdfResult};
use std::any::Any;
use std::collections::HashMap;
use std::rc::Rc;

/// Category of PDF resource as defined in the PDF specification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ResourceCategory {
    /// Font resources (/Font)
    Font,
    /// External objects: images, forms (/XObject)
    XObject,
    /// Color spaces (/ColorSpace)
    ColorSpace,
    /// Patterns (/Pattern)
    Pattern,
    /// Shading dictionaries (/Shading)
    Shading,
    /// Graphics state parameters (/ExtGState)
    ExtGState,
    /// Properties for marked content (/Properties)
    Properties,
}

impl ResourceCategory {
    /// Get the PDF dictionary key for this resource category.
    pub fn as_str(&self) -> &'static str {
        match self {
            ResourceCategory::Font => "Font",
            ResourceCategory::XObject => "XObject",
            ResourceCategory::ColorSpace => "ColorSpace",
            ResourceCategory::Pattern => "Pattern",
            ResourceCategory::Shading => "Shading",
            ResourceCategory::ExtGState => "ExtGState",
            ResourceCategory::Properties => "Properties",
        }
    }
}

/// A resource that can be embedded in a PDF and referenced from content streams.
///
/// Resources are objects that must be registered in the page's resource dictionary
/// before they can be used in content streams.
pub trait Resource: Any {
    /// Get the resource category (Font, XObject, Pattern, etc.)
    fn category(&self) -> ResourceCategory;

    /// Generate a unique identifier for this resource for deduplication.
    /// Resources with the same identifier will be deduplicated.
    fn resource_id(&self) -> String;

    /// Convert this resource into a PDF object that will be written to the PDF.
    fn to_pdf_object(&self) -> Rc<dyn PdfObject>;

    /// Get the resource name to use in content streams (e.g., "F1", "Im1", "GS1").
    /// If None, the ResourceManager will auto-generate a name.
    fn suggested_name(&self) -> Option<String> {
        None
    }

    /// Allows downcasting to concrete types
    fn as_any(&self) -> &dyn Any;
}

/// Manages resource registration, deduplication, and name generation.
///
/// The ResourceManager ensures that:
/// - Each unique resource is only embedded once in the PDF
/// - Resources get consistent names across the document
/// - Resource dictionaries are correctly populated
pub struct ResourceManager {
    /// Map from resource_id -> (object_id, resource_name)
    registry: HashMap<String, (usize, String)>,

    /// Counter for auto-generating resource names per category
    name_counters: HashMap<ResourceCategory, usize>,

    /// Cached resources for potential reuse
    resources: HashMap<String, Rc<dyn Resource>>,
}

impl ResourceManager {
    /// Create a new ResourceManager.
    pub fn new() -> Self {
        Self {
            registry: HashMap::new(),
            name_counters: HashMap::new(),
            resources: HashMap::new(),
        }
    }

    /// Register a resource and get its (object_id, resource_name).
    ///
    /// If the resource was already registered (based on resource_id), returns
    /// the existing object_id and name. Otherwise, allocates a new object_id
    /// and generates a name.
    pub fn register<F>(
        &mut self,
        resource: Rc<dyn Resource>,
        allocate_object_id: F,
    ) -> PdfResult<(usize, String)>
    where
        F: FnOnce() -> usize,
    {
        let resource_id = resource.resource_id();

        // Check if already registered
        if let Some(&(obj_id, ref name)) = self.registry.get(&resource_id) {
            return Ok((obj_id, name.clone()));
        }

        // Generate or use suggested name
        let name = if let Some(suggested) = resource.suggested_name() {
            suggested
        } else {
            self.generate_name(resource.category())
        };

        // Allocate object ID
        let obj_id = allocate_object_id();

        // Register
        self.registry.insert(resource_id.clone(), (obj_id, name.clone()));
        self.resources.insert(resource_id, resource);

        Ok((obj_id, name))
    }

    /// Get the object_id and name for a previously registered resource.
    pub fn get(&self, resource_id: &str) -> Option<(usize, String)> {
        self.registry.get(resource_id).map(|(id, name)| (*id, name.clone()))
    }

    /// Get a registered resource by its resource_id.
    pub fn get_resource(&self, resource_id: &str) -> Option<Rc<dyn Resource>> {
        self.resources.get(resource_id).cloned()
    }

    /// Get all registered resources for a specific category.
    pub fn get_by_category(&self, category: ResourceCategory) -> Vec<(usize, String, Rc<dyn Resource>)> {
        self.resources
            .iter()
            .filter_map(|(res_id, resource)| {
                if resource.category() == category {
                    self.registry.get(res_id).map(|(obj_id, name)| {
                        (*obj_id, name.clone(), resource.clone())
                    })
                } else {
                    None
                }
            })
            .collect()
    }

    /// Generate a unique name for a resource category.
    fn generate_name(&mut self, category: ResourceCategory) -> String {
        let counter = self.name_counters.entry(category).or_insert(0);
        let name = format!("{}{}", Self::category_prefix(category), counter);
        *counter += 1;
        name
    }

    /// Get the standard prefix for resource names by category.
    fn category_prefix(category: ResourceCategory) -> &'static str {
        match category {
            ResourceCategory::Font => "F",
            ResourceCategory::XObject => "Im",
            ResourceCategory::ColorSpace => "CS",
            ResourceCategory::Pattern => "P",
            ResourceCategory::Shading => "Sh",
            ResourceCategory::ExtGState => "GS",
            ResourceCategory::Properties => "Pr",
        }
    }

    /// Clear all registered resources.
    pub fn clear(&mut self) {
        self.registry.clear();
        self.name_counters.clear();
        self.resources.clear();
    }

    /// Get the total number of registered resources.
    pub fn count(&self) -> usize {
        self.registry.len()
    }

    /// Get the number of registered resources for a specific category.
    pub fn count_category(&self, category: ResourceCategory) -> usize {
        self.resources
            .values()
            .filter(|r| r.category() == category)
            .count()
    }
}

impl Default for ResourceManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::NameObject;

    // Mock resource for testing
    struct MockFont {
        name: String,
    }

    impl Resource for MockFont {
        fn category(&self) -> ResourceCategory {
            ResourceCategory::Font
        }

        fn resource_id(&self) -> String {
            format!("font:{}", self.name)
        }

        fn to_pdf_object(&self) -> Rc<dyn PdfObject> {
            Rc::new(NameObject::new(Some(self.name.clone())))
        }

        fn as_any(&self) -> &dyn Any {
            self
        }
    }

    #[test]
    fn test_resource_registration() {
        let mut manager = ResourceManager::new();
        let mut next_id = 1;

        let font1 = Rc::new(MockFont {
            name: "Helvetica".to_string(),
        });

        let (obj_id, name) = manager
            .register(font1.clone(), || {
                let id = next_id;
                next_id += 1;
                id
            })
            .unwrap();

        assert_eq!(obj_id, 1);
        assert_eq!(name, "F0");
        assert_eq!(manager.count(), 1);
    }

    #[test]
    fn test_resource_deduplication() {
        let mut manager = ResourceManager::new();
        let mut next_id = 1;

        let font1 = Rc::new(MockFont {
            name: "Helvetica".to_string(),
        });

        let (obj_id1, name1) = manager
            .register(font1.clone(), || {
                let id = next_id;
                next_id += 1;
                id
            })
            .unwrap();

        // Register the same resource again
        let (obj_id2, name2) = manager
            .register(font1.clone(), || {
                let id = next_id;
                next_id += 1;
                id
            })
            .unwrap();

        // Should return the same object_id and name
        assert_eq!(obj_id1, obj_id2);
        assert_eq!(name1, name2);
        assert_eq!(manager.count(), 1); // Only one resource registered
    }

    #[test]
    fn test_multiple_categories() {
        let mut manager = ResourceManager::new();
        let mut next_id = 1;

        let font = Rc::new(MockFont {
            name: "Helvetica".to_string(),
        });

        let (_, font_name) = manager
            .register(font, || {
                let id = next_id;
                next_id += 1;
                id
            })
            .unwrap();

        assert_eq!(font_name, "F0");

        // Different category would get different prefix
        assert_eq!(manager.count_category(ResourceCategory::Font), 1);
        assert_eq!(manager.count_category(ResourceCategory::XObject), 0);
    }

    #[test]
    fn test_get_by_category() {
        let mut manager = ResourceManager::new();
        let mut next_id = 1;

        let font1 = Rc::new(MockFont {
            name: "Helvetica".to_string(),
        });
        let font2 = Rc::new(MockFont {
            name: "Times".to_string(),
        });

        manager
            .register(font1, || {
                let id = next_id;
                next_id += 1;
                id
            })
            .unwrap();

        manager
            .register(font2, || {
                let id = next_id;
                next_id += 1;
                id
            })
            .unwrap();

        let fonts = manager.get_by_category(ResourceCategory::Font);
        assert_eq!(fonts.len(), 2);
    }
}
