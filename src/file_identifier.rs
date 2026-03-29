pub enum FileIdentifierMode {
    None,
    AutoMD5,
    Custom(Vec<u8>),
}
