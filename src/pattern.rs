//! Pattern framework for tiling and shading patterns.
//!
//! Patterns allow you to fill and stroke with repeating graphics (tiling)
//! or smooth color transitions (shading).

use std::any::Any;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[cfg(test)]
use crate::color::Color;
use crate::color::RGB;
use crate::objects::pdf_object::Pdf;
use crate::util::{Line, Matrix, Rect, ToPdf};
use crate::{
    PdfArrayObject, PdfDictionaryObject, PdfStreamObject, Resource,
    ResourceCategory,
};

//--------------------------- Axial Shading ----------------------//
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

    pub fn to_stream(&self) -> PdfStreamObject {
        let mut stream = PdfStreamObject::uncompressed();

        stream.add_to_content(self.content.clone());

        stream.dict.add("Type", Pdf::name("Pattern"));
        stream.dict.add("PatternType", Pdf::num(PatternType::Tiling as i64));
        stream.dict.add("PaintType", Pdf::num(self.paint_type as i64));
        stream.dict.add("TilingType", Pdf::num(self.tiling_type as i64));
        stream.dict.add("BBox", Pdf::array(self.bounding_box.as_pdf_array()));
        stream.dict.add("XStep", Pdf::num(self.x_step));
        stream.dict.add("YStep", Pdf::num(self.y_step));

        if let Some(matrix) = self.matrix {
            stream.dict.add("Matrix", Pdf::array(matrix.as_pdf_array()));
        }

        stream
    }

    fn generate_id(&self) -> String {
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
    pub line: Line,
    pub start_color: RGB,
    pub end_color: RGB,
    pub extend_start: bool,
    pub extend_end: bool,
}

impl AxialShading {
    pub fn new(line: Line, start_color: RGB, end_color: RGB) -> Self {
        Self {
            line,
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

    pub fn to_dict(&self) -> PdfDictionaryObject {
        let mut dict = PdfDictionaryObject::new();

        dict.add("ShadingType", Pdf::num(ShadingType::Axial as i64));
        dict.add("ColorSpace", Pdf::name("DeviceRGB"));
        dict.add("Coords", Pdf::array(self.line.as_pdf_array()));

        // Function (simplified: direct color interpolation)
        // In a full implementation, this would be a proper PDF function object
        // todo: For now, we use a simplified representation

        let mut extend_arr = PdfArrayObject::new();
        extend_arr.push(Pdf::bool(self.extend_start));
        extend_arr.push(Pdf::bool(self.extend_end));
        dict.add("Extend", Pdf::array(extend_arr));

        dict
    }

    fn generate_id(&self) -> String {
        format!(
            "axial:{}->{}: {}->{} ",
            self.line.start.as_string(),
            self.line.end.as_string(),
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

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::Posn;

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
            Line {
                start: Posn { x: 0.0, y: 0.0 },
                end: Posn { x: 100.0, y: 0.0 },
            },
            RGB::new(Color::new(1.0), Color::new(0.0), Color::new(0.0)), // Red
            RGB::new(Color::new(0.0), Color::new(0.0), Color::new(1.0)), // Blue
        );

        assert_eq!(shading.line.start, Posn { x: 0.0, y: 0.0 });
        assert_eq!(shading.line.end, Posn { x: 100.0, y: 0.0 });
    }

    #[test]
    fn test_axial_shading_to_dict() {
        let shading = AxialShading::new(
            Line {
                start: Posn { x: 0.0, y: 0.0 },
                end: Posn { x: 100.0, y: 100.0 },
            },
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
            Line {
                start: Posn { x: 0.0, y: 0.0 },
                end: Posn { x: 100.0, y: 0.0 },
            },
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
