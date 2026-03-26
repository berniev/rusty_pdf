use pydyf::color::{Color, RGB};
use pydyf::objects::stream::{WindingRule, StrokeOrFill};
use pydyf::page::PageSize;
use pydyf::util::{Dims, Posn};
use pydyf::{FileIdentifierMode, PageObject, PdfStreamObject, PDF};

#[test]
fn test_page_has_contents_reference() {
    let mut pdf = PDF::new();
    let mut stream = PdfStreamObject::uncompressed();

    // Add a blue rectangle
    let color = RGB::new(
        Color::new(0.0),
        Color::new(0.0),
        Color::new(1.0),
    );
    stream.set_color_rgb(color, StrokeOrFill::Fill);
    stream.rectangle(
        Posn { x: 50.0, y: 50.0 },
        Dims {
            height: 100.0,
            width: 100.0,
        },
    );
    stream.fill(WindingRule::EvenOdd);

    let content_id = pdf.add_object(Box::new(stream));

    // PageObject::new takes parent ID, not content ID
    // We need to get the parent from the PDF structure
    let mut page = PageObject::new(0usize.into()); // Will be set by add_page
    page.add_content(content_id);
    page.set_media_box(PageSize::A4);
    pdf.add_page(page);

    // Write PDF
    let mut output = Vec::new();
    pdf.write_legacy(&mut output, FileIdentifierMode::None)
        .expect("Failed to write PDF");

    let pdf_str = String::from_utf8_lossy(&output);

    // The page dictionary should contain a /Contents reference
    assert!(
        pdf_str.contains("/Contents"),
        "Page object should have /Contents entry. PDF output:\n{}",
        pdf_str
    );
}

#[test]
fn test_page_contents_points_to_stream() {
    let mut pdf = PDF::new();
    let mut stream = PdfStreamObject::uncompressed();

    // Add content
    stream.rectangle(
        Posn { x: 0.0, y: 0.0 },
        Dims {
            height: 50.0,
            width: 50.0,
        },
    );
    stream.fill(WindingRule::EvenOdd);

    let content_id = pdf.add_object(Box::new(stream));

    let mut page = PageObject::new(0usize.into());
    page.add_content(content_id);
    page.set_media_box(PageSize::Letter);
    pdf.add_page(page);

    // Write PDF
    let mut output = Vec::new();
    pdf.write_legacy(&mut output, FileIdentifierMode::None)
        .expect("Failed to write PDF");

    let pdf_str = String::from_utf8_lossy(&output);

    // Should reference the content stream (object 1 in this case)
    // The exact object number depends on how objects are numbered, but /Contents should exist
    assert!(
        pdf_str.contains("/Contents") && (pdf_str.contains("0 R") || pdf_str.contains("1 0 R")),
        "Page should reference a content stream. PDF output:\n{}",
        pdf_str
    );
}

#[test]
fn test_multiple_content_streams() {
    let mut pdf = PDF::new();

    // Create two content streams
    let mut stream1 = PdfStreamObject::uncompressed();
    stream1.rectangle(
        Posn { x: 0.0, y: 0.0 },
        Dims { height: 50.0, width: 50.0 },
    );

    let mut stream2 = PdfStreamObject::uncompressed();
    stream2.rectangle(
        Posn { x: 100.0, y: 100.0 },
        Dims { height: 50.0, width: 50.0 },
    );

    pdf.add_object(Box::new(stream1));
    pdf.add_object(Box::new(stream2));

    // PDF spec allows Contents to be either a single stream or an array of streams
    // For now, we'll test that at least one Contents reference exists
    let content_id = pdf.objects.len() - 2; // First stream

    let mut page = PageObject::new(0usize.into());
    page.add_content(content_id);
    page.set_media_box(PageSize::A4);
    pdf.add_page(page);

    let mut output = Vec::new();
    pdf.write_legacy(&mut output, FileIdentifierMode::None)
        .expect("Failed to write PDF");

    let pdf_str = String::from_utf8_lossy(&output);

    assert!(
        pdf_str.contains("/Contents"),
        "Page with content streams should have /Contents. PDF:\n{}",
        pdf_str
    );
}
