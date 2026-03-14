use pydyf::color::{Color, RGB};
use pydyf::objects::stream::{EvenOdd, StrokeOrFill};
use pydyf::page::PageSize;
use pydyf::util::{Dims, Posn};
use pydyf::{FileIdentifierMode, PDF, PageObject, StreamObject};

#[test]
fn test_create_pdf() {
    let pdf = PDF::new();
    // After refactoring, PDF::new() creates an empty objects list
    // Objects are added during write() or when explicitly added
    assert_eq!(pdf.objects.len(), 0);
}

#[test]
fn test_add_page() {
    let mut pdf = PDF::new();
    let stream = StreamObject::new();
    pdf.add_object(Box::new(stream));

    let next_num = pdf.objects.len() - 1;
    let mut page = PageObject::new(next_num.into());
    page.set_media_box(PageSize::A4);

    pdf.add_page(page);

    // After adding one object and one page, we should have at least 1 object
    // (Pages are not added to objects list until write() is called)
    assert!(!pdf.objects.is_empty());
}

#[test]
fn test_stream_operations() {
    let mut stream = StreamObject::compressed();

    let color = RGB {
        red: Color { color: 1.0 },
        green: Color { color: 0.0 },
        blue: Color { color: 0.0 },
    };
    let _ = stream.set_color_rgb(color, StrokeOrFill::Stroke);
    stream.rectangle(
        Posn { x: 100.0, y: 100.0 },
        Dims {
            height: 200.0,
            width: 150.0,
        },
    );
    stream.fill(EvenOdd::Odd);

    assert!(!stream.stream.is_empty());
}

#[test]
fn test_compressed_stream() {
    let stream = StreamObject::compressed();
    assert_eq!(stream.compress, pydyf::CompressionMethod::Flate);
}

/*#[test]
fn test_text_operations() {
    let mut stream = StreamObject::new();

    stream.begin_text();
    stream.set_font_size("Helvetica", 12.0);
    stream.set_text_matrix(1.0, 0.0, 0.0, 1.0, 100.0, 700.0);
    stream.show_text_string("Test");
    assert!(stream.stream.len() > 0);
}*/

#[test]
fn test_add_page_with_pagesize_adds_mediabox() {
    let mut pdf = PDF::new();
    let stream = StreamObject::new();
    pdf.add_object(Box::new(stream));

    let next_num = pdf.objects.len() - 1;

    let mut page = PageObject::new(next_num.into());
    page.set_media_box(PageSize::A4);

    // Should contain MediaBox because it was explicitly provided
    assert_eq!(page.media_box, Some(PageSize::A4));
}

#[test]
fn test_default_page_size() {
    let mut pdf = PDF::new();

    let stream = StreamObject::new();
    pdf.add_object(Box::new(stream));

    let next_num = pdf.objects.len() - 1;

    let page = PageObject::new(next_num.into());
    pdf.add_page(page);

    let page_obj = pdf.objects.last().unwrap();
    let data_str = page_obj.data();

    // Should NOT contain MediaBox because it's inherited. todo: from where. how?
    assert!(!data_str.contains("/MediaBox"));
}

// #[test]
// fn test_root_mediabox_inheritance() {
//     let pdf = PDF::new();
//     let pages_tree = &pdf.page_tree;
//     // TODO: PageTreeNode doesn't have a get() method anymore
//     // Need to access media_box field directly or through a different API
//     // let mediabox = pages_tree.get("MediaBox").unwrap();
//     // assert_eq!(String::from_utf8_lossy(mediabox), "[0 0 595 842]");
// }

#[test]
fn test_negative_pagesize_is_zeroed() {
    let size = PageSize::Custom(Dims {
        width: -100.0, // invalid width. should be made zero
        height: 500.0,
    });
    let dimensions = size.dimensions();
    assert_eq!(dimensions.width, 0.0);
    assert_eq!(dimensions.height, 500.0);
}

#[test]
fn test_pagesize_custom_validation() {
    let size = PageSize::Custom(Dims {
        width: 100.0,
        height: 500.0,
    });
    let dims = size.dimensions();
    assert_eq!(dims.width, 100.0);
    assert_eq!(dims.height, 500.0);
}

#[test]
fn test_compressed_pdf_generation() {
    let mut pdf = PDF::new();
    let mut stream = StreamObject::new();

    // Add some content to the stream
    let color = RGB {
        red: Color { color: 0.0 },
        green: Color { color: 0.0 },
        blue: Color { color: 1.0 },
    };
    let _ = stream.set_color_rgb(color, StrokeOrFill::Fill);
    stream.rectangle(
        Posn { x: 50.0, y: 50.0 },
        Dims {
            height: 100.0,
            width: 100.0,
        },
    );
    stream.fill(EvenOdd::Odd);

    pdf.add_object(Box::new(stream));
    let next_num = pdf.objects.len() - 1;
    let mut page = PageObject::new(next_num.into());
    page.set_media_box(PageSize::A4);
    pdf.add_page(page);

    // Write compressed PDF
    let mut compressed_output = Vec::new();
    pdf.write_compressed(&mut compressed_output, FileIdentifierMode::None)
        .expect("Failed to write compressed PDF");

    // Verify PDF header indicates version 1.5
    let output_str = String::from_utf8_lossy(&compressed_output);
    assert!(output_str.starts_with("%PDF-1.5"));

    // Verify it has content
    assert!(compressed_output.len() > 100);
}

#[test]
fn test_array_object_uses_data_not_reference() {
    use pydyf::{ArrayObject, NumberObject, NumberType, PdfObject};
    use std::rc::Rc;

    // Create array with numbers
    let array = ArrayObject::new(Some(vec![
        Rc::new(NumberObject::new(NumberType::Integer(1))),
        Rc::new(NumberObject::new(NumberType::Integer(2))),
        Rc::new(NumberObject::new(NumberType::Integer(3))),
    ]));

    let data = array.data();

    // Should be "[ 1 2 3 ]" not "[ 1 0 R 2 0 R 3 0 R ]"
    assert_eq!(data, "[ 1 2 3 ]");
    assert!(!data.contains(" R"), "Array should not contain indirect references");
}

#[test]
fn test_object_stream_format() {
    use pydyf::{NameObject, NumberObject, NumberType, PdfObject, StreamObject};
    use std::rc::Rc;

    // Create a simple object stream manually to test format
    // Object stream should contain: "1 0 2 45\n<</Type /Font>>\n<</Type /Pages>>"
    // Where "1 0 2 45" means: object 1 at offset 0, object 2 at offset 45
    
    let obj1_data = "<</Type /Font>>";
    let obj2_data = "<</Type /Pages>>";
    
    let index_section = format!("1 0 2 {}", obj1_data.len() + 1);
    let content = format!("{}\n{}\n{}", index_section, obj1_data, obj2_data);
    
    let extra = vec![
        ("Type".to_string(), Rc::new(NameObject::new(Some("ObjStm".to_string()))) as Rc<dyn PdfObject>),
        ("N".to_string(), Rc::new(NumberObject::new(NumberType::Integer(2))) as Rc<dyn PdfObject>),
        ("First".to_string(), Rc::new(NumberObject::new(NumberType::Integer((index_section.len() + 1) as i64))) as Rc<dyn PdfObject>),
    ];
    
    let obj_stream = StreamObject::new().with_data(Some(vec![content.into_bytes()]), Some(extra));
    
    let output = obj_stream.data();
    
    // Should contain dictionary with Type, N, First
    assert!(output.contains("/Type /ObjStm"), "Missing /Type /ObjStm");
    assert!(output.contains("/N 2"), "Missing /N 2");
    assert!(output.contains("/First"), "Missing /First");
    assert!(output.contains("stream"), "Missing stream keyword");
    assert!(output.contains("endstream"), "Missing endstream keyword");
}

#[test]
fn test_compressed_object_stream_is_valid() {
    use pydyf::{NameObject, NumberObject, NumberType, PdfObject, StreamObject};
    use std::rc::Rc;
    use flate2::read::ZlibDecoder;
    use std::io::Read;

    // Create compressed object stream
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
    
    // Extract the compressed stream data
    let start = output.find("stream\n").expect("No stream keyword") + 7;
    let end = output.find("\nendstream").expect("No endstream keyword");
    let compressed_data = &output[start..end];

    // Convert Latin-1 encoded string back to bytes
    // (StreamObject uses Latin-1 to preserve binary data in String format)
    let compressed_bytes: Vec<u8> = compressed_data.chars().map(|c| c as u8).collect();
    let mut decoder = ZlibDecoder::new(&compressed_bytes[..]);
    let mut decompressed = Vec::new();
    
    match decoder.read_to_end(&mut decompressed) {
        Ok(_) => {
            let decompressed_str = String::from_utf8_lossy(&decompressed);
            println!("Decompressed: {}", decompressed_str);
            assert!(decompressed_str.contains("1 0 2"), "Missing object indices");
            assert!(decompressed_str.contains("/Type /Font"), "Missing Font object");
            assert!(decompressed_str.contains("/Type /Pages"), "Missing Pages object");
        }
        Err(e) => {
            panic!("Failed to decompress: {}. Compressed length: {}", e, compressed_bytes.len());
        }
    }
}

#[test]
fn test_stream_object_preserves_binary_data() {
    use pydyf::{PdfObject, StreamObject};

    // Create stream with known binary data
    let binary_data = vec![0x78, 0x9c, 0x03, 0x00, 0x00, 0x00, 0x00, 0x01]; // Valid zlib header + empty data

    let stream = StreamObject::new().with_data(Some(vec![binary_data.clone()]), None);
    let output = stream.data();

    // Extract stream content
    let start = output.find("stream\n").unwrap() + 7;
    let end = output.find("\nendstream").unwrap();
    let stream_content = &output[start..end];

    // StreamObject now uses Latin-1 encoding to preserve binary data
    // Convert back from Latin-1 to get original bytes
    let extracted_bytes: Vec<u8> = stream_content.chars().map(|c| c as u8).collect();

    // Verify binary data is preserved exactly
    assert_eq!(extracted_bytes, binary_data,
        "Binary data should be preserved via Latin-1 encoding");
}
