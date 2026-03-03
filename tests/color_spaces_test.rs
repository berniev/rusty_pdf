use pydyf::{PDF, PageSize, Page, Stream};
use std::fs::File;

fn create_page_with_content(content_stream_ref: Vec<u8>) -> Page {
    let mut page = Page::new();
    page.set_contents(content_stream_ref);
    page
}

#[test]
fn test_cmyk_colors() {
    let mut pdf = PDF::new(PageSize::A4);
    let mut stream = Stream::new();

    let _ = stream.set_color_cmyk(1.0, 0.0, 0.0, 0.0, false);
    stream.rectangle(50.0, 650.0, 100.0, 100.0);
    stream.fill(false);

    let _ = stream.set_color_cmyk(0.0, 1.0, 0.0, 0.0, false);
    stream.rectangle(200.0, 650.0, 100.0, 100.0);
    stream.fill(false);

    let _ = stream.set_color_cmyk(0.0, 0.0, 1.0, 0.0, false);
    stream.rectangle(350.0, 650.0, 100.0, 100.0);
    stream.fill(false);

    let _ = stream.set_color_cmyk(0.0, 0.0, 0.0, 1.0, false);
    stream.rectangle(50.0, 500.0, 100.0, 100.0);
    stream.fill(false);

    let _ = stream.set_color_cmyk(0.5, 1.0, 0.0, 0.0, false);
    stream.rectangle(200.0, 500.0, 100.0, 100.0);
    stream.fill(false);

    let _ = stream.set_color_cmyk(0.0, 0.0, 0.0, 1.0, false);
    stream.begin_text();
    stream.set_font_size("Helvetica", 12.0);
    stream.set_text_matrix(1.0, 0.0, 0.0, 1.0, 50.0, 630.0);
    stream.show_text_string("CMYK Colors");
    stream.end_text();

    pdf.add_object(Box::new(stream));
    let content_ref = format!("{} 0 R", pdf.objects.len() - 1).into_bytes();
    let page = create_page_with_content(content_ref);
    pdf.add_page(page);

    std::fs::create_dir_all("/tmp/pydyf_test").unwrap();
    let mut file = File::create("/tmp/pydyf_test/cmyk.pdf").unwrap();
    pdf.write(&mut file, Some(b"1.7"), pydyf::Identifier::AutoMD5, false).unwrap();

    println!("✅ Generated: /tmp/pydyf_test/cmyk.pdf");
}

#[test]
fn test_grayscale_colors() {
    let mut pdf = PDF::new(PageSize::A4);
    let mut stream = Stream::new();

    let _ = stream.set_color_gray(0.0, false);
    stream.rectangle(50.0, 650.0, 80.0, 80.0);
    stream.fill(false);

    let _ = stream.set_color_gray(0.25, false);
    stream.rectangle(150.0, 650.0, 80.0, 80.0);
    stream.fill(false);

    let _ = stream.set_color_gray(0.5, false);
    stream.rectangle(250.0, 650.0, 80.0, 80.0);
    stream.fill(false);

    let _ = stream.set_color_gray(0.75, false);
    stream.rectangle(350.0, 650.0, 80.0, 80.0);
    stream.fill(false);

    let _ = stream.set_color_gray(1.0, false);
    stream.rectangle(450.0, 650.0, 80.0, 80.0);
    stream.fill(false);

    let _ = stream.set_color_gray(0.0, true);
    stream.set_line_width(2.0);
    stream.rectangle(450.0, 650.0, 80.0, 80.0);
    stream.stroke();

    let _ = stream.set_color_gray(0.0, false);
    stream.begin_text();
    stream.set_font_size("Helvetica", 12.0);
    stream.set_text_matrix(1.0, 0.0, 0.0, 1.0, 50.0, 630.0);
    stream.show_text_string("Grayscale: Black to White");
    stream.end_text();

    pdf.add_object(Box::new(stream));
    let content_ref = format!("{} 0 R", pdf.objects.len() - 1).into_bytes();
    let page = create_page_with_content(content_ref);
    pdf.add_page(page);

    std::fs::create_dir_all("/tmp/pydyf_test").unwrap();
    let mut file = File::create("/tmp/pydyf_test/gray.pdf").unwrap();
    pdf.write(&mut file, Some(b"1.7"), pydyf::Identifier::AutoMD5, false).unwrap();

    println!("✅ Generated: /tmp/pydyf_test/gray.pdf");
}

#[test]
fn test_mixed_color_spaces() {
    let mut pdf = PDF::new(PageSize::A4);
    let mut stream = Stream::new();

    let _ = stream.set_color_rgb(1.0, 0.0, 0.0, false);
    stream.rectangle(50.0, 650.0, 100.0, 100.0);
    stream.fill(false);

    let _ = stream.set_color_cmyk(1.0, 0.0, 0.0, 0.0, false);
    stream.rectangle(200.0, 650.0, 100.0, 100.0);
    stream.fill(false);

    let _ = stream.set_color_gray(0.5, false);
    stream.rectangle(350.0, 650.0, 100.0, 100.0);
    stream.fill(false);

    let _ = stream.set_color_gray(0.0, false);
    stream.begin_text();
    stream.set_font_size("Helvetica", 10.0);
    stream.set_text_matrix(1.0, 0.0, 0.0, 1.0, 70.0, 620.0);
    stream.show_text_string("RGB");
    stream.set_text_matrix(1.0, 0.0, 0.0, 1.0, 215.0, 620.0);
    stream.show_text_string("CMYK");
    stream.set_text_matrix(1.0, 0.0, 0.0, 1.0, 365.0, 620.0);
    stream.show_text_string("Gray");
    stream.end_text();

    pdf.add_object(Box::new(stream));
    let content_ref = format!("{} 0 R", pdf.objects.len() - 1).into_bytes();
    let page = create_page_with_content(content_ref);
    pdf.add_page(page);

    std::fs::create_dir_all("/tmp/pydyf_test").unwrap();
    let mut file = File::create("/tmp/pydyf_test/mixed.pdf").unwrap();
    pdf.write(&mut file, Some(b"1.7"), pydyf::Identifier::AutoMD5, false).unwrap();

    println!("✅ Generated: /tmp/pydyf_test/mixed.pdf");
}
