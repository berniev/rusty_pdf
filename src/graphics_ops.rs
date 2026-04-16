use std::cell::RefCell;
use crate::drawing_commands::DrawingCommands;
use crate::object_ops::ObjectOps;
use crate::{PdfDictionaryObject, PdfError};
use std::collections::HashMap;
use std::rc::Rc;

pub struct GraphicsOps {
    opacity_states: HashMap<u32, u64>, // opacity values (scaled to u32) to object numbers
    resource_counter: u32,
    soft_masks: Vec<SoftMask>, // for transparent gradients
    drawing_commands: DrawingCommands,
    object_ops: Rc<RefCell<ObjectOps>>,
}

/// for transparency effects.
pub struct SoftMask {
    pub name: String, // e.g., "GS1"
    pub shading_object_number: usize,
}

impl GraphicsOps {
    pub fn new(object_ops: Rc<RefCell<ObjectOps>>) -> Self {
        GraphicsOps {
            opacity_states: HashMap::new(),
            resource_counter: 0,
            soft_masks: Vec::new(),
            drawing_commands: DrawingCommands::new(),
            object_ops,
        }
    }

    pub fn get_or_create_opacity_state(&mut self, alpha: f32) -> Result<String,PdfError> {
        let opacity_key = (alpha * 1000.0) as u32; // by 1000 to avoid float precision issues
        if let Some(&obj_num) = self.opacity_states.get(&opacity_key) {
            // use existing resource name. Find the index of this object num to reconstruct the name
            let index = self
                .opacity_states
                .values()
                .position(|&v| v == obj_num)
                .unwrap();
            return Ok(format!("GS{}", index));
        }

        let resource_name = format!("GS{}", self.resource_counter);
        self.resource_counter += 1;

        let mut gs_dict = PdfDictionaryObject::new().typed("/ExtGState")?;
        gs_dict.add("CA", alpha as f64)?; // Stroke alpha
        gs_dict.add("ca", alpha as f64)?; // Fill alpha
        let obj_num = self.object_ops.borrow_mut().next_object_number();
        self.opacity_states.insert(opacity_key, obj_num);

        Ok(resource_name)
    }

    pub fn apply_opacity(&mut self, alpha: f32) ->Result<(),PdfError>{
        let resource_name = self.get_or_create_opacity_state(alpha)?;
        self.drawing_commands.set_state(&resource_name);

        Ok(())
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

//--------------------------- Tests -------------------------//

#[cfg(test)]
mod tests {
    use crate::Pdf;

    #[test]
    fn test_extgstate_dict() {
        let pdf = Pdf::new().expect(" ");
        let mut gs_manager = pdf.graphics_ops;

        gs_manager.get_or_create_opacity_state(0.5).expect("TODO: panic message");
        gs_manager.get_or_create_opacity_state(0.75).expect("TODO: panic message");

        let dict = gs_manager.get_extgstate_dict();
        assert_eq!(dict.len(), 2);
        assert!(dict.contains_key("GS0"));
        assert!(dict.contains_key("GS1"));
    }
}
