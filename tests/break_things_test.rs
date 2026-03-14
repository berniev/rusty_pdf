use std::fs::File;

use pydyf::StreamObject;
use pydyf::color::{Color, RGB};
use pydyf::objects::stream::{EvenOdd, StrokeOrFill};
use pydyf::page::PageSize;
use pydyf::util::{Dims, Matrix, Posn};
use pydyf::{PDF, PageObject};

fn create_page_with_content(page_size: PageSize, content_index: usize) -> PageObject {
    let mut page = PageObject::new(0usize.into());
    page.add_content(content_index);
    page.set_media_box(page_size);
    page
}

#[test]
fn test_empty_page() {
    let mut pdf = PDF::new();
    let stream = StreamObject::new();

    let content_id = pdf.add_object(Box::new(stream));

    let mut page = PageObject::new(0usize.into());
    page.add_content(content_id);
    page.set_media_box(PageSize::A4);
    pdf.add_page(page);

    std::fs::create_dir_all("/tmp/pydyf_test").unwrap();
    let file = File::create("/tmp/pydyf_test/break_empty.pdf").unwrap();
    pdf.write(file, pydyf::FileIdentifierMode::AutoMD5).unwrap();
}

#[test]
fn test_massive_page_count() {
    let mut pdf = PDF::new();

    for _ in 0..500 {
        let mut stream = StreamObject::new();
        let _ = stream.set_color_rgb(
            RGB {
                red: Color { color: 0.5 },
                green: Color { color: 0.5 },
                blue: Color { color: 0.5 },
            },
            StrokeOrFill::Fill,
        );
        stream.rectangle(
            Posn { x: 50.0, y: 50.0 },
            Dims {
                width: 100.0,
                height: 100.0,
            },
        );
        stream.fill(EvenOdd::Odd);

        let content_index = pdf.add_object(Box::new(stream));
        let page = create_page_with_content(PageSize::A4, content_index);
        pdf.add_page(page);
    }

    std::fs::create_dir_all("/tmp/pydyf_test").unwrap();
    let file = File::create("/tmp/pydyf_test/break_massive.pdf").unwrap();
    pdf.write(file, pydyf::FileIdentifierMode::AutoMD5).unwrap();
}

#[test]
fn test_extreme_coordinates() {
    let mut pdf = PDF::new();
    let mut stream = StreamObject::new();

    let _ = stream.set_color_rgb(
        RGB {
            red: Color { color: 1.0 },
            green: Color { color: 0.0 },
            blue: Color { color: 0.0 },
        },
        StrokeOrFill::Fill,
    );
    stream.rectangle(
        Posn {
            x: -1000.0,
            y: -1000.0,
        },
        Dims {
            width: 100.0,
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
    stream.rectangle(
        Posn {
            x: 10000.0,
            y: 10000.0,
        },
        Dims {
            width: 100.0,
            height: 100.0,
        },
    );
    stream.fill(EvenOdd::Odd);

    let _ = stream.set_color_rgb(
        RGB {
            red: Color { color: 0.0 },
            green: Color { color: 0.0 },
            blue: Color { color: 1.0 },
        },
        StrokeOrFill::Fill,
    );
    stream.rectangle(
        Posn { x: 0.0, y: 0.0 },
        Dims {
            width: 0.0,
            height: 0.0,
        },
    );
    stream.fill(EvenOdd::Odd);

    let _ = stream.set_color_rgb(
        RGB {
            red: Color { color: 0.5 },
            green: Color { color: 0.5 },
            blue: Color { color: 0.5 },
        },
        StrokeOrFill::Fill,
    );
    stream.rectangle(
        Posn { x: 100.0, y: 100.0 },
        Dims {
            width: 10000.0,
            height: 10000.0,
        },
    );
    stream.fill(EvenOdd::Odd);

    let content_index = pdf.add_object(Box::new(stream));
    let page = create_page_with_content(PageSize::A4, content_index);
    pdf.add_page(page);

    std::fs::create_dir_all("/tmp/pydyf_test").unwrap();
    let file = File::create("/tmp/pydyf_test/break_coords.pdf").unwrap();
    pdf.write(file, pydyf::FileIdentifierMode::AutoMD5).unwrap();
}

#[test]
fn test_very_long_text() {
    let mut pdf = PDF::new();
    let mut stream = StreamObject::new();

    let _ = stream.set_color_rgb(
        RGB {
            red: Color { color: 0.0 },
            green: Color { color: 0.0 },
            blue: Color { color: 0.0 },
        },
        StrokeOrFill::Fill,
    );
    stream.begin_text();
    stream.set_font_name_and_size("Courier", 8.0);

    for i in 0..1000 {
        stream.set_text_matrix(Matrix {
            a: 1.0,
            b: 0.0,
            c: 0.0,
            d: 1.0,
            e: 50.0,
            f: 700.0 - (i as f64 * 10.0),
        });
        stream.show_single_text_string(&format!("Line {}", i));
    }

    stream.end_text();

    let content_index = pdf.add_object(Box::new(stream));
    let page = create_page_with_content(PageSize::A4, content_index);
    pdf.add_page(page);

    std::fs::create_dir_all("/tmp/pydyf_test").unwrap();
    let file = File::create("/tmp/pydyf_test/break_longtext.pdf").unwrap();
    pdf.write(file, pydyf::FileIdentifierMode::AutoMD5).unwrap();
}

#[test]
fn test_special_characters_text() {
    let mut pdf = PDF::new();
    let mut stream = StreamObject::new();

    let _ = stream.set_color_rgb(
        RGB {
            red: Color { color: 0.0 },
            green: Color { color: 0.0 },
            blue: Color { color: 0.0 },
        },
        StrokeOrFill::Fill,
    );
    stream.begin_text();
    stream.set_font_name_and_size("Helvetica", 12.0);

    stream.set_text_matrix(Matrix {
        a: 1.0,
        b: 0.0,
        c: 0.0,
        d: 1.0,
        e: 50.0,
        f: 700.0,
    });
    stream.show_single_text_string("Parentheses: (test)");

    stream.set_text_matrix(Matrix {
        a: 1.0,
        b: 0.0,
        c: 0.0,
        d: 1.0,
        e: 50.0,
        f: 680.0,
    });
    stream.show_single_text_string("Backslash: \\ test");

    stream.set_text_matrix(Matrix {
        a: 1.0,
        b: 0.0,
        c: 0.0,
        d: 1.0,
        e: 50.0,
        f: 660.0,
    });
    stream.show_single_text_string("Quotes: \"test\"");

    stream.set_text_matrix(Matrix {
        a: 1.0,
        b: 0.0,
        c: 0.0,
        d: 1.0,
        e: 50.0,
        f: 640.0,
    });
    stream.show_single_text_string("Mixed: (\\) \"test\" \\n");

    stream.end_text();

    let content_index = pdf.add_object(Box::new(stream));
    let page = create_page_with_content(PageSize::A4, content_index);
    pdf.add_page(page);

    std::fs::create_dir_all("/tmp/pydyf_test").unwrap();
    let file = File::create("/tmp/pydyf_test/break_special_chars.pdf").unwrap();
    pdf.write(file, pydyf::FileIdentifierMode::AutoMD5).unwrap();
}

#[test]
fn test_huge_rectangle() {
    let mut pdf = PDF::new();
    let mut stream = StreamObject::new();

    let _ = stream.set_color_rgb(
        RGB {
            red: Color { color: 1.0 },
            green: Color { color: 0.0 },
            blue: Color { color: 0.0 },
        },
        StrokeOrFill::Fill,
    );
    stream.rectangle(
        Posn { x: 0.0, y: 0.0 },
        Dims {
            width: 5000.0,
            height: 5000.0,
        },
    );
    stream.fill(EvenOdd::Odd);

    let content_index = pdf.add_object(Box::new(stream));
    let page = create_page_with_content(PageSize::A4, content_index);
    pdf.add_page(page);

    std::fs::create_dir_all("/tmp/pydyf_test").unwrap();
    let file = File::create("/tmp/pydyf_test/break_huge.pdf").unwrap();
    pdf.write(file, pydyf::FileIdentifierMode::AutoMD5).unwrap();
}

#[test]
fn test_compressed_empty() {
    let mut pdf = PDF::new();
    let stream = StreamObject::compressed();

    let content_index = pdf.add_object(Box::new(stream));
    let page = create_page_with_content(PageSize::A4, content_index);
    pdf.add_page(page);

    std::fs::create_dir_all("/tmp/pydyf_test").unwrap();
    let file = File::create("/tmp/pydyf_test/break_compressed_empty.pdf").unwrap();
    pdf.write(file, pydyf::FileIdentifierMode::AutoMD5).unwrap();
}

#[test]
fn test_extreme_font_sizes() {
    let mut pdf = PDF::new();
    let mut stream = StreamObject::new();

    let _ = stream.set_color_rgb(
        RGB {
            red: Color { color: 0.0 },
            green: Color { color: 0.0 },
            blue: Color { color: 0.0 },
        },
        StrokeOrFill::Fill,
    );
    stream.begin_text();

    stream.set_font_name_and_size("Helvetica", 0.1);
    stream.set_text_matrix(Matrix {
        a: 1.0,
        b: 0.0,
        c: 0.0,
        d: 1.0,
        e: 50.0,
        f: 700.0,
    });
    stream.show_single_text_string("Tiny");

    stream.set_font_name_and_size("Helvetica", 1.0);
    stream.set_text_matrix(Matrix {
        a: 1.0,
        b: 0.0,
        c: 0.0,
        d: 1.0,
        e: 50.0,
        f: 650.0,
    });
    stream.show_single_text_string("Small");

    stream.set_font_name_and_size("Helvetica", 200.0);
    stream.set_text_matrix(Matrix {
        a: 1.0,
        b: 0.0,
        c: 0.0,
        d: 1.0,
        e: 50.0,
        f: 500.0,
    });
    stream.show_single_text_string("BIG");

    stream.end_text();

    let content_index = pdf.add_object(Box::new(stream));
    let page = create_page_with_content(PageSize::A4, content_index);
    pdf.add_page(page);

    std::fs::create_dir_all("/tmp/pydyf_test").unwrap();
    let file = File::create("/tmp/pydyf_test/break_fonts.pdf").unwrap();
    pdf.write(file, pydyf::FileIdentifierMode::AutoMD5).unwrap();
}

#[test]
fn test_overlapping_operations() {
    let mut pdf = PDF::new();
    let mut stream = StreamObject::new();

    stream.begin_text();
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
    stream.end_text();

    let content_index = pdf.add_object(Box::new(stream));
    let page = create_page_with_content(PageSize::A4, content_index);
    pdf.add_page(page);

    std::fs::create_dir_all("/tmp/pydyf_test").unwrap();
    let file = File::create("/tmp/pydyf_test/break_overlap.pdf").unwrap();
    pdf.write(file, pydyf::FileIdentifierMode::AutoMD5).unwrap();
}

#[test]
fn test_no_pages() {
    let mut pdf = PDF::new();

    std::fs::create_dir_all("/tmp/pydyf_test").unwrap();
    let file = File::create("/tmp/pydyf_test/break_no_pages.pdf").unwrap();
    pdf.write(file, pydyf::FileIdentifierMode::AutoMD5).unwrap();
}
