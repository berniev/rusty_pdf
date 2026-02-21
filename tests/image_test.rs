use pydyf::{PDF, Dictionary, Stream};
use std::collections::HashMap;
use std::fs::File;

fn create_page_with_content(content_stream_ref: Vec<u8>) -> Dictionary {
    let mut page_values = HashMap::new();
    page_values.insert("Type".to_string(), b"/Page".to_vec());
    page_values.insert("MediaBox".to_string(), b"[0 0 612 792]".to_vec());
    page_values.insert("Contents".to_string(), content_stream_ref);
    Dictionary::new(Some(page_values))
}

#[test]
fn test_inline_image() {
    let mut pdf = PDF::new();
    let mut stream = Stream::new(None, None, false);

    let image_data = vec![
        255, 0, 0,    255, 0, 0,
        0, 0, 255,    0, 0, 255,
    ];

    stream.push_state();
    stream.set_matrix(100.0, 0.0, 0.0, 100.0, 50.0, 500.0);
    stream.inline_image(2, 2, "RGB", 8, &image_data).unwrap();
    stream.pop_state();

    stream.begin_text();
    stream.set_font_size("Helvetica", 18.0);
    stream.set_text_matrix(1.0, 0.0, 0.0, 1.0, 50.0, 450.0);
    stream.show_text_string("Red square inline image");
    stream.end_text();

    pdf.add_object(Box::new(stream));
    let content_ref = format!("{} 0 R", pdf.objects.len() - 1).into_bytes();
    let page = create_page_with_content(content_ref);
    pdf.add_page(page);

    std::fs::create_dir_all("/tmp/pydyf_test").unwrap();
    let mut file = File::create("/tmp/pydyf_test/image.pdf").unwrap();
    pdf.write(&mut file, Some(b"1.7"), pydyf::Identifier::AutoMD5, false).unwrap();

    println!("✅ Generated: /tmp/pydyf_test/image.pdf");
}
