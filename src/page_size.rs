use crate::util::Dims;

//--------------------------- PageSize ---------------------------//

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum PageSize {
    #[default]
    A4,
    Letter,
    Legal,
    A3,
    Custom(Dims), // points
}

impl PageSize {
    /// PDF points (1 point = 1/72 inch) or 0.0 for negative custom dimensions
    pub fn dims(&self) -> Dims {
        match self {
            PageSize::A4 => Dims {
                width: 595.0,
                height: 842.0,
            },
            PageSize::Letter => Dims {
                width: 612.0,
                height: 792.0,
            },
            PageSize::Legal => Dims {
                width: 612.0,
                height: 1008.0,
            },
            PageSize::A3 => Dims {
                width: 842.0,
                height: 1191.0,
            },
            PageSize::Custom(dims) => Dims {
                width: dims.width.max(0.0),
                height: dims.height.max(0.0),
            },
        }
    }
}

