use pydyf::color::{Color, RGB};
use pydyf::objects::stream::{StrokeOrFill, WindingRule};
use pydyf::util::Posn;
use pydyf::{FileIdentifierMode, PDF, PageObject, Stream};
use std::fs::File;

fn create_page_with_content(content_stream_ref: Vec<u8>) -> PageObject {
    let content_index = String::from_utf8(content_stream_ref).unwrap();
    let id_str = content_index.split_whitespace().next().unwrap();
    let id: u64 = id_str.parse().unwrap();

    let mut page = PageObject::new(0usize.into());
    page.add_content(id as usize);
    page
}

/// Creates concentric circles to demonstrate winding rule differences
/// Even-Odd: Creates a "donut" (outer filled, inner hollow)
/// Non-Zero: Fills everything solid when both wound in same direction
fn draw_concentric_circles(
    stream: &mut Stream,
    center_x: f64,
    center_y: f64,
    outer_radius: f64,
    inner_radius: f64,
) {
    // Draw outer circle (counter-clockwise)
    let segments = 32;
    for i in 0..=segments {
        let angle = 2.0 * std::f64::consts::PI * (i as f64) / (segments as f64);
        let x = center_x + outer_radius * angle.cos();
        let y = center_y + outer_radius * angle.sin();
        if i == 0 {
            stream.move_to_x_y(Posn { x, y });
        } else {
            stream.line_to_x_y(Posn { x, y });
        }
    }
    stream.close();

    // Draw inner circle (counter-clockwise - same direction)
    for i in 0..=segments {
        let angle = 2.0 * std::f64::consts::PI * (i as f64) / (segments as f64);
        let posn = Posn {
            x: center_x + inner_radius * angle.cos(),
            y: center_y + inner_radius * angle.sin(),
        };
        if i == 0 {
            stream.move_to_x_y(posn);
        } else {
            stream.line_to_x_y(posn);
        }
    }
    stream.close();
}

#[test]
fn test_winding_rule_even_odd() {
    let mut pdf = PDF::new();
    let mut stream = Stream::uncompressed();

    // Set blue fill color
    stream.set_color_rgb(
        RGB::new(Color::new(0.0), Color::new(0.5), Color::new(1.0)),
        StrokeOrFill::Fill,
    );

    // Draw concentric circles using even-odd winding rule
    // Result: Creates a "donut" - outer ring filled, inner circle hollow
    draw_concentric_circles(&mut stream, 300.0, 400.0, 100.0, 60.0);
    stream.fill(WindingRule::EvenOdd);

    // Add title
    stream.begin_text();
    stream.set_font_name_and_size("Helvetica-Bold", 16.0);
    stream.set_text_position(Posn { x: 220.0, y: 550.0 });
    stream.show_single_text_string("Even-Odd Winding Rule");
    stream.end_text();

    // Add description
    stream.begin_text();
    stream.set_font_name_and_size("Helvetica", 12.0);
    stream.set_text_position(Posn { x: 140.0, y: 220.0 });
    stream.show_single_text_string("Result: Blue ring with white center (donut)");
    stream.end_text();

    stream.begin_text();
    stream.set_font_name_and_size("Helvetica", 11.0);
    stream.set_text_position(Posn { x: 100.0, y: 190.0 });
    stream.show_single_text_string("Two concentric circles wound in the same direction.");
    stream.end_text();

    stream.begin_text();
    stream.set_font_name_and_size("Helvetica", 11.0);
    stream.set_text_position(Posn { x: 80.0, y: 175.0 });
    stream.show_single_text_string("Even-odd rule counts path crossings: odd=fill, even=no fill.");
    stream.end_text();

    let content_id = pdf.add_object(Box::new(stream));
    let content_ref = format!("{} 0 R", content_id).into_bytes();
    let page = create_page_with_content(content_ref);
    pdf.add_page(page);

    std::fs::create_dir_all("/tmp/pydyf_test").unwrap();
    let file = File::create("/tmp/pydyf_test/winding_evenodd.pdf").unwrap();
    pdf.write_legacy(file, FileIdentifierMode::AutoMD5).unwrap();

    println!("✅ Generated: /tmp/pydyf_test/winding_evenodd.pdf");
}

#[test]
fn test_winding_rule_nonzero() {
    let mut pdf = PDF::new();
    let mut stream = Stream::uncompressed();

    // Set red fill color
    stream.set_color_rgb(
        RGB::new(Color::new(1.0), Color::new(0.0), Color::new(0.0)),
        StrokeOrFill::Fill,
    );

    // Draw concentric circles using non-zero winding rule
    // Result: Fills completely solid (no hole in center)
    draw_concentric_circles(&mut stream, 300.0, 400.0, 100.0, 60.0);
    stream.fill(WindingRule::NonZero);

    // Add title
    stream.begin_text();
    stream.set_font_name_and_size("Helvetica-Bold", 16.0);
    stream.set_text_position(Posn { x: 215.0, y: 550.0 });
    stream.show_single_text_string("Non-Zero Winding Rule");
    stream.end_text();

    // Add description
    stream.begin_text();
    stream.set_font_name_and_size("Helvetica", 12.0);
    stream.set_text_position(Posn { x: 170.0, y: 220.0 });
    stream.show_single_text_string("Result: Solid red circle, no hole");
    stream.end_text();

    stream.begin_text();
    stream.set_font_name_and_size("Helvetica", 11.0);
    stream.set_text_position(Posn { x: 100.0, y: 190.0 });
    stream.show_single_text_string("Two concentric circles wound in the same direction.");
    stream.end_text();

    stream.begin_text();
    stream.set_font_name_and_size("Helvetica", 11.0);
    stream.set_text_position(Posn { x: 70.0, y: 175.0 });
    stream.show_single_text_string("Non-zero rule uses winding direction: same direction = fill.");
    stream.end_text();

    let content_id = pdf.add_object(Box::new(stream));
    let content_ref = format!("{} 0 R", content_id).into_bytes();
    let page = create_page_with_content(content_ref);
    pdf.add_page(page);

    std::fs::create_dir_all("/tmp/pydyf_test").unwrap();
    let file = File::create("/tmp/pydyf_test/winding_nonzero.pdf").unwrap();
    pdf.write_legacy(file, FileIdentifierMode::AutoMD5).unwrap();

    println!("✅ Generated: /tmp/pydyf_test/winding_nonzero.pdf");
}

#[test]
fn test_winding_rule_side_by_side() {
    let mut pdf = PDF::new();
    let mut stream = Stream::uncompressed();

    // Left circles - Even-Odd (blue) - creates donut
    stream.set_color_rgb(
        RGB::new(Color::new(0.0), Color::new(0.5), Color::new(1.0)),
        StrokeOrFill::Fill,
    );
    draw_concentric_circles(&mut stream, 200.0, 400.0, 80.0, 48.0);
    stream.fill(WindingRule::EvenOdd);

    // Label for left
    stream.begin_text();
    stream.set_font_name_and_size("Helvetica", 11.0);
    stream.set_text_position(Posn { x: 135.0, y: 285.0 });
    stream.show_single_text_string("Even-Odd Rule");
    stream.end_text();

    stream.begin_text();
    stream.set_font_name_and_size("Helvetica", 9.0);
    stream.set_text_position(Posn { x: 110.0, y: 270.0 });
    stream.show_single_text_string("Blue ring, white center");
    stream.end_text();

    // Right circles - Non-Zero (red) - fills solid
    stream.set_color_rgb(
        RGB::new(
            Color::new(1.0),
            Color::new(0.0),
            Color::new(0.0),
        ),
        StrokeOrFill::Fill,
    );
    draw_concentric_circles(&mut stream, 450.0, 400.0, 80.0, 48.0);
    stream.fill(WindingRule::NonZero);

    // Label for right
    stream.begin_text();
    stream.set_font_name_and_size("Helvetica", 11.0);
    stream.set_text_position(Posn { x: 385.0, y: 285.0 });
    stream.show_single_text_string("Non-Zero Rule");
    stream.end_text();

    stream.begin_text();
    stream.set_font_name_and_size("Helvetica", 9.0);
    stream.set_text_position(Posn { x: 375.0, y: 270.0 });
    stream.show_single_text_string("Solid red circle");
    stream.end_text();

    // Title
    stream.begin_text();
    stream.set_font_name_and_size("Helvetica-Bold", 16.0);
    stream.set_text_position(Posn { x: 150.0, y: 550.0 });
    stream.show_single_text_string("Winding Rule Comparison");
    stream.end_text();

    let content_id = pdf.add_object(Box::new(stream));
    let content_ref = format!("{} 0 R", content_id).into_bytes();
    let page = create_page_with_content(content_ref);
    pdf.add_page(page);

    std::fs::create_dir_all("/tmp/pydyf_test").unwrap();
    let file = File::create("/tmp/pydyf_test/winding_comparison.pdf").unwrap();
    pdf.write_legacy(file, FileIdentifierMode::AutoMD5).unwrap();

    println!("✅ Generated: /tmp/pydyf_test/winding_comparison.pdf");
}
