use crate::PdfError;
use crate::version::Version;

#[derive(Clone)]
pub struct PdfStringObject {
    pub(crate) value: String,
}

impl PdfStringObject {
    pub fn new(value: &str) -> Self {
        Self {
            value: value.to_owned(),
        }
    }

    pub fn encode(&self, version: Version) -> Result<Vec<u8>, PdfError> {
        Ok(encode_text_string(&*self.value, version))
    }
}

fn encode_text_string(string: &str, version: Version) -> Vec<u8> {
    if string.is_ascii() {
        // < 128
        encode_ascii(string)
    } else if version >= Version::V2_2017 {
        encode_utf8(string)
    } else {
        encode_utf16(string)
    }
}

fn encode_ascii(string: &str) -> Vec<u8> {
    let mut result: Vec<u8> = vec![];
    result.push(b'(');
    for ch in string.chars() {
        if matches!(ch, '\\' | '(' | ')') {
            result.push(b'\\');
        }
        result.push(ch as u8);
    }
    result.push(b')');

    result
}

fn encode_utf16(string: &str) -> Vec<u8> {
    const BOM_UTF16: [u8; 2] = [0xFE, 0xFF];
    let mut bytes = Vec::with_capacity(2 + string.len() * 2);
    bytes.extend(&BOM_UTF16);
    for unit in string.encode_utf16() {
        bytes.extend(&unit.to_be_bytes());
    }
    bytes
}

fn encode_utf8(string: &str) -> Vec<u8> {
    const BOM_UTF8: [u8; 3] = [0xEF, 0xBB, 0xBF];
    let mut bytes = Vec::with_capacity(3 + string.len());
    bytes.extend(&BOM_UTF8);
    bytes.extend(string.as_bytes());
    bytes
}

//--------------------------- Tests -------------------------//

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_simple_string() {
        let obj = PdfStringObject::new("Hello, World!");
        assert_eq!(obj.encode(Version::V1_5).unwrap(), b"(Hello, World!)");
    }

    #[test]
    fn encode_empty_string() {
        let obj = PdfStringObject::new("");
        assert_eq!(obj.encode(Version::V1_5).unwrap(), b"()");
    }

    #[test]
    fn encode_string_with_newline() {
        let obj = PdfStringObject::new("line1\nline2");
        assert_eq!(obj.encode(Version::V1_5).unwrap(), b"(line1\nline2)");
    }

    #[test]
    fn test_is_ascii() {
        let str = "String";
        let res = str.is_ascii();
        assert_eq!(res, true);
    }

    #[test]
    fn encode_ascii() {
        let str = "String";
        let res = encode_text_string(str, Version::V1_4);
        assert_eq!(res, b"(String)");
    }

    #[test]
    fn encode_ascii_bytes() {
        let str = "String";
        let res = encode_text_string(str, Version::V1_4);
        assert_eq!(res, [40, 83, 116, 114, 105, 110, 103, 41]);
    }

    #[test]
    fn encode_chinese() {
        let str = "公共汽车";
        let res = encode_text_string(str, Version::V1_4);
        assert_eq!(res, [0xFE, 0xFF, 81, 108, 81, 113, 108, 125, 143, 102]);
    }

    #[test]
    fn encode_chinese_version_2() {
        let str = "公共汽车";
        let res = encode_text_string(str, Version::V2_2017);
        assert_eq!(
            res,
            [
                0xEF, 0xBB, 0xBF, 229, 133, 172, 229, 133, 177, 230, 177, 189, 232, 189, 166
            ]
        );
    }

    #[test]
    fn ascii_is_same_for_v_2_and_v_1_4() {
        let str = "String";
        let res1 = encode_text_string(str, Version::V2_2017);
        let res2 = encode_text_string(str, Version::V1_4);
        assert_eq!(res1, res2);
    }
}
