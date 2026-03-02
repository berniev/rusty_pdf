//! PDF String encoding functions

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
            // These characters ONLY exist as single bytes in UTF-8
            b'\\' | b'(' | b')' => {
                result.push(b'\\');
                result.push(byte);
            }
            // Everything else is pushed as-is (including multi-byte UTF-8)
            _ => {
                result.push(byte);
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
        // Build the hex string directly: {:04X} means "4 digits, hex, uppercase"
        hex_content.push_str(&format!("{:04X}", ch));
    }

    format!("<{}>", hex_content).into_bytes()
}