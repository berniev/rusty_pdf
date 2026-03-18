use std::collections::HashMap;

use crate::{DictionaryObject, NumberObject};
use crate::objects::stream::StreamObject;
use crate::pdf::PDF;

pub struct GraphicsStateManager {
    opacity_states: HashMap<u32, usize>, // opacity values (scaled to u32) to object numbers
    resource_counter: u32,
    soft_masks: Vec<SoftMask>, // for transparent gradients
}

/// for transparency effects.
pub struct SoftMask {
    pub name: String, // e.g., "GS1"
    pub shading_object_number: usize,
}

impl GraphicsStateManager {
    pub fn new() -> Self {
        GraphicsStateManager {
            opacity_states: HashMap::new(),
            resource_counter: 0,
            soft_masks: Vec::new(),
        }
    }

    pub fn get_or_create_opacity_state(&mut self, pdf: &mut PDF, alpha: f32) -> String {
        let opacity_key = (alpha * 1000.0) as u32; // by 1000 to avoid float precision issues
        if let Some(&obj_num) = self.opacity_states.get(&opacity_key) {
            // use existing resource name. Find the index of this object num to reconstruct the name
            let index = self
                .opacity_states
                .values()
                .position(|&v| v == obj_num)
                .unwrap();
            return format!("GS{}", index);
        }

        let resource_name = format!("GS{}", self.resource_counter);
        self.resource_counter += 1;

        let mut gs_dict = DictionaryObject::typed("/ExtGState");
        gs_dict.set("CA", NumberObject::make_pdf_obj(alpha)); // Stroke alpha
        gs_dict.set("ca", NumberObject::make_pdf_obj(alpha)); // Fill alpha
        let obj_num = pdf.objects.len();
        pdf.add_object(Box::new(gs_dict));

        self.opacity_states.insert(opacity_key, obj_num);

        resource_name
    }

    pub fn apply_opacity(&mut self, stream: &mut StreamObject, pdf: &mut PDF, alpha: f32) {
        let resource_name = self.get_or_create_opacity_state(pdf, alpha);
        stream.set_state(&resource_name);
    }

    /// Returns a HashMap that can be used to build the page Resources dictionary.
    /// Maps resource names (e.g., "GS0") to object references (e.g., "5 0 R").
    pub fn get_extgstate_dict(&self) -> HashMap<String, Vec<u8>> {
        let mut extgstate_dict = HashMap::new();

        for (i, (_opacity_key, &obj_num)) in self.opacity_states.iter().enumerate() {
            let resource_name = format!("GS{}", i);
            let reference = format!("{} 0 R", obj_num).into_bytes();
            extgstate_dict.insert(resource_name, reference);
        }

        extgstate_dict
    }

    /// Returns the graphics state resource name that should be applied before using the pattern.
    pub fn add_soft_mask(&mut self, shading_obj: usize) -> String {
        let name = format!("GS{}", self.resource_counter);
        self.resource_counter += 1;

        self.soft_masks.push(SoftMask {
            name: name.clone(),
            shading_object_number: shading_obj,
        });

        name
    }

    pub fn get_soft_masks(&self) -> &[SoftMask] {
        &self.soft_masks
    }

    /// useful when starting a new page
    pub fn reset(&mut self) {
        self.resource_counter = 0;
    }
}

impl Default for GraphicsStateManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_opacity_state_creation() {
        let mut pdf = PDF::new();
        let mut gs_manager = GraphicsStateManager::new();

        let name1 = gs_manager.get_or_create_opacity_state(&mut pdf, 0.5);
        assert_eq!(name1, "GS0");

        let name2 = gs_manager.get_or_create_opacity_state(&mut pdf, 0.5);
        assert_eq!(name2, "GS0"); // Should reuse same state

        let name3 = gs_manager.get_or_create_opacity_state(&mut pdf, 0.75);
        assert_eq!(name3, "GS1"); // Different opacity
    }

    #[test]
    fn test_extgstate_dict() {
        let mut pdf = PDF::new();
        let mut gs_manager = GraphicsStateManager::new();

        gs_manager.get_or_create_opacity_state(&mut pdf, 0.5);
        gs_manager.get_or_create_opacity_state(&mut pdf, 0.75);

        let dict = gs_manager.get_extgstate_dict();
        assert_eq!(dict.len(), 2);
        assert!(dict.contains_key("GS0"));
        assert!(dict.contains_key("GS1"));
    }
}
