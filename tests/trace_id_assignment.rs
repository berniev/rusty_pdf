// Trace exactly where each object gets its ID assigned

#[test]
fn trace_id_assignments_during_write_compressed() {
    /*    use rusty_pdf::{PdfFile, PdfStreamObject};

        let mut pdf = PdfFile::new();

        // Add a stream object (like the test does)
        let stream = Box::new(PdfStreamObject::new());
        let stream_id = pdf.add_object(Pdf::stream(stream);
        println!("1. Added stream via add_object(), got ID: {}", stream_id);
        //println!("   pdf.objects.len() = {}", pdf.objects.len());

        // Now call the initialization sequence that write_compressed does
        let resources_id = pdf.add_font_resources();
        println!("\n2. Called add_font_resources(), got ID: {}", resources_id);
        println!("   pdf.objects.len() = {}", pdf.objects.len());

        //pdf.initialize_page_tree(resources_id);
        println!("\n3. Called initialize_page_tree()");
       println!("   pdf.objects.len() = {}", pdf.objects.len());
        println!("   Page tree ID: {:?}", pdf.page_tree.metadata.object_identifier);

        // Show all current object IDs
        let ids: Vec<_> = pdf.objects.iter()
            .enumerate()
            .map(|(i, obj)| (i, obj.metadata().object_identifier))
            .collect();
        println!("   All objects: {:?}", ids);

        pdf.initialize_catalog();
        println!("\n4. Called initialize_catalog()");
        println!("   pdf.objects.len() = {}", pdf.objects.len());

        // Show all current object IDs again
        let ids: Vec<_> = pdf.objects.iter()
            .enumerate()
            .map(|(i, obj)| (i, obj.metadata().object_identifier))
            .collect();
        println!("   All objects: {:?}", ids);

        // Now write it
        println!("\n5. About to call write_compressed()");
        println!("   Max existing object ID: {:?}",
            pdf.objects.iter().filter_map(|o| o.metadata().object_identifier).max());

        let mut output = Vec::new();
        pdf.write_compressed(&mut output, FileIdentifierMode::None).unwrap();

        println!("\n6. After write_compressed()");

        // Parse output to see what actually got written
        let pdf_str = String::from_utf8_lossy(&output);
        println!("\n7. Objects written to PDF:");
        for line in pdf_str.lines() {
            if line.ends_with(" obj") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 3 && parts[1] == "0" && parts[2] == "obj" {
                    println!("   {}", line);
                }
            }
        }
    */
}

/*#[test]
fn trace_next_object_number_helper() {
    // Test what next_object_number() returns at each stage
    // Note: it's private, so we infer it from add_object()

    use rusty_pdf::{PdfFile, PdfStreamObject};

    let mut pdf = PdfFile::new();

    println!("Initial: pdf.objects.len() = {}", pdf.objects.len());

    let id1 = pdf.add_object(Pdf::stream(PdfStreamObject::new()));
    println!("After add_object #1: ID={}, pdf.objects.len()={}", id1, pdf.objects.len());

    let id2 = pdf.add_object(Pdf::stream(PdfStreamObject::new()));
    println!("After add_object #2: ID={}, pdf.objects.len()={}", id2, pdf.objects.len());

    let id3 = pdf.add_object(Pdf::stream(PdfStreamObject::new()));
    println!("After add_object #3: ID={}, pdf.objects.len()={}", id3, pdf.objects.len());

    // What next_object_number() should return is: objects.len() + 1
    let expected_next = pdf.objects.len() + 1;
    println!("\nExpected next object number: {}", expected_next);

    // Verify with actual add
    let id4 = pdf.add_object(Pdf::stream(PdfStreamObject::new()));
    println!("Actual next object number: {}", id4);

    assert_eq!(id4, expected_next);
    }
*/

/*#[test]
fn trace_page_tree_id_calculation() {
    // The page tree ID calculation is complex - let's trace it

    use rusty_pdf::{PdfFile, PdfStreamObject};

    let mut pdf = PdfFile::new();

    // Add one object
    pdf.add_object(Pdf::stream(PdfStreamObject::new()));
    println!("After adding 1 object:");
    println!("  pdf.objects.len() = {}", pdf.objects.len());
    println!("  next_object_number would be: {}", pdf.objects.len() + 1);

    // Now add resources
    let resources_id = pdf.add_font_resources();
    println!("\nAfter add_font_resources():");
    println!("  resources_id = {}", resources_id);
    println!("  pdf.objects.len() = {}", pdf.objects.len());
    println!("  next_object_number would be: {}", pdf.objects.len() + 1);

    pdf.initialize_page_tree(resources_id);

    println!("\nAfter initialize_page_tree():");
    println!("  pdf.objects.len() = {}", pdf.objects.len());
}
*/
