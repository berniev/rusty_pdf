use std::fmt;

/// PDF generation number:
///     0 = original version
/// 65535 = special value for the free object 0
///     1+ = updated versions
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Generation {
    Root,
    Normal,
}

impl Generation {
    /// PDF spec requires generation 65535 for the root free object (object 0)
    pub const ROOT_GENERATION: u16 = 65535;

    pub fn as_u16(&self) -> u16 {
        match self {
            Generation::Root => Self::ROOT_GENERATION,
            Generation::Normal => 0,
        }
    }
}

impl fmt::Display for Generation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_u16())
    }
}
