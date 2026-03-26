//! Optional Content (Layers) system for PDF.
//!
//! Optional Content Groups (OCGs) allow parts of a PDF to be selectively
//! visible or hidden, commonly used for layers in technical drawings.

use crate::{PdfArrayObject, PdfDictionaryObject, PdfNameObject, PdfObject, PdfStringObject};

//------------------ VisibilityInitialState -----------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VisibilityInitialState {
    On,
    Off,
}

impl VisibilityInitialState {
    pub fn to_string(&self) -> String {
        match self {
            VisibilityInitialState::On => "ON".to_string(),
            VisibilityInitialState::Off => "OFF".to_string(),
        }
    }
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

    pub fn to_dict(&self) -> PdfDictionaryObject {
        let mut dict = PdfDictionaryObject::new().typed("OCG");
        dict.add_string("Name", self.name.clone());
        //dict.set("Name", PdfStringObject::new(self.name.clone()).boxed());

        if let Some(ref intent) = self.intent {
            if intent.len() == 1 {
                dict.set("Intent", PdfNameObject::new(&intent[0]).boxed());
            } else {
                let mut arr = PdfArrayObject::new();
                for i in intent {
                    arr.push_name(i);
                }
                dict.set("Intent", arr.boxed());
            }
        }

        if let Some(ref usage) = self.usage {
            let mut usage_dict = PdfDictionaryObject::new();

            if let Some(ref print) = usage.print {
                let mut print_dict = PdfDictionaryObject::new();
                print_dict.set(
                    "PrintState",
                    PdfNameObject::new(match print.state {
                        VisibilityInitialState::On => "ON",
                        VisibilityInitialState::Off => "OFF",
                    })
                    .boxed(),
                );
                usage_dict.set("Print", print_dict.boxed());
            }

            if let Some(ref view) = usage.view {
                let mut view_dict = PdfDictionaryObject::new();
                view_dict.set(
                    "ViewState",
                    PdfNameObject::new(&*view.state.to_string()).boxed(),
                );
                usage_dict.set("View", view_dict.boxed());
            }

            if let Some(ref export) = usage.export {
                let mut export_dict = PdfDictionaryObject::new();
                export_dict.set(
                    "ExportState",
                    PdfNameObject::new(match export.state {
                        VisibilityInitialState::On => "ON",
                        VisibilityInitialState::Off => "OFF",
                    })
                    .boxed(),
                );
                usage_dict.set("Export", export_dict.boxed());
            }

            dict.set("Usage", usage_dict.boxed());
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

    pub fn to_dict(&self) -> PdfDictionaryObject {
        let mut dict = PdfDictionaryObject::new();

        dict.set("Name", PdfStringObject::new(self.name.clone()).boxed());

        if let Some(ref creator) = self.creator {
            dict.set("Creator", PdfStringObject::new(creator.clone()).boxed());
        }

        dict.set(
            "BaseState",
            PdfNameObject::new(match self.base_state {
                VisibilityInitialState::On => "ON",
                VisibilityInitialState::Off => "OFF",
            })
            .boxed(),
        );

        if !self.on_list.is_empty() {
            let mut arr = PdfArrayObject::new();
            for &id in &self.on_list {
                arr.push_indirect(id);
            }
            dict.set("ON", arr.boxed());
        }

        if !self.off_list.is_empty() {
            let mut arr = PdfArrayObject::new();
            for &id in &self.off_list {
                arr.push_indirect(id);
            }
            dict.set("OFF", arr.boxed());
        }

        // Order array (simplified - full implementation would handle nested groups)
        if !self.order.is_empty() {
            dict.add_pdf_array("Order", self.build_order_array(&self.order));
        }

        dict
    }

    fn build_order_array(&self, orders: &[LayerOrder]) -> PdfArrayObject {
        let mut arr = PdfArrayObject::new();

        for order in orders {
            match order {
                LayerOrder::Single(id) => {
                    arr.push_indirect(*id);
                }
                LayerOrder::Group { label, children } => {
                    arr.push_string(label.clone());
                    arr.push_pdf_array(self.build_order_array(children));
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
