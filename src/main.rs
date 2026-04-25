use rusty_pdf::color::RGB;
use rusty_pdf::drawing_commands::DrawingCommands;
use rusty_pdf::util::StrokeOrFill::Fill;
use rusty_pdf::util::{Dims, Posn, WindingRule};
use rusty_pdf::{PageSize, Pdf};

fn main() {
    println!("rusty_pdf - PDF library for Rust");
    println!("Originally based on Python rusty_pdf\n");

    let mut pdf = Pdf::new()
        .expect("Failed to create PDF")
        .with_page_size(PageSize::A4);

    let mut cmd = DrawingCommands::new();
    cmd.set_color_rgb(RGB::BLUE, Fill);
    cmd.add_rectangle(
        Posn { x: 50.0, y: 50.0 },
        Dims {
            height: 50.0,
            width: 300.0,
        },
    );
    cmd.fill(WindingRule::EvenOdd);

    cmd.set_color_rgb(RGB::RED, Fill);
    cmd.add_rectangle(
        Posn { x: 50.0, y: 100.0 },
        Dims {
            height: 50.0,
            width: 300.0,
        },
    );

    cmd.fill(WindingRule::EvenOdd);

    cmd.begin_text();
    cmd.set_font_name_and_size("Helvetica-Bold", 16.0);
    cmd.set_color_rgb(RGB::BLUE, Fill);
    cmd.set_text_position(Posn { x: 50.0, y: 250.0 });
    cmd.show_single_text_string("Hello, Blue World");
    cmd.end_text();

    cmd.begin_text();
    cmd.set_font_name_and_size("Helvetica-Bold", 16.0);
    cmd.set_color_rgb(RGB::RED, Fill);
    cmd.set_text_position(Posn { x: 50.0, y: 200.0 });
    cmd.show_single_text_string("Hello, RED World");
    cmd.end_text();

    let data = cmd.flush();

    let root_tree = pdf.page_ops.root_tree();

    let page = root_tree
        .make_page(data.clone())
        .expect("Failed to create page");
    root_tree.add_page(page).expect("Add page to tree failed");

    let page2 = root_tree
        .make_page(data.clone())
        .expect("Failed to create page");
    root_tree.add_page(page2).expect("Add page to tree failed");

    let mut new_tree = root_tree.make_tree().expect("Faied to create tree");

    cmd.set_color_rgb(RGB::ORANGE, Fill);
    cmd.add_rectangle(
        Posn { x: 50.0, y: 50.0 },
        Dims {
            height: 50.0,
            width: 300.0,
        },
    );
    cmd.fill(WindingRule::EvenOdd);

    cmd.set_color_rgb(RGB::GREEN, Fill);
    cmd.add_rectangle(
        Posn { x: 50.0, y: 100.0 },
        Dims {
            height: 50.0,
            width: 300.0,
        },
    );

    cmd.fill(WindingRule::EvenOdd);
    cmd.begin_text();
    cmd.set_font_name_and_size("Helvetica-Bold", 16.0);
    cmd.set_color_rgb(RGB::RED, Fill);
    cmd.set_text_position(Posn { x: 50.0, y: 200.0 });
    cmd.show_single_text_string("Page 3, A5");
    cmd.end_text();

    let data = cmd.flush();
    let page3 = new_tree
        .make_page(data)
        .expect("Failed to create page")
        .with_page_size(PageSize::A5);
    new_tree.add_page(page3).expect("Add page to tree failed");

    root_tree
        .add_tree(new_tree)
        .expect("Failed to add page to tree");

    let path = "output.pdf";
    pdf.finalise(path).expect("finalise failed");

    println!(
        "Created {path}:\n\n{}",
        std::fs::read_to_string(path).unwrap()
    );
}
