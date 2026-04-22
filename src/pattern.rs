use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use crate::objects::pdf_object::PdfObj;
use crate::util::{Matrix, Rectangle};
use crate::{PdfError, PdfStreamObject};

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

//----------------------------- TilingPattern --------------------------//

#[derive(Clone)]
pub struct TilingPattern {
    stream: PdfStreamObject,
    x_step: f64,
    y_step: f64,
    paint_type: PaintType,
}

impl TilingPattern {
    pub fn new(
        bbox: Rectangle,
        x_step: f64,
        y_step: f64,
        paint_type: PaintType,
        tiling_type: TilingType,
        content: Vec<u8>,
    ) -> Result<Self, PdfError> {
        let mut pat = TilingPattern {
            stream: PdfStreamObject::new(),
            x_step,
            y_step,
            paint_type,
        };
        pat.stream
            .dict
            .add("Type", PdfObj::make_name_obj("Pattern"))?;
        pat.stream.dict.add("BBox", bbox.as_pdf_array())?;
        pat.stream.dict.add("XStep", x_step)?;
        pat.stream.dict.add("YStep", y_step)?;
        pat.stream.dict.add("PaintType", paint_type as i64)?;
        pat.stream.dict.add("TilingType", tiling_type as i64)?;
        pat.stream.content = content;

        Ok(pat)
    }

    pub fn with_matrix(mut self, matrix: Matrix) -> Result<Self, PdfError> {
        self.stream.dict.add("Matrix", matrix.as_pdf_array())?;

        Ok(self)
    }

    pub fn hash(&self) ->String {
        let mut hasher = DefaultHasher::new();
        self.stream.content.hash(&mut hasher);

        format!(
            "tiling:{}:{}:{}:{}",
            self.x_step,
            self.y_step,
            self.paint_type as u64,
            hasher.finish()
        )
    }
}
