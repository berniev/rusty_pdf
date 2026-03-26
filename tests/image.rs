use pydyf::color::ColorSpace;
use pydyf::util::{Matrix, Posn};
use pydyf::{PDF, PageObject, Stream};
use std::fs::File;

fn create_page_with_content(content_stream_ref: Vec<u8>) -> PageObject {
    let content_index = String::from_utf8(content_stream_ref).unwrap();
    // Extract just the number from "N 0 R" format
    let id_str = content_index.split_whitespace().next().unwrap();
    let id: u64 = id_str.parse().unwrap();

    let mut page = PageObject::new(0usize.into());
    page.add_content(id as usize);
    page
}

#[test]
fn test_inline_image() {
    let mut pdf = PDF::new();
    let mut stream = Stream::uncompressed();

    let image_data = vec![255, 0, 0, 255, 0, 0, 0, 0, 255, 0, 0, 255];

    stream.push_state();
    stream.set_transformation_matrix(Matrix { a: 100.0, b: 0.0, c: 0.0, d: 100.0, e: 50.0, f: 500.0 });
    stream
        .inline_image(2, 2, ColorSpace::RGB, 8, &image_data)
        .unwrap();
    stream.pop_state();

    // Title
    stream.begin_text();
    stream.set_font_name_and_size("Helvetica-Bold", 16.0);
    stream.set_text_position(Posn { x: 50.0, y: 750.0 });
    stream.show_single_text_string("Inline Image Test");
    stream.end_text();

    // Description
    stream.begin_text();
    stream.set_font_name_and_size("Helvetica", 11.0);
    stream.set_text_position(Posn { x: 50.0, y: 735.0 });
    stream.show_single_text_string("2x2 pixel bitmap embedded directly in content stream");
    stream.end_text();

    stream.begin_text();
    stream.set_font_name_and_size("Helvetica", 10.0);
    stream.set_text_position(Posn { x: 50.0, y: 720.0 });
    stream.show_single_text_string("RGB color space, 8 bits per component, scaled 100x100");
    stream.end_text();

    stream.begin_text();
    stream.set_font_name_and_size("Helvetica", 9.0);
    stream.set_text_position(Posn { x: 50.0, y: 470.0 });
    stream.show_single_text_string("Pixels: Red, Red (top) | Blue, Blue (bottom)");
    stream.end_text();

    let content_id = pdf.add_object(Box::new(stream));
    let content_ref = format!("{} 0 R", content_id).into_bytes();
    let page = create_page_with_content(content_ref);
    pdf.add_page(page);

    std::fs::create_dir_all("/tmp/pydyf_test").unwrap();
    let file = File::create("/tmp/pydyf_test/image.pdf").unwrap();
    pdf.write_legacy(file, pydyf::FileIdentifierMode::AutoMD5).unwrap();

    println!("✅ Generated: /tmp/pydyf_test/image.pdf");
}
