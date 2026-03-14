use pydyf::color::{CMYK, Color, RGB};
use pydyf::objects::stream::{EvenOdd, StrokeOrFill};
use pydyf::util::{Dims, Matrix, Posn};
use pydyf::{PDF, PageObject, Stream};
use std::fs::File;

fn create_page_with_content(content_index: usize) -> PageObject {
    let mut page = PageObject::new(0usize.into());
    page.add_content(content_index);
    page
}

#[test]
fn test_cmyk_colors() {
    let mut pdf = PDF::new();
    let mut stream = Stream::new();

    let _ = stream.set_color_cmyk(
        CMYK {
            cyan: Color { color: 1.0 },
            magenta: Color { color: 0.0 },
            yellow: Color { color: 0.0 },
            black: Color { color: 0.0 },
        },
        StrokeOrFill::Fill,
    );
    stream.rectangle(
        Posn { x: 50.0, y: 650.0 },
        Dims {
            width: 100.0,
            height: 100.0,
        },
    );
    stream.fill(EvenOdd::Odd);

    let _ = stream.set_color_cmyk(
        CMYK {
            cyan: Color { color: 0.0 },
            magenta: Color { color: 1.0 },
            yellow: Color { color: 0.0 },
            black: Color { color: 0.0 },
        },
        StrokeOrFill::Fill,
    );
    stream.rectangle(
        Posn { x: 200.0, y: 650.0 },
        Dims {
            width: 100.0,
            height: 100.0,
        },
    );
    stream.fill(EvenOdd::Odd);

    let _ = stream.set_color_cmyk(
        CMYK {
            cyan: Color { color: 0.0 },
            magenta: Color { color: 0.0 },
            yellow: Color { color: 1.0 },
            black: Color { color: 0.0 },
        },
        StrokeOrFill::Fill,
    );
    stream.rectangle(
        Posn { x: 350.0, y: 650.0 },
        Dims {
            width: 100.0,
            height: 100.0,
        },
    );
    stream.fill(EvenOdd::Odd);

    let _ = stream.set_color_cmyk(
        CMYK {
            cyan: Color { color: 0.0 },
            magenta: Color { color: 0.0 },
            yellow: Color { color: 0.0 },
            black: Color { color: 1.0 },
        },
        StrokeOrFill::Fill,
    );
    stream.rectangle(
        Posn { x: 50.0, y: 500.0 },
        Dims {
            width: 100.0,
            height: 100.0,
        },
    );
    stream.fill(EvenOdd::Odd);

    let _ = stream.set_color_cmyk(
        CMYK {
            cyan: Color { color: 0.5 },
            magenta: Color { color: 1.0 },
            yellow: Color { color: 0.0 },
            black: Color { color: 0.0 },
        },
        StrokeOrFill::Fill,
    );
    stream.rectangle(
        Posn { x: 200.0, y: 500.0 },
        Dims {
            width: 100.0,
            height: 100.0,
        },
    );
    stream.fill(EvenOdd::Odd);

    let _ = stream.set_color_cmyk(
        CMYK {
            cyan: Color { color: 0.0 },
            magenta: Color { color: 0.0 },
            yellow: Color { color: 0.0 },
            black: Color { color: 1.0 },
        },
        StrokeOrFill::Fill,
    );
    stream.begin_text();
    stream.set_font_name_and_size("Helvetica", 12.0);
    stream.set_text_matrix(Matrix::new(1.0, 0.0, 0.0, 1.0, 50.0, 630.0));
    stream.show_single_text_string("CMYK Colors");
    stream.end_text();

    let content_index = pdf.add_object(Box::new(stream));
    let page = create_page_with_content(content_index);
    pdf.add_page(page);

    std::fs::create_dir_all("/tmp/pydyf_test").unwrap();
    let file = File::create("/tmp/pydyf_test/cmyk.pdf").unwrap();
    pdf.write(file, pydyf::FileIdentifierMode::AutoMD5).unwrap();

    println!("✅ Generated: /tmp/pydyf_test/cmyk.pdf");
}

#[test]
fn test_grayscale_colors() {
    let mut pdf = PDF::new();
    let mut stream = Stream::new();

    let _ = stream.set_color_grayscale(Color { color: 0.0 }, StrokeOrFill::Fill);
    stream.rectangle(
        Posn { x: 50.0, y: 650.0 },
        Dims {
            width: 80.0,
            height: 80.0,
        },
    );
    stream.fill(EvenOdd::Odd);

    let _ = stream.set_color_grayscale(Color { color: 0.25 }, StrokeOrFill::Fill);
    stream.rectangle(
        Posn { x: 150.0, y: 650.0 },
        Dims {
            width: 80.0,
            height: 80.0,
        },
    );
    stream.fill(EvenOdd::Odd);

    let _ = stream.set_color_grayscale(Color { color: 0.5 }, StrokeOrFill::Fill);
    stream.rectangle(
        Posn { x: 250.0, y: 650.0 },
        Dims {
            width: 80.0,
            height: 80.0,
        },
    );
    stream.fill(EvenOdd::Odd);

    let _ = stream.set_color_grayscale(Color { color: 0.75 }, StrokeOrFill::Fill);
    stream.rectangle(
        Posn { x: 350.0, y: 650.0 },
        Dims {
            width: 80.0,
            height: 80.0,
        },
    );
    stream.fill(EvenOdd::Odd);

    let _ = stream.set_color_grayscale(Color { color: 1.0 }, StrokeOrFill::Fill);
    stream.rectangle(
        Posn { x: 450.0, y: 650.0 },
        Dims {
            width: 80.0,
            height: 80.0,
        },
    );
    stream.fill(EvenOdd::Odd);

    let _ = stream.set_color_grayscale(Color { color: 0.0 }, StrokeOrFill::Stroke);
    stream.set_line_width(2.0);
    stream.rectangle(
        Posn { x: 450.0, y: 650.0 },
        Dims {
            width: 80.0,
            height: 80.0,
        },
    );
    stream.stroke_path();

    let _ = stream.set_color_grayscale(Color { color: 0.0 }, StrokeOrFill::Fill);
    stream.begin_text();
    stream.set_font_name_and_size("Helvetica", 12.0);
    stream.set_text_matrix(Matrix::new(1.0, 0.0, 0.0, 1.0, 50.0, 630.0));
    stream.show_single_text_string("Grayscale: Black to White");
    stream.end_text();

    let content_index = pdf.add_object(Box::new(stream));
    let page = create_page_with_content(content_index);
    pdf.add_page(page);

    std::fs::create_dir_all("/tmp/pydyf_test").unwrap();
    let file = File::create("/tmp/pydyf_test/gray.pdf").unwrap();
    pdf.write(file, pydyf::FileIdentifierMode::AutoMD5).unwrap();

    println!("✅ Generated: /tmp/pydyf_test/gray.pdf");
}

#[test]
fn test_mixed_color_spaces() {
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
        Posn { x: 50.0, y: 650.0 },
        Dims {
            width: 100.0,
            height: 100.0,
        },
    );
    stream.fill(EvenOdd::Odd);

    let _ = stream.set_color_cmyk(
        CMYK {
            cyan: Color { color: 1.0 },
            magenta: Color { color: 0.0 },
            yellow: Color { color: 0.0 },
            black: Color { color: 0.0 },
        },
        StrokeOrFill::Fill,
    );
    stream.rectangle(
        Posn { x: 200.0, y: 650.0 },
        Dims {
            width: 100.0,
            height: 100.0,
        },
    );
    stream.fill(EvenOdd::Odd);

    let _ = stream.set_color_grayscale(Color { color: 0.5 }, StrokeOrFill::Fill);
    stream.rectangle(
        Posn { x: 350.0, y: 650.0 },
        Dims {
            width: 100.0,
            height: 100.0,
        },
    );
    stream.fill(EvenOdd::Odd);

    let _ = stream.set_color_grayscale(Color { color: 0.0 }, StrokeOrFill::Fill);
    stream.begin_text();
    stream.set_font_name_and_size("Helvetica", 10.0);
    stream.set_text_matrix(Matrix::new(1.0, 0.0, 0.0, 1.0, 70.0, 620.0));
    stream.show_single_text_string("RGB");
    stream.set_text_matrix(Matrix::new(1.0, 0.0, 0.0, 1.0, 215.0, 620.0));
    stream.show_single_text_string("CMYK");
    stream.set_text_matrix(Matrix::new(1.0, 0.0, 0.0, 1.0, 365.0, 620.0));
    stream.show_single_text_string("Gray");
    stream.end_text();

    let content_index = pdf.add_object(Box::new(stream));
    let page = create_page_with_content(content_index);
    pdf.add_page(page);

    std::fs::create_dir_all("/tmp/pydyf_test").unwrap();
    let file = File::create("/tmp/pydyf_test/mixed.pdf").unwrap();
    pdf.write(file, pydyf::FileIdentifierMode::AutoMD5).unwrap();

    println!("✅ Generated: /tmp/pydyf_test/mixed.pdf");
}
