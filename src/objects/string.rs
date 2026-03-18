use crate::{PdfMetadata, PdfObject};

/// Spec:
/// String Object:
///     Consists of a series of bytes (unsigned integer values in the range 0 to 255) and the bytes
///     are not integer objects, but are stored in a more compact form
pub struct StringObject {
    value: Option<String>,
    metadata: PdfMetadata,
}

impl StringObject {
    pub fn new(value: Option<String>) -> Self {
        Self {
            value,
            metadata: Default::default(),
        }
    }

    pub fn build(value: impl Into<String>) -> std::rc::Rc<dyn PdfObject> {
        std::rc::Rc::new(Self::new(Some(value.into())))
    }
}

impl PdfObject for StringObject {
    fn data(&self) -> String {
        encode_pdf_string(&self.value.clone().unwrap_or("".to_string()))
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn metadata(&self) -> &PdfMetadata {
        &self.metadata
    }

    fn metadata_mut(&mut self) -> &mut PdfMetadata {
        &mut self.metadata
    }
}

pub fn encode_pdf_string(string: &str) -> String {
    if string.is_ascii() {
        encode_ascii(string)
    } else {
        encode_non_ascii(string) //
    }
}

/// escape and wrap in parentheses
pub fn encode_ascii(string: &str) -> String {
    let mut result = String::with_capacity(string.len() + 4);
    result.push('(');
    for ch in string.chars() {
        if matches!(ch, '\\' | '(' | ')') {
            result.push('\\');
        }
        result.push(ch);
    }
    result.push(')');
    result
}

/// UTF-16BE encode with BOM and hex-encode
pub fn encode_non_ascii(string: &str) -> String {
    let mut hex_content = String::with_capacity(string.len() * 4 + 4);
    for ch in string.encode_utf16() {
        hex_content.push_str(&format!("{:04X}", ch)); // {:04X} = "4 digits, hex, uppercase"
    }

    format!("FEFF<{}>", hex_content)
}

