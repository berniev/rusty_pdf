use pydyf::PdfDictionaryObject;
use pydyf::color::{Color, RGB};
use pydyf::drawing_commands::DrawingCommands;
use pydyf::file_identifier::FileIdentifierMode;
use pydyf::objects::pdf_object::PdfObj;
use pydyf::objects::stream::{StrokeOrFill, WindingRule};
use pydyf::page::*;
use pydyf::util::{Dims, Posn};
use pydyf::{Pdf, PdfStreamObject};
use std::fs::File;
use std::io::Write;

fn main() {
    println!("PyDyf - PDF library for Rust");
    println!("Ported from Python pydyf library\n");

    let mut pdf = Pdf::new();

    let mut page = make_page(pdf.next_object_number());
    page.add("MediaBox", PdfObj::array(PageSize::A4.to_rect()));
    page.add("Resources", PdfObj::dict(PdfDictionaryObject::new()));

    let mut stream = PdfStreamObject::new();

    let mut cmd = DrawingCommands::new(&mut stream);

    cmd.set_color_rgb(
        RGB::new(Color::new(0.0), Color::new(0.5), Color::new(1.0)),
        StrokeOrFill::Fill,
    );
    cmd.rectangle(
        Posn { x: 100.0, y: 100.0 },
        Dims {
            height: 200.0,
            width: 300.0,
        },
    );
    cmd.fill(WindingRule::EvenOdd);

    page.add("Contents", PdfObj::stream(stream));
    add_page_to_tree(&mut page, pdf.root_page_tree_dict_ref()).expect("Add page to tree failed");

    let mut output = Vec::new();

    ///////////////////////////
    pdf.write_legacy(&mut output, FileIdentifierMode::None)
        .expect("Failed to write PDF");
    ///////////////////////////

    let path = "output.pdf";
    let mut file = File::create(path).expect("Failed to create file");
    file.write_all(&output).expect("Failed to write file");

    println!(
        "Created {} with {} objects",
        path,
        pdf.next_object_number() - 1
    );
}
