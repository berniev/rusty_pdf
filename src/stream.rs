use flate2::Compression;
use flate2::write::ZlibEncoder;
use std::collections::HashMap;
use std::io::Write as IoWrite;

use crate::dictionary::Dictionary;
use crate::encoding::{ascii85_encode, to_pdf_num};
use crate::error::{PdfError, Result};
use crate::object::{PdfMetadata, PdfObject};
use crate::string::encode_pdf_string;

/// PDF content stream.
///
/// A Stream represents a sequence of PDF graphics and text operators.
/// Content streams are used to define page content, including:
/// - Graphics: paths, rectangles, curves
/// - Text: fonts, positioning, display
/// - Colors: RGB, CMYK, grayscale
/// - Images: inline images
/// - Transformations: matrices, state management
///
/// # Example
///
/// ```rust
/// use pydyf::Stream;
///
/// let mut stream = Stream::new();
/// stream.set_color_rgb(1.0, 0.0, 0.0, false).unwrap();
/// stream.rectangle(100.0, 100.0, 200.0, 150.0);
/// stream.fill(false);
/// ```
pub struct Stream {
    pub metadata: PdfMetadata,
    pub stream: Vec<Vec<u8>>, // sequence of operator calls
    pub extra: HashMap<String, Vec<u8>>,
    pub compress: bool, // using flate
}

impl Default for Stream {
    fn default() -> Self {
        Stream {
            metadata: PdfMetadata::default(),
            stream: Vec::new(),
            extra: HashMap::new(),
            compress: false,
        }
    }
}

/// to specify stream and dictionary, use with_data()
impl Stream {
    pub fn new() -> Self {
        Stream {
            compress: false,
            ..Default::default()
        }
    }

    pub fn new_compressed() -> Self {
        let mut s = Self::new();
        s.compress = true;
        s
    }

    /// * `stream` - Optional pre-existing stream content
    /// * `extra` - Optional extra dictionary entries
    /// ```
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

    fn windable_cmd(&mut self, cmd: char, even_odd: bool) {
        let mut op_bytes = vec![cmd as u8];
        if even_odd {
            op_bytes.push(b'*');
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
    /// Use the nonzero winding number rule to determine which regions lie
    /// inside the clipping path by default.
    ///
    /// Use the even-odd rule if `even_odd` set to `true`.
    pub fn clip(&mut self, even_odd: bool) {
        self.windable_cmd('W', even_odd);
    }

    /// Close current subpath.
    ///
    /// Append a straight line segment from the current point to the starting
    /// point of the subpath.
    pub fn close(&mut self) {
        self.cmd('h');
    }

    /// Add cubic Bézier curve to current path.
    ///
    /// The curve shall extend from `(x3, y3)` using `(x1, y1)` and `(x2, y2)`
    /// as the Bézier control points.
    pub fn curve_to(&mut self, x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64) {
        self.push_op(&[x1, y1, x2, y2, x3, y3], "c");
    }

    /// Add cubic Bézier curve to current path.
    ///
    /// The curve shall extend to `(x3, y3)` using the current point and
    /// `(x2, y2)` as the Bézier control points.
    pub fn curve_start_to(&mut self, x2: f64, y2: f64, x3: f64, y3: f64) {
        self.push_op(&[x2, y2, x3, y3], "v");
    }

    /// Add cubic Bézier curve to current path.
    ///
    /// The curve shall extend to `(x3, y3)` using `(x1, y1)` and `(x3, y3)`
    /// as the Bézier control points.
    pub fn curve_end_to(&mut self, x1: f64, y1: f64, x3: f64, y3: f64) {
        self.push_op(&[x1, y1, x3, y3], "y");
    }

    /// Draw object given by reference.
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

    /// End text object.
    pub fn end_text(&mut self) {
        self.stream.push(b"ET".to_vec());
    }

    pub fn fill(&mut self, even_odd: bool) {
        self.windable_cmd('f', even_odd);
    }

    pub fn fill_and_stroke(&mut self, even_odd: bool) {
        self.windable_cmd('B', even_odd);
    }

    pub fn fill_stroke_and_close(&mut self, even_odd: bool) {
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

        let data_to_encode = if self.compress {
            let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
            encoder.write_all(raw_pixel_data)?;
            encoder.finish()?
        } else {
            raw_pixel_data.to_vec()
        };

        let mut encoded_data = ascii85_encode(&data_to_encode);
        encoded_data.extend(b"~>"); // Required PDF ASCII85 end marker

        let filters = if self.compress { "/A85 /Fl" } else { "/A85" };

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
    ///
    /// # Example
    /// ```no_run
    /// # use pydyf::Stream;
    /// let mut stream = Stream::new();
    /// stream.push_state();
    /// stream.set_matrix(200.0, 0.0, 0.0, 200.0, 50.0, 500.0); // Scale to 200x200 at (50, 500)
    /// stream.inline_image_from_file("photo.jpg").unwrap();
    /// stream.pop_state();
    /// ```
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

    /// Add line from current point to point `(x, y)`.
    pub fn line_to(&mut self, x: f64, y: f64) {
        self.push_op(&[x, y], "l");
    }

    /// Begin new subpath by moving current point to `(x, y)`.
    pub fn move_to(&mut self, x: f64, y: f64) {
        self.push_op(&[x, y], "m");
    }

    /// Move text to next line at `(x, y)` distance from previous line.
    pub fn move_text_to(&mut self, x: f64, y: f64) {
        self.push_op(&[x, y], "T*");
    }

    /// Paint shape and color shading using shading dictionary `name`.
    pub fn paint_shading(&mut self, name: &str) {
        let mut cmd = b"/".to_vec();
        cmd.extend(name.as_bytes());
        cmd.extend(b" sh");
        self.stream.push(cmd);
    }

    /// Restore graphic state.
    pub fn pop_state(&mut self) {
        self.stream.push(b"Q".to_vec());
    }

    /// Save graphic state.
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
    pub fn set_color_rgb(&mut self, r: f64, g: f64, b: f64, stroke: bool) -> Result<()> {
        self.validate_color(&[r, g, b])?;
        let operator = if stroke { "RG" } else { "rg" };
        self.push_op(&[r, g, b], operator);
        Ok(())
    }

    /// Set CMYK color for non-stroking operations.
    ///
    /// Set CMYK color for stroking operations instead if `stroke` is set to `true`.
    /// Returns an error if color values are not in range 0.0-1.0.
    pub fn set_color_cmyk(&mut self, c: f64, m: f64, y: f64, k: f64, stroke: bool) -> Result<()> {
        self.validate_color(&[c, m, y, k])?;
        let operator = if stroke { "K" } else { "k" };
        self.push_op(&[c, m, y, k], operator);
        Ok(())
    }

    /// Set grayscale color for non-stroking operations.
    ///
    /// Set grayscale color for stroking operations instead if `stroke` is set to `true`.
    /// Returns an error if gray value is not in range 0.0-1.0.
    pub fn set_color_gray(&mut self, gray: f64, stroke: bool) -> Result<()> {
        self.validate_color(&[gray])?;
        let operator = if stroke { "G" } else { "g" };
        self.push_op(&[gray], operator);
        Ok(())
    }

    /// Set the non-stroking color space.
    ///
    /// If stroke is set to `true`, set the stroking color space instead.
    pub fn set_color_space(&mut self, space: &str, stroke: bool) {
        let operator = if stroke { "CS" } else { "cs" };
        self.stream
            .push(format!("/ {} {}", space, operator).into_bytes());
    }

    /// Set special color for non-stroking operations.
    ///
    /// Set special color for stroking operation if `stroke` is set to `true`.
    pub fn set_color_special(&mut self, name: Option<&str>, stroke: bool, operands: &[f64]) {
        let mut cmd_parts: Vec<String> = operands.iter().map(|&n| to_pdf_num(n)).collect();
        if let Some(n) = name {
            cmd_parts.push(format!("/{}", n));
        }
        cmd_parts.push((if stroke { "SCN" } else { "scn" }).to_string());
        self.stream.push(cmd_parts.join(" ").into_bytes());
    }

    /// Set dash line pattern.
    pub fn set_dash(&mut self, dash_array: &[f64], dash_phase: i32) {
        // Build the [n n n] part directly
        let array_str: Vec<String> = dash_array.iter().map(|&n| to_pdf_num(n)).collect();

        // Build the entire command in one single allocation
        let cmd = format!("[{}] {} d", array_str.join(" "), dash_phase).into_bytes();

        self.stream.push(cmd);
    }
    /// Set font name and size.
    pub fn set_font_size(&mut self, font: &str, size: f64) {
        self.stream
            .push(format!("/{} {} Tf", font, to_pdf_num(size)).into_bytes());
    }
    /// Set text rendering mode.
    pub fn set_text_rendering(&mut self, mode: i32) {
        self.int_cmd("Tr", mode);
    }

    /// Set text rise.
    pub fn set_text_rise(&mut self, height: f64) {
        self.float_cmd("Ts", height);
    }

    /// Set line cap style.
    pub fn set_line_cap(&mut self, line_cap: i32) {
        self.int_cmd("J", line_cap);
    }

    /// Set line join style.
    pub fn set_line_join(&mut self, line_join: i32) {
        self.int_cmd("j", line_join);
    }

    /// Set line width.
    pub fn set_line_width(&mut self, width: f64) {
        self.float_cmd("w", width);
    }

    /// Set current transformation matrix.
    pub fn set_matrix(&mut self, a: f64, b: f64, c: f64, d: f64, e: f64, f: f64) {
        self.push_op(&[a, b, c, d, e, f], "cm");
    }

    /// Set miter limit.
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

    /// Show text strings with individual glyph positioning.
    pub fn show_text(&mut self, text: &str) {
        self.stream.push(format!("[{text}] TJ").into_bytes());
    }

    /// Show single text string.
    pub fn show_text_string(&mut self, text: &str) {
        let mut cmd = encode_pdf_string(text);
        cmd.extend(b" Tj");
        self.stream.push(cmd);
    }

    /// Stroke path.
    pub fn stroke(&mut self) {
        self.stream.push(b"S".to_vec());
    }

    /// Stroke and close path.
    pub fn stroke_and_close(&mut self) {
        self.stream.push(b"s".to_vec());
    }

    /// Draw a rounded rectangle using Bezier curves.
    ///
    /// Creates a rectangle with rounded corners at the specified position and size.
    /// Corner radii can be specified individually for each corner.
    ///
    /// # Arguments
    ///
    /// * `x` - X coordinate of the rectangle (left edge)
    /// * `y` - Y coordinate of the rectangle (bottom edge in PDF coordinates)
    /// * `width` - Width of the rectangle
    /// * `height` - Height of the rectangle
    /// * `top_left` - Radius of the top-left corner
    /// * `top_right` - Radius of the top-right corner
    /// * `bottom_right` - Radius of the bottom-right corner
    /// * `bottom_left` - Radius of the bottom-left corner
    ///
    /// # Example
    ///
    /// ```rust
    /// use pydyf::Stream;
    ///
    /// let mut stream = Stream::new();
    /// stream.set_color_rgb(1.0, 0.0, 0.0, false).unwrap();
    /// stream.rounded_rectangle(100.0, 100.0, 200.0, 150.0, 10.0, 10.0, 10.0, 10.0);
    /// stream.fill(false);
    /// ```
    pub fn rounded_rectangle(
        &mut self,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        top_left: f64,
        top_right: f64,
        bottom_right: f64,
        bottom_left: f64,
    ) {
        // If no radius, draw simple rectangle
        if top_left == 0.0 && top_right == 0.0 && bottom_right == 0.0 && bottom_left == 0.0 {
            self.rectangle(x, y, width, height);
            return;
        }

        // KAPPA constant for circular arcs with cubic Bezier
        // This is the magic number that makes a cubic Bezier curve approximate a circular arc
        const KAPPA: f64 = 0.5522847498307933;

        // Start at top-left corner (after radius)
        self.move_to(x + top_left, y + height);

        // Top edge and top-right corner
        if top_right > 0.0 {
            self.line_to(x + width - top_right, y + height);
            self.curve_to(
                x + width - top_right + top_right * KAPPA,
                y + height,
                x + width,
                y + height - top_right + top_right * KAPPA,
                x + width,
                y + height - top_right,
            );
        } else {
            self.line_to(x + width, y + height);
        }

        // Right edge and bottom-right corner
        if bottom_right > 0.0 {
            self.line_to(x + width, y + bottom_right);
            self.curve_to(
                x + width,
                y + bottom_right - bottom_right * KAPPA,
                x + width - bottom_right + bottom_right * KAPPA,
                y,
                x + width - bottom_right,
                y,
            );
        } else {
            self.line_to(x + width, y);
        }

        // Bottom edge and bottom-left corner
        if bottom_left > 0.0 {
            self.line_to(x + bottom_left, y);
            self.curve_to(
                x + bottom_left - bottom_left * KAPPA,
                y,
                x,
                y + bottom_left - bottom_left * KAPPA,
                x,
                y + bottom_left,
            );
        } else {
            self.line_to(x, y);
        }

        // Left edge and top-left corner
        if top_left > 0.0 {
            self.line_to(x, y + height - top_left);
            self.curve_to(
                x,
                y + height - top_left + top_left * KAPPA,
                x + top_left - top_left * KAPPA,
                y + height,
                x + top_left,
                y + height,
            );
        } else {
            self.line_to(x, y + height);
        }

        self.close();
    }

    /// Apply a gradient pattern to the stream.
    ///
    /// Sets the color space to Pattern and applies the specified pattern for fill or stroke.
    /// If a graphics state name is provided (for transparency), it will be applied first.
    ///
    /// # Arguments
    ///
    /// * `pattern_name` - Name of the pattern resource (e.g., "P0")
    /// * `stroke` - Whether to apply for stroke (true) or fill (false)
    /// * `gs_name` - Optional graphics state name for transparency
    pub fn apply_pattern(&mut self, pattern_name: &str, stroke: bool, gs_name: Option<&str>) {
        // Apply soft mask graphics state if provided
        if let Some(gs) = gs_name {
            self.set_state(gs);
        }

        // Set Pattern color space
        self.set_color_space("Pattern", stroke);

        // Apply the pattern
        self.set_color_special(Some(pattern_name), stroke, &[]);
    }
}

impl PdfObject for Stream {
    fn metadata(&self) -> &PdfMetadata {
        &self.metadata
    }

    fn metadata_mut(&mut self) -> &mut PdfMetadata {
        &mut self.metadata
    }

    fn data(&self) -> Vec<u8> {
        let mut stream_bytes = self.stream.join(&b'\n');
        let mut extra = self.extra.clone();

        if self.compress {
            extra.insert("Filter".to_string(), b"/FlateDecode".to_vec());
            let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
            encoder.write_all(&stream_bytes).unwrap();
            stream_bytes = encoder.finish().unwrap();
        }

        extra.insert(
            "Length".to_string(),
            stream_bytes.len().to_string().into_bytes(),
        );

        let extra_dict = Dictionary {
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
