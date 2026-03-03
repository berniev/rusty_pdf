use pydyf::{PDF, PageSize, Page, Stream};

fn create_page_with_content(content_stream_ref: Vec<u8>) -> Page {
    let mut page = Page::new();
    page.set_contents(content_stream_ref);
    page
}

#[test]
fn test_create_pdf() {
    let pdf = PDF::new(PageSize::A4);
    assert_eq!(pdf.objects.len(), 1);
}

#[test]
fn test_add_page() {
    let mut pdf = PDF::new(PageSize::A4);
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
    assert!(stream.stream.len() > 0);
}

#[test]
fn test_add_page_simple_with_pagesize() {
    let mut pdf = PDF::new(PageSize::A4);
    let stream = Stream::new();
    pdf.add_object(Box::new(stream));
    let content_ref = format!("{} 0 R", pdf.objects.len() - 1).into_bytes();

    // A4 size should be 595x842
    pdf.add_page_simple(Some(PageSize::A4), &content_ref);

    let page_obj = pdf.objects.last().unwrap();
    let data = page_obj.data();
    let data_str = String::from_utf8_lossy(&data);

    // Should contain MediaBox because it was explicitly provided
    assert!(data_str.contains("/MediaBox [0 0 595 842]"));
    assert!(data_str.contains("/Type /Page"));
}

#[test]
fn test_add_page_simple_default_size() {
    let mut pdf = PDF::new(PageSize::Letter);
    let stream = Stream::new();
    pdf.add_object(Box::new(stream));
    let content_ref = format!("{} 0 R", pdf.objects.len() - 1).into_bytes();

    // Should use Letter size (612x792) inherited from root
    pdf.add_page_simple(None, &content_ref);

    let page_obj = pdf.objects.last().unwrap();
    let data = page_obj.data();
    let data_str = String::from_utf8_lossy(&data);

    // Should NOT contain MediaBox because it's inherited
    assert!(!data_str.contains("/MediaBox"));
}

#[test]
fn test_root_mediabox_inheritance() {
    let pdf = PDF::new(PageSize::A4);
    let pages_dict = &pdf.pages;
    let mediabox = pages_dict.values.get("MediaBox").unwrap();
    assert_eq!(String::from_utf8_lossy(mediabox), "[0 0 595 842]");
}

#[test]
fn test_pagesize_custom_validation() {
    let size = PageSize::Custom(-100.0, 500.0);
    let dimensions = size.dimensions();
    assert_eq!(dimensions.0, 0.0);
    assert_eq!(dimensions.1, 500.0);

    let mediabox = size.to_mediabox();
    assert_eq!(String::from_utf8_lossy(&mediabox), "[0 0 0 500]");
}
