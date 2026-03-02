/// Convert numeric values to bytes for PDF output
pub fn to_bytes_num(n: f64) -> Vec<u8> {
    if n.fract() == 0.0 {
        format!("{}", n as i64).into_bytes()
    } else {
        format!("{}", n).trim_end_matches('0').as_bytes().to_vec()
    }
}

/// Convert numeric values to String for PDF formatting
pub fn to_pdf_num(n: f64) -> String {
    if n.fract() == 0.0 {
        format!("{}", n as i64)
    } else {
        // Use precision to avoid excessive digits, and trim zeros
        format!("{:.4}", n)
            .trim_end_matches('0')
            .trim_end_matches('.')
            .to_string()
    }
}

/// Convert string to bytes
pub fn to_bytes_str(s: &str) -> Vec<u8> {
    s.as_bytes().to_vec()
}

/// ASCII85 encode data (equivalent to Python's base64.a85encode)
pub fn ascii85_encode(data: &[u8]) -> Vec<u8> {
    let mut result = Vec::new();

    for chunk in data.chunks(4) {
        if chunk.len() == 4 {
            let value = ((chunk[0] as u32) << 24)
                | ((chunk[1] as u32) << 16)
                | ((chunk[2] as u32) << 8)
                | (chunk[3] as u32);

            if value == 0 {
                result.push(b'z');
            } else {
                let mut encoded = [0u8; 5];
                let mut val = value;
                for i in (0..5).rev() {
                    encoded[i] = (val % 85) as u8 + 33;
                    val /= 85;
                }
                result.extend_from_slice(&encoded);
            }
        } else {
            // Handle partial chunk at end
            let mut value = 0u32;
            for (i, &byte) in chunk.iter().enumerate() {
                value |= (byte as u32) << (24 - i * 8);
            }

            let mut encoded = [0u8; 5];
            let mut val = value;
            for i in (0..5).rev() {
                encoded[i] = (val % 85) as u8 + 33;
                val /= 85;
            }
            result.extend_from_slice(&encoded[..chunk.len() + 1]);
        }
    }

    result
}
