use rusty_pdf::color::{Color, RGB};
use rusty_pdf::drawing_commands::DrawingCommands;
use rusty_pdf::util::{Dims, Posn, StrokeOrFill, WindingRule};
use rusty_pdf::{PageSize, Pdf, PdfDictionaryObject, PdfStreamObject};

fn main() {
    println!("rusty_pdf - PDF library for Rust");
    println!("Originally based on Python rusty_pdf\n");

    let mut pdf = Pdf::new().expect("Failed to create PDF");

    let mut page_dict = pdf.page_ops.new_page().expect("Failed to create page");
    page_dict
        .add("MediaBox", PageSize::A4.to_rect())
        .expect("failure:");
    page_dict
        .add("Resources", PdfDictionaryObject::new())
        .expect("failure:");

    let stream =
        PdfStreamObject::new().with_object_number(pdf.object_ops.borrow_mut().next_object_number());

    let mut cmd = DrawingCommands::new();

    cmd.set_color_rgb(
        RGB::new(Color::new(0.0), Color::new(0.5), Color::new(1.0)),
        StrokeOrFill::Fill,
    );
    cmd.add_rectangle(
        Posn { x: 100.0, y: 100.0 },
        Dims {
            height: 200.0,
            width: 300.0,
        },
    );
    cmd.fill(WindingRule::EvenOdd);

    page_dict.add("Contents", stream).expect("failure:");

    pdf.page_ops
        .add_page_to_tree(page_dict, &mut pdf.root_page_tree_dict)
        .expect("Add page to tree failed");

    let path = "output.pdf";
    pdf.finalise(path).expect("finalise failed");

    println!(
        "Created {path}:\n\n{}",
        std::fs::read_to_string(path).unwrap()
    );
}
