use pydyf::{PDF, Dictionary, Stream};
use std::collections::HashMap;

fn create_page_with_content(content_stream_ref: Vec<u8>) -> Dictionary {
    let mut page_values = HashMap::new();
    page_values.insert("Type".to_string(), b"/Page".to_vec());
    page_values.insert("MediaBox".to_string(), b"[0 0 612 792]".to_vec());
    page_values.insert("Contents".to_string(), content_stream_ref);
    Dictionary::new(Some(page_values))
}

#[test]
fn test_create_pdf() {
    let pdf = PDF::new();
    assert_eq!(pdf.objects.len(), 1);
}

#[test]
fn test_add_page() {
    let mut pdf = PDF::new();
    let stream = Stream::new();
    pdf.add_object(Box::new(stream));

    let content_ref = format!("{} 0 R", pdf.objects.len() - 1).into_bytes();
    let page = create_page_with_content(content_ref);
    pdf.add_page(page);

    assert!(pdf.objects.len() > 1);
}

#[test]
fn test_stream_operations() {
    let mut stream = Stream::new_compressed();

    let _ = stream.set_color_rgb(1.0, 0.0, 0.0, false);
    stream.rectangle(100.0, 100.0, 200.0, 150.0);
    stream.fill(false);

    assert!(stream.stream.len() > 0);
}

#[test]
fn test_compressed_stream() {
    let stream = Stream::new_compressed();
    assert!(stream.compress);
}

#[test]
fn test_text_operations() {
    let mut stream = Stream::new();

    stream.begin_text();
    stream.set_font_size("Helvetica", 12.0);
    stream.set_text_matrix(1.0, 0.0, 0.0, 1.0, 100.0, 700.0);
    stream.show_text_string("Test");
    stream.end_text();

    assert!(stream.stream.len() > 0);
}
