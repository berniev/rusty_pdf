//! PDF String encoding functions

use regex::Regex;

pub fn encode_pdf_string(string: &str) -> Vec<u8> {
    if string.is_ascii() {
        encode_ascii(string) // escaped and wrapped in parentheses
    } else {
        encode_non_ascii(string) // UTF-16BE encoded with BOM and hex-encoded
    }
}

fn encode_ascii(string: &str) -> Vec<u8> {
    let re = Regex::new(r"([\\()])").unwrap();
    let escaped = re.replace_all(string, r"\$1");
    let mut result = b"(".to_vec();
    result.extend(escaped.as_bytes());
    result.push(b')');
    result
}

fn encode_non_ascii(string: &str) -> Vec<u8> {
    let mut encoded = b"\xFE\xFF".to_vec();
    for ch in string.encode_utf16() {
        encoded.extend(&ch.to_be_bytes());
    }
    let hex_string = hex::encode(&encoded);
    let mut result = b"<".to_vec();
    result.extend(hex_string.as_bytes());
    result.push(b'>');
    result
}
