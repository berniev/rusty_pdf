use crate::{NumberType, PdfError};

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

    pub fn serialise(&mut self) -> Result<Vec<u8>, PdfError> {
        Ok(match self.value {
            NumberType::Integer(i) => i.to_string().into_bytes(),
            NumberType::Real(f) => {
                format!("{:.4}", f) // use a reasonable precisio
                    .trim_end_matches('0')
                    .trim_end_matches('.')
                    .to_string()
                    .into_bytes()
            }
        })
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

