use pydyf::{PDF, Dictionary, Stream};
use std::collections::HashMap;
use std::fs::File;
use image::{RgbImage, Rgb};

fn create_page_with_content(content_stream_ref: Vec<u8>) -> Dictionary {
    let mut page_values = HashMap::new();
    page_values.insert("Type".to_string(), b"/Page".to_vec());
    page_values.insert("MediaBox".to_string(), b"[0 0 612 792]".to_vec());
    page_values.insert("Contents".to_string(), content_stream_ref);
    Dictionary::new(Some(page_values))
}

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

    let mut pdf = PDF::new();
    let mut stream = Stream::new();

    stream.push_state();
    stream.set_matrix(200.0, 0.0, 0.0, 200.0, 50.0, 500.0);
    stream.inline_image_from_file("/tmp/pydyf_test/gradient.png").unwrap();
    stream.pop_state();

    stream.begin_text();
    stream.set_font_size("Helvetica", 18.0);
    stream.set_text_matrix(1.0, 0.0, 0.0, 1.0, 50.0, 450.0);
    stream.show_text_string("Gradient image from PNG file");
    stream.end_text();

    pdf.add_object(Box::new(stream));
    let content_ref = format!("{} 0 R", pdf.objects.len() - 1).into_bytes();
    let page = create_page_with_content(content_ref);
    pdf.add_page(page);

    let mut file = File::create("/tmp/pydyf_test/ext.pdf").unwrap();
    pdf.write(&mut file, Some(b"1.7"), pydyf::Identifier::AutoMD5, false).unwrap();

    println!("✅ Generated: /tmp/pydyf_test/ext.pdf");
}
