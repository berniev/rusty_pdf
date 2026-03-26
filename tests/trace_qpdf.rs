/// Trace the exact sequence from qpdf test

#[test]
fn trace_qpdf_test_sequence() {
    use pydyf::{FileIdentifierMode, PageObject, PdfStreamObject, PDF};
    use pydyf::page::PageSize;

    let mut pdf = PDF::new();
    let stream = PdfStreamObject::uncompressed();

    // Add stream
    let content_id = pdf.add_object(Box::new(stream));
    println!("1. Added stream object");
    println!("   pdf.objects.len() = {}", pdf.objects.len());

    let ids: Vec<_> = pdf.objects.iter()
        .map(|obj| obj.metadata().object_identifier)
        .collect();
    println!("   Object IDs: {:?}", ids);

    // Add page
    let mut page = PageObject::new(0usize.into());
    page.add_content(content_id);
    page.set_media_box(PageSize::A4);
    pdf.add_page(page);

    println!("\n2. Called pdf.add_page()");
    println!("   pdf.objects.len() = {}", pdf.objects.len());

    let ids: Vec<_> = pdf.objects.iter()
        .map(|obj| obj.metadata().object_identifier)
        .collect();
    println!("   Object IDs: {:?}", ids);

    // Now write_compressed calls these
    let resources_id = pdf.add_font_resources();
    println!("\n3. add_font_resources() returned {}", resources_id);
    println!("   pdf.objects.len() = {}", pdf.objects.len());

    let ids: Vec<_> = pdf.objects.iter()
        .map(|obj| obj.metadata().object_identifier)
        .collect();
    println!("   Object IDs: {:?}", ids);

    pdf.initialize_page_tree(resources_id);
    println!("\n4. initialize_page_tree()");
    println!("   pdf.objects.len() = {}", pdf.objects.len());

    let ids: Vec<_> = pdf.objects.iter()
        .map(|obj| obj.metadata().object_identifier)
        .collect();
    println!("   Object IDs: {:?}", ids);

    pdf.initialize_catalog();
    println!("\n5. initialize_catalog()");
    println!("   pdf.objects.len() = {}", pdf.objects.len());

    let ids: Vec<_> = pdf.objects.iter()
        .map(|obj| obj.metadata().object_identifier)
        .collect();
    println!("   Object IDs: {:?}", ids);

    println!("\n6. Max object ID before write: {:?}",
        pdf.objects.iter().filter_map(|o| o.metadata().object_identifier).max());

    // Write
    let mut output = Vec::new();
    pdf.write_compressed(&mut output, FileIdentifierMode::None).unwrap();

    // Show what was written
    let pdf_str = String::from_utf8_lossy(&output);
    println!("\n7. Objects in final PDF:");
    for line in pdf_str.lines() {
        if line.ends_with(" obj") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 3 && parts[1] == "0" && parts[2] == "obj" {
                println!("   {}", line);
            }
        }
    }
}
