use pydyf::{PDF, PageSize, Page, Stream, Identifier};
use std::fs::File;

fn create_page_with_content(content_stream_ref: Vec<u8>) -> Page {
    let mut page = Page::new();
    page.set_contents(content_stream_ref);
    page
}

#[test]
fn test_generate_simple_uncompressed_pdf() {
    let mut pdf = PDF::new(PageSize::A4);
    let mut stream = Stream::new();

    let _ = stream.set_color_rgb(1.0, 0.0, 0.0, false);
    stream.rectangle(100.0, 100.0, 200.0, 150.0);
    stream.fill(false);

    stream.begin_text();
    stream.set_font_size("Helvetica", 24.0);
    stream.set_text_matrix(1.0, 0.0, 0.0, 1.0, 100.0, 300.0);
    stream.show_text_string("Hello PDF!");
    stream.end_text();

    pdf.add_object(Box::new(stream));
    let content_ref = format!("{} 0 R", pdf.objects.len() - 1).into_bytes();
    let page = create_page_with_content(content_ref);
    pdf.add_page(page);

    std::fs::create_dir_all("/tmp/pydyf_test").unwrap();
    let mut file = File::create("/tmp/pydyf_test/u.pdf").unwrap();
    pdf.write(&mut file, Some(b"1.7"), Identifier::AutoMD5, false).unwrap();

    println!("✅ Generated: /tmp/pydyf_test/u.pdf");
}

#[test]
fn test_generate_circle_over_rectangle() {
    let mut pdf = PDF::new(PageSize::A4);
    let mut stream = Stream::new();

    let _ = stream.set_color_rgb(0.0, 0.0, 1.0, false);
    stream.rectangle(50.0, 650.0, 200.0, 100.0);
    stream.fill(false);

    let _ = stream.set_color_rgb(0.0, 1.0, 0.0, false);
    stream.move_to(150.0, 700.0);
    stream.curve_to(150.0, 727.6, 127.6, 750.0, 100.0, 750.0);
    stream.curve_to(72.4, 750.0, 50.0, 727.6, 50.0, 700.0);
    stream.curve_to(50.0, 672.4, 72.4, 650.0, 100.0, 650.0);
    stream.curve_to(127.6, 650.0, 150.0, 672.4, 150.0, 700.0);
    stream.close();
    stream.fill(false);

    pdf.add_object(Box::new(stream));
    let content_ref = format!("{} 0 R", pdf.objects.len() - 1).into_bytes();
    let page = create_page_with_content(content_ref);
    pdf.add_page(page);

    std::fs::create_dir_all("/tmp/pydyf_test").unwrap();
    let mut file = File::create("/tmp/pydyf_test/c.pdf").unwrap();
    pdf.write(&mut file, Some(b"1.7"), Identifier::AutoMD5, false).unwrap();

    println!("✅ Generated: /tmp/pydyf_test/c.pdf");
}

#[test]
fn test_multipage_pdf() {
    let mut pdf = PDF::new(PageSize::A4);

    let mut stream1 = Stream::new();
    let _ = stream1.set_color_rgb(1.0, 0.0, 0.0, false);
    stream1.rectangle(50.0, 650.0, 200.0, 100.0);
    stream1.fill(false);
    pdf.add_object(Box::new(stream1));
    let content_ref1 = format!("{} 0 R", pdf.objects.len() - 1).into_bytes();
    let page1 = create_page_with_content(content_ref1);
    pdf.add_page(page1);

    let mut stream2 = Stream::new();
    let _ = stream2.set_color_rgb(0.0, 1.0, 0.0, false);
    stream2.rectangle(150.0, 550.0, 200.0, 100.0);
    stream2.fill(false);
    pdf.add_object(Box::new(stream2));
    let content_ref2 = format!("{} 0 R", pdf.objects.len() - 1).into_bytes();
    let page2 = create_page_with_content(content_ref2);
    pdf.add_page(page2);

    let mut stream3 = Stream::new();
    let _ = stream3.set_color_rgb(0.0, 0.0, 1.0, false);
    stream3.rectangle(250.0, 450.0, 200.0, 100.0);
    stream3.fill(false);
    pdf.add_object(Box::new(stream3));
    let content_ref3 = format!("{} 0 R", pdf.objects.len() - 1).into_bytes();
    let page3 = create_page_with_content(content_ref3);
    pdf.add_page(page3);

    std::fs::create_dir_all("/tmp/pydyf_test").unwrap();
    let mut file = File::create("/tmp/pydyf_test/m.pdf").unwrap();
    pdf.write(&mut file, Some(b"1.7"), Identifier::AutoMD5, false).unwrap();

    println!("✅ Generated: /tmp/pydyf_test/m.pdf (3 pages)");
}

#[test]
fn test_graphics_operations() {
    let mut pdf = PDF::new(PageSize::A4);
    let mut stream = Stream::new();

    let _ = stream.set_color_rgb(1.0, 0.0, 0.0, false);
    stream.rectangle(50.0, 700.0, 100.0, 50.0);
    stream.fill(false);

    let _ = stream.set_color_rgb(0.0, 1.0, 0.0, true);
    stream.set_line_width(3.0);
    stream.rectangle(200.0, 700.0, 100.0, 50.0);
    stream.stroke();

    let _ = stream.set_color_rgb(1.0, 1.0, 0.0, false);
    let _ = stream.set_color_rgb(0.0, 0.0, 1.0, true);
    stream.set_line_width(2.0);
    stream.rectangle(350.0, 700.0, 100.0, 50.0);
    stream.fill_and_stroke(false);

    let _ = stream.set_color_rgb(0.0, 0.0, 0.0, true);
    stream.set_dash(&[5.0, 3.0, 1.0, 3.0], 0);
    stream.move_to(50.0, 650.0);
    stream.line_to(450.0, 650.0);
    stream.stroke();

    let _ = stream.set_color_rgb(1.0, 0.0, 1.0, true);
    stream.set_dash(&[], 0);
    stream.move_to(50.0, 600.0);
    stream.curve_to(150.0, 650.0, 200.0, 550.0, 300.0, 600.0);
    stream.stroke();

    let _ = stream.set_color_rgb(0.0, 1.0, 1.0, false);
    stream.move_to(50.0, 500.0);
    stream.line_to(100.0, 550.0);
    stream.line_to(150.0, 500.0);
    stream.close();
    stream.fill(false);

    let _ = stream.set_color_rgb(1.0, 0.5, 0.0, false);
    stream.move_to(250.0, 550.0);
    stream.line_to(300.0, 550.0);
    stream.line_to(325.0, 500.0);
    stream.line_to(275.0, 500.0);
    stream.close();
    stream.fill(false);

    pdf.add_object(Box::new(stream));
    let content_ref = format!("{} 0 R", pdf.objects.len() - 1).into_bytes();
    let page = create_page_with_content(content_ref);
    pdf.add_page(page);

    std::fs::create_dir_all("/tmp/pydyf_test").unwrap();
    let mut file = File::create("/tmp/pydyf_test/g.pdf").unwrap();
    pdf.write(&mut file, Some(b"1.7"), Identifier::AutoMD5, false).unwrap();

    println!("✅ Generated: /tmp/pydyf_test/g.pdf");
}

#[test]
fn test_comparison_uncompressed() {
    let mut pdf = PDF::new(PageSize::A4);
    let mut stream = Stream::new();

    let _ = stream.set_color_rgb(0.9, 0.9, 0.9, false);
    stream.rectangle(50.0, 50.0, 512.0, 692.0);
    stream.fill(false);

    let _ = stream.set_color_rgb(0.0, 0.0, 0.0, false);
    stream.begin_text();
    stream.set_font_size("Courier", 10.0);

    for i in 0..29 {
        stream.set_text_matrix(1.0, 0.0, 0.0, 1.0, 60.0, 700.0 - (i as f64 * 20.0));
        stream.show_text_string(&format!("Line {} - Testing PDF generation", i + 1));
    }

    stream.end_text();

    pdf.add_object(Box::new(stream));
    let content_ref = format!("{} 0 R", pdf.objects.len() - 1).into_bytes();
    let page = create_page_with_content(content_ref);
    pdf.add_page(page);

    std::fs::create_dir_all("/tmp/pydyf_test").unwrap();
    let mut file = File::create("/tmp/pydyf_test/cu.pdf").unwrap();
    pdf.write(&mut file, Some(b"1.7"), Identifier::AutoMD5, false).unwrap();

    println!("✅ Generated: /tmp/pydyf_test/cu.pdf");
}

#[test]
fn test_comparison_compressed() {
    let mut pdf = PDF::new(PageSize::A4);
    let mut stream = Stream::new_compressed();

    let _ = stream.set_color_rgb(0.9, 0.9, 0.9, false);
    stream.rectangle(50.0, 50.0, 512.0, 692.0);
    stream.fill(false);

    let _ = stream.set_color_rgb(0.0, 0.0, 0.0, false);
    stream.begin_text();
    stream.set_font_size("Courier", 10.0);

    for i in 0..29 {
        stream.set_text_matrix(1.0, 0.0, 0.0, 1.0, 60.0, 700.0 - (i as f64 * 20.0));
        stream.show_text_string(&format!("Line {} - Testing PDF generation", i + 1));
    }

    stream.end_text();

    pdf.add_object(Box::new(stream));
    let content_ref = format!("{} 0 R", pdf.objects.len() - 1).into_bytes();
    let page = create_page_with_content(content_ref);
    pdf.add_page(page);

    std::fs::create_dir_all("/tmp/pydyf_test").unwrap();
    let mut file = File::create("/tmp/pydyf_test/cc.pdf").unwrap();
    pdf.write(&mut file, Some(b"1.7"), Identifier::AutoMD5, false).unwrap();

    println!("✅ Generated: /tmp/pydyf_test/cc.pdf");
}
