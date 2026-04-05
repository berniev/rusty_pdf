use image::{Rgb, RgbImage};
use rusty_pdf::drawing_commands::DrawingCommands;
use rusty_pdf::util::{Matrix, Posn};
use rusty_pdf::{CompressionMethod, Stream};

/*fn create_page_with_content(content_stream_ref: Vec<u8>) -> Page {
    let content_index = String::from_utf8(content_stream_ref).unwrap();
    // Extract just the number from "N 0 R" format
    let id_str = content_index.split_whitespace().next().unwrap();
    let id: u64 = id_str.parse().unwrap();

    let mut page = make_page_dict(id);//Page::new(0usize.into());
    page.add_content(id as usize);
    page
}
*/

#[test]
fn test_external_image_from_file() {
    let mut img = RgbImage::new(100, 100);
    for y in 0..100 {
        for x in 0..100 {
            let r = (255.0 * (x as f32 / 100.0)) as u8;
            let b = (255.0 * (y as f32 / 100.0)) as u8;
            img.put_pixel(x, y, Rgb([r, 0, b]));
        }
    }

    std::fs::create_dir_all("/tmp/pydyf_test").unwrap();
    img.save("/tmp/pydyf_test/gradient.png").unwrap();

    let mut stream = Stream::new();

    let mut cmd = DrawingCommands::new(&mut stream);
    cmd.push_state();
    cmd.set_transformation_matrix(Matrix { a: 200.0, b: 0.0, c: 0.0, d: 200.0, e: 50.0, f: 500.0 });
    cmd
        .inline_image_from_file("/tmp/pydyf_test/gradient.png", CompressionMethod::None);
    cmd.pop_state();

    // Title
    cmd.begin_text();
    cmd.set_font_name_and_size("Helvetica-Bold", 16.0);
    cmd.set_text_position(Posn { x: 50.0, y: 750.0 });
    cmd.show_single_text_string("External Image Test");
    cmd.end_text();

    // Description
    cmd.begin_text();
    cmd.set_font_name_and_size("Helvetica", 11.0);
    cmd.set_text_position(Posn { x: 50.0, y: 735.0 });
    cmd.show_single_text_string("100x100 pixel PNG file loaded and embedded in PDF");
    cmd.end_text();

    cmd.begin_text();
    cmd.set_font_name_and_size("Helvetica", 10.0);
    cmd.set_text_position(Posn { x: 50.0, y: 720.0 });
    cmd.show_single_text_string("Gradient: Red increases left-to-right, Blue increases top-to-bottom");
    cmd.end_text();

    cmd.begin_text();
    cmd.set_font_name_and_size("Helvetica", 10.0);
    cmd.set_text_position(Posn { x: 50.0, y: 705.0 });
    cmd.show_single_text_string("Scaled to 200x200 points");
    cmd.end_text();

 /*   stream.add();
    let content_id = pdf.add_object(Pdf::stream(stream));
    let content_ref = format!("{} 0 R", content_id).into_bytes();
    let page_dict = create_page_with_content(content_ref);
    pdf.add_page(page);

    let file = File::create("/tmp/pydyf_test/ext.pdf").unwrap();
    pdf.write_legacy(file, pydyf::FileIdentifierMode::AutoMD5).unwrap();

    println!("✅ Generated: /tmp/pydyf_test/ext.pdf");
*/}
