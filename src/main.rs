use rusty_pdf::color::{Color, RGB};
use rusty_pdf::drawing_commands::DrawingCommands;
use rusty_pdf::objects::pdf_object::PdfObj;
use rusty_pdf::objects::stream::{StrokeOrFill, WindingRule};
use rusty_pdf::page::*;
use rusty_pdf::util::{Dims, Posn};
use rusty_pdf::PdfDictionaryObject;
use rusty_pdf::{Pdf, PdfStreamObject};

fn main() {
    println!("rusty_pdf - PDF library for Rust");
    println!("Originally based on Python pydyf\n");

    let mut pdf = Pdf::new();

    let mut page_dict = make_page_dict(pdf.next_object_number());
    page_dict.add("MediaBox", PdfObj::array(PageSize::A4.to_rect()));
    page_dict.add("Resources", PdfObj::dict(PdfDictionaryObject::new()));

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

    page_dict.add("Contents", PdfObj::stream(stream));
    add_page_to_tree(&mut page_dict, pdf.root_page_tree_dict_ref()).expect("Add page to tree failed");

    ///////////////////////////
    let path = "output.pdf";
    pdf.finalise(path).expect("finalise failed");
    ///////////////////////////


    println!(
        "Created {} with {} objects",
        path,
        pdf.next_object_number() - 1
    );
}
