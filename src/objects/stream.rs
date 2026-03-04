use flate2::Compression;
use flate2::write::ZlibEncoder;
use std::collections::HashMap;
use std::io::Write as IoWrite;

use crate::encoding::{ascii85_encode, to_pdf_num};
use crate::error::{PdfError, Result};
use crate::objects::metadata::PdfMetadata;
use crate::objects::string::encode_pdf_string;
use crate::{DictionaryObject, PdfObject};

#[derive(Debug, Clone, Copy, PartialEq)]
enum StrokeOrFill {
    Stroke,
    Fill,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum EvenOdd {
    Even,
    Odd,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum CompressionMethod {
    None,
    Flate,
}

/*pub struct StreamObject {
    pub metadata: PdfMetadata,
    pub values: Vec<u8>,
}

impl StreamObject {
    pub fn new(values: Vec<u8>) -> Self {
        Self {
            metadata: PdfMetadata::default(),
            values,
        }
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
        let mut result = b"<< /Length ".to_vec();
        result.extend(self.values.len().to_string().as_bytes());
        result.extend(b" >>\nstream\n");
        result.extend(&self.values);
        result.extend(b"\nendstream");
        result
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn is_compressible(&self) -> bool {
        false // Streams are not usually compressible in object streams
    }
}
*/
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

    fn validate_color(&self, values: &[f64]) -> Result<()> {
        for &v in values {
            if !(0.0..=1.0).contains(&v) {
                return Err(PdfError::InvalidColor { r: v, g: v, b: v }); // map to first error
            }
        }
        Ok(())
    }

    fn push_op(&mut self, operands: &[f64], operator: &str) {
        let mut cmd_parts: Vec<String> = operands.iter().map(|&n| to_pdf_num(n)).collect();
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
    pub fn curve_to(&mut self, x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64) {
        self.push_op(&[x1, y1, x2, y2, x3, y3], "c");
    }

    /// Add cubic Bézier curve to current path.
    ///
    /// Extend curve to `(x3, y3)` using current point, and `(x2, y2)` as Bézier control points.
    pub fn curve_start_to(&mut self, x2: f64, y2: f64, x3: f64, y3: f64) {
        self.push_op(&[x2, y2, x3, y3], "v");
    }

    /// Add cubic Bézier curve to current path.
    ///
    /// extend curve to `(x3, y3)` using `(x1, y1)`, and `(x3, y3)` as Bézier control points.
    pub fn curve_end_to(&mut self, x1: f64, y1: f64, x3: f64, y3: f64) {
        self.push_op(&[x1, y1, x3, y3], "y");
    }

    pub fn draw_x_object(&mut self, reference: &str) {
        self.stream.push(format!("/{} Do", reference).into_bytes());
    }

    /// End path without filling or stroking.
    pub fn end(&mut self) {
        self.stream.push(b"n".to_vec());
    }

    /// End marked-content sequence.
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

        let data_to_encode =
            match self.compress {
                CompressionMethod::Flate => {
                    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
                    encoder.write_all(raw_pixel_data)?;
                    encoder.finish()?
                }
                CompressionMethod::None =>
                    raw_pixel_data.to_vec()
            };

        let mut encoded_data = ascii85_encode(&data_to_encode);
        encoded_data.extend(b"~>"); // ASCII85 end marker

        let filters = match self.compress{
            CompressionMethod::Flate => "/A85 /Fl" ,
            CompressionMethod::None => "/A85" };

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

    pub fn line_to_x_y(&mut self, x: f64, y: f64) {
        self.push_op(&[x, y], "l");
    }

    /// Begin new subpath by moving current point to `(x, y)`.
    pub fn move_to_x_y(&mut self, x: f64, y: f64) {
        self.push_op(&[x, y], "m");
    }

    /// Move text to next line at `(x, y)` distance from previous line.
    pub fn move_text_to_x_y(&mut self, x: f64, y: f64) {
        self.push_op(&[x, y], "T*");
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
    /// `(x, y)` is the lower-left corner and width and height the dimensions.
    pub fn rectangle(&mut self, x: f64, y: f64, width: f64, height: f64) {
        self.push_op(&[x, y, width, height], "re");
    }

    /// Set RGB color for non-stroking operations.
    ///
    /// Set RGB color for stroking operations instead if `stroke` is set to `true`.
    /// Returns an error if color values are not in range 0.0-1.0.
    pub fn set_color_rgb(&mut self, r: f64, g: f64, b: f64, stroke: StrokeOrFill) -> Result<()> {
        self.validate_color(&[r, g, b])?;
        let operator = match stroke {
            StrokeOrFill::Stroke => "RG",
            StrokeOrFill::Fill => "rg",
        };
        self.push_op(&[r, g, b], operator);
        Ok(())
    }

    /// Set CMYK color for non-stroking operations.
    ///
    /// Set CMYK color for stroking operations instead if `stroke` is set to `true`.
    /// Returns an error if color values are not in range 0.0-1.0.
    pub fn set_color_cmyk(
        &mut self,
        c: f64,
        m: f64,
        y: f64,
        k: f64,
        stroke: StrokeOrFill,
    ) -> Result<()> {
        self.validate_color(&[c, m, y, k])?;
        let operator = match stroke {
            StrokeOrFill::Stroke => "K",
            StrokeOrFill::Fill => "k",
        };
        self.push_op(&[c, m, y, k], operator);
        Ok(())
    }

    /// Set grayscale color for non-stroking operations.
    ///
    /// Set grayscale color for stroking operations instead if `stroke` is set to `true`.
    /// Returns an error if gray value is not in range 0.0-1.0.
    pub fn set_color_gray(&mut self, gray: f64, stroke: StrokeOrFill) -> Result<()> {
        self.validate_color(&[gray])?;
        let operator = match stroke {
            StrokeOrFill::Stroke => "G",
            StrokeOrFill::Fill => "g",
        };
        self.push_op(&[gray], operator);
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

    pub fn set_transformation_matrix(&mut self, a: f64, b: f64, c: f64, d: f64, e: f64, f: f64) {
        self.push_op(&[a, b, c, d, e, f], "cm");
    }

    pub fn set_miter_limit(&mut self, miter_limit: f64) {
        self.float_cmd("M", miter_limit);
    }

    /// Set specified parameters in graphic state.
    pub fn set_state(&mut self, state_name: &str) {
        self.stream.push(format!("/{state_name} gs").into_bytes());
    }
    /// Set current text and text line transformation matrix.
    pub fn set_text_matrix(&mut self, a: f64, b: f64, c: f64, d: f64, e: f64, f: f64) {
        self.push_op(&[a, b, c, d, e, f], "Tm");
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
        pos_x: f64,
        pos_y: f64,
        width: f64,
        height: f64,
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
                pos_x + width - radius + radius * KAPPA,
                pos_y + height,
                pos_x + width,
                pos_y + height - radius + radius * KAPPA,
                pos_x + width,
                pos_y + height - radius,
            );
        };

        self.move_to_x_y(pos_x + radius_top_left, pos_y + height);

        draw_corner(self, radius_top_left, [width, height]); // top right
        self.line_to_x_y(pos_x + width - radius_top_right, pos_y + height); // right
        draw_corner(self, radius_top_right, [width, 0.0]); // bottom right
        self.line_to_x_y(pos_x + width, pos_y + radius_bottom_right); // bottom
        draw_corner(self, radius_bottom_right, [0.0, 0.0]); // bottom left
        self.line_to_x_y(pos_x + width, pos_y); // left
        draw_corner(self, radius_bottom_left, [0.0, height]); // top left
        self.line_to_x_y(pos_x + radius_bottom_left, pos_y); // top

        self.close();
    }

    /// Apply a gradient pattern
    ///
    /// Sets the color space to Pattern and applies the specified pattern for fill or stroke.
    /// If a graphics state name is provided (for transparency), it will be applied first.
    ///
    /// # Arguments
    ///
    /// * `pattern_name` - Name of the pattern resource (e.g., "P0")
    /// * `stroke` - Whether to apply for stroke (true) or fill (false)
    /// * `gs_name` - Optional graphics state name for transparency
    pub fn apply_gradient_pattern(
        &mut self,
        pattern_name: &str,
        stroke: StrokeOrFill,
        gs_name: Option<&str>,
    ) {
        if let Some(gs) = gs_name {
            self.set_state(gs); // Apply soft mask graphics state if provided
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

        match self.compress {CompressionMethod::Flate => {
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

