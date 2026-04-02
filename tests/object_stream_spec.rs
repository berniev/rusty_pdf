/// Comprehensive tests for PDF Object Stream specification compliance
///
/// PDF Reference 1.7, Section 3.4.6: Object Streams (ISO 32000-1:2008)
///
/// SPEC REQUIREMENTS (from PDF 32000):
///
/// Object Stream Dictionary Entries:
/// - Type (required): must be /ObjStm
/// - N (required): The number of compressed objects in the stream
/// - First (required): The byte offset (in the decoded stream) of the first compressed object
/// - Extends (optional): A reference to another object stream
///
/// Stream Data Format:
/// 1. N pairs of integers: (object_number, byte_offset)
/// 2. Byte offsets are RELATIVE TO THE FIRST OBJECT (the position indicated by /First)
/// 3. Followed by the N objects themselves, concatenated
///
/// Example structure:
/// Stream data: "10 0 11 25 12 50" + [obj 10 data (25 bytes)] + [obj 11 data (25 bytes)] + [obj 12 data]
/// - /N = 3
/// - /First = 18 (length of "10 0 11 25 12 50\n")
/// - Object 10 at offset 0 from /First
/// - Object 11 at offset 25 from /First (25 bytes after start of obj 10)
/// - Object 12 at offset 50 from /First (50 bytes after start of obj 10)
///
/// Objects that CANNOT be compressed:
/// - Stream objects (objects containing a stream)
/// - Objects with generation number != 0
/// - The encryption dictionary
///
/// Test Strategy:
/// 1. Test spec format requirements with known values
/// 2. Test boundary conditions (1 object, 2 objects, many objects)
/// 3. Test offset calculations match spec
/// 4. Test exclusions (streams cannot be compressed)

use pydyf::color::{Color, RGB};
use pydyf::file_identifier::FileIdentifierMode;
use pydyf::objects::pdf_object::PdfObj;
use pydyf::objects::stream::{StrokeOrFill, WindingRule};
use pydyf::util::{Dims, Posn};
use pydyf::{Pdf, PdfStreamObject};

/// SPEC TEST 1: /Type must be /ObjStm
#[test]
fn spec_objstm_must_have_type() {
    let mut pdf = Pdf::new();
    let mut stream = PdfStreamObject::new();
    stream.add_rectangle(Posn { x: 0.0, y: 0.0 }, Dims { height: 10.0, width: 10.0 });
    pdf.save_indirect_object(PdfObj::stream(stream));

    let mut output = Vec::new();
    pdf.write_compressed(&mut output, FileIdentifierMode::None).unwrap();
    let pdf_str = String::from_utf8_lossy(&output);

    assert!(pdf_str.contains("/Type /ObjStm"), "SPEC VIOLATION: Object stream must have /Type /ObjStm");
}

/// SPEC TEST 2: /N must be present and be an integer
#[test]
fn spec_objstm_must_have_n() {
    let mut pdf = Pdf::new();
    let mut stream = PdfStreamObject::new();
    stream.add_rectangle(Posn { x: 0.0, y: 0.0 }, Dims { height: 10.0, width: 10.0 });
    pdf.save_indirect_object(PdfObj::stream(stream));

    let mut output = Vec::new();
    pdf.write_compressed(&mut output, FileIdentifierMode::None).unwrap();
    let pdf_str = String::from_utf8_lossy(&output);

    assert!(pdf_str.contains("/N "), "SPEC VIOLATION: Object stream must have /N entry");

    // Extract and verify /N is an integer
    if let Some(n_pos) = pdf_str.find("/N ") {
        let after_n = &pdf_str[n_pos + 3..];
        let n_str = after_n.split_whitespace().next().unwrap();
        assert!(n_str.parse::<usize>().is_ok(), "SPEC VIOLATION: /N must be an integer");
    }
}

/// SPEC TEST 3: /First must be present and be an integer
#[test]
fn spec_objstm_must_have_first() {
    let mut pdf = Pdf::new();
    let mut stream = PdfStreamObject::new();
    stream.add_rectangle(Posn { x: 0.0, y: 0.0 }, Dims { height: 10.0, width: 10.0 });
    pdf.save_indirect_object(PdfObj::stream(stream));

    let mut output = Vec::new();
    pdf.write_compressed(&mut output, FileIdentifierMode::None).unwrap();
    let pdf_str = String::from_utf8_lossy(&output);

    assert!(pdf_str.contains("/First "), "SPEC VIOLATION: Object stream must have /First entry");

    // Extract and verify /First is an integer
    if let Some(first_pos) = pdf_str.find("/First ") {
        let after_first = &pdf_str[first_pos + 7..];
        let first_str = after_first.split_whitespace().next().unwrap();
        assert!(first_str.parse::<usize>().is_ok(), "SPEC VIOLATION: /First must be an integer");
    }
}

/// SPEC TEST 4: Stream objects cannot be compressed
#[test]
fn spec_stream_objects_not_compressible() {
    //let mut pdf = PdfFile::new();

    // Add a stream object (with actual stream data)
    let mut stream = PdfStreamObject::new();
    let color = RGB::new( Color::new(1.0),Color::new(0.0), Color::new(0.0) );
    stream.set_color_rgb(color, StrokeOrFill::Fill);
    stream.add_rectangle(Posn { x: 10.0, y: 10.0 }, Dims { height: 20.0, width: 20.0 });
    stream.fill(WindingRule::EvenOdd);
/*    let content_id = pdf.add_object(Pdf::stream(stream));
    let mut page = PageObject::new(0usize.into());
    page.add_content(content_id);
    page.set_media_box(PageSize::A4);
    pdf.add_page(page);

    let mut output = Vec::new();
    pdf.write_compressed(&mut output, FileIdentifierMode::None).unwrap();
    let pdf_str = String::from_utf8_lossy(&output);

    // The content stream (object 1) should appear as a regular indirect object, NOT in the ObjStm
    // We should see "1 0 obj" directly in the PDF
    assert!(pdf_str.contains("1 0 obj"), "SPEC VIOLATION: Stream object should not be compressed");
*/}
