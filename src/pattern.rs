//! Pattern framework for tiling and shading patterns.
//!
//! Patterns allow you to fill and stroke with repeating graphics (tiling)
//! or smooth color transitions (shading).

use std::any::Any;
use std::rc::Rc;

#[cfg(test)]
use crate::color::Color;
use crate::color::RGB;
use crate::util::{Matrix, Posn, Rect, ToPdf};
use crate::{
    ArrayObject, DictionaryObject, NameObject, NumberObject, PdfObject, Resource, ResourceCategory,
};

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
    pub bounding_box: Rect,
    pub x_step: f64,
    pub y_step: f64,
    pub paint_type: PaintType,
    pub tiling_type: TilingType,
    pub content: Vec<u8>,
    pub matrix: Option<Matrix>,
}

impl TilingPattern {
    pub fn new(bbox: Rect, x_step: f64, y_step: f64, content: Vec<u8>) -> Self {
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

    pub fn with_matrix(mut self, matrix: Matrix) -> Self {
        self.matrix = Some(matrix);
        self
    }

    pub fn to_stream(&self) -> crate::StreamObject {
        let mut extra_entries = Vec::new();

        extra_entries.push(("Type".to_string(), NameObject::make_pdf_obj("Pattern")));

        extra_entries.push((
            "PatternType".to_string(),
            NumberObject::make_pdf_obj(PatternType::Tiling as i64),
        ));

        extra_entries.push((
            "PaintType".to_string(),
            NumberObject::make_pdf_obj(self.paint_type as i64),
        ));

        extra_entries.push((
            "TilingType".to_string(),
            NumberObject::make_pdf_obj(self.tiling_type as i64),
        ));

        extra_entries.push(("BBox".to_string(), self.bounding_box.make_pdf_obj()));

        extra_entries.push(("XStep".to_string(), NumberObject::make_pdf_obj(self.x_step)));

        extra_entries.push(("YStep".to_string(), NumberObject::make_pdf_obj(self.y_step)));

        if let Some(matrix) = self.matrix {
            extra_entries.push(("Matrix".to_string(), matrix.make_pdf_obj()));
        }

        crate::StreamObject::new().with_data(Some(vec![self.content.clone()]), Some(extra_entries))
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
    pub start: Posn<f64>,
    pub end: Posn<f64>,
    pub start_color: RGB,
    pub end_color: RGB,
    pub extend_start: bool,
    pub extend_end: bool,
}

impl AxialShading {
    pub fn new(start: Posn<f64>, end: Posn<f64>, start_color: RGB, end_color: RGB) -> Self {
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

        dict.set(
            "ShadingType",
            NumberObject::make_pdf_obj(ShadingType::Axial as i64),
        );
        dict.set("ColorSpace", NameObject::make_pdf_obj("DeviceRGB"));
        let coords_array = ArrayObject::from_points(self.start, self.end);
        dict.set("Coords", ArrayObject::make_pdf_obj(coords_array.values));

        // Function (simplified: direct color interpolation)
        // In a full implementation, this would be a proper PDF function object
        // For now, we use a simplified representation

        let mut extend = ArrayObject::new(None);
        extend.push_bool(self.extend_start);
        extend.push_bool(self.extend_end);
        dict.set("Extend", ArrayObject::make_pdf_obj(extend.values));

        dict
    }

    fn generate_id(&self) -> String {
        format!(
            "axial:{}->{}: {}->{} ",
            self.start.as_string(),
            self.end.as_string(),
            self.start_color.as_string(),
            self.end_color.as_string()
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
            Rect {
                x1: 0.0,
                y1: 0.0,
                x2: 10.0,
                y2: 10.0,
            },
            10.0,
            10.0,
            b"0 0 m 10 10 l S".to_vec(),
        );

        assert_eq!(
            pattern.bounding_box,
            Rect {
                x1: 0.0,
                y1: 0.0,
                x2: 10.0,
                y2: 10.0
            }
        );
        assert_eq!(pattern.x_step, 10.0);
        assert_eq!(pattern.y_step, 10.0);
    }

    #[test]
    fn test_tiling_pattern_resource_trait() {
        let pattern = TilingPattern::new(
            Rect {
                x1: 0.0,
                y1: 0.0,
                x2: 10.0,
                y2: 10.0,
            },
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
            Posn { x: 0.0, y: 0.0 },
            Posn { x: 100.0, y: 0.0 },
            RGB::new(Color::new(1.0), Color::new(0.0), Color::new(0.0)), // Red
            RGB::new(Color::new(0.0), Color::new(0.0), Color::new(1.0)), // Blue
        );

        assert_eq!(shading.start, Posn { x: 0.0, y: 0.0 });
        assert_eq!(shading.end, Posn { x: 100.0, y: 0.0 });
    }

    #[test]
    fn test_axial_shading_to_dict() {
        let shading = AxialShading::new(
            Posn { x: 0.0, y: 0.0 },
            Posn { x: 100.0, y: 100.0 },
            RGB::new(Color::new(1.0), Color::new(0.0), Color::new(0.0)),
            RGB::new(Color::new(0.0), Color::new(1.0), Color::new(0.0)),
        );

        let dict = shading.to_dict();
        assert!(dict.contains_key("ShadingType"));
        assert!(dict.contains_key("ColorSpace"));
        assert!(dict.contains_key("Coords"));
    }

    #[test]
    fn test_axial_shading_resource_trait() {
        let shading = AxialShading::new(
            Posn { x: 0.0, y: 0.0 },
            Posn { x: 100.0, y: 0.0 },
            RGB::new(Color::new(0.0), Color::new(0.0), Color::new(0.0)),
            RGB::new(Color::new(1.0), Color::new(1.0), Color::new(1.0)),
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
