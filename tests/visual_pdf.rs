use pydyf::color::{Color, RGB};
use pydyf::objects::pdf_object::PdfObj;
use pydyf::objects::stream::{StrokeOrFill, WindingRule};
use pydyf::util::{Dims, Matrix, Posn};
use pydyf::{Pdf, Stream};
/*fn create_page_with_content(content_stream_ref: Vec<u8>) -> PageObject {
    let content_index = String::from_utf8(content_stream_ref).unwrap();
    // Extract just the number from "N 0 R" format
    let id_str = content_index.split_whitespace().next().unwrap();
    let id: u64 = id_str.parse().unwrap();

    let mut page = PageObject::new(0usize.into());
    page.add_content(id as usize);
    page
}

*/#[test]
fn test_generate_simple_uncompressed_pdf() {
    let _pdf = Pdf::new();
    let mut stream = Stream::new();

    stream.set_color_rgb(
        RGB::new(Color::new(1.0), Color::new(0.0), Color::new(0.0)),
        StrokeOrFill::Fill,
    );
    stream.add_rectangle(
        Posn { x: 100.0, y: 100.0 },
        Dims {
            width: 200.0,
            height: 150.0,
        },
    );
    stream.fill(WindingRule::EvenOdd);

    // Title
    stream.begin_text();
    stream.set_font_name_and_size("Helvetica-Bold", 18.0);
    stream.set_text_matrix(Matrix {
        a: 1.0,
        b: 0.0,
        c: 0.0,
        d: 1.0,
        e: 100.0,
        f: 550.0,
    });
    stream.show_single_text_string("Simple Uncompressed PDF Test");
    stream.end_text();

    // Description
    stream.begin_text();
    stream.set_font_name_and_size("Helvetica", 12.0);
    stream.set_text_matrix(Matrix {
        a: 1.0,
        b: 0.0,
        c: 0.0,
        d: 1.0,
        e: 100.0,
        f: 300.0,
    });
    stream.show_single_text_string("Red rectangle (200x150) with text");
    stream.end_text();

    stream.begin_text();
    stream.set_font_name_and_size("Helvetica", 11.0);
    stream.set_text_matrix(Matrix {
        a: 1.0,
        b: 0.0,
        c: 0.0,
        d: 1.0,
        e: 100.0,
        f: 280.0,
    });
    stream.show_single_text_string("Tests basic shapes and text rendering");
    stream.end_text();

 /*   let content_id = pdf.add_indirect_object(Box::new(stream));
    let content_ref = format!("{} 0 R", content_id).into_bytes();
    let page = create_page_with_content(content_ref);
    pdf.add_page(page);

    std::fs::create_dir_all("/tmp/pydyf_test").unwrap();
    let file = File::create("/tmp/pydyf_test/u.pdf").unwrap();
    pdf.write_legacy(file, FileIdentifierMode::AutoMD5).unwrap();

    println!("✅ Generated: /tmp/pydyf_test/u.pdf");
*/}

#[test]
fn test_generate_circle_over_rectangle() {
    let mut pdf = Pdf::new();
    let mut stream = Stream::new();

    stream.set_color_rgb(
        RGB::new(Color::new(0.0), Color::new(0.0), Color::new(1.0)),
        StrokeOrFill::Fill,
    );
    stream.add_rectangle(
        Posn { x: 50.0, y: 650.0 },
        Dims {
            width: 200.0,
            height: 100.0,
        },
    );
    stream.fill(WindingRule::EvenOdd);

    stream.set_color_rgb(
        RGB::new(Color::new(0.0), Color::new(1.0), Color::new(0.0)),
        StrokeOrFill::Fill,
    );
    stream.move_to_x_y(Posn { x: 150.0, y: 700.0 });
    stream.curve_to(
        Posn { x: 150.0, y: 727.6 },
        Posn { x: 127.6, y: 750.0 },
        Posn { x: 100.0, y: 750.0 },
    );
    stream.curve_to(
        Posn { x: 72.4, y: 750.0 },
        Posn { x: 50.0, y: 727.6 },
        Posn { x: 50.0, y: 700.0 },
    );
    stream.curve_to(
        Posn { x: 50.0, y: 672.4 },
        Posn { x: 72.4, y: 650.0 },
        Posn { x: 100.0, y: 650.0 },
    );
    stream.curve_to(
        Posn { x: 127.6, y: 650.0 },
        Posn { x: 150.0, y: 672.4 },
        Posn { x: 150.0, y: 700.0 },
    );
    stream.close();
    stream.fill(WindingRule::EvenOdd);

    // Add title
    stream.begin_text();
    stream.set_font_name_and_size("Helvetica-Bold", 16.0);
    stream.set_text_matrix(Matrix {
        a: 1.0,
        b: 0.0,
        c: 0.0,
        d: 1.0,
        e: 50.0,
        f: 580.0,
    });
    stream.show_single_text_string("Overlapping Shapes Test");
    stream.end_text();

    // Add description
    stream.begin_text();
    stream.set_font_name_and_size("Helvetica", 11.0);
    stream.set_text_matrix(Matrix {
        a: 1.0,
        b: 0.0,
        c: 0.0,
        d: 1.0,
        e: 50.0,
        f: 560.0,
    });
    stream.show_single_text_string("Blue rectangle with green circle on top");
    stream.end_text();

    stream.begin_text();
    stream.set_font_name_and_size("Helvetica", 10.0);
    stream.set_text_matrix(Matrix {
        a: 1.0,
        b: 0.0,
        c: 0.0,
        d: 1.0,
        e: 50.0,
        f: 545.0,
    });
    stream.show_single_text_string("Tests Bezier curves for circular paths");
    stream.end_text();

    let content_id = pdf.save_indirect_object(PdfObj::stream(stream));
    let _content_ref = format!("{} 0 R", content_id).into_bytes();
/*    let page = create_page_with_content(content_ref);
    pdf.add_page(page);

    std::fs::create_dir_all("/tmp/pydyf_test").unwrap();
    let file = File::create("/tmp/pydyf_test/c.pdf").unwrap();
    pdf.write_legacy(file, FileIdentifierMode::AutoMD5).unwrap();

    println!("✅ Generated: /tmp/pydyf_test/c.pdf");
*/}

#[test]
fn test_multipage_pdf() {
    let mut pdf = Pdf::new();

    let mut stream1 = Stream::new();
    stream1.set_color_rgb(
        RGB::new(Color::new(1.0), Color::new(0.0), Color::new(0.0)),
        StrokeOrFill::Fill,
    );
    stream1.add_rectangle(
        Posn { x: 50.0, y: 650.0 },
        Dims {
            width: 200.0,
            height: 100.0,
        },
    );
    stream1.fill(WindingRule::EvenOdd);

    stream1.begin_text();
    stream1.set_font_name_and_size("Helvetica-Bold", 20.0);
    stream1.set_text_matrix(Matrix {
        a: 1.0,
        b: 0.0,
        c: 0.0,
        d: 1.0,
        e: 50.0,
        f: 550.0,
    });
    stream1.show_single_text_string("Page 1 of 3");
    stream1.end_text();

    stream1.begin_text();
    stream1.set_font_name_and_size("Helvetica", 12.0);
    stream1.set_text_matrix(Matrix {
        a: 1.0,
        b: 0.0,
        c: 0.0,
        d: 1.0,
        e: 50.0,
        f: 530.0,
    });
    stream1.show_single_text_string("Red rectangle - Tests multi-page PDF generation");
    stream1.end_text();

    let content_id1 = pdf.save_indirect_object(PdfObj::stream(stream1));
    let _content_ref1 = format!("{} 0 R", content_id1).into_bytes();
 /*   let page1 = create_page_with_content(content_ref1);
    pdf.add_page(page1);

    let mut stream2 = Stream::new();
    stream2.set_color_rgb(
        RGB::new(Color::new(0.0), Color::new(1.0), Color::new(0.0)),
        StrokeOrFill::Fill,
    );
    stream2.add_rectangle(
        Posn { x: 150.0, y: 550.0 },
        Dims {
            width: 200.0,
            height: 100.0,
        },
    );
    stream2.fill(WindingRule::EvenOdd);

    stream2.begin_text();
    stream2.set_font_name_and_size("Helvetica-Bold", 20.0);
    stream2.set_text_matrix(Matrix {
        a: 1.0,
        b: 0.0,
        c: 0.0,
        d: 1.0,
        e: 150.0,
        f: 450.0,
    });
    stream2.show_single_text_string("Page 2 of 3");
    stream2.end_text();

    stream2.begin_text();
    stream2.set_font_name_and_size("Helvetica", 12.0);
    stream2.set_text_matrix(Matrix {
        a: 1.0,
        b: 0.0,
        c: 0.0,
        d: 1.0,
        e: 150.0,
        f: 430.0,
    });
    stream2.show_single_text_string("Green rectangle");
    stream2.end_text();

    let content_id2 = pdf.add_indirect_object(Box::new(stream2));
    let content_ref2 = format!("{} 0 R", content_id2).into_bytes();
    let page2 = create_page_with_content(content_ref2);
    pdf.add_page(page2);

    let mut stream3 = Stream::new();
    stream3.set_color_rgb(
        RGB::new(Color::new(0.0), Color::new(0.0), Color::new(1.0)),
        StrokeOrFill::Fill,
    );
    stream3.add_rectangle(
        Posn { x: 250.0, y: 450.0 },
        Dims {
            width: 200.0,
            height: 100.0,
        },
    );
    stream3.fill(WindingRule::EvenOdd);

    stream3.begin_text();
    stream3.set_font_name_and_size("Helvetica-Bold", 20.0);
    stream3.set_text_matrix(Matrix {
        a: 1.0,
        b: 0.0,
        c: 0.0,
        d: 1.0,
        e: 250.0,
        f: 350.0,
    });
    stream3.show_single_text_string("Page 3 of 3");
    stream3.end_text();

    stream3.begin_text();
    stream3.set_font_name_and_size("Helvetica", 12.0);
    stream3.set_text_matrix(Matrix {
        a: 1.0,
        b: 0.0,
        c: 0.0,
        d: 1.0,
        e: 250.0,
        f: 330.0,
    });
    stream3.show_single_text_string("Blue rectangle");
    stream3.end_text();

    let content_id3 = pdf.add_indirect_object(Box::new(stream3));
    let content_ref3 = format!("{} 0 R", content_id3).into_bytes();
    let page3 = create_page_with_content(content_ref3);
    pdf.add_page(page3);

    std::fs::create_dir_all("/tmp/pydyf_test").unwrap();
    let file = File::create("/tmp/pydyf_test/m.pdf").unwrap();
    pdf.write_legacy(file, FileIdentifierMode::AutoMD5).unwrap();

    println!("✅ Generated: /tmp/pydyf_test/m.pdf (3 pages)");
*/}

#[test]
fn test_graphics_operations() {
    let mut pdf = Pdf::new();
    let mut stream = Stream::new();

    stream.set_color_rgb(
        RGB::new(Color::new(1.0), Color::new(0.0), Color::new(0.0)),
        StrokeOrFill::Fill,
    );
    stream.add_rectangle(
        Posn { x: 50.0, y: 700.0 },
        Dims {
            width: 100.0,
            height: 50.0,
        },
    );
    stream.fill(WindingRule::EvenOdd);

    stream.set_color_rgb(
        RGB::new(Color::new(0.0), Color::new(1.0), Color::new(0.0)),
        StrokeOrFill::Stroke,
    );
    stream.set_line_width(3.0);
    stream.add_rectangle(
        Posn { x: 200.0, y: 700.0 },
        Dims {
            width: 100.0,
            height: 50.0,
        },
    );
    stream.stroke_path();

    stream.set_color_rgb(
        RGB::new(Color::new(1.0), Color::new(1.0), Color::new(0.0)),
        StrokeOrFill::Fill,
    );
    stream.set_color_rgb(
        RGB::new(Color::new(0.0), Color::new(0.0), Color::new(1.0)),
        StrokeOrFill::Stroke,
    );
    stream.set_line_width(2.0);
    stream.add_rectangle(
        Posn { x: 350.0, y: 700.0 },
        Dims {
            width: 100.0,
            height: 50.0,
        },
    );
    stream.fill_and_stroke(WindingRule::EvenOdd);

    stream.set_color_rgb(
        RGB::new(Color::new(0.0), Color::new(0.0), Color::new(0.0)),
        StrokeOrFill::Stroke,
    );
    stream.set_dash_line_pattern(&[5.0, 3.0, 1.0, 3.0], 0);
    stream.move_to_x_y(Posn { x: 50.0, y: 650.0 });
    stream.line_to_x_y(Posn { x: 450.0, y: 650.0 });
    stream.stroke_path();

    stream.set_color_rgb(
        RGB::new(Color::new(1.0), Color::new(0.0), Color::new(1.0)),
        StrokeOrFill::Stroke,
    );
    stream.set_dash_line_pattern(&[], 0);
    stream.move_to_x_y(Posn { x: 50.0, y: 600.0 });
    stream.curve_to(
        Posn { x: 150.0, y: 650.0 },
        Posn { x: 200.0, y: 550.0 },
        Posn { x: 300.0, y: 600.0 },
    );
    stream.stroke_path();

    stream.set_color_rgb(
        RGB::new(Color::new(0.0), Color::new(1.0), Color::new(1.0)),
        StrokeOrFill::Fill,
    );
    stream.move_to_x_y(Posn { x: 50.0, y: 500.0 });
    stream.line_to_x_y(Posn { x: 100.0, y: 550.0 });
    stream.line_to_x_y(Posn { x: 150.0, y: 500.0 });
    stream.close();
    stream.fill(WindingRule::EvenOdd);

    stream.set_color_rgb(
        RGB::new(Color::new(1.0), Color::new(0.5), Color::new(0.0)),
        StrokeOrFill::Fill,
    );
    stream.move_to_x_y(Posn { x: 250.0, y: 550.0 });
    stream.line_to_x_y(Posn { x: 300.0, y: 550.0 });
    stream.line_to_x_y(Posn { x: 325.0, y: 500.0 });
    stream.line_to_x_y(Posn { x: 275.0, y: 500.0 });
    stream.close();
    stream.fill(WindingRule::EvenOdd);

    // Add title
    stream.begin_text();
    stream.set_font_name_and_size("Helvetica-Bold", 16.0);
    stream.set_text_matrix(Matrix {
        a: 1.0,
        b: 0.0,
        c: 0.0,
        d: 1.0,
        e: 50.0,
        f: 780.0,
    });
    stream.show_single_text_string("Graphics Operations Test");
    stream.end_text();

    // Add descriptions
    stream.begin_text();
    stream.set_font_name_and_size("Helvetica", 9.0);
    stream.set_text_matrix(Matrix {
        a: 1.0,
        b: 0.0,
        c: 0.0,
        d: 1.0,
        e: 50.0,
        f: 765.0,
    });
    stream.show_single_text_string(
        "Red filled rect | Green stroked rect | Yellow filled+blue stroked rect",
    );
    stream.end_text();

    stream.begin_text();
    stream.set_font_name_and_size("Helvetica", 9.0);
    stream.set_text_matrix(Matrix {
        a: 1.0,
        b: 0.0,
        c: 0.0,
        d: 1.0,
        e: 50.0,
        f: 665.0,
    });
    stream.show_single_text_string("Black dashed line pattern | Magenta Bezier curve");
    stream.end_text();

    stream.begin_text();
    stream.set_font_name_and_size("Helvetica", 9.0);
    stream.set_text_matrix(Matrix {
        a: 1.0,
        b: 0.0,
        c: 0.0,
        d: 1.0,
        e: 50.0,
        f: 565.0,
    });
    stream.show_single_text_string("Cyan triangle | Orange trapezoid");
    stream.end_text();

    let content_id = pdf.save_indirect_object(PdfObj::stream(stream));
    let _content_ref = format!("{} 0 R", content_id).into_bytes();
 /*   let page = create_page_with_content(content_ref);
    pdf.add_page(page);

    std::fs::create_dir_all("/tmp/pydyf_test").unwrap();
    let file = File::create("/tmp/pydyf_test/g.pdf").unwrap();
    pdf.write_legacy(file, FileIdentifierMode::AutoMD5).unwrap();

    println!("✅ Generated: /tmp/pydyf_test/g.pdf");
*/}

#[test]
fn test_comparison_uncompressed() {
    let mut pdf = Pdf::new();
    let mut stream = Stream::new();

    stream.set_color_rgb(
        RGB::new(Color::new(0.9), Color::new(0.9), Color::new(0.9)),
        StrokeOrFill::Fill,
    );
    stream.add_rectangle(
        Posn { x: 50.0, y: 50.0 },
        Dims {
            width: 512.0,
            height: 692.0,
        },
    );
    stream.fill(WindingRule::EvenOdd);

    stream.set_color_rgb(
        RGB::new(Color::new(0.0), Color::new(0.0), Color::new(0.0)),
        StrokeOrFill::Fill,
    );
    stream.begin_text();
    stream.set_font_name_and_size("Courier", 10.0);

    for i in 0..29 {
        stream.set_text_matrix(Matrix {
            a: 1.0,
            b: 0.0,
            c: 0.0,
            d: 1.0,
            e: 60.0,
            f: 700.0 - (i as f64 * 20.0),
        });
        stream.show_single_text_string(&format!("Line {} - Testing PDF generation", i + 1));
    }

    stream.end_text();

    let content_id = pdf.save_indirect_object(PdfObj::stream(stream));
    let _content_ref = format!("{} 0 R", content_id).into_bytes();
/*    let page = create_page_with_content(content_ref);
    pdf.add_page(page);

    std::fs::create_dir_all("/tmp/pydyf_test").unwrap();
    let file = File::create("/tmp/pydyf_test/cu.pdf").unwrap();
    pdf.write_legacy(file, FileIdentifierMode::AutoMD5).unwrap();

    println!("✅ Generated: /tmp/pydyf_test/cu.pdf");
*/}

#[test]
fn test_comparison_compressed() {
    let mut pdf = Pdf::new();
    let mut stream = Stream::new().compressed();

    stream.set_color_rgb(
        RGB::new(Color::new(0.9), Color::new(0.9), Color::new(0.9)),
        StrokeOrFill::Fill,
    );
    stream.add_rectangle(
        Posn { x: 50.0, y: 50.0 },
        Dims {
            width: 512.0,
            height: 692.0,
        },
    );
    stream.fill(WindingRule::EvenOdd);

    stream.set_color_rgb(
        RGB::new(Color::new(0.0), Color::new(0.0), Color::new(0.0)),
        StrokeOrFill::Fill,
    );
    stream.begin_text();
    stream.set_font_name_and_size("Courier", 10.0);

    for i in 0..29 {
        stream.set_text_matrix(Matrix {
            a: 1.0,
            b: 0.0,
            c: 0.0,
            d: 1.0,
            e: 60.0,
            f: 700.0 - (i as f64 * 20.0),
        });
        stream.show_single_text_string(&format!("Line {} - Testing PDF generation", i + 1));
    }

    stream.end_text();

    let content_id = pdf.save_indirect_object(PdfObj::stream(stream));
    let _content_ref = format!("{} 0 R", content_id).into_bytes();
/*    let page = create_page_with_content(content_ref);
    pdf.add_page(page);

    std::fs::create_dir_all("/tmp/pydyf_test").unwrap();
    let file = File::create("/tmp/pydyf_test/cc.pdf").unwrap();
    pdf.write_legacy(file, FileIdentifierMode::AutoMD5).unwrap();

    println!("✅ Generated: /tmp/pydyf_test/cc.pdf");
*/}
