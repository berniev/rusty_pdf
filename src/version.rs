//--------------------------- Version -------------------------

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub enum Version {
    #[default]
    Auto,
    V1_4,
    V1_5,
}

impl Version {
    pub fn as_str(&self) -> &str {
        match self {
            Version::Auto => "1.7",
            Version::V1_4 => "1.4",
            Version::V1_5 => "1.5",
        }
    }
}

