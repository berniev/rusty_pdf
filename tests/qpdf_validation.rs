/*#[test]
fn test_qpdf_validates_compressed_pdf() {
    let mut pdf = PdfFile::new();
    let mut stream = PdfStreamObject::new();

    // Add some content
    let color = RGB::new(Color::new(0.0), Color::new(0.0), Color::new(1.0));
    stream.set_color_rgb(color, StrokeOrFill::Fill);
    stream.add_rectangle(
        Posn { x: 50.0, y: 50.0 },
        Dims {
            height: 100.0,
            width: 100.0,
        },
    );
    stream.fill(WindingRule::EvenOdd);

    let content_id = pdf.add_indirect_object(Box::new(stream));
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
    let mut pdf = PdfFile::new();
    let mut stream = PdfStreamObject::new();

    // Add some content
    let color = RGB::new(Color::new(1.0), Color::new(0.0), Color::new(0.0));

    stream.set_color_rgb(color, StrokeOrFill::Fill);
    stream.add_rectangle(
        Posn { x: 100.0, y: 100.0 },
        Dims {
            height: 50.0,
            width: 50.0,
        },
    );
    stream.fill(WindingRule::EvenOdd);

    let content_id = pdf.add_indirect_object(Box::new(stream));
    let mut page = PageObject::new(0usize.into());
    page.add_content(content_id);
    page.set_media_box(PageSize::A4);
    pdf.add_page(page);

    // Write uncompressed PDF
    let mut output = Vec::new();
    pdf.write_legacy(&mut output, FileIdentifierMode::None)
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
*/
