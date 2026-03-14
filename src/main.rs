// Main entry point for pydyf
// This is a simple example/test program

use pydyf::{PageObject, PDF, StreamObject, FileIdentifierMode};
use pydyf::page::PageSize;
use pydyf::color::{Color, RGB};
use pydyf::objects::stream::{StrokeOrFill, EvenOdd};
use pydyf::util::{Posn, Dims};
use std::fs::File;
use std::io::Write;

fn main() {
    println!("PyDyf - PDF library for Rust");
    println!("Ported from Python pydyf library\n");

    let mut pdf = PDF::new();
    let mut stream = StreamObject::new();

    let color = RGB {
        red: Color { color: 0.0 },
        green: Color { color: 0.5 },
        blue: Color { color: 1.0 },
    };
    let _ = stream.set_color_rgb(color, StrokeOrFill::Fill);
    stream.rectangle(
        Posn { x: 100.0, y: 100.0 },
        Dims { height: 200.0, width: 300.0 },
    );
    stream.fill(EvenOdd::Odd);

    let content_id = pdf.add_object(Box::new(stream));
    let mut page = PageObject::new(0usize.into());
    page.add_content(content_id);
    page.set_media_box(PageSize::A4);
    pdf.add_page(page);

    let mut output = Vec::new();
    pdf.write(&mut output, FileIdentifierMode::None)
        .expect("Failed to write PDF");

    let path = "output.pdf";
    let mut file = File::create(path).expect("Failed to create file");
    file.write_all(&output).expect("Failed to write file");

    println!("Created {} with {} objects", path, pdf.objects.len());
}
