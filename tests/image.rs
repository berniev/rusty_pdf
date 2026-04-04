use pydyf::color::ColorSpace;
use pydyf::drawing_commands::*;
use pydyf::file_identifier::FileIdentifierMode;
use pydyf::objects::pdf_object::PdfObj;
use pydyf::page::{add_page_to_tree, make_page};
use pydyf::util::{Matrix, Posn};
use pydyf::{PageSize, Pdf, Stream};
use std::fs::File;

#[test]
fn test_inline_image() {
    let image_data = vec![255, 0, 0, 255, 0, 0, 0, 0, 255, 0, 0, 255];

    let mut stream = Stream::new();
    let compression = stream.compression_method();

    let mut cmd = DrawingCommands::new(&mut stream);

    cmd.push_state();
    cmd.set_transformation_matrix(Matrix {
        a: 100.0,
        b: 0.0,
        c: 0.0,
        d: 100.0,
        e: 50.0,
        f: 500.0,
    });
    cmd.inline_image(2, 2, ColorSpace::RGB, 8, &image_data, compression);
    cmd.pop_state();

    // Title
    cmd.begin_text();
    cmd.set_font_name_and_size("Helvetica-Bold", 16.0);
    cmd.set_text_position(Posn { x: 50.0, y: 750.0 });
    cmd.show_single_text_string("Inline Image Test");
    cmd.end_text();

    // Description
    cmd.begin_text();
    cmd.set_font_name_and_size("Helvetica", 11.0);
    cmd.set_text_position(Posn { x: 50.0, y: 735.0 });
    cmd.show_single_text_string("2x2 pixel bitmap embedded directly in content stream");
    cmd.end_text();

    cmd.begin_text();
    cmd.set_font_name_and_size("Helvetica", 10.0);
    cmd.set_text_position(Posn { x: 50.0, y: 720.0 });
    cmd.show_single_text_string("RGB color space, 8 bits per component, scaled 100x100");
    cmd.end_text();

    cmd.begin_text();
    cmd.set_font_name_and_size("Helvetica", 9.0);
    cmd.set_text_position(Posn { x: 50.0, y: 470.0 });
    cmd.show_single_text_string("Pixels: Red, Red (top) | Blue, Blue (bottom)");
    cmd.end_text();

    let mut pdf = Pdf::new();
    let mut page_dict = make_page(pdf.next_object_number());
    page_dict.add("MediaBox", PdfObj::array(PageSize::A4.to_rect()));
    page_dict.add("Contents", PdfObj::stream(stream));
    add_page_to_tree(&mut page_dict, pdf.root_page_tree_dict_ref()).expect("fail");

    std::fs::create_dir_all("/tmp/pydyf_test").unwrap();
    let file = File::create("/tmp/pydyf_test/image.pdf").unwrap();
    pdf.write_legacy(file, FileIdentifierMode::AutoMD5).unwrap();

    println!("✅ Generated: /tmp/pydyf_test/image.pdf");
}
