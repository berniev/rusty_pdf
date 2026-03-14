use pydyf::color::{Color, RGB};
use pydyf::objects::stream::{EvenOdd, StrokeOrFill};
use pydyf::page::ObjectId;
use pydyf::util::{Dims, Matrix, Posn};
use pydyf::{FileIdentifierMode, PDF, PageObject, Stream};
use std::fs::File;

fn create_page_with_content(content_stream_ref: Vec<u8>) -> PageObject {
    let content_index = String::from_utf8(content_stream_ref).unwrap();
    // Extract just the number from "N 0 R" format
    let id_str = content_index.split_whitespace().next().unwrap();
    let id: u64 = id_str.parse().unwrap();
    
    PageObject::new(ObjectId::from(id))
}

#[test]
fn test_generate_simple_uncompressed_pdf() {
    let mut pdf = PDF::new();
    let mut stream = Stream::new();

    let _ = stream.set_color_rgb(
        RGB {
            red: Color { color: 1.0 },
            green: Color { color: 0.0 },
            blue: Color { color: 0.0 },
        },
        StrokeOrFill::Fill,
    );
    stream.rectangle(
        Posn { x: 100.0, y: 100.0 },
        Dims {
            width: 200.0,
            height: 150.0,
        },
    );
    stream.fill(EvenOdd::Odd);

    stream.begin_text();
    stream.set_font_name_and_size("Helvetica", 24.0);
    stream.set_text_matrix(Matrix {
        a: 1.0,
        b: 0.0,
        c: 0.0,
        d: 1.0,
        e: 100.0,
        f: 300.0,
    });
    stream.show_single_text_string("Hello PDF!");
    stream.end_text();

    pdf.add_object(Box::new(stream));
    let content_ref = format!("{} 0 R", pdf.objects.len() - 1).into_bytes();
    let page = create_page_with_content(content_ref);
    pdf.add_page(page);

    std::fs::create_dir_all("/tmp/pydyf_test").unwrap();
    let file = File::create("/tmp/pydyf_test/u.pdf").unwrap();
    pdf.write(file, FileIdentifierMode::AutoMD5).unwrap();

    println!("✅ Generated: /tmp/pydyf_test/u.pdf");
}

#[test]
fn test_generate_circle_over_rectangle() {
    let mut pdf = PDF::new();
    let mut stream = Stream::new();

    let _ = stream.set_color_rgb(
        RGB {
            red: Color { color: 0.0 },
            green: Color { color: 0.0 },
            blue: Color { color: 1.0 },
        },
        StrokeOrFill::Fill,
    );
    stream.rectangle(
        Posn { x: 50.0, y: 650.0 },
        Dims {
            width: 200.0,
            height: 100.0,
        },
    );
    stream.fill(EvenOdd::Odd);

    let _ = stream.set_color_rgb(
        RGB {
            red: Color { color: 0.0 },
            green: Color { color: 1.0 },
            blue: Color { color: 0.0 },
        },
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
    stream.fill(EvenOdd::Odd);

    pdf.add_object(Box::new(stream));
    let content_ref = format!("{} 0 R", pdf.objects.len() - 1).into_bytes();
    let page = create_page_with_content(content_ref);
    pdf.add_page(page);

    std::fs::create_dir_all("/tmp/pydyf_test").unwrap();
    let file = File::create("/tmp/pydyf_test/c.pdf").unwrap();
    pdf.write(file, FileIdentifierMode::AutoMD5).unwrap();

    println!("✅ Generated: /tmp/pydyf_test/c.pdf");
}

#[test]
fn test_multipage_pdf() {
    let mut pdf = PDF::new();

    let mut stream1 = Stream::new();
    let _ = stream1.set_color_rgb(
        RGB {
            red: Color { color: 1.0 },
            green: Color { color: 0.0 },
            blue: Color { color: 0.0 },
        },
        StrokeOrFill::Fill,
    );
    stream1.rectangle(
        Posn { x: 50.0, y: 650.0 },
        Dims {
            width: 200.0,
            height: 100.0,
        },
    );
    stream1.fill(EvenOdd::Odd);
    pdf.add_object(Box::new(stream1));
    let content_ref1 = format!("{} 0 R", pdf.objects.len() - 1).into_bytes();
    let page1 = create_page_with_content(content_ref1);
    pdf.add_page(page1);

    let mut stream2 = Stream::new();
    let _ = stream2.set_color_rgb(
        RGB {
            red: Color { color: 0.0 },
            green: Color { color: 1.0 },
            blue: Color { color: 0.0 },
        },
        StrokeOrFill::Fill,
    );
    stream2.rectangle(
        Posn { x: 150.0, y: 550.0 },
        Dims {
            width: 200.0,
            height: 100.0,
        },
    );
    stream2.fill(EvenOdd::Odd);
    pdf.add_object(Box::new(stream2));
    let content_ref2 = format!("{} 0 R", pdf.objects.len() - 1).into_bytes();
    let page2 = create_page_with_content(content_ref2);
    pdf.add_page(page2);

    let mut stream3 = Stream::new();
    let _ = stream3.set_color_rgb(
        RGB {
            red: Color { color: 0.0 },
            green: Color { color: 0.0 },
            blue: Color { color: 1.0 },
        },
        StrokeOrFill::Fill,
    );
    stream3.rectangle(
        Posn { x: 250.0, y: 450.0 },
        Dims {
            width: 200.0,
            height: 100.0,
        },
    );
    stream3.fill(EvenOdd::Odd);
    pdf.add_object(Box::new(stream3));
    let content_ref3 = format!("{} 0 R", pdf.objects.len() - 1).into_bytes();
    let page3 = create_page_with_content(content_ref3);
    pdf.add_page(page3);

    std::fs::create_dir_all("/tmp/pydyf_test").unwrap();
    let file = File::create("/tmp/pydyf_test/m.pdf").unwrap();
    pdf.write(file, FileIdentifierMode::AutoMD5).unwrap();

    println!("✅ Generated: /tmp/pydyf_test/m.pdf (3 pages)");
}

#[test]
fn test_graphics_operations() {
    let mut pdf = PDF::new();
    let mut stream = Stream::new();

    let _ = stream.set_color_rgb(
        RGB {
            red: Color { color: 1.0 },
            green: Color { color: 0.0 },
            blue: Color { color: 0.0 },
        },
        StrokeOrFill::Fill,
    );
    stream.rectangle(
        Posn { x: 50.0, y: 700.0 },
        Dims {
            width: 100.0,
            height: 50.0,
        },
    );
    stream.fill(EvenOdd::Odd);

    let _ = stream.set_color_rgb(
        RGB {
            red: Color { color: 0.0 },
            green: Color { color: 1.0 },
            blue: Color { color: 0.0 },
        },
        StrokeOrFill::Stroke,
    );
    stream.set_line_width(3.0);
    stream.rectangle(
        Posn { x: 200.0, y: 700.0 },
        Dims {
            width: 100.0,
            height: 50.0,
        },
    );
    stream.stroke_path();

    let _ = stream.set_color_rgb(
        RGB {
            red: Color { color: 1.0 },
            green: Color { color: 1.0 },
            blue: Color { color: 0.0 },
        },
        StrokeOrFill::Fill,
    );
    let _ = stream.set_color_rgb(
        RGB {
            red: Color { color: 0.0 },
            green: Color { color: 0.0 },
            blue: Color { color: 1.0 },
        },
        StrokeOrFill::Stroke,
    );
    stream.set_line_width(2.0);
    stream.rectangle(
        Posn { x: 350.0, y: 700.0 },
        Dims {
            width: 100.0,
            height: 50.0,
        },
    );
    stream.fill_and_stroke(EvenOdd::Odd);

    let _ = stream.set_color_rgb(
        RGB {
            red: Color { color: 0.0 },
            green: Color { color: 0.0 },
            blue: Color { color: 0.0 },
        },
        StrokeOrFill::Stroke,
    );
    stream.set_dash_line_pattern(&[5.0, 3.0, 1.0, 3.0], 0);
    stream.move_to_x_y(Posn { x: 50.0, y: 650.0 });
    stream.line_to_x_y(Posn { x: 450.0, y: 650.0 });
    stream.stroke_path();

    let _ = stream.set_color_rgb(
        RGB {
            red: Color { color: 1.0 },
            green: Color { color: 0.0 },
            blue: Color { color: 1.0 },
        },
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

    let _ = stream.set_color_rgb(
        RGB {
            red: Color { color: 0.0 },
            green: Color { color: 1.0 },
            blue: Color { color: 1.0 },
        },
        StrokeOrFill::Fill,
    );
    stream.move_to_x_y(Posn { x: 50.0, y: 500.0 });
    stream.line_to_x_y(Posn { x: 100.0, y: 550.0 });
    stream.line_to_x_y(Posn { x: 150.0, y: 500.0 });
    stream.close();
    stream.fill(EvenOdd::Odd);

    let _ = stream.set_color_rgb(
        RGB {
            red: Color { color: 1.0 },
            green: Color { color: 0.5 },
            blue: Color { color: 0.0 },
        },
        StrokeOrFill::Fill,
    );
    stream.move_to_x_y(Posn { x: 250.0, y: 550.0 });
    stream.line_to_x_y(Posn { x: 300.0, y: 550.0 });
    stream.line_to_x_y(Posn { x: 325.0, y: 500.0 });
    stream.line_to_x_y(Posn { x: 275.0, y: 500.0 });
    stream.close();
    stream.fill(EvenOdd::Odd);

    pdf.add_object(Box::new(stream));
    let content_ref = format!("{} 0 R", pdf.objects.len() - 1).into_bytes();
    let page = create_page_with_content(content_ref);
    pdf.add_page(page);

    std::fs::create_dir_all("/tmp/pydyf_test").unwrap();
    let file = File::create("/tmp/pydyf_test/g.pdf").unwrap();
    pdf.write(file, FileIdentifierMode::AutoMD5).unwrap();

    println!("✅ Generated: /tmp/pydyf_test/g.pdf");
}

#[test]
fn test_comparison_uncompressed() {
    let mut pdf = PDF::new();
    let mut stream = Stream::new();

    let _ = stream.set_color_rgb(
        RGB {
            red: Color { color: 0.9 },
            green: Color { color: 0.9 },
            blue: Color { color: 0.9 },
        },
        StrokeOrFill::Fill,
    );
    stream.rectangle(
        Posn { x: 50.0, y: 50.0 },
        Dims {
            width: 512.0,
            height: 692.0,
        },
    );
    stream.fill(EvenOdd::Odd);

    let _ = stream.set_color_rgb(
        RGB {
            red: Color { color: 0.0 },
            green: Color { color: 0.0 },
            blue: Color { color: 0.0 },
        },
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

    pdf.add_object(Box::new(stream));
    let content_ref = format!("{} 0 R", pdf.objects.len() - 1).into_bytes();
    let page = create_page_with_content(content_ref);
    pdf.add_page(page);

    std::fs::create_dir_all("/tmp/pydyf_test").unwrap();
    let file = File::create("/tmp/pydyf_test/cu.pdf").unwrap();
    pdf.write(file, FileIdentifierMode::AutoMD5).unwrap();

    println!("✅ Generated: /tmp/pydyf_test/cu.pdf");
}

#[test]
fn test_comparison_compressed() {
    let mut pdf = PDF::new();
    let mut stream = Stream::compressed();

    let _ = stream.set_color_rgb(
        RGB {
            red: Color { color: 0.9 },
            green: Color { color: 0.9 },
            blue: Color { color: 0.9 },
        },
        StrokeOrFill::Fill,
    );
    stream.rectangle(
        Posn { x: 50.0, y: 50.0 },
        Dims {
            width: 512.0,
            height: 692.0,
        },
    );
    stream.fill(EvenOdd::Odd);

    let _ = stream.set_color_rgb(
        RGB {
            red: Color { color: 0.0 },
            green: Color { color: 0.0 },
            blue: Color { color: 0.0 },
        },
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

    pdf.add_object(Box::new(stream));
    let content_ref = format!("{} 0 R", pdf.objects.len() - 1).into_bytes();
    let page = create_page_with_content(content_ref);
    pdf.add_page(page);

    std::fs::create_dir_all("/tmp/pydyf_test").unwrap();
    let file = File::create("/tmp/pydyf_test/cc.pdf").unwrap();
    pdf.write(file, FileIdentifierMode::AutoMD5).unwrap();

    println!("✅ Generated: /tmp/pydyf_test/cc.pdf");
}
