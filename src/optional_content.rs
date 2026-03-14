//! Optional Content (Layers) system for PDF.
//!
//! Optional Content Groups (OCGs) allow parts of a PDF to be selectively
//! visible or hidden, commonly used for layers in technical drawings.

use crate::{DictionaryObject, NameObject, StringObject, ArrayObject};
use std::rc::Rc;

/// An Optional Content Group (layer).
///
/// OCGs control the visibility of content in the PDF.
#[derive(Clone)]
pub struct OptionalContentGroup {
    /// Human-readable name for the layer.
    pub name: String,

    /// Intent (View, Design, or custom).
    pub intent: Option<Vec<String>>,

    /// Initial visibility state.
    pub initial_state: VisibilityState,

    /// Usage dictionary (Print, View, Export settings).
    pub usage: Option<UsageDict>,
}

/// Visibility state for layers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VisibilityState {
    /// Layer is initially visible.
    On,
    /// Layer is initially hidden.
    Off,
}

/// Usage dictionary for optional content.
///
/// Specifies how the layer should behave in different contexts.
#[derive(Clone, Default)]
pub struct UsageDict {
    /// Print settings.
    pub print: Option<UsageEntry>,

    /// View settings.
    pub view: Option<UsageEntry>,

    /// Export settings.
    pub export: Option<UsageEntry>,
}

/// Usage entry for a specific context.
#[derive(Clone)]
pub struct UsageEntry {
    /// Visibility state in this context.
    pub state: VisibilityState,
}

impl OptionalContentGroup {
    /// Create a new optional content group (layer).
    pub fn new(name: String) -> Self {
        Self {
            name,
            intent: None,
            initial_state: VisibilityState::On,
            usage: None,
        }
    }

    /// Set the initial visibility state.
    pub fn with_state(mut self, state: VisibilityState) -> Self {
        self.initial_state = state;
        self
    }

    /// Set the intent.
    pub fn with_intent(mut self, intent: Vec<String>) -> Self {
        self.intent = Some(intent);
        self
    }

    /// Set print visibility.
    pub fn with_print_state(mut self, state: VisibilityState) -> Self {
        let mut usage = self.usage.unwrap_or_default();
        usage.print = Some(UsageEntry { state });
        self.usage = Some(usage);
        self
    }

    /// Set view visibility.
    pub fn with_view_state(mut self, state: VisibilityState) -> Self {
        let mut usage = self.usage.unwrap_or_default();
        usage.view = Some(UsageEntry { state });
        self.usage = Some(usage);
        self
    }

    /// Convert to PDF dictionary.
    pub fn to_dict(&self) -> DictionaryObject {
        let mut dict = DictionaryObject::new(None);

        dict.set("Type", Rc::new(NameObject::new(Some("OCG".to_string()))));
        dict.set("Name", Rc::new(StringObject::new(Some(self.name.clone()))));

        // Intent
        if let Some(ref intent) = self.intent {
            if intent.len() == 1 {
                dict.set("Intent", Rc::new(NameObject::new(Some(intent[0].clone()))));
            } else {
                let mut arr = ArrayObject::new(None);
                for i in intent {
                    arr.push_object(Rc::new(NameObject::new(Some(i.clone()))));
                }
                dict.set("Intent", Rc::new(arr));
            }
        }

        // Usage
        if let Some(ref usage) = self.usage {
            let mut usage_dict = DictionaryObject::new(None);

            if let Some(ref print) = usage.print {
                let mut print_dict = DictionaryObject::new(None);
                print_dict.set("PrintState", Rc::new(NameObject::new(Some(
                    match print.state {
                        VisibilityState::On => "ON",
                        VisibilityState::Off => "OFF",
                    }.to_string()
                ))));
                usage_dict.set("Print", Rc::new(print_dict));
            }

            if let Some(ref view) = usage.view {
                let mut view_dict = DictionaryObject::new(None);
                view_dict.set("ViewState", Rc::new(NameObject::new(Some(
                    match view.state {
                        VisibilityState::On => "ON",
                        VisibilityState::Off => "OFF",
                    }.to_string()
                ))));
                usage_dict.set("View", Rc::new(view_dict));
            }

            if let Some(ref export) = usage.export {
                let mut export_dict = DictionaryObject::new(None);
                export_dict.set("ExportState", Rc::new(NameObject::new(Some(
                    match export.state {
                        VisibilityState::On => "ON",
                        VisibilityState::Off => "OFF",
                    }.to_string()
                ))));
                usage_dict.set("Export", Rc::new(export_dict));
            }

            dict.set("Usage", Rc::new(usage_dict));
        }

        dict
    }
}

/// Optional Content Configuration.
///
/// Defines the default layer visibility and ordering.
pub struct OptionalContentConfig {
    /// Name of this configuration.
    pub name: String,

    /// Creator of the configuration.
    pub creator: Option<String>,

    /// Base state (ON or OFF).
    pub base_state: VisibilityState,

    /// OCGs that are on by default (if base_state is OFF).
    pub on_list: Vec<usize>,

    /// OCGs that are off by default (if base_state is ON).
    pub off_list: Vec<usize>,

    /// Display order for layer panel.
    pub order: Vec<LayerOrder>,
}

/// Layer ordering for display.
#[derive(Clone)]
pub enum LayerOrder {
    /// Single OCG (object ID).
    Single(usize),

    /// Group of OCGs with label.
    Group {
        label: String,
        children: Vec<LayerOrder>,
    },
}

impl OptionalContentConfig {
    /// Create a new optional content configuration.
    pub fn new(name: String) -> Self {
        Self {
            name,
            creator: None,
            base_state: VisibilityState::On,
            on_list: Vec::new(),
            off_list: Vec::new(),
            order: Vec::new(),
        }
    }

    /// Set base state.
    pub fn with_base_state(mut self, state: VisibilityState) -> Self {
        self.base_state = state;
        self
    }

    /// Add OCG to ON list.
    pub fn add_on(mut self, ocg_id: usize) -> Self {
        self.on_list.push(ocg_id);
        self
    }

    /// Add OCG to OFF list.
    pub fn add_off(mut self, ocg_id: usize) -> Self {
        self.off_list.push(ocg_id);
        self
    }

    /// Add layer to display order.
    pub fn add_to_order(mut self, layer: LayerOrder) -> Self {
        self.order.push(layer);
        self
    }

    /// Convert to PDF dictionary.
    pub fn to_dict(&self) -> DictionaryObject {
        let mut dict = DictionaryObject::new(None);

        dict.set("Name", Rc::new(StringObject::new(Some(self.name.clone()))));

        if let Some(ref creator) = self.creator {
            dict.set("Creator", Rc::new(StringObject::new(Some(creator.clone()))));
        }

        dict.set("BaseState", Rc::new(NameObject::new(Some(
            match self.base_state {
                VisibilityState::On => "ON",
                VisibilityState::Off => "OFF",
            }.to_string()
        ))));

        if !self.on_list.is_empty() {
            let mut arr = ArrayObject::new(None);
            for &id in &self.on_list {
                arr.push_object(Rc::new(crate::IndirectObject::new(Some(id))));
            }
            dict.set("ON", Rc::new(arr));
        }

        if !self.off_list.is_empty() {
            let mut arr = ArrayObject::new(None);
            for &id in &self.off_list {
                arr.push_object(Rc::new(crate::IndirectObject::new(Some(id))));
            }
            dict.set("OFF", Rc::new(arr));
        }

        // Order array (simplified - full implementation would handle nested groups)
        if !self.order.is_empty() {
            let order_arr = self.build_order_array(&self.order);
            dict.set("Order", Rc::new(order_arr));
        }

        dict
    }

    fn build_order_array(&self, orders: &[LayerOrder]) -> ArrayObject {
        let mut arr = ArrayObject::new(None);

        for order in orders {
            match order {
                LayerOrder::Single(id) => {
                    arr.push_object(Rc::new(crate::IndirectObject::new(Some(*id))));
                }
                LayerOrder::Group { label, children } => {
                    arr.push_object(Rc::new(StringObject::new(Some(label.clone()))));
                    let child_arr = self.build_order_array(children);
                    arr.push_object(Rc::new(child_arr));
                }
            }
        }

        arr
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ocg_creation() {
        let ocg = OptionalContentGroup::new("Layer 1".to_string());
        assert_eq!(ocg.name, "Layer 1");
        assert_eq!(ocg.initial_state, VisibilityState::On);
    }

    #[test]
    fn test_ocg_with_state() {
        let ocg = OptionalContentGroup::new("Hidden Layer".to_string())
            .with_state(VisibilityState::Off);

        assert_eq!(ocg.initial_state, VisibilityState::Off);
    }

    #[test]
    fn test_ocg_to_dict() {
        let ocg = OptionalContentGroup::new("Test Layer".to_string())
            .with_print_state(VisibilityState::Off);

        let dict = ocg.to_dict();
        assert!(dict.contains_key("Type"));
        assert!(dict.contains_key("Name"));
        assert!(dict.contains_key("Usage"));
    }

    #[test]
    fn test_oc_config_creation() {
        let config = OptionalContentConfig::new("Default".to_string())
            .with_base_state(VisibilityState::On)
            .add_off(5);

        assert_eq!(config.name, "Default");
        assert_eq!(config.base_state, VisibilityState::On);
        assert_eq!(config.off_list, vec![5]);
    }

    #[test]
    fn test_oc_config_to_dict() {
        let config = OptionalContentConfig::new("Config".to_string())
            .with_base_state(VisibilityState::Off)
            .add_on(1)
            .add_on(2);

        let dict = config.to_dict();
        assert!(dict.contains_key("Name"));
        assert!(dict.contains_key("BaseState"));
        assert!(dict.contains_key("ON"));
    }

    #[test]
    fn test_layer_order() {
        let order = LayerOrder::Group {
            label: "Group 1".to_string(),
            children: vec![
                LayerOrder::Single(1),
                LayerOrder::Single(2),
            ],
        };

        match order {
            LayerOrder::Group { label, children } => {
                assert_eq!(label, "Group 1");
                assert_eq!(children.len(), 2);
            }
            _ => panic!("Expected Group"),
        }
    }
}
