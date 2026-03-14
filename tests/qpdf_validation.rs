use pydyf::color::{Color, RGB};
use pydyf::objects::stream::{EvenOdd, StrokeOrFill};
use pydyf::page::PageSize;
use pydyf::util::{Dims, Posn};
use pydyf::{FileIdentifierMode, PageObject, StreamObject, PDF};
use std::fs::File;
use std::io::Write;
use std::process::Command;

#[test]
fn test_qpdf_validates_compressed_pdf() {
    let mut pdf = PDF::new();
    let mut stream = StreamObject::new();

    // Add some content
    let color = RGB {
        red: Color { color: 0.0 },
        green: Color { color: 0.0 },
        blue: Color { color: 1.0 },
    };
    let _ = stream.set_color_rgb(color, StrokeOrFill::Fill);
    stream.rectangle(
        Posn { x: 50.0, y: 50.0 },
        Dims {
            height: 100.0,
            width: 100.0,
        },
    );
    stream.fill(EvenOdd::Odd);

    let content_id = pdf.add_object(Box::new(stream));
    let mut page = PageObject::new(0usize.into());
    page.add_content(content_id);
    page.set_media_box(PageSize::A4);
    pdf.add_page(page);

    // Write compressed PDF
    let mut output = Vec::new();
    pdf.write_compressed(&mut output, FileIdentifierMode::None)
        .expect("Failed to write compressed PDF");

    // Save to file
    let path = "/tmp/pydyf_test/test_compressed.pdf";
    std::fs::create_dir_all("/tmp/pydyf_test").ok();
    let mut file = File::create(path).expect("Failed to create file");
    file.write_all(&output).expect("Failed to write file");

    // Validate with qpdf
    let result = Command::new("qpdf")
        .args(&["--check", path])
        .output()
        .expect("Failed to run qpdf");

    println!("qpdf stdout:\n{}", String::from_utf8_lossy(&result.stdout));
    println!("qpdf stderr:\n{}", String::from_utf8_lossy(&result.stderr));

    assert!(
        result.status.success(),
        "qpdf validation failed with exit code {:?}",
        result.status.code()
    );
}

#[test]
fn test_qpdf_validates_uncompressed_pdf() {
    let mut pdf = PDF::new();
    let mut stream = StreamObject::new();

    // Add some content
    let color = RGB {
        red: Color { color: 1.0 },
        green: Color { color: 0.0 },
        blue: Color { color: 0.0 },
    };
    let _ = stream.set_color_rgb(color, StrokeOrFill::Fill);
    stream.rectangle(
        Posn { x: 100.0, y: 100.0 },
        Dims {
            height: 50.0,
            width: 50.0,
        },
    );
    stream.fill(EvenOdd::Odd);

    let content_id = pdf.add_object(Box::new(stream));
    let mut page = PageObject::new(0usize.into());
    page.add_content(content_id);
    page.set_media_box(PageSize::A4);
    pdf.add_page(page);

    // Write uncompressed PDF
    let mut output = Vec::new();
    pdf.write(&mut output, FileIdentifierMode::None)
        .expect("Failed to write PDF");

    // Save to file
    let path = "/tmp/pydyf_test/test_uncompressed.pdf";
    std::fs::create_dir_all("/tmp/pydyf_test").ok();
    let mut file = File::create(path).expect("Failed to create file");
    file.write_all(&output).expect("Failed to write file");

    // Validate with qpdf
    let result = Command::new("qpdf")
        .args(&["--check", path])
        .output()
        .expect("Failed to run qpdf");

    println!("qpdf stdout:\n{}", String::from_utf8_lossy(&result.stdout));
    println!("qpdf stderr:\n{}", String::from_utf8_lossy(&result.stderr));

    assert!(
        result.status.success(),
        "qpdf validation failed with exit code {:?}",
        result.status.code()
    );
}
