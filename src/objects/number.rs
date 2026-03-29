/// Integer Object:
///     mathematical integers with an implementation specified interval centered at 0 and written
///     as one or more decimal digits optionally preceded by a sign
/// Numeric Object:
///     either an integer object or a real (float) object
/// Real Object:
///     approximate mathematical real numbers, but with limited range and precision and written as
///     one or more decimal digits with an optional sign and a leading, trailing, or embedded
///     PERIOD (2Eh) (decimal point)
///
use crate::PdfObject;

//---------------- PdfNumberObject -----------------

#[derive(Debug, Clone, PartialEq)]
pub struct PdfNumberObject {
    pub value: NumberType,
}

impl PdfNumberObject {
    pub fn new(value: NumberType) -> Self {
        Self { value }
    }

    pub fn set_value<T: Into<NumberType>>(&mut self, value: T) {
        self.value = value.into();
    }

    pub fn as_int(&self) -> i64 {
        match self.value {
            NumberType::Integer(i) => i,
            NumberType::Real(f) => f as i64,
        }
    }

    pub fn as_real(&self) -> f64 {
        match self.value {
            NumberType::Integer(i) => i as f64,
            NumberType::Real(f) => f,
        }
    }
}

impl PdfObject for PdfNumberObject {
    fn serialise(&mut self) -> Vec<u8> {
        match self.value {
            NumberType::Integer(i) => i.to_string().into_bytes(),
            NumberType::Real(f) => {
                format!("{:.4}", f) // use a reasonable precisio
                    .trim_end_matches('0')
                    .trim_end_matches('.')
                    .to_string()
                    .into_bytes()
            }
        }
    }
}

impl From<NumberType> for PdfNumberObject {
    fn from(value: NumberType) -> Self {
        Self::new(value)
    }
}

impl From<i64> for PdfNumberObject {
    fn from(i: i64) -> Self {
        Self::new(NumberType::Integer(i))
    }
}

impl From<f64> for PdfNumberObject {
    fn from(f: f64) -> Self {
        Self::new(NumberType::Real(f))
    }
}

impl From<i32> for PdfNumberObject {
    fn from(i: i32) -> Self {
        Self::new(NumberType::Integer(i as i64))
    }
}

impl From<f32> for PdfNumberObject {
    fn from(f: f32) -> Self {
        Self::new(NumberType::Real(f as f64))
    }
}

//---------------- NumberType -----------------

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum NumberType {
    Integer(i64),
    Real(f64),
}

impl From<u8> for NumberType {
    fn from(i: u8) -> Self {
        Self::Integer(i as i64)
    }
}

impl From<usize> for NumberType {
    fn from(i: usize) -> Self {
        Self::Integer(i as i64)
    }
}

impl From<i64> for NumberType {
    fn from(i: i64) -> Self {
        Self::Integer(i)
    }
}

impl From<f64> for NumberType {
    fn from(f: f64) -> Self {
        Self::Real(f)
    }
}

impl From<i32> for NumberType {
    fn from(i: i32) -> Self {
        Self::Integer(i as i64)
    }
}

impl From<f32> for NumberType {
    fn from(f: f32) -> Self {
        Self::Real(f as f64)
    }
}
