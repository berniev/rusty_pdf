/// Debug test to see exactly what our object stream contains

use pydyf::{FileIdentifierMode, PageObject, StreamObject, PDF};
use pydyf::page::PageSize;

#[test]
fn debug_what_we_produce() {
    let mut pdf = PDF::new();
    let mut stream = StreamObject::new();
    stream.rectangle(pydyf::util::Posn { x: 0.0, y: 0.0 }, pydyf::util::Dims { height: 10.0, width: 10.0 });
    pdf.add_object(Box::new(stream));

    let next_num = pdf.objects.len() - 1;
    let mut page = PageObject::new(next_num.into());
    page.set_media_box(PageSize::A4);
    pdf.add_page(page);

    let mut output = Vec::new();
    pdf.write_compressed(&mut output, FileIdentifierMode::None).unwrap();

    let pdf_str = String::from_utf8_lossy(&output);

    println!("=== FULL PDF ===");
    println!("{}", pdf_str);
    println!("=== END ===");

    // Find the ObjStm
    if let Some(objstm_pos) = pdf_str.find("/Type /ObjStm") {
        println!("\nFound ObjStm at position {}", objstm_pos);

        // Extract /N
        if let Some(n_pos) = pdf_str[objstm_pos..].find("/N ") {
            let n_section = &pdf_str[objstm_pos + n_pos + 3..];
            let n_val = n_section.split_whitespace().next().unwrap();
            println!("/N = {}", n_val);
        }

        // Extract /First
        if let Some(first_pos) = pdf_str[objstm_pos..].find("/First ") {
            let first_section = &pdf_str[objstm_pos + first_pos + 7..];
            let first_val = first_section.split_whitespace().next().unwrap();
            println!("/First = {}", first_val);
        }
    }
}
