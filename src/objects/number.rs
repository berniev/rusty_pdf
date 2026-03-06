use crate::objects::metadata::PdfMetadata;
use crate::PdfObject;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum NumberType {
    Integer(i64),
    Real(f64),
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

#[derive(Debug, Clone, PartialEq)]
pub struct NumberObject {
    pub metadata: PdfMetadata,
    pub value: NumberType,
}

impl NumberObject {
    pub fn new(value: NumberType) -> Self {
        Self {
            metadata: PdfMetadata::default(),
            value,
        }
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

    pub fn set_value<T: Into<NumberType>>(&mut self, value: T) {
        self.value = value.into();
    }
}

impl PdfObject for NumberObject {
    fn metadata(&self) -> &PdfMetadata {
        &self.metadata
    }

    fn metadata_mut(&mut self) -> &mut PdfMetadata {
        &mut self.metadata
    }

    fn data(&self) -> Vec<u8> {
        match self.value {
            NumberType::Integer(i) => i.to_string().into_bytes(),
            NumberType::Real(f) => {
                // Formatting real numbers for PDF: usually avoid scientific notation
                // and use a reasonable precision.
                format!("{:.4}", f)
                    .trim_end_matches('0')
                    .trim_end_matches('.')
                    .to_string()
                    .into_bytes()
            }
        }
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn is_compressible(&self) -> bool {
        self.metadata.generation == 0
    }
}

impl From<NumberType> for NumberObject {
    fn from(value: NumberType) -> Self {
        Self::new(value)
    }
}

impl From<i64> for NumberObject {
    fn from(i: i64) -> Self {
        Self::new(NumberType::Integer(i))
    }
}

impl From<f64> for NumberObject {
    fn from(f: f64) -> Self {
        Self::new(NumberType::Real(f))
    }
}

impl From<i32> for NumberObject {
    fn from(i: i32) -> Self {
        Self::new(NumberType::Integer(i as i64))
    }
}

impl From<f32> for NumberObject {
    fn from(f: f32) -> Self {
        Self::new(NumberType::Real(f as f64))
    }
}
