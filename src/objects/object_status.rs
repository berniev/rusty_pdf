use std::fmt;

//---------------- ObjectStatus -----------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ObjectStatus {
    Free, // deleted or never used
    #[default]
    InUse, // normal, active object
}

impl ObjectStatus {
    pub fn as_char(&self) -> char {
        match self {
            ObjectStatus::Free => 'f',
            ObjectStatus::InUse => 'n',
        }
    }
}

impl fmt::Display for ObjectStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_char())
    }
}
