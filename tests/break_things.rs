use RustyPDF::color::{Color, RGB};
use RustyPDF::objects::pdf_object::PdfObj;
use RustyPDF::objects::stream::{StrokeOrFill, WindingRule};
use RustyPDF::util::{Dims, Matrix, Posn};
use RustyPDF::PdfStreamObject;
use RustyPDF::Pdf;
/*fn create_page_with_content(page_size: PageSize, content_index: usize) -> PageObject {
    let mut page = PageObject::new(0usize.into());
    page.add_content(content_index);
    page.set_media_box(page_size);
    page
}
*/
/*#[test]
fn test_empty_page() {
    let mut pdf = PdfFile::new();
    let stream = PdfStreamObject::new();

    let content_id = pdf.add_indirect_object(Box::new(stream));

    let mut page = PageObject::new(0usize.into());
    page.add_content(content_id);
    page.set_media_box(PageSize::A4);
    pdf.add_page(page);

    std::fs::create_dir_all("/tmp/pydyf_test").unwrap();
    let file = File::create("/tmp/pydyf_test/break_empty.pdf").unwrap();
    pdf.write_legacy(file, pydyf::FileIdentifierMode::AutoMD5).unwrap();
}
*/
/*#[test]
fn test_massive_page_count() {
    let mut pdf = PdfFile::new();

    for _ in 0..500 {
        let mut stream = PdfStreamObject::new();
        stream.set_color_rgb(
            RGB::new(Color::new(0.5), Color::new(0.5), Color::new(0.5)),
            StrokeOrFill::Fill,
        );
        stream.add_rectangle(
            Posn { x: 50.0, y: 50.0 },
            Dims {
                width: 100.0,
                height: 100.0,
            },
        );
        stream.fill(WindingRule::EvenOdd);

        let content_index = pdf.add_indirect_object(Box::new(stream));
        let page = create_page_with_content(PageSize::A4, content_index);
        pdf.add_page(page);
    }

    std::fs::create_dir_all("/tmp/pydyf_test").unwrap();
    let file = File::create("/tmp/pydyf_test/break_massive.pdf").unwrap();
    pdf.write_legacy(file, pydyf::FileIdentifierMode::AutoMD5).unwrap();
}
*/
/*#[test]
fn test_extreme_coordinates() {
    let mut pdf = PdfFile::new();
    let mut stream = PdfStreamObject::new();

    stream.set_color_rgb(
        RGB::new(Color::new(1.0), Color::new(0.0), Color::new(0.0)),
        StrokeOrFill::Fill,
    );
    stream.add_rectangle(
        Posn {
            x: -1000.0,
            y: -1000.0,
        },
        Dims {
            width: 100.0,
            height: 100.0,
        },
    );
    stream.fill(WindingRule::EvenOdd);

    stream.set_color_rgb(
        RGB::new(Color::new(0.0), Color::new(1.0), Color::new(0.0)),
        StrokeOrFill::Fill,
    );
    stream.add_rectangle(
        Posn {
            x: 10000.0,
            y: 10000.0,
        },
        Dims {
            width: 100.0,
            height: 100.0,
        },
    );
    stream.fill(WindingRule::EvenOdd);

    stream.set_color_rgb(
        RGB::new(Color::new(0.0), Color::new(0.0), Color::new(1.0)),
        StrokeOrFill::Fill,
    );
    stream.add_rectangle(
        Posn { x: 0.0, y: 0.0 },
        Dims {
            width: 0.0,
            height: 0.0,
        },
    );
    stream.fill(WindingRule::EvenOdd);

    stream.set_color_rgb(
        RGB::new(Color::new(0.5), Color::new(0.5), Color::new(0.5)),
        StrokeOrFill::Fill,
    );
    stream.add_rectangle(
        Posn { x: 100.0, y: 100.0 },
        Dims {
            width: 10000.0,
            height: 10000.0,
        },
    );
    stream.fill(WindingRule::EvenOdd);

    let content_index = pdf.add_indirect_object(Box::new(stream));
    let page = create_page_with_content(PageSize::A4, content_index);
    pdf.add_page(page);

    std::fs::create_dir_all("/tmp/pydyf_test").unwrap();
    let file = File::create("/tmp/pydyf_test/break_coords.pdf").unwrap();
    pdf.write_legacy(file, pydyf::FileIdentifierMode::AutoMD5).unwrap();
}
*/
/*#[test]
fn test_very_long_text() {
    let mut pdf = PdfFile::new();
    let mut stream = PdfStreamObject::new();

    stream.set_color_rgb(
        RGB::new(Color::new(0.0), Color::new(0.0), Color::new(0.0)),
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

    let content_index = pdf.add_indirect_object(Box::new(stream));
    let page = create_page_with_content(PageSize::A4, content_index);
    pdf.add_page(page);

    std::fs::create_dir_all("/tmp/pydyf_test").unwrap();
    let file = File::create("/tmp/pydyf_test/break_longtext.pdf").unwrap();
    pdf.write_legacy(file, pydyf::FileIdentifierMode::AutoMD5).unwrap();
}
*/
/*#[test]
fn test_special_characters_text() {
    let mut pdf = PdfFile::new();
    let mut stream = PdfStreamObject::new();

    stream.set_color_rgb(
        RGB::new(Color::new(0.0), Color::new(0.0), Color::new(0.0)),
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

    let content_index = pdf.add_indirect_object(Box::new(stream));
    let page = create_page_with_content(PageSize::A4, content_index);
    pdf.add_page(page);

    std::fs::create_dir_all("/tmp/pydyf_test").unwrap();
    let file = File::create("/tmp/pydyf_test/break_special_chars.pdf").unwrap();
    pdf.write_legacy(file, pydyf::FileIdentifierMode::AutoMD5).unwrap();
}
*/
/*#[test]
fn test_huge_rectangle() {
    let mut pdf = PdfFile::new();
    let mut stream = PdfStreamObject::new();

    stream.set_color_rgb(
        RGB::new(Color::new(1.0), Color::new(0.0), Color::new(0.0)),
        StrokeOrFill::Fill,
    );
    stream.add_rectangle(
        Posn { x: 0.0, y: 0.0 },
        Dims {
            width: 5000.0,
            height: 5000.0,
        },
    );
    stream.fill(WindingRule::EvenOdd);

    let content_index = pdf.add_indirect_object(Box::new(stream));
    let page = create_page_with_content(PageSize::A4, content_index);
    pdf.add_page(page);

    std::fs::create_dir_all("/tmp/pydyf_test").unwrap();
    let file = File::create("/tmp/pydyf_test/break_huge.pdf").unwrap();
    pdf.write_legacy(file, pydyf::FileIdentifierMode::AutoMD5).unwrap();
}
*/
/*#[test]
fn test_compressed_empty() {
    let mut pdf = PdfFile::new();
    let stream = PdfStreamObject::new().compressed();

    let content_index = pdf.add_indirect_object(Box::new(stream));
    let page = create_page_with_content(PageSize::A4, content_index);
    pdf.add_page(page);

    std::fs::create_dir_all("/tmp/pydyf_test").unwrap();
    let file = File::create("/tmp/pydyf_test/break_compressed_empty.pdf").unwrap();
    pdf.write_legacy(file, pydyf::FileIdentifierMode::AutoMD5).unwrap();
}
*/
#[test]
fn test_extreme_font_sizes() {
    let  _pdf = Pdf::new();
    let mut stream = PdfStreamObject::new();

    stream.set_color_rgb(
        RGB::new(Color::new(0.0), Color::new(0.0), Color::new(0.0)),
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

/*    let content_index = pdf.add_object(Pdf::stream(stream));
    let page = create_page_with_content(PageSize::A4, content_index);
    pdf.add_page(page);

    std::fs::create_dir_all("/tmp/pydyf_test").unwrap();
    let file = File::create("/tmp/pydyf_test/break_fonts.pdf").unwrap();
    pdf.write_legacy(file, pydyf::FileIdentifierMode::AutoMD5).unwrap();
*/}

#[test]
fn test_overlapping_operations() {
    let mut pdf = Pdf::new();
    let mut stream = PdfStreamObject::new();

    stream.begin_text();
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
    stream.end_text();

    let _content_index = pdf.save_indirect_object(PdfObj::stream(stream));
/*    let page = create_page_with_content(PageSize::A4, content_index);
    pdf.add_page(page);

    std::fs::create_dir_all("/tmp/pydyf_test").unwrap();
    let file = File::create("/tmp/pydyf_test/break_overlap.pdf").unwrap();
    pdf.write_legacy(file, pydyf::FileIdentifierMode::AutoMD5).unwrap();
*/}

/*#[test]
fn test_no_pages() {
    let mut pdf = PdfFile::new();

    std::fs::create_dir_all("/tmp/pydyf_test").unwrap();
    let file = File::create("/tmp/pydyf_test/break_no_pages.pdf").unwrap();
    pdf.write_legacy(file, pydyf::FileIdentifierMode::AutoMD5).unwrap();
}
*/