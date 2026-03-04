use crate::encoding::{ascii85_encode, to_pdf_num};
use crate::error::{PdfError, Result};
use crate::objects::metadata::PdfMetadata;
use crate::objects::string::encode_pdf_string;
use crate::{DictionaryObject, PdfObject};
use flate2::Compression;
use flate2::write::ZlibEncoder;
use std::collections::HashMap;
use std::io::Write as IoWrite;

trait ToPdf {
    fn to_pdf(&self) -> String;
}

impl ToPdf for f64 {
    fn to_pdf(&self) -> String {
        format!("{}", to_pdf_num(*self))
    }
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum StrokeOrFill {
    Stroke,
    Fill,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EvenOdd {
    Even,
    Odd,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CompressionMethod {
    None,
    Flate,
}

#[derive(Debug)]
pub struct Color {
    pub color: f64,
}

impl Color {
    pub fn validate(&self) -> Result<()> {
        if !(0.0..=1.0).contains(&self.color) {
            return Err(PdfError::InvalidColorChannel {
                color: { Color { color: self.color } },
            });
        }

        Ok(())
    }
}

impl ToPdf for Color {
    fn to_pdf(&self) -> String {
        format!("{}", to_pdf_num(self.color))
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RGB {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}

impl RGB {
    pub fn validate(&self) -> Result<()> {
        for &v in &[self.red, self.green, self.blue] {
            if !(0.0..=1.0).contains(&v) {
                return Err(PdfError::InvalidRGB {
                    rgb: RGB {
                        red: self.red,
                        green: self.green,
                        blue: self.blue,
                    },
                });
            }
        }
        Ok(())
    }
}

impl ToPdf for RGB {
    fn to_pdf(&self) -> String {
        format!(
            "{} {} {}",
            to_pdf_num(self.red),
            to_pdf_num(self.green),
            to_pdf_num(self.blue),
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RGBA {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
    pub alpha: f64,
}

impl RGBA {
    pub fn validate(&self) -> Result<()> {
        for &v in &[self.red, self.green, self.blue, self.alpha] {
            if !(0.0..=1.0).contains(&v) {
                return Err(PdfError::InvalidRGBA {
                    rgb: RGBA {
                        red: self.red,
                        green: self.green,
                        blue: self.blue,
                        alpha: self.alpha,
                    },
                });
            }
        }
        Ok(())
    }
}

impl ToPdf for RGBA {
    fn to_pdf(&self) -> String {
        format!(
            "{} {} {} {}",
            to_pdf_num(self.red),
            to_pdf_num(self.green),
            to_pdf_num(self.blue),
            to_pdf_num(self.alpha)
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CMYK {
    pub cyan: f64,
    pub magenta: f64,
    pub yellow: f64,
    pub black: f64,
}

impl CMYK {
    pub fn validate(&self) -> Result<()> {
        for &v in &[self.cyan, self.magenta, self.yellow, self.black] {
            if !(0.0..=1.0).contains(&v) {
                return Err(PdfError::InvalidCMYK {
                    cmyk: CMYK {
                        cyan: self.cyan,
                        magenta: self.magenta,
                        yellow: self.yellow,
                        black: self.black,
                    },
                });
            }
        }
        Ok(())
    }
}

impl ToPdf for CMYK {
    fn to_pdf(&self) -> String {
        format!(
            "{} {} {} {}",
            to_pdf_num(self.cyan),
            to_pdf_num(self.magenta),
            to_pdf_num(self.yellow),
            to_pdf_num(self.black)
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PosnXY {
    pub x: f64,
    pub y: f64,
}

impl ToPdf for PosnXY {
    fn to_pdf(&self) -> String {
        format!("{} {}", to_pdf_num(self.x), to_pdf_num(self.y),)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Matrix {
    pub a: f64,
    pub b: f64,
    pub c: f64,
    pub d: f64,
    pub e: f64,
    pub f: f64,
}

impl Matrix {
    pub fn new(a: f64, b: f64, c: f64, d: f64, e: f64, f: f64) -> Self {
        Matrix { a, b, c, d, e, f }
    }
}

impl ToPdf for Matrix {
    fn to_pdf(&self) -> String {
        format!(
            "{} {} {} {} {} {}",
            to_pdf_num(self.a),
            to_pdf_num(self.b),
            to_pdf_num(self.c),
            to_pdf_num(self.d)
            to_pdf_num(self.e)
            to_pdf_num(self.f)
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Size {
    pub width: f64,
    pub height: f64,
}

impl ToPdf for Size {
    fn to_pdf(&self) -> String {
        format!("{} {}", to_pdf_num(self.width), to_pdf_num(self.height),)
    }
}

/// PDF content stream.
///
/// A Stream represents a sequence of PDF graphics and text operators.
/// Content streams are used to define page content, including:
/// - Graphics: paths, rectangles, curves
/// - Text: fonts, positioning, display
/// - Colors: RGB, CMYK, grayscale
/// - Images: inline images
/// - Transformations: matrices, state management
pub struct StreamObject {
    pub metadata: PdfMetadata,
    pub stream: Vec<Vec<u8>>, // sequence of operator calls
    pub extra: HashMap<String, Vec<u8>>,
    pub compress: CompressionMethod, // using flate
}

impl Default for StreamObject {
    fn default() -> Self {
        StreamObject {
            metadata: PdfMetadata::default(),
            stream: Vec::new(),
            extra: HashMap::new(),
            compress: CompressionMethod::None,
        }
    }
}

/// to specify stream and dictionary, use with_data()
impl StreamObject {
    pub fn new() -> Self {
        StreamObject {
            compress: CompressionMethod::None,
            ..Default::default()
        }
    }

    pub fn new_compressed() -> Self {
        let mut s = Self::new();
        s.compress = CompressionMethod::Flate;
        s
    }

    /// * `stream` - Optional pre-existing stream content
    /// * `extra` - Optional extra dictionary entries
    pub fn with_data(
        mut self,
        stream: Option<Vec<Vec<u8>>>,
        extra: Option<HashMap<String, Vec<u8>>>,
    ) -> Self {
        if let Some(s) = stream {
            self.stream = s;
        }
        if let Some(e) = extra {
            self.extra = e;
        }
        self
    }

    fn push_op(&mut self, operands: &[&dyn ToPdf], operator: &str) {
        let mut cmd_parts: Vec<String> = operands.iter().map(|n| n.to_pdf()).collect();

        cmd_parts.push(operator.to_string());
        self.stream.push(cmd_parts.join(" ").into_bytes());
    }

    fn cmd(&mut self, cmd: char) {
        self.stream.push(vec![cmd as u8]);
    }

    fn windable_cmd(&mut self, cmd: char, even_odd: EvenOdd) {
        let mut op_bytes = vec![cmd as u8];
        match even_odd {
            EvenOdd::Odd => op_bytes.push(b'*'),
            EvenOdd::Even => op_bytes.push(b' '),
        }
        self.stream.push(op_bytes);
    }

    fn float_cmd(&mut self, string: &str, value: f64) {
        self.stream
            .push(format!("{} {}", to_pdf_num(value), string).into_bytes());
    }

    fn int_cmd(&mut self, string: &str, value: i32) {
        self.float_cmd(string, value as f64);
    }

    pub fn begin_marked_content(&mut self, tag: &str, property_list: Option<Vec<u8>>) {
        match property_list {
            None => {
                self.stream.push(format!("/{tag} BMC").into_bytes());
            }

            Some(props) => {
                let mut cmd = format!("/{tag} ").into_bytes();
                cmd.extend(props);
                cmd.extend(b" BDC");
                self.stream.push(cmd);
            }
        }
    }

    pub fn begin_text(&mut self) {
        self.stream.push(b"BT".to_vec());
    }

    /// Modify current clipping path by intersecting it with current path.
    ///
    /// Use the nonzero winding number rule to determine which regions lie inside the clipping path by default.
    /// Use the even-odd rule if `even_odd` set to `true`.
    pub fn clip(&mut self, even_odd: EvenOdd) {
        self.windable_cmd('W', even_odd);
    }

    /// Close current subpath.
    ///
    /// Append a straight line segment from the current point to the starting point of the subpath.
    pub fn close(&mut self) {
        self.cmd('h');
    }

    /// Add cubic Bézier curve to current path.
    ///
    /// extend curve from `(x3, y3)` using `(x1, y1)` and `(x2, y2)` as Bézier control points.
    pub fn curve_to(&mut self, pos1: PosnXY, pos2: PosnXY, pos3: PosnXY) {
        self.push_op(&[&pos1, &pos2, &pos3], "c");
    }

    /// Add cubic Bézier curve to current path.
    ///
    /// Extend curve to `(x3, y3)` using current point, and `(x2, y2)` as Bézier control points.
    pub fn curve_start_to(&mut self, pos2: PosnXY, pos3: PosnXY) {
        self.push_op(&[&pos2, &pos3], "v");
    }

    /// Add cubic Bézier curve to current path.
    ///
    /// extend curve to `(x3, y3)` using `(x1, y1)`, and `(x3, y3)` as Bézier control points.
    pub fn curve_end_to(&mut self, pos1: PosnXY, pos3: PosnXY) {
        self.push_op(&[&pos1, &pos3], "y");
    }

    pub fn draw_x_object(&mut self, reference: &str) {
        self.stream.push(format!("/{} Do", reference).into_bytes());
    }

    /// End path without filling or stroking.
    pub fn end(&mut self) {
        self.stream.push(b"n".to_vec());
    }

    pub fn end_marked_content(&mut self) {
        self.stream.push(b"EMC".to_vec());
    }

    pub fn end_text(&mut self) {
        self.stream.push(b"ET".to_vec());
    }

    pub fn fill(&mut self, even_odd: EvenOdd) {
        self.windable_cmd('f', even_odd);
    }

    pub fn fill_and_stroke(&mut self, even_odd: EvenOdd) {
        self.windable_cmd('B', even_odd);
    }

    pub fn fill_stroke_and_close(&mut self, even_odd: EvenOdd) {
        self.windable_cmd('b', even_odd);
    }

    /// Add an inline image from raw pixel data.
    ///
    /// # Arguments
    /// * `width` - Image width in pixels
    /// * `height` - Image height in pixels
    /// * `color_space` - Color space: "RGB", "Gray", or "CMYK"
    /// * `bits_per_component` - Bits per color component (typically 8)
    /// * `raw_pixel_data` - Raw pixel data bytes
    pub fn inline_image(
        &mut self,
        width: u32,
        height: u32,
        color_space: &str,
        bits_per_component: u8,
        raw_pixel_data: &[u8],
    ) -> Result<()> {
        if width == 0 || height == 0 {
            return Err(PdfError::InvalidImage(format!(
                "Invalid image dimensions: {}x{}",
                width, height
            )));
        }

        let valid_spaces = ["RGB", "Gray", "CMYK"];
        if !valid_spaces.contains(&color_space) {
            return Err(PdfError::InvalidImage(format!(
                "Invalid color space: {}. Must be one of: RGB, Gray, CMYK",
                color_space
            )));
        }

        let data_to_encode = match self.compress {
            CompressionMethod::Flate => {
                let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
                encoder.write_all(raw_pixel_data)?;
                encoder.finish()?
            }
            CompressionMethod::None => raw_pixel_data.to_vec(),
        };

        let mut encoded_data = ascii85_encode(&data_to_encode);
        encoded_data.extend(b"~>"); // ASCII85 end marker

        let filters = match self.compress {
            CompressionMethod::Flate => "/A85 /Fl",
            CompressionMethod::None => "/A85",
        };

        let header_string = format!(
            "BI /W {} /H {} /BPC {} /CS /Device{} /F {} /L {} ID ",
            to_pdf_num(width as f64),
            to_pdf_num(height as f64),
            to_pdf_num(bits_per_component as f64),
            color_space,
            filters,
            encoded_data.len()
        );

        let mut final_command_bytes = header_string.into_bytes();
        final_command_bytes.extend(encoded_data); // image data
        final_command_bytes.extend(b" EI"); // End Image marker

        self.stream.push(final_command_bytes);

        Ok(())
    }

    /// Load and add an inline image from a file (PNG, JPEG, etc.).
    ///
    /// The image will be automatically converted to RGB format and embedded.
    /// Use `push_state()` and `set_matrix()` before this call to position and scale the image.
    pub fn inline_image_from_file(&mut self, path: &str) -> Result<()> {
        // Load image from file
        let img = image::open(path).map_err(|e| {
            PdfError::InvalidImage(format!("Failed to load image from {}: {}", path, e))
        })?;

        // Convert to RGB
        let rgb_img = img.to_rgb8();
        let (width, height) = rgb_img.dimensions();

        // Get raw pixel data
        let raw_data = rgb_img.into_raw();

        // Add as inline image
        self.inline_image(width, height, "RGB", 8, &raw_data)
    }

    pub fn line_to_x_y(&mut self, posn: PosnXY) {
        self.push_op(&[&posn], "l");
    }

    /// Begin new subpath by moving current point to `(x, y)`.
    pub fn move_to_x_y(&mut self, posn: PosnXY) {
        self.push_op(&[&posn], "m");
    }

    /// Move text to next line at `(x, y)` distance from previous line.
    pub fn move_text_to_x_y(&mut self, posn: PosnXY) {
        self.push_op(&[&posn], "T*");
    }

    /// Paint shape and color shading using shading dictionary `name`.
    pub fn paint_shading(&mut self, name: &str) {
        let mut cmd = b"/".to_vec();
        cmd.extend(name.as_bytes());
        cmd.extend(b" sh");
        self.stream.push(cmd);
    }

    pub fn pop_state(&mut self) {
        self.stream.push(b"Q".to_vec());
    }

    pub fn push_state(&mut self) {
        self.stream.push(b"q".to_vec());
    }

    /// Add rectangle to current path as complete subpath.
    ///
    /// `posn` is the lower-left corner and `size` the dimensions.
    pub fn rectangle(&mut self, posn: PosnXY, size: Size) {
        self.push_op(&[&posn, &size], "re");
    }

    /// Set RGB color for non-stroking operations.
    ///
    /// Set RGB color for stroking operations instead if `stroke` is set to `true`.
    /// Returns an error if color values are not in range 0.0-1.0.
    pub fn set_color_rgb(&mut self, rgb: RGB, stroke: StrokeOrFill) -> Result<()> {
        rgb.validate()?;
        let operator = match stroke {
            StrokeOrFill::Stroke => "RG",
            StrokeOrFill::Fill => "rg",
        };
        self.push_op(&[&rgb], operator);
        Ok(())
    }

    /// Set CMYK color for non-stroking operations.
    ///
    /// Set CMYK color for stroking operations instead if `stroke` is set to `true`.
    /// Returns an error if color values are not in range 0.0-1.0.
    pub fn set_color_cmyk(&mut self, cmyk: CMYK, stroke: StrokeOrFill) -> Result<()> {
        cmyk.validate()?;
        let operator = match stroke {
            StrokeOrFill::Stroke => "K",
            StrokeOrFill::Fill => "k",
        };
        self.push_op(&[&cmyk], operator);
        Ok(())
    }

    /// Set grayscale color for non-stroking operations.
    ///
    /// Set grayscale color for stroking operations instead if `stroke` is set to `true`.
    /// Returns an error if gray value is not in range 0.0-1.0.
    pub fn set_color_grayscale(&mut self, grayscale: Color, stroke: StrokeOrFill) -> Result<()> {
        grayscale.validate()?;
        let operator = match stroke {
            StrokeOrFill::Stroke => "G",
            StrokeOrFill::Fill => "g",
        };
        self.push_op(&[&grayscale], operator);
        Ok(())
    }

    /// Set the non-stroking color space. stroke=`true` set stroking color space instead.
    pub fn set_color_space(&mut self, space: &str, stroke: StrokeOrFill) {
        let operator = match stroke {
            StrokeOrFill::Stroke => "CS",
            StrokeOrFill::Fill => "cs",
        };
        self.stream
            .push(format!("/ {space} {operator}").into_bytes());
    }

    /// Set special color. For non-stroking operations unless `stroke`=`true` (stroking operation)
    pub fn set_color_special(
        &mut self,
        name: Option<&str>,
        stroke: StrokeOrFill,
        operands: &[f64],
    ) {
        let mut cmd_parts: Vec<String> = operands.iter().map(|&n| to_pdf_num(n)).collect();
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
        self.stream.push(cmd_parts.join(" ").into_bytes());
    }

    pub fn set_dash_line_pattern(&mut self, dash_array: &[f64], dash_phase: i32) {
        // Build the [n n n] part directly
        let array_str: Vec<String> = dash_array.iter().map(|&n| to_pdf_num(n)).collect();

        // Build the entire command in one single allocation
        let cmd = format!("[{}] {} d", array_str.join(" "), dash_phase).into_bytes();

        self.stream.push(cmd);
    }
    /// Set font name and size.
    pub fn set_font_name_and_size(&mut self, font: &str, size: f64) {
        self.stream
            .push(format!("/{} {} Tf", font, to_pdf_num(size)).into_bytes());
    }

    pub fn set_text_rendering_mode(&mut self, mode: i32) {
        self.int_cmd("Tr", mode);
    }

    pub fn set_text_rise(&mut self, height: f64) {
        self.float_cmd("Ts", height);
    }

    pub fn set_line_cap_style(&mut self, line_cap: i32) {
        self.int_cmd("J", line_cap);
    }

    pub fn set_line_join_style(&mut self, line_join: i32) {
        self.int_cmd("j", line_join);
    }

    pub fn set_line_width(&mut self, width: f64) {
        self.float_cmd("w", width);
    }

    pub fn set_transformation_matrix(&mut self, matrix: Matrix) {
        self.push_op(&[&matrix], "cm");
    }

    pub fn set_miter_limit(&mut self, miter_limit: f64) {
        self.float_cmd("M", miter_limit);
    }

    /// Set specified parameters in graphic state.
    pub fn set_state(&mut self, state_name: &str) {
        self.stream.push(format!("/{state_name} gs").into_bytes());
    }
    /// Set current text and text line transformation matrix.
    pub fn set_text_matrix(&mut self, matrix: Matrix) {
        self.push_op(&[&matrix], "Tm");
    }

    pub fn show_text_strings(&mut self, text: &str) {
        self.stream.push(format!("[{text}] TJ").into_bytes());
    }

    pub fn show_single_text_string(&mut self, text: &str) {
        let mut cmd = encode_pdf_string(text);
        cmd.extend(b" Tj");
        self.stream.push(cmd);
    }

    pub fn stroke_path(&mut self) {
        self.stream.push(b"S".to_vec());
    }

    pub fn stroke_and_close_path(&mut self) {
        self.stream.push(b"s".to_vec());
    }

    pub fn rounded_rectangle(
        &mut self,
        posn: PosnXY,
        size: Size,
        radius_top_left: f64,
        radius_top_right: f64,
        radius_bottom_right: f64,
        radius_bottom_left: f64,
    ) {
        let draw_corner = |s: &mut StreamObject, radius: f64, rel_corner_pos: [f64; 2]| {
            if radius < 0.0001 {
                return;
            }

            const KAPPA: f64 = 0.5522847498307933; // makes cubic Bezier curve like circular arc
            s.curve_to(
                PosnXY {
                    x: posn.x + size.width - radius + radius * KAPPA,
                    y: posn.y + size.height,
                },
                PosnXY {
                    x: posn.x + size.width,
                    y: posn.y + size.height - radius + radius * KAPPA,
                },
                PosnXY {
                    x: posn.x + size.width,
                    y: posn.y + size.height - radius,
                },
            );
        };

        self.move_to_x_y(PosnXY {
            x: posn.x + radius_top_left,
            y: posn.y + size.height,
        });

        // top right
        draw_corner(self, radius_top_left, [size.width, size.height]);
        // right
        self.line_to_x_y(PosnXY {
            x: posn.x + size.width - radius_top_right,
            y: posn.y + size.height,
        });

        // bottom right
        draw_corner(self, radius_top_right, [size.width, 0.0]);

        // bottom
        self.line_to_x_y(PosnXY {
            x: posn.x + size.width,
            y: posn.y + radius_bottom_right,
        });

        // bottom left
        draw_corner(self, radius_bottom_right, [0.0, 0.0]);

        // left
        self.line_to_x_y(PosnXY {
            x: posn.x + size.width,
            y: posn.y,
        });

        // top left
        draw_corner(self, radius_bottom_left, [0.0, size.height]);

        // top
        self.line_to_x_y(PosnXY {
            x: posn.x + radius_bottom_left,
            y: posn.y,
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
            self.set_state(gs); // Apply provided soft mask graphics state
        }
        self.set_color_space("Pattern", stroke);
        self.set_color_special(Some(pattern_name), stroke, &[]);
    }
}

impl PdfObject for StreamObject {
    fn metadata(&self) -> &PdfMetadata {
        &self.metadata
    }

    fn metadata_mut(&mut self) -> &mut PdfMetadata {
        &mut self.metadata
    }

    fn data(&self) -> Vec<u8> {
        let mut stream_bytes = self.stream.join(&b'\n');
        let mut extra = self.extra.clone();

        match self.compress {
            CompressionMethod::Flate => {
                let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
                encoder.write_all(&stream_bytes).unwrap();
                stream_bytes = encoder.finish().unwrap();
            }
            CompressionMethod::None => {}
        };

        extra.insert(
            "Length".to_string(),
            stream_bytes.len().to_string().into_bytes(),
        );

        let extra_dict = DictionaryObject {
            metadata: PdfMetadata::default(),
            values: extra,
        };

        let mut result = extra_dict.data();
        result.extend(b"\nstream\n");
        result.extend(stream_bytes);
        result.extend(b"\nendstream");

        result
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    /// Stream objects are never compressible in PDF object streams.
    fn is_compressible(&self) -> bool {
        false
    }
}
