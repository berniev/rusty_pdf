/// Tests to verify object stream format matches PDF specification
/// PDF Reference 1.7, Section 3.4.6 - Object Streams
///
/// These tests verify the DICTIONARY entries of object streams.
/// The internal stream format is compressed with FlateDecode and not tested here.
use pydyf::color::{Color, RGB};
use pydyf::objects::stream::{StrokeOrFill, WindingRule};
use pydyf::page::PageSize;
use pydyf::util::{Dims, Posn};
use pydyf::{FileIdentifierMode, PdfFile, PageObject, PdfStreamObject};

/// Test: Object stream dictionary must have /Type /ObjStm
#[test]
fn test_objstm_has_type() {
    let mut pdf = PdfFile::new();
    let mut stream = PdfStreamObject::uncompressed();
    let color = RGB::new(Color::new(0.0), Color::new(0.0), Color::new(1.0));
    stream.set_color_rgb(color, StrokeOrFill::Fill);
    stream.add_rectangle(
        Posn { x: 50.0, y: 50.0 },
        Dims {
            height: 100.0,
            width: 100.0,
        },
    );
    stream.fill(WindingRule::EvenOdd);

    let content_id = pdf.add_object(Box::new(stream));
    let mut page = PageObject::new(0usize.into());
    page.add_content(content_id);
    page.set_media_box(PageSize::A4);
    pdf.add_page(page);

    let mut output = Vec::new();
    pdf.write_compressed(&mut output, FileIdentifierMode::None)
        .expect("write failed");

    let pdf_str = String::from_utf8_lossy(&output);
    assert!(
        pdf_str.contains("/Type /ObjStm"),
        "Object stream must have /Type /ObjStm"
    );
}

/// Test: Object stream dictionary must have /N entry (integer)
#[test]
fn test_objstm_has_n() {
    let mut pdf = PdfFile::new();
    let mut stream = PdfStreamObject::uncompressed();
    stream.add_rectangle(
        Posn { x: 0.0, y: 0.0 },
        Dims {
            height: 10.0,
            width: 10.0,
        },
    );
    let content_id = pdf.add_object(Box::new(stream));

    let mut page = PageObject::new(0usize.into());
    page.add_content(content_id);
    page.set_media_box(PageSize::A4);
    pdf.add_page(page);

    let mut output = Vec::new();
    pdf.write_compressed(&mut output, FileIdentifierMode::None)
        .expect("write failed");

    let pdf_str = String::from_utf8_lossy(&output);
    assert!(pdf_str.contains("/N "), "Object stream must have /N entry");

    // Verify /N is an integer
    if let Some(n_pos) = pdf_str.find("/N ") {
        let after_n = &pdf_str[n_pos + 3..];
        let n_str = after_n.split_whitespace().next().unwrap();
        assert!(
            n_str.parse::<usize>().is_ok(),
            "/N must be an integer, got: {}",
            n_str
        );
    }
}

/// Test: Object stream dictionary must have /First entry (integer)
#[test]
fn test_objstm_has_first() {
    let mut pdf = PdfFile::new();
    let mut stream = PdfStreamObject::uncompressed();
    stream.add_rectangle(
        Posn { x: 0.0, y: 0.0 },
        Dims {
            height: 10.0,
            width: 10.0,
        },
    );
    let content_id = pdf.add_object(Box::new(stream));

    let mut page = PageObject::new(0usize.into());
    page.add_content(content_id);
    page.set_media_box(PageSize::A4);
    pdf.add_page(page);

    let mut output = Vec::new();
    pdf.write_compressed(&mut output, FileIdentifierMode::None)
        .expect("write failed");

    let pdf_str = String::from_utf8_lossy(&output);
    assert!(
        pdf_str.contains("/First "),
        "Object stream must have /First entry"
    );

    // Verify /First is an integer
    if let Some(first_pos) = pdf_str.find("/First ") {
        let after_first = &pdf_str[first_pos + 7..];
        let first_str = after_first.split_whitespace().next().unwrap();
        assert!(
            first_str.parse::<usize>().is_ok(),
            "/First must be an integer, got: {}",
            first_str
        );
    }
}

/// Test: Object stream must have /Filter /FlateDecode
#[test]
fn test_objstm_has_filter() {
    let mut pdf = PdfFile::new();
    let mut stream = PdfStreamObject::uncompressed();
    stream.add_rectangle(
        Posn { x: 0.0, y: 0.0 },
        Dims {
            height: 10.0,
            width: 10.0,
        },
    );
    let content_id = pdf.add_object(Box::new(stream));

    let mut page = PageObject::new(0usize.into());
    page.add_content(content_id);
    page.set_media_box(PageSize::A4);
    pdf.add_page(page);

    let mut output = Vec::new();
    pdf.write_compressed(&mut output, FileIdentifierMode::None)
        .expect("write failed");

    let pdf_str = String::from_utf8_lossy(&output);

    // Find the ObjStm dictionary - check the entire PDF since it contains binary data
    assert!(
        pdf_str.contains("/Type /ObjStm"),
        "Object stream must have /Type /ObjStm"
    );
    assert!(
        pdf_str.contains("/Filter"),
        "Object stream must have /Filter entry"
    );
    assert!(
        pdf_str.contains("FlateDecode"),
        "Object stream must use FlateDecode filter"
    );
}

/// Test: Content streams should NOT be in object stream (they have stream data)
#[test]
fn test_content_streams_not_compressed() {
    let mut pdf = PdfFile::new();

    // Add a content stream
    let mut stream = PdfStreamObject::uncompressed();
    let color = RGB::new(Color::new(1.0), Color::new(0.0), Color::new(0.0));
    stream.set_color_rgb(color, StrokeOrFill::Fill);
    stream.add_rectangle(
        Posn { x: 10.0, y: 10.0 },
        Dims {
            height: 20.0,
            width: 20.0,
        },
    );
    stream.fill(WindingRule::EvenOdd);
    let content_id = pdf.add_object(Box::new(stream));
    let mut page = PageObject::new(0usize.into());
    page.add_content(content_id);
    page.set_media_box(PageSize::A4);
    pdf.add_page(page);

    let mut output = Vec::new();
    pdf.write_compressed(&mut output, FileIdentifierMode::None)
        .expect("write failed");

    let pdf_str = String::from_utf8_lossy(&output);

    // The content stream (object 1) should appear as a regular indirect object, NOT in the ObjStm
    assert!(
        pdf_str.contains("1 0 obj"),
        "Content stream should be written directly as object 1"
    );
}
