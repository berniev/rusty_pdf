use std::fmt;

/// PDF object status as specified in the xref table.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObjectStatus {
    Free,  // deleted or never used
    InUse, // normal, active object
}

impl ObjectStatus {
    /// Returns the PDF character representation ('f' or 'n') for xref table
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

