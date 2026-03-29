/// Find the TWO objects that both become "4 0 obj"

#[test]
fn trace_both_object_4s() {
    use pydyf::{FileIdentifierMode, PageObject, PdfStreamObject, PdfFile};
    use pydyf::page::PageSize;

    let mut pdf = PdfFile::new();
    let stream = PdfStreamObject::uncompressed();
    let content_id = pdf.add_object(Box::new(stream));
    let mut page = PageObject::new(0usize.into());
    page.add_content(content_id);
    page.set_media_box(PageSize::A4);
    pdf.add_page(page);

    let mut output = Vec::new();
    pdf.write_compressed(&mut output, FileIdentifierMode::None).unwrap();

    // Parse and find both "4 0 obj"
    let pdf_str = String::from_utf8_lossy(&output);
    let lines: Vec<&str> = pdf_str.lines().collect();

    println!("\n=== Finding both '4 0 obj' occurrences ===\n");

    let mut occurrence = 0;
    for (i, line) in lines.iter().enumerate() {
        if line == &"4 0 obj" {
            occurrence += 1;
            println!("Occurrence #{} at line {}:", occurrence, i);
            println!("  Previous line: {}", if i > 0 { lines[i-1] } else { "" });
            println!("  This line: {}", line);

            // Show next 3 lines
            for j in 1..=3 {
                if i + j < lines.len() {
                    println!("  Next line {}: {}", j, lines[i+j]);
                }
            }
            println!();
        }
    }

    // Show all objects in the PDF
    println!("=== All objects in PDF ===");
    for line in lines.iter() {
        if line.ends_with(" obj") {
            println!("  {}", line);
        }
    }

    if occurrence != 1 {
        println!("\nExpected 1 occurrence of '4 0 obj', found {}", occurrence);
    }
}
