use rusty_pdf::color::RGB;
use rusty_pdf::drawing_commands::DrawingCommands;
use rusty_pdf::util::{Dims, Posn};
use rusty_pdf::{PageSize, Pdf, PdfError};

fn main() -> Result<(), PdfError> {
    println!("rusty_pdf - PDF library for Rust");
    println!("Originally inspired by Python pydyf\n");

    let mut pdf = Pdf::new()?.with_default_page_size(PageSize::A4);

    let mut cmd = DrawingCommands::new();
    cmd.set_color_rgb_fill(RGB::BLUE);
    cmd.add_rectangle(
        Posn { x: 50.0, y: 50.0 },
        Dims {
            height: 50.0,
            width: 300.0,
        },
    );
    cmd.fill_even_odd();

    cmd.set_color_rgb_fill(RGB::RED);
    cmd.add_rectangle(
        Posn { x: 50.0, y: 100.0 },
        Dims {
            height: 50.0,
            width: 300.0,
        },
    );

    cmd.fill_even_odd();

    cmd.begin_text();
    cmd.set_font_name_and_size("Helvetica-Bold", 16.0);
    cmd.set_color_rgb_fill(RGB::BLUE);
    cmd.set_text_position(Posn { x: 50.0, y: 250.0 });
    cmd.show_single_text_string("Hello, Blue World");
    cmd.set_font_name_and_size("Helvetica-Bold", 12.0);
    cmd.set_text_position(Posn { x: 70.0, y: 270.0 });
    cmd.show_single_text_string("Second text line");
    cmd.set_color_rgb_fill(RGB::PURPLE);
    cmd.set_font_name_and_size("Helvetica-Bold", 14.0);
    cmd.set_text_position(Posn { x: 90.0, y: 290.0 });
    cmd.show_single_text_string("Third text line");
    cmd.end_text();

    let root_tree = pdf.page_ops.root_tree();

    root_tree.add_page_using(cmd.read())?;
    root_tree.add_page_using(cmd.flush())?;

    cmd.set_color_rgb_fill(RGB::ORANGE);
    cmd.add_rectangle(
        Posn { x: 50.0, y: 50.0 },
        Dims {
            height: 50.0,
            width: 300.0,
        },
    );
    cmd.fill_even_odd();

    cmd.set_color_rgb_fill(RGB::GREEN);
    cmd.add_rectangle(
        Posn { x: 50.0, y: 100.0 },
        Dims {
            height: 50.0,
            width: 300.0,
        },
    );

    cmd.begin_text();
    cmd.fill_even_odd();
    cmd.set_font_name_and_size("Helvetica-Bold", 16.0);
    cmd.set_color_rgb_fill(RGB::RED);
    cmd.set_text_position(Posn { x: 50.0, y: 200.0 });
    cmd.show_single_text_string("Page 3, A5");
    cmd.end_text();

    let mut new_tree = root_tree.make_tree()?.with_default_page_size(PageSize::A5);
    new_tree.add_page_using(cmd.flush())?;

    root_tree.add_tree(new_tree)?;

    let path = "output.pdf";
    pdf.finalise(path)?;

    println!("Created {path}:\n\n{}", std::fs::read_to_string(path)?);

    Ok(())
}
