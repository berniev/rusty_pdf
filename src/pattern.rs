//! Pattern framework for tiling and shading patterns.
//!
//! Patterns allow you to fill and stroke with repeating graphics (tiling)
//! or smooth color transitions (shading).

use crate::{DictionaryObject, NameObject, NumberObject, NumberType, PdfObject, Resource, ResourceCategory, ArrayObject};
use std::any::Any;
use std::rc::Rc;

/// Pattern type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PatternType {
    /// Tiling pattern (repeating graphic).
    Tiling = 1,
    /// Shading pattern (smooth color transition).
    Shading = 2,
}

/// Tiling type for tiling patterns.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TilingType {
    /// Constant spacing.
    ConstantSpacing = 1,
    /// No distortion.
    NoDistortion = 2,
    /// Faster tiling.
    FasterTiling = 3,
}

/// Paint type for tiling patterns.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PaintType {
    /// Colored tiling pattern.
    Colored = 1,
    /// Uncolored tiling pattern.
    Uncolored = 2,
}

/// A tiling pattern (repeating graphic).
#[derive(Clone)]
pub struct TilingPattern {
    /// Bounding box of the pattern cell [xmin ymin xmax ymax].
    pub bbox: (f64, f64, f64, f64),

    /// Horizontal spacing.
    pub x_step: f64,

    /// Vertical spacing.
    pub y_step: f64,

    /// Paint type.
    pub paint_type: PaintType,

    /// Tiling type.
    pub tiling_type: TilingType,

    /// Pattern content stream (drawing commands).
    pub content: Vec<u8>,

    /// Transformation matrix [a b c d e f].
    pub matrix: Option<[f64; 6]>,
}

impl TilingPattern {
    /// Create a new tiling pattern.
    pub fn new(
        bbox: (f64, f64, f64, f64),
        x_step: f64,
        y_step: f64,
        content: Vec<u8>,
    ) -> Self {
        Self {
            bbox,
            x_step,
            y_step,
            paint_type: PaintType::Colored,
            tiling_type: TilingType::ConstantSpacing,
            content,
            matrix: None,
        }
    }

    /// Set paint type.
    pub fn with_paint_type(mut self, paint_type: PaintType) -> Self {
        self.paint_type = paint_type;
        self
    }

    /// Set tiling type.
    pub fn with_tiling_type(mut self, tiling_type: TilingType) -> Self {
        self.tiling_type = tiling_type;
        self
    }

    /// Set transformation matrix.
    pub fn with_matrix(mut self, matrix: [f64; 6]) -> Self {
        self.matrix = Some(matrix);
        self
    }

    /// Convert to PDF stream object.
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
        bbox_arr.push_object(Rc::new(NumberObject::new(NumberType::Real(self.bbox.0))));
        bbox_arr.push_object(Rc::new(NumberObject::new(NumberType::Real(self.bbox.1))));
        bbox_arr.push_object(Rc::new(NumberObject::new(NumberType::Real(self.bbox.2))));
        bbox_arr.push_object(Rc::new(NumberObject::new(NumberType::Real(self.bbox.3))));
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

    fn resource_id(&self) -> String {
        self.generate_id()
    }

    fn to_pdf_object(&self) -> Rc<dyn PdfObject> {
        Rc::new(self.to_stream())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Shading type for shading patterns.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShadingType {
    /// Function-based shading.
    Function = 1,
    /// Axial (linear gradient).
    Axial = 2,
    /// Radial (radial gradient).
    Radial = 3,
    /// Free-form Gouraud triangle mesh.
    FreeFormGouraud = 4,
    /// Lattice-form Gouraud triangle mesh.
    LatticeGouraud = 5,
    /// Coons patch mesh.
    CoonsPatch = 6,
    /// Tensor-product patch mesh.
    TensorPatch = 7,
}

/// An axial (linear) shading.
///
/// Defines a smooth transition between colors along a line.
#[derive(Clone)]
pub struct AxialShading {
    /// Starting point (x, y).
    pub start: (f64, f64),

    /// Ending point (x, y).
    pub end: (f64, f64),

    /// Starting color (RGB).
    pub start_color: (f64, f64, f64),

    /// Ending color (RGB).
    pub end_color: (f64, f64, f64),

    /// Extend shading beyond starting point.
    pub extend_start: bool,

    /// Extend shading beyond ending point.
    pub extend_end: bool,
}

impl AxialShading {
    /// Create a new axial shading (linear gradient).
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

    /// Set whether to extend shading.
    pub fn with_extend(mut self, start: bool, end: bool) -> Self {
        self.extend_start = start;
        self.extend_end = end;
        self
    }

    /// Convert to PDF shading dictionary.
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

    fn resource_id(&self) -> String {
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

        assert_eq!(pattern.bbox, (0.0, 0.0, 10.0, 10.0));
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
        assert!(!pattern.resource_id().is_empty());
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
        assert!(!shading.resource_id().is_empty());
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
