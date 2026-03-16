//! Pattern framework for tiling and shading patterns.
//!
//! Patterns allow you to fill and stroke with repeating graphics (tiling)
//! or smooth color transitions (shading).

use crate::{DictionaryObject, NameObject, NumberObject, NumberType, PdfObject, Resource, ResourceCategory, ArrayObject};
use std::any::Any;
use std::rc::Rc;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PatternType {
    Tiling = 1,
    Shading = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TilingType {
    ConstantSpacing = 1,
    NoDistortion = 2,
    FasterTiling = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PaintType {
    Colored = 1,
    Uncolored = 2,
}

#[derive(Clone)]
pub struct TilingPattern {
    pub bounding_box: (f64, f64, f64, f64), // [xmin ymin xmax ymax]
    pub x_step: f64,
    pub y_step: f64,
    pub paint_type: PaintType,
    pub tiling_type: TilingType,
    pub content: Vec<u8>,
    pub matrix: Option<[f64; 6]>, // [a b c d e f]
}

impl TilingPattern {
    pub fn new(
        bbox: (f64, f64, f64, f64),
        x_step: f64,
        y_step: f64,
        content: Vec<u8>,
    ) -> Self {
        Self {
            bounding_box: bbox,
            x_step,
            y_step,
            paint_type: PaintType::Colored,
            tiling_type: TilingType::ConstantSpacing,
            content,
            matrix: None,
        }
    }

    pub fn with_paint_type(mut self, paint_type: PaintType) -> Self {
        self.paint_type = paint_type;
        self
    }

    pub fn with_tiling_type(mut self, tiling_type: TilingType) -> Self {
        self.tiling_type = tiling_type;
        self
    }

    pub fn with_matrix(mut self, matrix: [f64; 6]) -> Self {
        self.matrix = Some(matrix);
        self
    }

    pub fn to_stream(&self) -> crate::StreamObject {
        let mut extra_entries = Vec::new();

        // Pattern type
        extra_entries.push((
            "Type".to_string(),
            Rc::new(NameObject::new(Some("Pattern".to_string()))) as Rc<dyn PdfObject>,
        ));

        extra_entries.push((
            "PatternType".to_string(),
            Rc::new(NumberObject::new(NumberType::Integer(PatternType::Tiling as i64))) as Rc<dyn PdfObject>,
        ));

        // Paint type
        extra_entries.push((
            "PaintType".to_string(),
            Rc::new(NumberObject::new(NumberType::Integer(self.paint_type as i64))) as Rc<dyn PdfObject>,
        ));

        // Tiling type
        extra_entries.push((
            "TilingType".to_string(),
            Rc::new(NumberObject::new(NumberType::Integer(self.tiling_type as i64))) as Rc<dyn PdfObject>,
        ));

        // BBox
        let mut bbox_arr = ArrayObject::new(None);
        bbox_arr.push_object(Rc::new(NumberObject::new(NumberType::Real(self.bounding_box.0))));
        bbox_arr.push_object(Rc::new(NumberObject::new(NumberType::Real(self.bounding_box.1))));
        bbox_arr.push_object(Rc::new(NumberObject::new(NumberType::Real(self.bounding_box.2))));
        bbox_arr.push_object(Rc::new(NumberObject::new(NumberType::Real(self.bounding_box.3))));
        extra_entries.push((
            "BBox".to_string(),
            Rc::new(bbox_arr) as Rc<dyn PdfObject>,
        ));

        // XStep
        extra_entries.push((
            "XStep".to_string(),
            Rc::new(NumberObject::new(NumberType::Real(self.x_step))) as Rc<dyn PdfObject>,
        ));

        // YStep
        extra_entries.push((
            "YStep".to_string(),
            Rc::new(NumberObject::new(NumberType::Real(self.y_step))) as Rc<dyn PdfObject>,
        ));

        // Matrix (optional)
        if let Some(matrix) = self.matrix {
            let mut matrix_arr = ArrayObject::new(None);
            for &val in &matrix {
                matrix_arr.push_object(Rc::new(NumberObject::new(NumberType::Real(val))));
            }
            extra_entries.push((
                "Matrix".to_string(),
                Rc::new(matrix_arr) as Rc<dyn PdfObject>,
            ));
        }

        crate::StreamObject::new()
            .with_data(
                Some(vec![self.content.clone()]),
                Some(extra_entries)
            )
    }

    fn generate_id(&self) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        self.content.hash(&mut hasher);
        format!(
            "tiling:{}:{}:{}:{}",
            self.x_step,
            self.y_step,
            self.paint_type as u8,
            hasher.finish()
        )
    }
}

impl Resource for TilingPattern {
    fn category(&self) -> ResourceCategory {
        ResourceCategory::Pattern
    }

    fn resource_unique_id(&self) -> String {
        self.generate_id()
    }

    fn to_pdf_object(&self) -> Rc<dyn PdfObject> {
        Rc::new(self.to_stream())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShadingType {
    Function = 1,
    Axial = 2,
    Radial = 3,
    FreeFormGouraud = 4,
    LatticeGouraud = 5,
    CoonsPatch = 6,
    TensorPatch = 7,
}

/// Defines a smooth transition between colors along a line.
#[derive(Clone)]
pub struct AxialShading {
    pub start: (f64, f64), // x,y
    pub end: (f64, f64), // x,y
    pub start_color: (f64, f64, f64), // RGB
    pub end_color: (f64, f64, f64), // RGB
    pub extend_start: bool,
    pub extend_end: bool,
}

impl AxialShading {
    pub fn new(
        start: (f64, f64),
        end: (f64, f64),
        start_color: (f64, f64, f64),
        end_color: (f64, f64, f64),
    ) -> Self {
        Self {
            start,
            end,
            start_color,
            end_color,
            extend_start: false,
            extend_end: false,
        }
    }

    pub fn with_extend(mut self, start: bool, end: bool) -> Self {
        self.extend_start = start;
        self.extend_end = end;
        self
    }

    pub fn to_dict(&self) -> DictionaryObject {
        let mut dict = DictionaryObject::new(None);

        dict.set("ShadingType", Rc::new(NumberObject::new(NumberType::Integer(ShadingType::Axial as i64))));

        // ColorSpace
        dict.set("ColorSpace", Rc::new(NameObject::new(Some("DeviceRGB".to_string()))));

        // Coords [x0 y0 x1 y1]
        let mut coords = ArrayObject::new(None);
        coords.push_object(Rc::new(NumberObject::new(NumberType::Real(self.start.0))));
        coords.push_object(Rc::new(NumberObject::new(NumberType::Real(self.start.1))));
        coords.push_object(Rc::new(NumberObject::new(NumberType::Real(self.end.0))));
        coords.push_object(Rc::new(NumberObject::new(NumberType::Real(self.end.1))));
        dict.set("Coords", Rc::new(coords));

        // Function (simplified: direct color interpolation)
        // In a full implementation, this would be a proper PDF function object
        // For now, we use a simplified representation

        // Extend
        let mut extend = ArrayObject::new(None);
        extend.push_object(Rc::new(crate::BooleanObject::new(Some(self.extend_start))));
        extend.push_object(Rc::new(crate::BooleanObject::new(Some(self.extend_end))));
        dict.set("Extend", Rc::new(extend));

        dict
    }

    fn generate_id(&self) -> String {
        format!(
            "axial:{},{}->{},{}:rgb({},{},{})->rgb({},{},{})",
            self.start.0, self.start.1,
            self.end.0, self.end.1,
            self.start_color.0, self.start_color.1, self.start_color.2,
            self.end_color.0, self.end_color.1, self.end_color.2
        )
    }
}

impl Resource for AxialShading {
    fn category(&self) -> ResourceCategory {
        ResourceCategory::Shading
    }

    fn resource_unique_id(&self) -> String {
        self.generate_id()
    }

    fn to_pdf_object(&self) -> Rc<dyn PdfObject> {
        Rc::new(self.to_dict())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tiling_pattern_creation() {
        let pattern = TilingPattern::new(
            (0.0, 0.0, 10.0, 10.0),
            10.0,
            10.0,
            b"0 0 m 10 10 l S".to_vec(),
        );

        assert_eq!(pattern.bounding_box, (0.0, 0.0, 10.0, 10.0));
        assert_eq!(pattern.x_step, 10.0);
        assert_eq!(pattern.y_step, 10.0);
    }

    #[test]
    fn test_tiling_pattern_resource_trait() {
        let pattern = TilingPattern::new(
            (0.0, 0.0, 10.0, 10.0),
            10.0,
            10.0,
            vec![],
        );

        assert_eq!(pattern.category(), ResourceCategory::Pattern);
        assert!(!pattern.resource_unique_id().is_empty());
    }

    #[test]
    fn test_axial_shading_creation() {
        let shading = AxialShading::new(
            (0.0, 0.0),
            (100.0, 0.0),
            (1.0, 0.0, 0.0), // Red
            (0.0, 0.0, 1.0), // Blue
        );

        assert_eq!(shading.start, (0.0, 0.0));
        assert_eq!(shading.end, (100.0, 0.0));
    }

    #[test]
    fn test_axial_shading_to_dict() {
        let shading = AxialShading::new(
            (0.0, 0.0),
            (100.0, 100.0),
            (1.0, 0.0, 0.0),
            (0.0, 1.0, 0.0),
        );

        let dict = shading.to_dict();
        assert!(dict.contains_key("ShadingType"));
        assert!(dict.contains_key("ColorSpace"));
        assert!(dict.contains_key("Coords"));
    }

    #[test]
    fn test_axial_shading_resource_trait() {
        let shading = AxialShading::new(
            (0.0, 0.0),
            (100.0, 0.0),
            (0.0, 0.0, 0.0),
            (1.0, 1.0, 1.0),
        );

        assert_eq!(shading.category(), ResourceCategory::Shading);
        assert!(!shading.resource_unique_id().is_empty());
    }

    #[test]
    fn test_pattern_types() {
        assert_eq!(PatternType::Tiling as i64, 1);
        assert_eq!(PatternType::Shading as i64, 2);
    }

    #[test]
    fn test_shading_types() {
        assert_eq!(ShadingType::Axial as i64, 2);
        assert_eq!(ShadingType::Radial as i64, 3);
    }
}
