use flate2::Compression;
use flate2::write::ZlibEncoder;
use std::collections::HashMap;
use std::io::Write as IoWrite;

use crate::array::Array;
use crate::dictionary::Dictionary;
use crate::encoding::{ascii85_encode, to_bytes_num};
use crate::error::{PdfError, Result};
use crate::object::{PdfObject, PdfMetadata};
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
/// let mut stream = Stream::new(None, None, false);
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

impl Stream {
    /// Create a new content stream.
    ///
    /// # Arguments
    ///
    /// * `stream` - Optional pre-existing stream content
    /// * `extra` - Optional extra dictionary entries
    /// * `compress` - Enable flate compression
    ///
    /// # Example
    ///
    /// ```rust
    /// use pydyf::Stream;
    ///
    /// // Simple uncompressed stream
    /// let stream = Stream::new(None, None, false);
    ///
    /// // Compressed stream
    /// let compressed = Stream::new(None, None, true);
    /// ```
    pub fn new(
        stream: Option<Vec<Vec<u8>>>,
        extra: Option<HashMap<String, Vec<u8>>>,
        compress: bool,
    ) -> Self {
        Stream {
            metadata: PdfMetadata::default(),
            stream: stream.unwrap_or_default(),
            extra: extra.unwrap_or_default(),
            compress,
        }
    }

    fn cmd(&mut self, cmd: char) {
        self.stream.push(vec![cmd as u8]);
    }

    fn windable_cmd(&mut self, cmd: char, even_odd: bool) {
        let mut cmd = vec![cmd as u8];
        if even_odd {
            cmd.push(b'*');
        }
        self.stream.push(cmd);
    }

    fn float_cmd(&mut self, string: &str, value: f64) {
        let mut cmd = to_bytes_num(value);
        cmd.push(b' ');
        cmd.extend(string.as_bytes());
        self.stream.push(cmd);
    }

    fn int_cmd(&mut self, string: &str, value: i32) {
        self.float_cmd(string, value as f64);
    }

    pub fn begin_marked_content(&mut self, tag: &str, property_list: Option<Vec<u8>>) {
        self.stream.push(format!("/{tag}").into_bytes());

        if let Some(props) = property_list {
            self.stream.push(props);
            self.stream.push(b"BDC".to_vec());
        } else {
            self.stream.push(b"BMC".to_vec());
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
        let parts = vec![
            to_bytes_num(x1),
            to_bytes_num(y1),
            to_bytes_num(x2),
            to_bytes_num(y2),
            to_bytes_num(x3),
            to_bytes_num(y3),
            b"c".to_vec(),
        ];
        self.stream.push(parts.join(&b' '));
    }

    /// Add cubic Bézier curve to current path.
    ///
    /// The curve shall extend to `(x3, y3)` using the current point and
    /// `(x2, y2)` as the Bézier control points.
    pub fn curve_start_to(&mut self, x2: f64, y2: f64, x3: f64, y3: f64) {
        let parts = vec![
            to_bytes_num(x2),
            to_bytes_num(y2),
            to_bytes_num(x3),
            to_bytes_num(y3),
            b"v".to_vec(),
        ];
        self.stream.push(parts.join(&b' '));
    }

    /// Add cubic Bézier curve to current path.
    ///
    /// The curve shall extend to `(x3, y3)` using `(x1, y1)` and `(x3, y3)`
    /// as the Bézier control points.
    pub fn curve_end_to(&mut self, x1: f64, y1: f64, x3: f64, y3: f64) {
        let parts = vec![
            to_bytes_num(x1),
            to_bytes_num(y1),
            to_bytes_num(x3),
            to_bytes_num(y3),
            b"y".to_vec(),
        ];
        self.stream.push(parts.join(&b' '));
    }

    /// Draw object given by reference.
    pub fn draw_x_object(&mut self, reference: &str) {
        let mut cmd = b"/".to_vec();
        cmd.extend(reference.as_bytes());
        cmd.extend(b" Do");
        self.stream.push(cmd);
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
        self.windable_cmd('S', even_odd);
    }

    pub fn fill_stroke_and_close(&mut self, even_odd: bool) {
        self.windable_cmd('s', even_odd);
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

        // Validate dimensions
        if width == 0 || height == 0 {
            return Err(PdfError::InvalidImage(
                format!("Invalid image dimensions: {}x{}", width, height)
            ));
        }

        // Validate color space
        let valid_spaces = ["RGB", "Gray", "CMYK"];
        if !valid_spaces.contains(&color_space) {
            return Err(PdfError::InvalidImage(
                format!("Invalid color space: {}. Must be one of: RGB, Gray, CMYK", color_space)
            ));
        }

        let data = if self.compress {
            let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
            encoder.write_all(raw_pixel_data)?;
            encoder.finish()?
        } else {
            raw_pixel_data.to_vec()
        };

        let mut a85_data = ascii85_encode(&data);
        a85_data.extend(b"~>");

        let mut parts = vec![
            b"BI".to_vec(),
            b"/W".to_vec(),
            to_bytes_num(width as f64),
            b"/H".to_vec(),
            to_bytes_num(height as f64),
            b"/BPC".to_vec(),
            to_bytes_num(bits_per_component as f64),
            b"/CS".to_vec(),
        ];

        let mut device = b"/Device".to_vec();
        device.extend(color_space.as_bytes());
        parts.push(device);

        parts.push(b"/F".to_vec());
        parts.push( b"/A85".to_vec());
        if self.compress {
            parts.push(b" /Fl".to_vec());
        };

        parts.push(b"/L".to_vec());
        parts.push(to_bytes_num(a85_data.len() as f64));
        parts.push(b"ID".to_vec());
        parts.push(a85_data);
        parts.push(b"EI".to_vec());

        self.stream.push(parts.join(&b' '));
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
    /// let mut stream = Stream::new(None, None, false);
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
        let parts = vec![to_bytes_num(x), to_bytes_num(y), b"l".to_vec()];
        self.stream.push(parts.join(&b' '));
    }

    /// Begin new subpath by moving current point to `(x, y)`.
    pub fn move_to(&mut self, x: f64, y: f64) {
        let parts = vec![to_bytes_num(x), to_bytes_num(y), b"m".to_vec()];
        self.stream.push(parts.join(&b' '));
    }

    /// Move text to next line at `(x, y)` distance from previous line.
    pub fn move_text_to(&mut self, x: f64, y: f64) {
        let parts = vec![to_bytes_num(x), to_bytes_num(y), b"Td".to_vec()];
        self.stream.push(parts.join(&b' '));
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
        let parts = vec![
            to_bytes_num(x),
            to_bytes_num(y),
            to_bytes_num(width),
            to_bytes_num(height),
            b"re".to_vec(),
        ];
        self.stream.push(parts.join(&b' '));
    }

    /// Set RGB color for non-stroking operations.
    ///
    /// Set RGB color for stroking operations instead if `stroke` is set to `true`.
    /// Returns an error if color values are not in range 0.0-1.0.
    pub fn set_color_rgb(&mut self, r: f64, g: f64, b: f64, stroke: bool) -> Result<()> {

        // Validate color values
        if !(0.0..=1.0).contains(&r) || !(0.0..=1.0).contains(&g) || !(0.0..=1.0).contains(&b) {
            return Err(PdfError::InvalidColor { r, g, b });
        }

        let parts = vec![
            to_bytes_num(r),
            to_bytes_num(g),
            to_bytes_num(b),
            if stroke {
                b"RG".to_vec()
            } else {
                b"rg".to_vec()
            },
        ];
        self.stream.push(parts.join(&b' '));
        Ok(())
    }

    /// Set CMYK color for non-stroking operations.
    ///
    /// Set CMYK color for stroking operations instead if `stroke` is set to `true`.
    /// Returns an error if color values are not in range 0.0-1.0.
    pub fn set_color_cmyk(&mut self, c: f64, m: f64, y: f64, k: f64, stroke: bool) -> Result<()> {
        // Validate color values
        if !(0.0..=1.0).contains(&c) || !(0.0..=1.0).contains(&m)
            || !(0.0..=1.0).contains(&y) || !(0.0..=1.0).contains(&k) {
            return Err(PdfError::InvalidColor { r: c, g: m, b: y });
        }

        let parts = vec![
            to_bytes_num(c),
            to_bytes_num(m),
            to_bytes_num(y),
            to_bytes_num(k),
            if stroke {
                b"K".to_vec()
            } else {
                b"k".to_vec()
            },
        ];
        self.stream.push(parts.join(&b' '));
        Ok(())
    }

    /// Set grayscale color for non-stroking operations.
    ///
    /// Set grayscale color for stroking operations instead if `stroke` is set to `true`.
    /// Returns an error if gray value is not in range 0.0-1.0.
    pub fn set_color_gray(&mut self, gray: f64, stroke: bool) -> Result<()> {
        // Validate gray value
        if !(0.0..=1.0).contains(&gray) {
            return Err(PdfError::InvalidColor { r: gray, g: gray, b: gray });
        }

        let parts = vec![
            to_bytes_num(gray),
            if stroke {
                b"G".to_vec()
            } else {
                b"g".to_vec()
            },
        ];
        self.stream.push(parts.join(&b' '));
        Ok(())
    }

    /// Set the non-stroking color space.
    ///
    /// If stroke is set to `true`, set the stroking color space instead.
    pub fn set_color_space(&mut self, space: &str, stroke: bool) {
        let mut cmd = b"/".to_vec();
        cmd.extend(space.as_bytes());
        cmd.push(b' ');
        cmd.extend(if stroke { b"CS" } else { b"cs" });
        self.stream.push(cmd);
    }

    /// Set special color for non-stroking operations.
    ///
    /// Set special color for stroking operation if `stroke` is set to `true`.
    pub fn set_color_special(&mut self, name: Option<&str>, stroke: bool, operands: &[f64]) {
        let mut parts: Vec<Vec<u8>> = operands.iter().map(|&op| to_bytes_num(op)).collect();

        if let Some(n) = name {
            let mut name_part = b"/".to_vec();
            name_part.extend(n.as_bytes());
            parts.push(name_part);
        }

        let mut cmd = parts.join(&b' ');
        cmd.push(b' ');
        cmd.extend(if stroke { b"SCN" } else { b"scn" });
        self.stream.push(cmd);
    }

    /// Set dash line pattern.
    pub fn set_dash(&mut self, dash_array: &[f64], dash_phase: i32) {
        let array = Array::new(Some(dash_array.to_vec()));
        let parts = vec![array.data(), to_bytes_num(dash_phase as f64), b"d".to_vec()];
        self.stream.push(parts.join(&b' '));
    }

    /// Set font name and size.
    pub fn set_font_size(&mut self, font: &str, size: f64) {
        let mut cmd = b"/".to_vec();
        cmd.extend(font.as_bytes());
        cmd.push(b' ');
        cmd.extend(to_bytes_num(size));
        cmd.extend(b" Tf");
        self.stream.push(cmd);
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
        let parts = vec![
            to_bytes_num(a),
            to_bytes_num(b),
            to_bytes_num(c),
            to_bytes_num(d),
            to_bytes_num(e),
            to_bytes_num(f),
            b"cm".to_vec(),
        ];
        self.stream.push(parts.join(&b' '));
    }

    /// Set miter limit.
    pub fn set_miter_limit(&mut self, miter_limit: f64) {
        self.float_cmd("M", miter_limit);
    }

    /// Set specified parameters in graphic state.
    pub fn set_state(&mut self, state_name: &str) {
        let mut cmd = b"/".to_vec();
        cmd.extend(state_name.as_bytes());
        cmd.extend(b" gs");
        self.stream.push(cmd);
    }

    /// Set current text and text line transformation matrix.
    pub fn set_text_matrix(&mut self, a: f64, b: f64, c: f64, d: f64, e: f64, f: f64) {
        let parts = vec![
            to_bytes_num(a),
            to_bytes_num(b),
            to_bytes_num(c),
            to_bytes_num(d),
            to_bytes_num(e),
            to_bytes_num(f),
            b"Tm".to_vec(),
        ];
        self.stream.push(parts.join(&b' '));
    }

    /// Show text strings with individual glyph positioning.
    pub fn show_text(&mut self, text: &str) {
        let mut cmd = b"[".to_vec();
        cmd.extend(text.as_bytes());
        cmd.extend(b"] TJ");
        self.stream.push(cmd);
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
}

impl PdfObject for Stream {
    fn metadata(&self) -> &PdfMetadata {
        &self.metadata
    }

    fn metadata_mut(&mut self) -> &mut PdfMetadata {
        &mut self.metadata
    }

    fn data(&self) -> Vec<u8> {
        // Use the existing Stream::data() implementation
        let stream_data: Vec<Vec<u8>> = self.stream.iter().map(|item| item.clone()).collect();
        let mut stream = stream_data.join(&b'\n');
        let mut extra = self.extra.clone();

        if self.compress {
            extra.insert("Filter".to_string(), b"/FlateDecode".to_vec());
            let mut encoder = ZlibEncoder::new(Vec::new(), Compression::new(9));
            encoder.write_all(&stream).unwrap();
            stream = encoder.finish().unwrap();
        }

        extra.insert("Length".to_string(), stream.len().to_string().into_bytes());
        let extra_dict = Dictionary {
            metadata: PdfMetadata::default(),
            values: extra,
        };

        let parts = vec![
            extra_dict.data(),
            b"stream".to_vec(),
            stream,
            b"endstream".to_vec(),
        ];
        parts.join(&b'\n')
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    /// Stream objects are never compressible in PDF object streams.
    fn is_compressible(&self) -> bool {
        false
    }
}
