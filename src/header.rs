use crate::pdf_version::PdfVersion;

pub struct Header {
    version: PdfVersion,

}

impl Header {
    pub fn new() -> Self {
        Header {
            version: PdfVersion::Auto,
        }
    }
    
    pub fn set_version(&mut self, version: PdfVersion) {
        self.version = version;
    }
    
    pub fn serialise(&self) -> String {
        format!("%PDF-{} \r\nâãÏÓ\r\n", self.version.as_str())
    }
}
