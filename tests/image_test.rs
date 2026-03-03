use pydyf::{PDF, PageSize, Page, Stream};
use std::fs::File;

fn create_page_with_content(content_stream_ref: Vec<u8>) -> Page {
    let mut page = Page::new();
    page.set_contents(content_stream_ref);
    page
}

#[test]
fn test_inline_image() {
    let mut pdf = PDF::new(PageSize::A4);
    let mut stream = Stream::new();

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
