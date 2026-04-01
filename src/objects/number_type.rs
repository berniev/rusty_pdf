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
