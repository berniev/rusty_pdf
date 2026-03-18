//! Optional Content (Layers) system for PDF.
//!
//! Optional Content Groups (OCGs) allow parts of a PDF to be selectively
//! visible or hidden, commonly used for layers in technical drawings.

use crate::{ArrayObject, DictionaryObject, NameObject, StringObject};

//------------------ VisibilityInitialState -----------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VisibilityInitialState {
    On,
    Off,
}

//------------------ UsageDict ---------------------

#[derive(Clone, Default)]
pub struct UsageDict {
    pub print: Option<UsageEntry>,
    pub view: Option<UsageEntry>,
    pub export: Option<UsageEntry>,
}

//------------------ UsageEntry ---------------------

#[derive(Clone)]
pub struct UsageEntry {
    pub state: VisibilityInitialState,
}

//------------------ OptionalContentGroup -----------------------

#[derive(Clone)]
pub struct OptionalContentGroup {
    pub name: String,
    pub intent: Option<Vec<String>>,
    pub initial_state: VisibilityInitialState,
    pub usage: Option<UsageDict>,
}

impl OptionalContentGroup {
    pub fn new(name: String) -> Self {
        Self {
            name,
            intent: None,
            initial_state: VisibilityInitialState::On,
            usage: None,
        }
    }

    pub fn with_state(mut self, state: VisibilityInitialState) -> Self {
        self.initial_state = state;
        self
    }

    pub fn with_intent(mut self, intent: Vec<String>) -> Self {
        self.intent = Some(intent);
        self
    }

    pub fn with_print_state(mut self, state: VisibilityInitialState) -> Self {
        let mut usage = self.usage.unwrap_or_default();
        usage.print = Some(UsageEntry { state });
        self.usage = Some(usage);
        self
    }

    pub fn with_view_state(mut self, state: VisibilityInitialState) -> Self {
        let mut usage = self.usage.unwrap_or_default();
        usage.view = Some(UsageEntry { state });
        self.usage = Some(usage);
        self
    }

    pub fn to_dict(&self) -> DictionaryObject {
        let mut dict = DictionaryObject::new(None);

        dict.set("Type", NameObject::make_pdf_obj("OCG"));
        dict.set("Name", StringObject::make_pdf_obj(self.name.clone()));

        if let Some(ref intent) = self.intent {
            if intent.len() == 1 {
                dict.set("Intent", NameObject::make_pdf_obj(&intent[0]));
            } else {
                let mut arr = ArrayObject::new(None);
                for i in intent {
                    arr.push_name(i);
                }
                dict.set("Intent", ArrayObject::make_pdf_obj(arr.values));
            }
        }

        if let Some(ref usage) = self.usage {
            let mut usage_dict = DictionaryObject::new(None);

            if let Some(ref print) = usage.print {
                let mut print_dict = DictionaryObject::new(None);
                print_dict.set(
                    "PrintState",
                    NameObject::make_pdf_obj(match print.state {
                        VisibilityInitialState::On => "ON",
                        VisibilityInitialState::Off => "OFF",
                    }),
                );
                usage_dict.set("Print", DictionaryObject::make_pdf_obj(print_dict.values));
            }

            if let Some(ref view) = usage.view {
                let mut view_dict = DictionaryObject::new(None);
                view_dict.set(
                    "ViewState",
                    NameObject::make_pdf_obj(match view.state {
                        VisibilityInitialState::On => "ON",
                        VisibilityInitialState::Off => "OFF",
                    }),
                );
                usage_dict.set("View", DictionaryObject::make_pdf_obj(view_dict.values));
            }

            if let Some(ref export) = usage.export {
                let mut export_dict = DictionaryObject::new(None);
                export_dict.set(
                    "ExportState",
                    NameObject::make_pdf_obj(match export.state {
                        VisibilityInitialState::On => "ON",
                        VisibilityInitialState::Off => "OFF",
                    }),
                );
                usage_dict.set("Export", DictionaryObject::make_pdf_obj(export_dict.values));
            }

            dict.set("Usage", DictionaryObject::make_pdf_obj(usage_dict.values));
        }

        dict
    }
}

//------------------ LayerOrder -----------------------

#[derive(Clone)]
pub enum LayerOrder {
    Single(usize),
    Group {
        label: String,
        children: Vec<LayerOrder>,
    },
}

//------------------ OptionalContentConfig -----------------------

/// Defines the default layer visibility and ordering.
pub struct OptionalContentConfig {
    pub name: String,
    pub creator: Option<String>,
    pub base_state: VisibilityInitialState,
    pub on_list: Vec<usize>,
    pub off_list: Vec<usize>,
    pub order: Vec<LayerOrder>,
}

impl OptionalContentConfig {
    pub fn new(name: String) -> Self {
        Self {
            name,
            creator: None,
            base_state: VisibilityInitialState::On,
            on_list: Vec::new(),
            off_list: Vec::new(),
            order: Vec::new(),
        }
    }

    pub fn with_base_state(mut self, state: VisibilityInitialState) -> Self {
        self.base_state = state;
        self
    }

    pub fn add_on(mut self, ocg_id: usize) -> Self {
        self.on_list.push(ocg_id);
        self
    }

    pub fn add_off(mut self, ocg_id: usize) -> Self {
        self.off_list.push(ocg_id);
        self
    }

    pub fn add_to_order(mut self, layer: LayerOrder) -> Self {
        self.order.push(layer);
        self
    }

    pub fn to_dict(&self) -> DictionaryObject {
        let mut dict = DictionaryObject::new(None);

        dict.set("Name", StringObject::make_pdf_obj(self.name.clone()));

        if let Some(ref creator) = self.creator {
            dict.set("Creator", StringObject::make_pdf_obj(creator.clone()));
        }

        dict.set(
            "BaseState",
            NameObject::make_pdf_obj(match self.base_state {
                VisibilityInitialState::On => "ON",
                VisibilityInitialState::Off => "OFF",
            }),
        );

        if !self.on_list.is_empty() {
            let mut arr = ArrayObject::new(None);
            for &id in &self.on_list {
                arr.push_indirect(id);
            }
            dict.set("ON", ArrayObject::make_pdf_obj(arr.values));
        }

        if !self.off_list.is_empty() {
            let mut arr = ArrayObject::new(None);
            for &id in &self.off_list {
                arr.push_indirect(id);
            }
            dict.set("OFF", ArrayObject::make_pdf_obj(arr.values));
        }

        // Order array (simplified - full implementation would handle nested groups)
        if !self.order.is_empty() {
            let order_arr = self.build_order_array(&self.order);
            dict.set("Order", ArrayObject::make_pdf_obj(order_arr.values));
        }

        dict
    }

    fn build_order_array(&self, orders: &[LayerOrder]) -> ArrayObject {
        let mut arr = ArrayObject::new(None);

        for order in orders {
            match order {
                LayerOrder::Single(id) => {
                    arr.push_indirect(*id);
                }
                LayerOrder::Group { label, children } => {
                    arr.push_string(label.clone());
                    let child_arr = self.build_order_array(children);
                    arr.push_array(child_arr);
                }
            }
        }

        arr
    }
}

//------------------ test -----------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ocg_creation() {
        let ocg = OptionalContentGroup::new("Layer 1".to_string());
        assert_eq!(ocg.name, "Layer 1");
        assert_eq!(ocg.initial_state, VisibilityInitialState::On);
    }

    #[test]
    fn test_ocg_with_state() {
        let ocg = OptionalContentGroup::new("Hidden Layer".to_string())
            .with_state(VisibilityInitialState::Off);

        assert_eq!(ocg.initial_state, VisibilityInitialState::Off);
    }

    #[test]
    fn test_ocg_to_dict() {
        let ocg = OptionalContentGroup::new("Test Layer".to_string())
            .with_print_state(VisibilityInitialState::Off);

        let dict = ocg.to_dict();
        assert!(dict.contains_key("Type"));
        assert!(dict.contains_key("Name"));
        assert!(dict.contains_key("Usage"));
    }

    #[test]
    fn test_oc_config_creation() {
        let config = OptionalContentConfig::new("Default".to_string())
            .with_base_state(VisibilityInitialState::On)
            .add_off(5);

        assert_eq!(config.name, "Default");
        assert_eq!(config.base_state, VisibilityInitialState::On);
        assert_eq!(config.off_list, vec![5]);
    }

    #[test]
    fn test_oc_config_to_dict() {
        let config = OptionalContentConfig::new("Config".to_string())
            .with_base_state(VisibilityInitialState::Off)
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
            children: vec![LayerOrder::Single(1), LayerOrder::Single(2)],
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
