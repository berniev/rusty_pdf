//! Resource management for PDF objects.
//!
//! Resources are objects that can be referenced from content streams (fonts, images, patterns, 
//! graphics states, etc.).

use std::any::Any;

pub(crate) use crate::resource_category::ResourceCategory;

//------------------------ Resource -----------------------//

/// Must be registered in the page's resource dictionary before they can be used in content streams.
pub trait Resource: Any {
    fn category(&self) -> ResourceCategory;
    fn resource_unique_id(&self) -> String;

    /// Get the resource name to use in content streams (e.g., "F1", "Im1", "GS1").
    /// If None, the ResourceManager will auto-generate a name.
    fn suggest_name(&self) -> Option<String> {
        None // todo: ?
    }

    fn as_any(&self) -> &dyn Any; // downcast to concrete types
}
