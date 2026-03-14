use pydyf::{NameObject, NumberObject, NumberType, PdfObject, StreamObject};
use std::rc::Rc;

#[test]
fn debug_compressed_stream_bytes() {
    let obj1_data = "<</Type /Font>>";
    let obj2_data = "<</Type /Pages>>";

    let index_section = format!("1 0 2 {}", obj1_data.len() + 1);
    let content = format!("{}\n{}\n{}", index_section, obj1_data, obj2_data);

    let extra = vec![
        ("Type".to_string(), Rc::new(NameObject::new(Some("ObjStm".to_string()))) as Rc<dyn PdfObject>),
        ("N".to_string(), Rc::new(NumberObject::new(NumberType::Integer(2))) as Rc<dyn PdfObject>),
        ("First".to_string(), Rc::new(NumberObject::new(NumberType::Integer((index_section.len() + 1) as i64))) as Rc<dyn PdfObject>),
    ];

    let obj_stream = StreamObject::compressed().with_data(Some(vec![content.into_bytes()]), Some(extra));
    let output = obj_stream.data();

    println!("\n=== Full output ===\n{}", output);

    let start = output.find("stream\n").expect("No stream keyword") + 7;
    let end = output.find("\nendstream").expect("No endstream keyword");
    let compressed_str = &output[start..end];

    println!("\n=== Compressed string length: {} ===", compressed_str.len());
    println!("First 20 chars as hex (via .as_bytes()):");
    for (i, byte) in compressed_str.as_bytes().iter().take(20).enumerate() {
        print!("{:02x} ", byte);
        if (i + 1) % 10 == 0 { println!(); }
    }
    println!();

    // Extract bytes properly by converting Latin-1 back
    let compressed_bytes: Vec<u8> = compressed_str.chars().map(|c| c as u8).collect();
    println!("\nFirst 20 bytes (via Latin-1 conversion):");
    for (i, byte) in compressed_bytes.iter().take(20).enumerate() {
        print!("{:02x} ", byte);
        if (i + 1) % 10 == 0 { println!(); }
    }
    println!();

    // Try to decompress using proper byte extraction
    use flate2::read::ZlibDecoder;
    use std::io::Read;

    let mut decoder = ZlibDecoder::new(&compressed_bytes[..]);
    let mut decompressed = Vec::new();

    match decoder.read_to_end(&mut decompressed) {
        Ok(_) => {
            let decompressed_str = String::from_utf8_lossy(&decompressed);
            println!("\n✅ Successfully decompressed!");
            println!("Decompressed content:\n{}", decompressed_str);
        }
        Err(e) => {
            println!("\n❌ Failed to decompress: {}", e);
        }
    }
}
