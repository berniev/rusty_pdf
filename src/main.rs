use rusty_pdf::color::RGB;
use rusty_pdf::drawing_commands::DrawingCommands;
use rusty_pdf::util::StrokeOrFill::Fill;
use rusty_pdf::util::{Dims, Posn, StrokeOrFill, WindingRule};
use rusty_pdf::{PageSize, Pdf};

fn main() {
    println!("rusty_pdf - PDF library for Rust");
    println!("Originally based on Python rusty_pdf\n");

    let mut pdf = Pdf::new().expect("Failed to create PDF");

    let mut cmd = DrawingCommands::new();
    cmd.set_color_rgb(RGB::BLUE, StrokeOrFill::Fill);
    cmd.add_rectangle(
        Posn { x: 50.0, y: 50.0 },
        Dims {
            height: 50.0,
            width: 300.0,
        },
    );
    cmd.fill(WindingRule::EvenOdd);

    cmd.set_color_rgb(RGB::RED, StrokeOrFill::Fill);
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
    cmd.set_text_position(Posn { x: 50.0, y: 750.0 });
    cmd.show_single_text_string("Hello, Blue World");
    cmd.end_text();

    cmd.begin_text();
    cmd.set_font_name_and_size("Helvetica-Bold", 16.0);
    cmd.set_color_rgb(RGB::RED, Fill);
    cmd.set_text_position(Posn { x: 50.0, y: 550.0 });
    cmd.show_single_text_string("Hello, RED World");
    cmd.end_text();

    let data = cmd.flush();

    let page_dict = pdf
        .page_ops
        .new_page_dict(PageSize::A4, data.clone())
        .expect("Failed to create page");

    let page_dict2 = pdf
        .page_ops
        .new_page_dict(PageSize::A4, data)
        .expect("Failed to create page");

    pdf.page_ops
        .add_page_dict_to_root(page_dict)
        .expect("Add page to tree failed");

    pdf.page_ops
        .add_page_dict_to_root(page_dict2)
        .expect("Add page to tree failed");

    let path = "output.pdf";
    pdf.finalise(path).expect("finalise failed");

    println!(
        "Created {path}:\n\n{}",
        std::fs::read_to_string(path).unwrap()
    );
}
