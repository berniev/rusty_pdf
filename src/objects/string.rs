use crate::PdfObject;
use crate::objects::metadata::PdfMetadata;

pub struct StringObject {
    pub metadata: PdfMetadata,
    pub value: String,
}

impl StringObject {
    pub fn new(value: String) -> Self {
        Self {
            metadata: PdfMetadata::default(),
            value,
        }
    }
}

impl PdfObject for StringObject {
    fn metadata(&self) -> &PdfMetadata {
        &self.metadata
    }

    fn metadata_mut(&mut self) -> &mut PdfMetadata {
        &mut self.metadata
    }

    fn data(&self) -> Vec<u8> {
        encode_pdf_string(&self.value)
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn is_compressible(&self) -> bool {
        self.metadata.generation == 0
    }
}

pub fn encode_pdf_string(string: &str) -> Vec<u8> {
    if string.is_ascii() {
        encode_ascii(string) // escape and wrap in parentheses
    } else {
        encode_non_ascii(string) // UTF-16BE encode with BOM and hex-encode
    }
}

pub fn encode_ascii(string: &str) -> Vec<u8> {
    let mut result = Vec::with_capacity(string.len() + 4);
    result.push(b'(');
    for &byte in string.as_bytes() {
        match byte {
            b'\\' | b'(' | b')' => {
                // ONLY exist as single bytes in UTF-8
                result.push(b'\\');
                result.push(byte);
            }
            _ => {
                result.push(byte); // as-is (including multi-byte UTF-8)
            }
        }
    }

    result.push(b')');
    result
}

pub fn encode_non_ascii(string: &str) -> Vec<u8> {
    let mut hex_content = String::with_capacity(string.len() * 4 + 4);
    hex_content.push_str("FEFF");
    for ch in string.encode_utf16() {
        hex_content.push_str(&format!("{:04X}", ch)); // {:04X} = "4 digits, hex, uppercase"
    }

    format!("<{}>", hex_content).into_bytes()
}
