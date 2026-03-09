use std::rc::Rc;
use pydyf::{PDF, Page, StreamObject, PdfObject};
use pydyf::page::PageSize;
use pydyf::color::{RGB, Color};
use pydyf::util::{Dims, EvenOdd, Posn, StrokeOrFill};

#[test]
fn test_create_pdf() {
    let pdf = PDF::new();
    assert_eq!(pdf.objects.len(), 1);
}

#[test]
fn test_add_page() {
    let mut pdf = PDF::new();
    let stream = StreamObject::new();
    pdf.add_object(Box::new(stream));

    let content_ref = Some(format!("{} 0 R", pdf.objects.len() - 1).into_bytes().into());
    let mut page = Page::new(PageSize::A4);
    page.set_contents(content_ref);
    pdf.add_page(page);

    assert!(pdf.objects.len() > 1);
}

#[test]
fn test_stream_operations() {
    let mut stream = StreamObject::compressed();

    let color = RGB{ red:Color{color:1.0}, green: Color {color:0.0}, blue:Color{color:0.0}};
    let _ = stream.set_color_rgb(color, StrokeOrFill::Stroke);
    stream.rectangle(Posn {x:100.0, y:100.0}, Dims {height:200.0, width:150.0});
    stream.fill(EvenOdd::Odd);

    assert!(stream.stream.len() > 0);
}

#[test]
fn test_compressed_stream() {
    let stream = StreamObject::compressed();
    assert!(stream.compress);
}

/*#[test]
fn test_text_operations() {
    let mut stream = StreamObject::new();

    stream.begin_text();
    stream.set_font_size("Helvetica", 12.0);
    stream.set_text_matrix(1.0, 0.0, 0.0, 1.0, 100.0, 700.0);
    stream.show_text_string("Test");
    assert!(stream.stream.len() > 0);
}*/

#[test]
fn test_add_page_simple_with_pagesize() {
    let mut pdf = PDF::new();
    let stream = StreamObject::new();
    pdf.add_object(Box::new(stream));
    let content_ref = format!("{} 0 R", pdf.objects.len() - 1).into_bytes();

    // A4 size should be 595x842
    let mut page = Page::new(PageSize::A4);
    page.set_contents(Some(Rc::new(content_ref)));
    pdf.add_page(page);

    let page_obj = pdf.objects.last().unwrap();
    let data = page_obj.data();
    let data_str = String::from_utf8_lossy(&data);

    // Should contain MediaBox because it was explicitly provided
    assert!(data_str.contains("/MediaBox [0 0 595 842]"));
    assert!(data_str.contains("/Type /Page"));
}

#[test]
fn test_add_page_simple_default_size() {
    let mut pdf = PDF::new();
    let stream = StreamObject::new();
    pdf.add_object(Box::new(stream));
    let content_ref = format!("{} 0 R", pdf.objects.len() - 1).into_bytes();

    let mut page = Page::new(PageSize::A4);
    page.set_contents(Some(Rc::new(content_ref)));
    pdf.add_page(page);

    let page_obj = pdf.objects.last().unwrap();
    let data = page_obj.data();
    let data_str = String::from_utf8_lossy(&data);

    // Should NOT contain MediaBox because it's inherited
    assert!(!data_str.contains("/MediaBox"));
}

#[test]
fn test_root_mediabox_inheritance() {
    let pdf = PDF::new();
    let pages_dict = &pdf.page_tree;
    let mediabox = pages_dict.values.get("MediaBox").unwrap();
    assert_eq!(String::from_utf8_lossy(mediabox), "[0 0 595 842]");
}

#[test]
fn test_pagesize_custom_validation() {
    let size = PageSize::Custom(Dims { width: -100.0, height: 500.0 });
    let dimensions = size.dimensions();
    assert_eq!(dimensions.width, 0.0);
    assert_eq!(dimensions.height, 500.0);

    let mediabox = size.as_array().data();
    assert_eq!(String::from_utf8_lossy(&mediabox), "[0 0 0 500]");
}