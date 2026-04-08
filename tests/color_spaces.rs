/*fn create_page_with_content(content_index: usize) -> PageObject {
    let mut page = PageObject::new(0usize.into());
    page.add_content(content_index);
    page
}
*/
/*#[test]
fn test_cmyk_colors() {
    let mut pdf = PdfFile::new();
    let mut stream = Stream::new();

    stream.set_color_cmyk(
        CMYK::new(
         Color::new(1.0),
         Color::new(0.0),
         Color::new(0.0),
         Color::new(0.0),
        ),
        StrokeOrFill::Fill,
    );
    stream.add_rectangle(
        Posn { x: 50.0, y: 650.0 },
        Dims {
            width: 100.0,
            height: 100.0,
        },
    );
    stream.fill(WindingRule::EvenOdd);

    stream.set_color_cmyk(
        CMYK::new(
         Color::new(0.0),
         Color::new(1.0),
         Color::new(0.0),
         Color::new(0.0),
        ),
        StrokeOrFill::Fill,
    );
    stream.add_rectangle(
        Posn { x: 200.0, y: 650.0 },
        Dims {
            width: 100.0,
            height: 100.0,
        },
    );
    stream.fill(WindingRule::EvenOdd);

    stream.set_color_cmyk(
        CMYK::new(
         Color::new(0.0),
         Color::new(0.0),
         Color::new(1.0),
         Color::new(0.0),
        ),
        StrokeOrFill::Fill,
    );
    stream.add_rectangle(
        Posn { x: 350.0, y: 650.0 },
        Dims {
            width: 100.0,
            height: 100.0,
        },
    );
    stream.fill(WindingRule::EvenOdd);

    stream.set_color_cmyk(
        CMYK::new(
         Color::new(0.0),
         Color::new(0.0),
         Color::new(0.0),
         Color::new(1.0),
        ),
        StrokeOrFill::Fill,
    );
    stream.add_rectangle(
        Posn { x: 50.0, y: 500.0 },
        Dims {
            width: 100.0,
            height: 100.0,
        },
    );
    stream.fill(WindingRule::EvenOdd);

    stream.set_color_cmyk(
        CMYK::new(
         Color::new(0.5),
         Color::new(1.0),
         Color::new(0.0),
         Color::new(0.0),
        ),
        StrokeOrFill::Fill,
    );
    stream.add_rectangle(
        Posn { x: 200.0, y: 500.0 },
        Dims {
            width: 100.0,
            height: 100.0,
        },
    );
    stream.fill(WindingRule::EvenOdd);

    stream.set_color_cmyk(
        CMYK::new(
         Color::new(0.0),
         Color::new(0.0),
         Color::new(0.0),
         Color::new(1.0),
        ),
        StrokeOrFill::Fill,
    );
    // Title
    stream.begin_text();
    stream.set_font_name_and_size("Helvetica-Bold", 16.0);
    stream.set_text_position(Posn { x: 50.0, y: 780.0 });
    stream.show_single_text_string("CMYK Color Space Test");
    stream.end_text();

    // Description
    stream.begin_text();
    stream.set_font_name_and_size("Helvetica", 10.0);
    stream.set_text_position(Posn { x: 50.0, y: 765.0 });
    stream.show_single_text_string("Top row: Pure cyan (C:100%) | Pure magenta (M:100%) | Pure yellow (Y:100%)");
    stream.end_text();

    stream.begin_text();
    stream.set_font_name_and_size("Helvetica", 10.0);
    stream.set_text_position(Posn { x: 50.0, y: 750.0 });
    stream.show_single_text_string("Bottom row: Black (K:100%) | Blue (C:50%+M:100%)");
    stream.end_text();

    let content_index = pdf.add_indirect_object(Box::new(stream));
    let page = create_page_with_content(content_index);
    pdf.add_page(page);

    std::fs::create_dir_all("/tmp/pydyf_test").unwrap();
    let file = File::create("/tmp/pydyf_test/cmyk.pdf").unwrap();
    pdf.write_legacy(file, rusty_pdf::FileIdentifierMode::AutoMD5).unwrap();

    println!("✅ Generated: /tmp/pydyf_test/cmyk.pdf");
}
*/
/*#[test]
fn test_grayscale_colors() {
    let mut pdf = PdfFile::new();
    let mut stream = Stream::new();

    let _ = stream.set_color_grayscale(Color::new(0.0), StrokeOrFill::Fill);
    stream.add_rectangle(
        Posn { x: 50.0, y: 650.0 },
        Dims {
            width: 80.0,
            height: 80.0,
        },
    );
    stream.fill(WindingRule::EvenOdd);

    let _ = stream.set_color_grayscale(Color::new(0.25), StrokeOrFill::Fill);
    stream.add_rectangle(
        Posn { x: 150.0, y: 650.0 },
        Dims {
            width: 80.0,
            height: 80.0,
        },
    );
    stream.fill(WindingRule::EvenOdd);

    let _ = stream.set_color_grayscale(Color::new(0.5), StrokeOrFill::Fill);
    stream.add_rectangle(
        Posn { x: 250.0, y: 650.0 },
        Dims {
            width: 80.0,
            height: 80.0,
        },
    );
    stream.fill(WindingRule::EvenOdd);

    let _ = stream.set_color_grayscale(Color::new(0.75), StrokeOrFill::Fill);
    stream.add_rectangle(
        Posn { x: 350.0, y: 650.0 },
        Dims {
            width: 80.0,
            height: 80.0,
        },
    );
    stream.fill(WindingRule::EvenOdd);

    let _ = stream.set_color_grayscale(Color::new(1.0), StrokeOrFill::Fill);
    stream.add_rectangle(
        Posn { x: 450.0, y: 650.0 },
        Dims {
            width: 80.0,
            height: 80.0,
        },
    );
    stream.fill(WindingRule::EvenOdd);

    let _ = stream.set_color_grayscale(Color::new(0.0), StrokeOrFill::Stroke);
    stream.set_line_width(2.0);
    stream.add_rectangle(
        Posn { x: 450.0, y: 650.0 },
        Dims {
            width: 80.0,
            height: 80.0,
        },
    );
    stream.stroke_path();

    let _ = stream.set_color_grayscale(Color::new(0.0), StrokeOrFill::Fill);

    // Title
    stream.begin_text();
    stream.set_font_name_and_size("Helvetica-Bold", 16.0);
    stream.set_text_position(Posn { x: 50.0, y: 780.0 });
    stream.show_single_text_string("Grayscale Color Space Test");
    stream.end_text();

    // Description
    stream.begin_text();
    stream.set_font_name_and_size("Helvetica", 10.0);
    stream.set_text_position(Posn { x: 50.0, y: 765.0 });
    stream.show_single_text_string("Five shades from black (0%) to white (100%) in 25% increments");
    stream.end_text();

    stream.begin_text();
    stream.set_font_name_and_size("Helvetica", 10.0);
    stream.set_text_position(Posn { x: 50.0, y: 750.0 });
    stream.show_single_text_string("Black: 0% | Dark gray: 25% | Mid gray: 50% | Light gray: 75% | White: 100% (black border)");
    stream.end_text();

    let content_index = pdf.add_indirect_object(Box::new(stream));
    let page = create_page_with_content(content_index);
    pdf.add_page(page);

    std::fs::create_dir_all("/tmp/pydyf_test").unwrap();
    let file = File::create("/tmp/pydyf_test/gray.pdf").unwrap();
    pdf.write_legacy(file, rusty_pdf::FileIdentifierMode::AutoMD5).unwrap();

    println!("✅ Generated: /tmp/pydyf_test/gray.pdf");
}
*/
/*#[test]
fn test_mixed_color_spaces() {
    let mut pdf = PdfFile::new();
    let mut stream = Stream::new();

    stream.set_color_rgb(
        RGB::new(
            Color::new(1.0),
            Color::new(0.0),
            Color::new(0.0),
        ),
        StrokeOrFill::Fill,
    );
    stream.add_rectangle(
        Posn { x: 50.0, y: 650.0 },
        Dims {
            width: 100.0,
            height: 100.0,
        },
    );
    stream.fill(WindingRule::EvenOdd);

    stream.set_color_cmyk(
        CMYK::new(
         Color::new(0.0),
         Color::new(0.0),
         Color::new(0.0),
         Color::new(1.0),
        ),
        StrokeOrFill::Fill,
    );
    stream.add_rectangle(
        Posn { x: 200.0, y: 650.0 },
        Dims {
            width: 100.0,
            height: 100.0,
        },
    );
    stream.fill(WindingRule::EvenOdd);

    let _ = stream.set_color_grayscale(Color::new(0.5), StrokeOrFill::Fill);
    stream.add_rectangle(
        Posn { x: 350.0, y: 650.0 },
        Dims {
            width: 100.0,
            height: 100.0,
        },
    );
    stream.fill(WindingRule::EvenOdd);

    let _ = stream.set_color_grayscale(Color::new(0.0), StrokeOrFill::Fill);

    // Title
    stream.begin_text();
    stream.set_font_name_and_size("Helvetica-Bold", 16.0);
    stream.set_text_position(Posn { x: 50.0, y: 780.0 });
    stream.show_single_text_string("Mixed Color Spaces Test");
    stream.end_text();

    // Description
    stream.begin_text();
    stream.set_font_name_and_size("Helvetica", 11.0);
    stream.set_text_position(Posn { x: 50.0, y: 765.0 });
    stream.show_single_text_string("Demonstrates using RGB, CMYK, and Grayscale color spaces in one document");
    stream.end_text();

    // Labels
    stream.begin_text();
    stream.set_font_name_and_size("Helvetica", 10.0);
    stream.set_text_position(Posn { x: 70.0, y: 620.0 });
    stream.show_single_text_string("RGB Red");
    stream.set_text_position(Posn { x: 205.0, y: 620.0 });
    stream.show_single_text_string("CMYK Cyan");
    stream.set_text_position(Posn { x: 355.0, y: 620.0 });
    stream.show_single_text_string("Gray 50%");
    stream.end_text();

    let content_index = pdf.add_indirect_object(Box::new(stream));
    let page = create_page_with_content(content_index);
    pdf.add_page(page);

    std::fs::create_dir_all("/tmp/pydyf_test").unwrap();
    let file = File::create("/tmp/pydyf_test/mixed.pdf").unwrap();
    pdf.write_legacy(file, rusty_pdf::FileIdentifierMode::AutoMD5).unwrap();

    println!("✅ Generated: /tmp/pydyf_test/mixed.pdf");
}
*/
