use pydyf::color::ColorSpace;
use pydyf::drawing_commands::*;
use pydyf::util::{Matrix, Posn};
use pydyf::{Pdf, Stream};
/*fn create_page_with_content(content_stream_ref: Vec<u8>) -> PageObject {
    let content_index = String::from_utf8(content_stream_ref).unwrap();
    // Extract just the number from "N 0 R" format
    let id_str = content_index.split_whitespace().next().unwrap();
    let id: u64 = id_str.parse().unwrap();

    let mut page = PageObject::new(0usize.into());
    page.add_content(id as usize);
    page
}
*/
#[test]
fn test_inline_image() {
    let _pdf = Pdf::new();
    let mut stream = Stream::new();

    let image_data = vec![255, 0, 0, 255, 0, 0, 0, 0, 255, 0, 0, 255];
    let compression = stream.compression_method();
    
    let mut push = |vec:Vec<u8>| {stream.add_content(vec);};

    push(push_state());
    push(set_transformation_matrix(Matrix {
        a: 100.0,
        b: 0.0,
        c: 0.0,
        d: 100.0,
        e: 50.0,
        f: 500.0,
    }));
    push(
        inline_image(2, 2, ColorSpace::RGB, 8, &image_data, compression)
        .unwrap());
    push(pop_state());

    // Title
    push(begin_text());
    push(set_font_name_and_size("Helvetica-Bold", 16.0));
    push(set_text_position(Posn { x: 50.0, y: 750.0 }));
    push(show_single_text_string("Inline Image Test"));
    push(end_text());

    // Description
    push(begin_text());
    push(set_font_name_and_size("Helvetica", 11.0));
    push(set_text_position(Posn { x: 50.0, y: 735.0 }));
    push(show_single_text_string("2x2 pixel bitmap embedded directly in content stream"));
    push(end_text());

    push(begin_text());
    push(set_font_name_and_size("Helvetica", 10.0));
    push(set_text_position(Posn { x: 50.0, y: 720.0 }));
    push(show_single_text_string("RGB color space, 8 bits per component, scaled 100x100"));
    push(end_text());

    push(begin_text());
    push(set_font_name_and_size("Helvetica", 9.0));
    push(set_text_position(Posn { x: 50.0, y: 470.0 }));
    push(show_single_text_string("Pixels: Red, Red (top) | Blue, Blue (bottom)"));
    push(end_text());

    /*    let content_id = pdf.add_object(Pdf::stream(stream));
        let content_ref = format!("{} 0 R", content_id).into_bytes();
        let page = create_page_with_content(content_ref);
        pdf.add_page(page);

        std::fs::create_dir_all("/tmp/pydyf_test").unwrap();
        let file = File::create("/tmp/pydyf_test/image.pdf").unwrap();
        pdf.write_legacy(file, pydyf::FileIdentifierMode::AutoMD5).unwrap();

        println!("✅ Generated: /tmp/pydyf_test/image.pdf");
    */
}
