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
fn test_empty_page() {
    let mut pdf = PDF::new();
    let stream = Stream::new();

    pdf.add_object(Box::new(stream));
    let content_ref = format!("{} 0 R", pdf.objects.len() - 1).into_bytes();
    let page = create_page_with_content(content_ref);
    pdf.add_page(page);

    std::fs::create_dir_all("/tmp/pydyf_test").unwrap();
    let mut file = File::create("/tmp/pydyf_test/break_empty.pdf").unwrap();
    pdf.write(&mut file, Some(b"1.7"), pydyf::Identifier::AutoMD5, false).unwrap();
}

#[test]
fn test_massive_page_count() {
    let mut pdf = PDF::new();

    for _ in 0..500 {
        let mut stream = Stream::new();
        let _ = stream.set_color_rgb(0.5, 0.5, 0.5, false);
        stream.rectangle(50.0, 50.0, 100.0, 100.0);
        stream.fill(false);

        pdf.add_object(Box::new(stream));
        let content_ref = format!("{} 0 R", pdf.objects.len() - 1).into_bytes();
        let page = create_page_with_content(content_ref);
        pdf.add_page(page);
    }

    std::fs::create_dir_all("/tmp/pydyf_test").unwrap();
    let mut file = File::create("/tmp/pydyf_test/break_massive.pdf").unwrap();
    pdf.write(&mut file, Some(b"1.7"), pydyf::Identifier::AutoMD5, false).unwrap();
}

#[test]
fn test_extreme_coordinates() {
    let mut pdf = PDF::new();
    let mut stream = Stream::new();

    let _ = stream.set_color_rgb(1.0, 0.0, 0.0, false);
    stream.rectangle(-1000.0, -1000.0, 100.0, 100.0);
    stream.fill(false);

    let _ = stream.set_color_rgb(0.0, 1.0, 0.0, false);
    stream.rectangle(10000.0, 10000.0, 100.0, 100.0);
    stream.fill(false);

    let _ = stream.set_color_rgb(0.0, 0.0, 1.0, false);
    stream.rectangle(0.0, 0.0, 0.0, 0.0);
    stream.fill(false);

    let _ = stream.set_color_rgb(0.5, 0.5, 0.5, false);
    stream.rectangle(100.0, 100.0, 10000.0, 10000.0);
    stream.fill(false);

    pdf.add_object(Box::new(stream));
    let content_ref = format!("{} 0 R", pdf.objects.len() - 1).into_bytes();
    let page = create_page_with_content(content_ref);
    pdf.add_page(page);

    std::fs::create_dir_all("/tmp/pydyf_test").unwrap();
    let mut file = File::create("/tmp/pydyf_test/break_coords.pdf").unwrap();
    pdf.write(&mut file, Some(b"1.7"), pydyf::Identifier::AutoMD5, false).unwrap();
}

#[test]
fn test_very_long_text() {
    let mut pdf = PDF::new();
    let mut stream = Stream::new();

    let _ = stream.set_color_rgb(0.0, 0.0, 0.0, false);
    stream.begin_text();
    stream.set_font_size("Courier", 8.0);

    for i in 0..1000 {
        stream.set_text_matrix(1.0, 0.0, 0.0, 1.0, 50.0, 700.0 - (i as f64 * 10.0));
        stream.show_text_string(&format!("Line {}", i));
    }

    stream.end_text();

    pdf.add_object(Box::new(stream));
    let content_ref = format!("{} 0 R", pdf.objects.len() - 1).into_bytes();
    let page = create_page_with_content(content_ref);
    pdf.add_page(page);

    std::fs::create_dir_all("/tmp/pydyf_test").unwrap();
    let mut file = File::create("/tmp/pydyf_test/break_longtext.pdf").unwrap();
    pdf.write(&mut file, Some(b"1.7"), pydyf::Identifier::AutoMD5, false).unwrap();
}

#[test]
fn test_special_characters_text() {
    let mut pdf = PDF::new();
    let mut stream = Stream::new();

    let _ = stream.set_color_rgb(0.0, 0.0, 0.0, false);
    stream.begin_text();
    stream.set_font_size("Helvetica", 12.0);

    stream.set_text_matrix(1.0, 0.0, 0.0, 1.0, 50.0, 700.0);
    stream.show_text_string("Parentheses: (test)");

    stream.set_text_matrix(1.0, 0.0, 0.0, 1.0, 50.0, 680.0);
    stream.show_text_string("Backslash: \\ test");

    stream.set_text_matrix(1.0, 0.0, 0.0, 1.0, 50.0, 660.0);
    stream.show_text_string("Quotes: \"test\"");

    stream.set_text_matrix(1.0, 0.0, 0.0, 1.0, 50.0, 640.0);
    stream.show_text_string("Mixed: (\\) \"test\" \\n");

    stream.end_text();

    pdf.add_object(Box::new(stream));
    let content_ref = format!("{} 0 R", pdf.objects.len() - 1).into_bytes();
    let page = create_page_with_content(content_ref);
    pdf.add_page(page);

    std::fs::create_dir_all("/tmp/pydyf_test").unwrap();
    let mut file = File::create("/tmp/pydyf_test/break_special_chars.pdf").unwrap();
    pdf.write(&mut file, Some(b"1.7"), pydyf::Identifier::AutoMD5, false).unwrap();
}

#[test]
fn test_huge_rectangle() {
    let mut pdf = PDF::new();
    let mut stream = Stream::new();

    let _ = stream.set_color_rgb(1.0, 0.0, 0.0, false);
    stream.rectangle(0.0, 0.0, 5000.0, 5000.0);
    stream.fill(false);

    pdf.add_object(Box::new(stream));
    let content_ref = format!("{} 0 R", pdf.objects.len() - 1).into_bytes();
    let page = create_page_with_content(content_ref);
    pdf.add_page(page);

    std::fs::create_dir_all("/tmp/pydyf_test").unwrap();
    let mut file = File::create("/tmp/pydyf_test/break_huge.pdf").unwrap();
    pdf.write(&mut file, Some(b"1.7"), pydyf::Identifier::AutoMD5, false).unwrap();
}

#[test]
fn test_compressed_empty() {
    let mut pdf = PDF::new();
    let stream = Stream::new_compressed();

    pdf.add_object(Box::new(stream));
    let content_ref = format!("{} 0 R", pdf.objects.len() - 1).into_bytes();
    let page = create_page_with_content(content_ref);
    pdf.add_page(page);

    std::fs::create_dir_all("/tmp/pydyf_test").unwrap();
    let mut file = File::create("/tmp/pydyf_test/break_compressed_empty.pdf").unwrap();
    pdf.write(&mut file, Some(b"1.7"), pydyf::Identifier::AutoMD5, false).unwrap();
}

#[test]
fn test_extreme_font_sizes() {
    let mut pdf = PDF::new();
    let mut stream = Stream::new();

    let _ = stream.set_color_rgb(0.0, 0.0, 0.0, false);
    stream.begin_text();

    stream.set_font_size("Helvetica", 0.1);
    stream.set_text_matrix(1.0, 0.0, 0.0, 1.0, 50.0, 700.0);
    stream.show_text_string("Tiny");

    stream.set_font_size("Helvetica", 1.0);
    stream.set_text_matrix(1.0, 0.0, 0.0, 1.0, 50.0, 650.0);
    stream.show_text_string("Small");

    stream.set_font_size("Helvetica", 200.0);
    stream.set_text_matrix(1.0, 0.0, 0.0, 1.0, 50.0, 500.0);
    stream.show_text_string("BIG");

    stream.end_text();

    pdf.add_object(Box::new(stream));
    let content_ref = format!("{} 0 R", pdf.objects.len() - 1).into_bytes();
    let page = create_page_with_content(content_ref);
    pdf.add_page(page);

    std::fs::create_dir_all("/tmp/pydyf_test").unwrap();
    let mut file = File::create("/tmp/pydyf_test/break_fonts.pdf").unwrap();
    pdf.write(&mut file, Some(b"1.7"), pydyf::Identifier::AutoMD5, false).unwrap();
}

#[test]
fn test_overlapping_operations() {
    let mut pdf = PDF::new();
    let mut stream = Stream::new();

    stream.begin_text();
    let _ = stream.set_color_rgb(1.0, 0.0, 0.0, false);
    stream.rectangle(100.0, 100.0, 200.0, 150.0);
    stream.fill(false);
    stream.end_text();

    pdf.add_object(Box::new(stream));
    let content_ref = format!("{} 0 R", pdf.objects.len() - 1).into_bytes();
    let page = create_page_with_content(content_ref);
    pdf.add_page(page);

    std::fs::create_dir_all("/tmp/pydyf_test").unwrap();
    let mut file = File::create("/tmp/pydyf_test/break_overlap.pdf").unwrap();
    pdf.write(&mut file, Some(b"1.7"), pydyf::Identifier::AutoMD5, false).unwrap();
}

#[test]
fn test_no_pages() {
    let mut pdf = PDF::new();

    std::fs::create_dir_all("/tmp/pydyf_test").unwrap();
    let mut file = File::create("/tmp/pydyf_test/break_no_pages.pdf").unwrap();
    pdf.write(&mut file, Some(b"1.7"), pydyf::Identifier::AutoMD5, false).unwrap();
}
