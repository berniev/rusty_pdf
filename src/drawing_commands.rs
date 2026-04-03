use crate::color::{CMYK, Color, ColorSpace, RGB};
use crate::encoding::{ascii85_encode, f_to_pdf_num};
use crate::objects::string::encode_pdf_string;
use crate::util::{Dims, Matrix, Posn, StrokeOrFill, ToPdf, WindingRule};
use crate::{CompressionMethod, PdfError, PdfResult};
use flate2::Compression;
use flate2::write::ZlibEncoder;
use std::io::Write;

//-------------------------- Drawing Commands --------------------------

fn cmd(cmd: char) -> u8 {
    cmd as u8
}

fn windable_cmd(cmd: char, even_odd: WindingRule) -> Vec<u8> {
    let mut op_bytes = vec![cmd as u8];
    match even_odd {
        WindingRule::EvenOdd => op_bytes.push(b'*'),
        WindingRule::NonZero => op_bytes.push(b' '),
    }
    op_bytes
}

fn float_cmd(string: &str, value: f64) -> Vec<u8> {
    format!("{} {}", f_to_pdf_num(value), string).into_bytes()
}

fn int_cmd(string: &str, value: i32) -> Vec<u8> {
    float_cmd(string, value as f64)
}

pub fn begin_marked_content(tag: &str, property_list: Option<Vec<u8>>) -> Vec<u8> {
    match property_list {
        None => format!("/{tag} BMC").into_bytes(),

        Some(props) => {
            let mut cmd = format!("/{tag} ").into_bytes();
            cmd.extend(props);
            cmd.extend(b" BDC");
            cmd
        }
    }
}

/// Use the nonzero winding number rule to determine which regions lie inside the clipping path by default.
pub fn clip(even_odd: WindingRule) -> Vec<u8> {
    windable_cmd('W', even_odd)
}

pub fn close() -> u8 {
    cmd('h')
}

fn build_op(operands: &[&dyn ToPdf], operator: &str) -> Vec<u8> {
    let mut cmd_parts: Vec<String> = operands.iter().map(|n| n.to_pdf()).collect();
    cmd_parts.push(operator.to_string());

    cmd_parts.join(" ").into_bytes()
}

/// extend curve from `pos3` using `pos1` and `pos2` as Bézier control points.
pub fn curve_to(pos1: Posn, pos2: Posn, pos3: Posn) -> Vec<u8> {
    build_op(&[&pos1, &pos2, &pos3], "c")
}

/// Extend curve to `pos3` using current point, and `pos2` as Bézier control points.
pub fn curve_start_to(pos2: Posn, pos3: Posn) -> Vec<u8> {
    build_op(&[&pos2, &pos3], "v")
}

/// extend curve to `pos3` using `pos1`, and `pos3` as Bézier control points.
pub fn curve_end_to(pos1: Posn, pos3: Posn) -> Vec<u8> {
    build_op(&[&pos1, &pos3], "y")
}

pub fn draw_x_object(reference: &str) -> Vec<u8> {
    format!("/{} Do", reference).into_bytes()
}

/// End path without filling or stroking.
pub fn end() -> Vec<u8> {
    b"n".to_vec()
}

pub fn end_marked_content() -> Vec<u8> {
    b"EMC".to_vec()
}

pub fn begin_text() -> Vec<u8> {
    b"BT".to_vec()
}

pub fn end_text() -> Vec<u8> {
    b"ET".to_vec()
}

pub fn fill(even_odd: WindingRule) -> Vec<u8> {
    windable_cmd('f', even_odd)
}

pub fn fill_and_stroke(even_odd: WindingRule) -> Vec<u8> {
    windable_cmd('B', even_odd)
}

pub fn fill_stroke_and_close(even_odd: WindingRule) -> Vec<u8> {
    windable_cmd('b', even_odd)
}

pub fn inline_image(
    width_pixels: u32,
    height_pixels: u32,
    color_space: ColorSpace,
    bits_per_component: u8, // typ 8
    raw_pixel_data: &[u8],
    compression_method: CompressionMethod,
) -> PdfResult<Vec<u8>> {
    if width_pixels == 0 || height_pixels == 0 {
        let msg = format!("Invalid image dimensions: {width_pixels} x {height_pixels} pixels",);
        return Err(PdfError::InvalidImage(msg));
    }

    let data_to_encode = match compression_method {
        CompressionMethod::Flate => {
            let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
            encoder.write_all(raw_pixel_data)?;
            encoder.finish()?
        }
        CompressionMethod::None => raw_pixel_data.to_vec(),
    };

    let mut encoded_data = ascii85_encode(&data_to_encode);
    encoded_data.extend(b"~>"); // ASCII85 end marker

    let header_string = format!(
        "BI /W {} /H {} /BPC {} /CS /Device{} /F {} /L {} ID\n",
        width_pixels,
        height_pixels,
        bits_per_component,
        color_space,
        compression_method.to_string(),
        encoded_data.len()
    );

    let mut final_command_bytes = header_string.into_bytes();
    final_command_bytes.extend(encoded_data); // image data
    final_command_bytes.extend(b"\nEI\n"); // End Image marker

    Ok(final_command_bytes)
}

/// image converted to RGB format and embedded.
/// Use `push_state()` and `set_matrix()` before this call to position and scale the image.
pub fn inline_image_from_file(path: &str, compression_method:CompressionMethod) -> PdfResult<Vec<u8>> {
    let img = image::open(path).map_err(|e| {
        PdfError::InvalidImage(format!("Failed to load image from {}: {}", path, e))
    })?;

    let rgb_img = img.to_rgb8();
    let (width_pixels, height_pixels) = rgb_img.dimensions();

    inline_image(
        width_pixels,
        height_pixels,
        ColorSpace::RGB,
        8,
        &rgb_img.into_raw(),
        compression_method,
    )
}

pub fn line_to_x_y(posn: Posn) -> Vec<u8> {
    build_op(&[&posn], "l")
}

pub fn move_to_x_y(posn: Posn) -> Vec<u8> {
    build_op(&[&posn], "m")
}

pub fn move_text_to_next_line_at(posn: Posn) -> Vec<u8> {
    build_op(&[&posn], "T*")
}

pub fn paint_shading(name: &str) -> Vec<u8> {
    let mut cmd = b"/".to_vec();
    cmd.extend(name.as_bytes());
    cmd.extend(b" sh");

    cmd
}

pub fn pop_state() -> Vec<u8> {
    b"Q".to_vec()
}

pub fn push_state() -> Vec<u8> {
    b"q".to_vec()
}

pub fn add_rectangle(posn: Posn, size: Dims) -> Vec<u8> {
    build_op(&[&posn, &size], "re")
}

pub fn set_color_rgb(rgb: RGB, stroke: StrokeOrFill) -> Vec<u8> {
    let operator = match stroke {
        StrokeOrFill::Stroke => "RG",
        StrokeOrFill::Fill => "rg",
    };
    build_op(&[&rgb], operator)
}

pub fn set_color_cmyk(cmyk: CMYK, stroke: StrokeOrFill) -> Vec<u8> {
    let operator = match stroke {
        StrokeOrFill::Stroke => "K",
        StrokeOrFill::Fill => "k",
    };
    build_op(&[&cmyk], operator)
}

pub fn set_color_grayscale(grayscale: Color, stroke: StrokeOrFill) -> Vec<u8> {
    let operator = match stroke {
        StrokeOrFill::Stroke => "G",
        StrokeOrFill::Fill => "g",
    };
    build_op(&[&grayscale], operator)
}

pub fn set_color_space(space: &str, stroke: StrokeOrFill) -> Vec<u8> {
    let operator = match stroke {
        StrokeOrFill::Stroke => "CS",
        StrokeOrFill::Fill => "cs",
    };

    format!("/ {space} {operator}").into_bytes()
}

pub fn set_color_special(name: Option<&str>, stroke: StrokeOrFill, operands: &[f64]) -> Vec<u8> {
    let mut cmd_parts = operands
        .iter()
        .map(|&n| f_to_pdf_num(n))
        .collect::<Vec<String>>();
    if let Some(n) = name {
        cmd_parts.push(format!("/{n}"));
    }
    cmd_parts.push(
        (match stroke {
            StrokeOrFill::Stroke => "SCN",
            StrokeOrFill::Fill => "scn",
        })
        .to_string(),
    );

    cmd_parts.join(" ").into_bytes()
}

// font

pub fn set_font_name_and_size(font: &str, size: f64) -> Vec<u8> {
    format!("/{} {} Tf", font, f_to_pdf_num(size)).into_bytes()
}

// text

pub fn set_text_rendering_mode(mode: i32) -> Vec<u8> {
    int_cmd("Tr", mode)
}

pub fn set_text_rise(height: f64) -> Vec<u8> {
    float_cmd("Ts", height)
}

/// Set current text and text line transformation matrix.
pub fn set_text_matrix(matrix: Matrix) -> Vec<u8> {
    build_op(&[&matrix], "Tm")
}

/// Set text position without scaling, rotation, or skewing.
///
/// equivalent to calling `set_text_matrix` with an identity matrix.
pub fn set_text_position(posn: Posn) -> Vec<u8> {
    set_text_matrix(Matrix {
        a: 1.0,
        b: 0.0,
        c: 0.0,
        d: 1.0,
        e: posn.x,
        f: posn.y,
    })
}

pub fn show_text_strings(text: &str) -> Vec<u8> {
    format!("[{text}] TJ").into_bytes()
}

pub fn show_single_text_string(text: &str) -> Vec<u8> {
    let mut cmd = encode_pdf_string(text);
    cmd.push_str(" Tj");

    Vec::from(cmd)
}

// line

pub fn set_dash_line_pattern(dash_array: &[f64], dash_phase: i32) -> Vec<u8> {
    // Build the [n n n] part directly
    let array_str: Vec<String> = dash_array.iter().map(|&n| f_to_pdf_num(n)).collect();
    let cmd = format!("[{}] {} d", array_str.join(" "), dash_phase).into_bytes();

    cmd
}

pub fn set_line_cap_style(line_cap: i32) -> Vec<u8> {
    int_cmd("J", line_cap)
}

pub fn set_line_join_style(line_join: i32) -> Vec<u8> {
    int_cmd("j", line_join)
}

pub fn set_line_width(width: f64) -> Vec<u8> {
    float_cmd("w", width)
}

// matrix

pub fn set_transformation_matrix(matrix: Matrix) -> Vec<u8> {
    build_op(&[&matrix], "cm")
}

// mitre

pub fn set_miter_limit(miter_limit: f64) {
    float_cmd("M", miter_limit);
}

// state

/// Set specified parameters in graphic state.
pub fn set_state(state_name: &str) -> Vec<u8> {
    format!("/{state_name} gs").into_bytes()
}

// stroke

pub fn stroke_path() -> Vec<u8> {
    b"S".to_vec()
}

pub fn stroke_and_close_path() -> Vec<u8> {
    b"s".to_vec()
}

// rounded rectangle

fn draw_corner(radius: f64, size: Dims, rel_corner_pos: Posn) -> Vec<u8> {
    const KAPPA: f64 = 0.5522847498307933; // makes cubic Bezier curve like circular arc
    if radius < 0.0001 {
        return vec![];
    }

    let Posn { x, y } = rel_corner_pos;
    let Dims { width, height } = size;
    curve_to(
        Posn {
            x: x + width - radius + radius * KAPPA,
            y: y + height,
        },
        Posn {
            x: x + width,
            y: y + height - radius + radius * KAPPA,
        },
        Posn {
            x: x + width,
            y: y + height - radius,
        },
    )
}

pub fn add_rounded_rectangle(
    posn: Posn,
    dims: Dims,
    radius_top_left: f64,
    radius_top_right: f64,
    radius_bottom_right: f64,
    radius_bottom_left: f64,
) -> Vec<u8> {
    let Posn { x, y } = posn;
    let Dims { width, height } = dims;

    let mut vec: Vec<u8> = vec![];

    vec.extend(move_to_x_y(Posn {
        x: x + radius_top_left,
        y: y + height,
    }));

    vec.extend(draw_corner(
        // top right
        radius_top_left,
        dims,
        Posn {
            x: width,
            y: height,
        },
    ));

    vec.extend(line_to_x_y(Posn {
        // right
        x: x + width - radius_top_right,
        y: y + height,
    }));

    vec.extend(draw_corner(
        radius_top_right,
        dims,
        Posn { x: width, y: 0.0 },
    )); // bottom right

    vec.extend(line_to_x_y(Posn {
        // bottom
        x: x + width,
        y: y + radius_bottom_right,
    }));

    vec.extend(draw_corner(
        radius_bottom_right,
        dims,
        Posn { x: 0.0, y: 0.0 },
    )); // bottom left

    vec.extend(line_to_x_y(Posn { x: x + width, y })); // left

    vec.extend(draw_corner(
        radius_bottom_left,
        dims,
        Posn { x: 0.0, y: height },
    )); // top left

    vec.extend(line_to_x_y(Posn {
        // top
        x: x + radius_bottom_left,
        y,
    }));

    vec.push(close());

    vec
}

pub fn apply_gradient_pattern(
    pattern_name: &str,
    stroke: StrokeOrFill,
    graphics_state_name: Option<&str>,
) -> Vec<u8> {
    let mut vec: Vec<u8> = vec![];
    if let Some(gs) = graphics_state_name {
        vec.extend(set_state(gs));
    }
    vec.extend(set_color_space("Pattern", stroke));
    vec.extend(set_color_special(Some(pattern_name), stroke, &[]));

    vec
}
