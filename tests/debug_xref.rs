use pydyf::color::{Color, RGB};
use pydyf::objects::stream::{StrokeOrFill, WindingRule};
use pydyf::page::PageSize;
use pydyf::util::{Dims, Posn};
use pydyf::{FileIdentifierMode, PDF, PageObject, PdfStreamObject};
use std::fs::File;
use std::io::Write as IoWrite;

#[test]
#[ignore]
fn debug_xref_structure() {
    let mut pdf = PDF::new();
    let mut stream = PdfStreamObject::uncompressed();

    let color = RGB::new(Color::new(0.0), Color::new(0.0), Color::new(1.0));
    stream.set_color_rgb(color, StrokeOrFill::Fill);
    stream.rectangle(
        Posn { x: 50.0, y: 50.0 },
        Dims {
            height: 100.0,
            width: 100.0,
        },
    );
    stream.fill(WindingRule::EvenOdd);

    let content_id = pdf.add_object(Box::new(stream));
    let mut page = PageObject::new(0usize.into());
    page.add_content(content_id);
    page.set_media_box(PageSize::A4);
    pdf.add_page(page);

    // Before writing
    println!("\n=== BEFORE write_compressed ===");
    println!("pdf.objects.len() = {}", pdf.objects.len());
    for (i, obj) in pdf.objects.iter().enumerate() {
        println!(
            "  objects[{}]: id={:?}, type={}",
            i,
            obj.metadata().object_identifier,
            std::any::type_name_of_val(obj.as_ref())
        );
    }

    // Write compressed PDF
    let mut output = Vec::new();
    pdf.write_compressed(&mut output, FileIdentifierMode::None)
        .expect("Failed to write compressed PDF");

    // After writing
    println!("\n=== AFTER write_compressed ===");
    println!("pdf.objects.len() = {}", pdf.objects.len());
    for (i, obj) in pdf.objects.iter().enumerate() {
        println!(
            "  objects[{}]: id={:?}, offset={}, type={}",
            i,
            obj.metadata().object_identifier,
            obj.metadata().offset,
            std::any::type_name_of_val(obj.as_ref())
        );
    }

    // Save and show structure
    let path = "/tmp/pydyf_test/test_xref_debug.pdf";
    std::fs::create_dir_all("/tmp/pydyf_test").ok();
    let mut file = File::create(path).expect("Failed to create file");
    file.write_all(&output).expect("Failed to write file");

    println!("\n=== PDF Structure ===");
    let pdf_str = String::from_utf8_lossy(&output);

    // Find all "N 0 obj" patterns
    for (i, line) in pdf_str.lines().enumerate() {
        if line.contains(" obj") {
            println!("Line {}: {}", i, line);
        }
    }
}
