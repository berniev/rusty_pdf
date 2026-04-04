use crate::color::{CMYK, Color, ColorSpace, RGB};
use crate::encoding::{ascii85_encode, f_to_pdf_num};
use crate::objects::string::encode_pdf_string;
use crate::util::{Dims, Matrix, Posn, StrokeOrFill, ToPdf, WindingRule};
use crate::{CompressionMethod, PdfError, PdfStreamObject};
use flate2::Compression;
use flate2::write::ZlibEncoder;
use std::io::Write;

//-------------------------- Drawing Commands --------------------------

pub struct DrawingCommands<'a> {
    pub(crate) stream: &'a mut PdfStreamObject,
}

impl<'a> DrawingCommands<'a> {
    pub fn new(stream: &'a mut PdfStreamObject) -> DrawingCommands<'a> {
        Self { stream }
    }

    //-------------------------- Helper Methods --------------------------

    fn add(&mut self, cmd: Vec<u8>) {
        self.stream.add(cmd);
    }

    fn add_windable_cmd(&mut self, cmd: char, even_odd: WindingRule) {
        let mut op_bytes = vec![cmd as u8];
        match even_odd {
            WindingRule::EvenOdd => op_bytes.push(b'*'),
            WindingRule::NonZero => op_bytes.push(b' '),
        }
        self.add(op_bytes);
    }

    fn add_cmd(&mut self, cmd: char) {
        self.add(vec![cmd as u8]);
    }

    fn add_parts(&mut self, operands: &[&dyn ToPdf], operator: &str) {
        let mut cmd_parts: Vec<String> = operands.iter().map(|n| n.to_pdf()).collect();
        cmd_parts.push(operator.to_string());

        self.add(cmd_parts.join(" ").into_bytes());
    }

    fn add_float_cmd(&mut self, string: &str, value: f64) {
        self.add(format!("{} {}", f_to_pdf_num(value), string).into_bytes());
    }

    fn add_int_cmd(&mut self, string: &str, value: i32) {
        self.add_float_cmd(string, value as f64)
    }

    //-------------------------- Drawing Commands --------------------------

    pub fn begin_marked_content(&mut self, tag: &str, property_list: Option<Vec<u8>>) {
        self.add(match property_list {
            None => format!("/{tag} BMC").into_bytes(),

            Some(props) => {
                let mut cmd = format!("/{tag} ").into_bytes();
                cmd.extend(props);
                cmd.extend(b" BDC");
                cmd
            }
        });
    }

    /// Use the nonzero winding number rule to determine which regions lie inside the clipping path by default.
    pub fn clip(&mut self, even_odd: WindingRule) {
        self.add_windable_cmd('W', even_odd)
    }

    pub fn close(&mut self) {
        self.add_cmd('h')
    }

    /// extend curve from `pos3` using `pos1` and `pos2` as Bézier control points.
    pub fn curve_to(&mut self, pos1: Posn, pos2: Posn, pos3: Posn) {
        self.add_parts(&[&pos1, &pos2, &pos3], "c")
    }

    /// Extend curve to `pos3` using current point, and `pos2` as Bézier control points.
    pub fn curve_start_to(&mut self, pos2: Posn, pos3: Posn) {
        self.add_parts(&[&pos2, &pos3], "v")
    }

    /// extend curve to `pos3` using `pos1`, and `pos3` as Bézier control points.
    pub fn curve_end_to(&mut self, pos1: Posn, pos3: Posn) {
        self.add_parts(&[&pos1, &pos3], "y")
    }

    pub fn draw_x_object(&mut self, reference: &str) {
        self.add(format!("/{} Do", reference).into_bytes());
    }

    /// End path without filling or stroking.
    pub fn end(&mut self) {
        self.add(b"n".to_vec());
    }

    pub fn end_marked_content(&mut self) {
        self.add(b"EMC".to_vec());
    }

    pub fn begin_text(&mut self) {
        self.add(b"BT".to_vec());
    }

    pub fn end_text(&mut self) {
        self.add(b"ET".to_vec());
    }

    pub fn fill(&mut self, even_odd: WindingRule) {
        self.add_windable_cmd('f', even_odd);
    }

    pub fn fill_and_stroke(&mut self, even_odd: WindingRule) {
        self.add_windable_cmd('B', even_odd);
    }

    pub fn fill_stroke_and_close(&mut self, even_odd: WindingRule) {
        self.add_windable_cmd('b', even_odd);
    }

    pub fn inline_image(
        &mut self,
        width_pixels: u32,
        height_pixels: u32,
        color_space: ColorSpace,
        bits_per_component: u8, // typ 8
        raw_pixel_data: &[u8],
        compression_method: CompressionMethod,
    ) {
        if width_pixels == 0 || height_pixels == 0 {
            let msg = format!("Invalid image dimensions: {width_pixels} x {height_pixels} pixels",);
            PdfError::InvalidImage(msg);
        }

        let data_to_encode = match compression_method {
            CompressionMethod::Flate => {
                let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
                encoder
                    .write_all(raw_pixel_data)
                    .expect("Failed to write image data");
                encoder.finish().expect("Failed to finish encoding")
            }
            CompressionMethod::None => raw_pixel_data.to_vec(),
        };

        let mut encoded_data = ascii85_encode(&data_to_encode);
        encoded_data.extend(b"~>"); // ASCII85 end marker

        self.add(
            format!(
                "BI /W {} /H {} /BPC {} /CS /Device{} /F {} /L {} ID\n",
                width_pixels,
                height_pixels,
                bits_per_component,
                color_space,
                compression_method.to_string(),
                encoded_data.len()
            )
            .into_bytes(),
        );

        self.add(encoded_data); // image data
        self.add(b"\nEI\n".to_vec()); // End Image marker
    }

    /// image converted to RGB format and embedded.
    /// Use `push_state()` and `set_matrix()` before this call to position and scale the image.
    pub fn inline_image_from_file(&mut self, path: &str, compression_method: CompressionMethod) {
        let img = image::open(path).expect(&format!("Failed to load image from {}", path));

        let rgb_img = img.to_rgb8();
        let (width_pixels, height_pixels) = rgb_img.dimensions();

        self.inline_image(
            width_pixels,
            height_pixels,
            ColorSpace::RGB,
            8,
            &rgb_img.into_raw(),
            compression_method,
        );
    }

    pub fn line_to_x_y(&mut self, posn: Posn) {
        self.add_parts(&[&posn], "l")
    }

    pub fn move_text_to_next_line_at(&mut self, posn: Posn) {
        self.add_parts(&[&posn], "T*")
    }

    pub fn move_to_x_y(&mut self, posn: Posn) {
        self.add_parts(&[&posn], "m")
    }

    pub fn paint_shading(&mut self, name: &str) {
        let mut cmd = b"/".to_vec();
        cmd.extend(name.as_bytes());
        cmd.extend(b" sh");

        self.add(cmd);
    }

    pub fn pop_state(&mut self) {
        self.add_cmd('Q')
    }

    pub fn push_state(&mut self) {
        self.add_cmd('q');
    }

    pub fn rectangle(&mut self, posn: Posn, size: Dims) {
        self.add_parts(&[&posn, &size], "re")
    }

    pub fn set_color_rgb(&mut self, rgb: RGB, stroke: StrokeOrFill) {
        let operator = match stroke {
            StrokeOrFill::Stroke => "RG",
            StrokeOrFill::Fill => "rg",
        };
        self.add_parts(&[&rgb], operator);
    }

    pub fn set_color_cmyk(&mut self, cmyk: CMYK, stroke: StrokeOrFill) {
        let operator = match stroke {
            StrokeOrFill::Stroke => "K",
            StrokeOrFill::Fill => "k",
        };
        self.add_parts(&[&cmyk], operator)
    }

    pub fn set_color_grayscale(&mut self, grayscale: Color, stroke: StrokeOrFill) {
        let operator = match stroke {
            StrokeOrFill::Stroke => "G",
            StrokeOrFill::Fill => "g",
        };
        self.add_parts(&[&grayscale], operator)
    }

    pub fn set_color_space(&mut self, space: &str, stroke: StrokeOrFill) {
        let operator = match stroke {
            StrokeOrFill::Stroke => "CS",
            StrokeOrFill::Fill => "cs",
        };

        self.add(format!("/ {space} {operator}").into_bytes());
    }

    pub fn set_color_special(
        &mut self,
        name: Option<&str>,
        stroke: StrokeOrFill,
        operands: &[f64],
    ) {
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

        self.add(cmd_parts.join(" ").into_bytes());
    }

    // font

    pub fn set_font_name_and_size(&mut self, font: &str, size: f64) {
        self.add(format!("/{} {} Tf", font, f_to_pdf_num(size)).into_bytes());
    }

    // text

    pub fn set_text_rendering_mode(&mut self, mode: i32) {
        self.add_int_cmd("Tr", mode);
    }

    pub fn set_text_rise(&mut self, height: f64) {
        self.add_float_cmd("Ts", height)
    }

    /// Set current text and text line transformation matrix.
    pub fn set_text_matrix(&mut self, matrix: Matrix) {
        self.add_parts(&[&matrix], "Tm")
    }

    /// Set text position without scaling, rotation, or skewing.
    ///
    /// equivalent to calling `set_text_matrix` with an identity matrix.
    pub fn set_text_position(&mut self, posn: Posn) {
        self.set_text_matrix(Matrix {
            a: 1.0,
            b: 0.0,
            c: 0.0,
            d: 1.0,
            e: posn.x,
            f: posn.y,
        })
    }

    pub fn show_single_text_string(&mut self, text: &str) {
        let mut cmd = encode_pdf_string(text);
        cmd.push_str(" Tj");

        self.add(Vec::from(cmd));
    }

    pub fn show_text_strings(&mut self, text: &str) {
        self.add(format!("[{text}] TJ").into_bytes());
    }

    // line

    pub fn set_dash_line_pattern(&mut self, dash_array: &[f64], dash_phase: i32) {
        // Build the [n n n] part directly
        let array_str: Vec<String> = dash_array.iter().map(|&n| f_to_pdf_num(n)).collect();
        let cmd = format!("[{}] {} d", array_str.join(" "), dash_phase).into_bytes();

        self.add(cmd);
    }

    pub fn set_line_cap_style(&mut self, line_cap: i32) {
        self.add_int_cmd("J", line_cap)
    }

    pub fn set_line_join_style(&mut self, line_join: i32) {
        self.add_int_cmd("j", line_join)
    }

    pub fn set_line_width(&mut self, width: f64) {
        self.add_float_cmd("w", width)
    }

    // matrix

    pub fn set_transformation_matrix(&mut self, matrix: Matrix) {
        self.add_parts(&[&matrix], "cm")
    }

    // mitre

    pub fn set_miter_limit(&mut self, miter_limit: f64) {
        self.add_float_cmd("M", miter_limit);
    }

    // state

    pub fn set_state(&mut self, state_name: &str) {
        self.add(format!("/{state_name} gs").into_bytes());
    }

    // stroke

    pub fn stroke_path(&mut self) {
        self.add_cmd('S');
    }

    pub fn stroke_and_close_path(&mut self) {
        self.add_cmd('s');
    }

    // rounded rectangle

    fn draw_corner(&mut self, radius: f64, size: Dims, rel_corner_pos: Posn) {
        const KAPPA: f64 = 0.5522847498307933; // makes cubic Bezier curve like circular arc
        if radius < 0.0001 {
            return;
        }

        let Posn { x, y } = rel_corner_pos;
        let Dims { width, height } = size;
        self.curve_to(
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

    pub fn rounded_rectangle(
        &mut self,
        posn: Posn,
        dims: Dims,
        radius_top_left: f64,
        radius_top_right: f64,
        radius_bottom_right: f64,
        radius_bottom_left: f64,
    ) {
        let Posn { x, y } = posn;
        let Dims { width, height } = dims;

        self.move_to_x_y(Posn {
            x: x + radius_top_left,
            y: y + height,
        });

        self.draw_corner(
            // top right
            radius_top_left,
            dims,
            Posn {
                x: width,
                y: height,
            },
        );

        self.line_to_x_y(Posn {
            // right
            x: x + width - radius_top_right,
            y: y + height,
        });

        self.draw_corner(radius_top_right, dims, Posn { x: width, y: 0.0 }); // bottom right

        self.line_to_x_y(Posn {
            // bottom
            x: x + width,
            y: y + radius_bottom_right,
        });

        self.draw_corner(radius_bottom_right, dims, Posn { x: 0.0, y: 0.0 }); // bottom left

        self.line_to_x_y(Posn { x: x + width, y }); // left

        self.draw_corner(radius_bottom_left, dims, Posn { x: 0.0, y: height }); // top left

        self.line_to_x_y(Posn {
            // top
            x: x + radius_bottom_left,
            y,
        });

        self.close();
    }

    pub fn apply_gradient_pattern(
        &mut self,
        pattern_name: &str,
        stroke: StrokeOrFill,
        graphics_state_name: Option<&str>,
    ) {
        if let Some(gs) = graphics_state_name {
            self.set_state(gs);
        }
        self.set_color_space("Pattern", stroke);
        self.set_color_special(Some(pattern_name), stroke, &[]);
    }
}
