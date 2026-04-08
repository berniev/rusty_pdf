// Find exactly where BOTH object 5s get their IDs assigned

/*#[test]
fn find_both_object_5_assignments() {
    use rusty_pdf::{FileIdentifierMode, PageObject, PdfStreamObject, PdfFile};
    use rusty_pdf::page::PageSize;

    let mut pdf = PdfFile::new();
    let stream = PdfStreamObject::new();
    let content_id = pdf.add_indirect_object(Box::new(stream));
    let mut page = PageObject::new(0usize.into());
    page.add_content(content_id);
    page.set_media_box(PageSize::A4);
    pdf.add_page(page);

    println!("=== BEFORE write_compressed ===");
    println!("pdf.objects.len() = {}", pdf.objects.len());
    for (i, obj) in pdf.objects.iter().enumerate() {
        println!("  objects[{}]: id = {:?}", i, obj.metadata().object_identifier);
    }

    // Manually call what write_compressed does
    let resources_id = pdf.add_font_resources();
    println!("\n=== After add_font_resources (returns {}) ===", resources_id);
    for (i, obj) in pdf.objects.iter().enumerate() {
        println!("  objects[{}]: id = {:?}", i, obj.metadata().object_identifier);
    }

    pdf.initialize_page_tree(resources_id);
    println!("\n=== After initialize_page_tree ===");
    for (i, obj) in pdf.objects.iter().enumerate() {
        println!("  objects[{}]: id = {:?}", i, obj.metadata().object_identifier);
    }

    pdf.initialize_catalog();
    println!("\n=== After initialize_catalog ===");
    for (i, obj) in pdf.objects.iter().enumerate() {
        println!("  objects[{}]: id = {:?}", i, obj.metadata().object_identifier);
    }

    // Find which one is ID 5
    let obj_with_id_5: Vec<_> = pdf.objects.iter()
        .enumerate()
        .filter(|(_, obj)| obj.metadata().object_identifier == Some(5))
        .collect();

    println!("\n=== Objects with ID 5 BEFORE write ===");
    println!("Count: {}", obj_with_id_5.len());
    for (idx, _) in obj_with_id_5 {
        println!("  objects[{}] has id=5", idx);
    }

    // Now write
    let mut output = Vec::new();
    pdf.write_compressed(&mut output, FileIdentifierMode::None).unwrap();

    // Check pdf.objects AFTER write - did anything change?
    println!("\n=== After write_compressed ===");
    println!("pdf.objects.len() = {}", pdf.objects.len());

    let obj_with_id_5_after: Vec<_> = pdf.objects.iter()
        .enumerate()
        .filter(|(_, obj)| obj.metadata().object_identifier == Some(5))
        .collect();

    println!("\n=== Objects with ID 5 AFTER write ===");
    println!("Count: {}", obj_with_id_5_after.len());
    for (idx, _) in obj_with_id_5_after {
        println!("  objects[{}] has id=5", idx);
    }

    // Parse PDF output
    let pdf_str = String::from_utf8_lossy(&output);
    let mut obj_5_count = 0;
    for line in pdf_str.lines() {
        if line == "5 0 obj" {
            obj_5_count += 1;
        }
    }

    println!("\n=== In final PDF output ===");
    println!("'5 0 obj' appears {} time(s)", obj_5_count);

    assert_eq!(obj_5_count, 1, "Object 5 should appear exactly once in PDF");
}
*/
