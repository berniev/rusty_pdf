use pydyf::color::{Color, RGB};
use pydyf::file_identifier::FileIdentifierMode;
use pydyf::objects::pdf_object::PdfObj;
use pydyf::objects::stream::{StrokeOrFill, WindingRule};
use pydyf::page::*;
use pydyf::util::{Dims, Posn};
use pydyf::{drawing_commands, PdfDictionaryObject};
use pydyf::{Pdf, PdfStreamObject};
use std::fs::File;
use std::io::Write;

fn main() {
    println!("PyDyf - PDF library for Rust");
    println!("Ported from Python pydyf library\n");

    let mut pdf = Pdf::new();

    let mut stream = PdfStreamObject::new();

    stream.add_content(drawing_commands::set_color_rgb(
        RGB::new(Color::new(0.0), Color::new(0.5), Color::new(1.0)),
        StrokeOrFill::Fill,
    ));
    stream.add_content(drawing_commands::add_rectangle(
        Posn { x: 100.0, y: 100.0 },
        Dims {
            height: 200.0,
            width: 300.0,
        },
    ));
    stream.add_content(drawing_commands::fill(WindingRule::EvenOdd));

    let resources = PdfDictionaryObject::new();
    let mut page = make_page(pdf.next_object_number());
    page.add("MediaBox", PdfObj::array(PageSize::A4.to_rect()));
    page.add("Contents", PdfObj::stream(stream));
    page.add("Resources", PdfObj::dict(resources));

    add_page_to_tree(&mut page, pdf.root_page_tree_dict_ref()).expect("Add page to tree failed");

    let mut output = Vec::new();
    pdf.write_legacy(&mut output, FileIdentifierMode::None)
        .expect("Failed to write PDF");

    let path = "output.pdf";
    let mut file = File::create(path).expect("Failed to create file");
    file.write_all(&output).expect("Failed to write file");

    println!("Created {} with {} objects", path, pdf.next_object_number()-1);
}
