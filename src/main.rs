use rusty_pdf::PageSize;
use rusty_pdf::PdfDictionaryObject;
use rusty_pdf::color::{Color, RGB};
use rusty_pdf::drawing_commands::DrawingCommands;
use rusty_pdf::object_ops::ObjectOps;
use rusty_pdf::objects::stream::{StrokeOrFill, WindingRule};
use rusty_pdf::page_ops::PageOps;
use rusty_pdf::util::{Dims, Posn};
use rusty_pdf::{Pdf, PdfStreamObject};
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    println!("rusty_pdf - PDF library for Rust");
    println!("Originally based on Python rusty_pdf\n");

    let mut pdf = Pdf::new();
    let obj_ops: Rc<RefCell<ObjectOps>> = Rc::new(RefCell::new(ObjectOps::new()));
    let page_ops = PageOps::new(obj_ops.clone());

    let mut page_dict = page_ops.new_page();
    page_dict.add("MediaBox", PageSize::A4.to_rect());

    let resource_dict = PdfDictionaryObject::new();
    page_dict.add("Resources", resource_dict);

    let mut stream = PdfStreamObject::new(obj_ops.borrow_mut().next_object_number());

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

    page_dict.add("Contents", stream);

    page_ops
        .add_page_to_tree(page_dict, pdf.root_page_tree_dict_ref())
        .expect("Add page to tree failed");

    let path = "output.pdf";
    pdf.finalise(path).expect("finalise failed");

    println!(
        "Created {path}:\n\n{}",
        std::fs::read_to_string(path).unwrap()
    );
}
