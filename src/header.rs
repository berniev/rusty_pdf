use crate::version::Version;

pub struct Header {
    version: Version,
}

impl Header {
    pub fn new() -> Self {
        Header {
            version: Version::Auto,
        }
    }
    
    pub fn set_version(&mut self, version: Version) {
        self.version = version;
    }

    pub fn version(&self) -> Version {
        self.version
    }

    pub fn serialise(&self) -> Vec<u8> {
        let mut arr :Vec<u8> = vec![];
        arr.extend("%PDF-".to_string().as_bytes());
        arr.extend(self.version.as_str().as_bytes());
        arr.extend("\r\n".as_bytes());
        arr.extend("âãÏÓ\r\n".as_bytes());

        arr
    }
}
