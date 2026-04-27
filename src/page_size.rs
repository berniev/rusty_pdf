use crate::PdfArrayObject;
use crate::util::Dims;

//--------------------------- PageSize ---------------------------//

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum PageSize {
    A0,
    A1,
    A2,
    A3,
    #[default]
    A4,
    A5,
    Letter,
    Legal,
    Custom(Dims),
}
const MM_TO_POINTS: f64 = 2.8346456693;

impl PageSize {
    /// PDF points (1 point = 1/72 inch) or 0.0 for negative custom dimensions
    /// 1 point = 0.3527777778 mm
    pub fn dims_points(&self) -> Dims {
        match self {
            PageSize::A0 => Dims {
                width: 842.0 * MM_TO_POINTS,
                height: 1189.0 * MM_TO_POINTS,
            },
            PageSize::A1 => Dims {
                width: 594.0 * MM_TO_POINTS,
                height: 841.0 * MM_TO_POINTS,
            },
            PageSize::A2 => Dims {
                width: 420.0 * MM_TO_POINTS,
                height: 594.0 * MM_TO_POINTS,
            },
            PageSize::A3 => Dims {
                width: 297.0 * MM_TO_POINTS,
                height: 420.0 * MM_TO_POINTS,
            },
            PageSize::A4 => Dims {
                width: 210.0 * MM_TO_POINTS,
                height: 297.0 * MM_TO_POINTS,
            },
            PageSize::A5 => Dims {
                width: 148.0 * MM_TO_POINTS,
                height: 210.0 * MM_TO_POINTS,
            },
            PageSize::Letter => Dims {
                width: 612.0,
                height: 792.0,
            },
            PageSize::Legal => Dims {
                width: 612.0,
                height: 1008.0,
            },
            PageSize::Custom(dims) => Dims {
                width: dims.width.max(0.0),
                height: dims.height.max(0.0),
            },
        }
    }

    pub fn to_rect(&self) -> PdfArrayObject {
        let dims = self.dims_points();
        let mut arr = PdfArrayObject::new();
        arr.push(0.0);
        arr.push(0.0);
        arr.push(dims.width);
        arr.push(dims.height);

        arr
    }
}
