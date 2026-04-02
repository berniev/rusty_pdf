/// PDF content stream.
///
/// Content streams define page content, eg:
/// - Graphics: paths, rectangles, curves
/// - Text: fonts, positioning, display
/// - Colors: RGB, CMYK, grayscale
/// - Images: inline images
/// - Transformations: matrices, state management
///
///   A stream object, like a string object, is a sequence of bytes. Furthermore, a stream may be
///   of unlimited length, whereas a string shall be subject to an implementation limit. For this
///   reason, objects with potentially large amounts of data, such as images and page
///   descriptions, shall be represented as streams.
///
///   A stream shall consist of a dictionary followed by zero or more bytes bracketed between the
///   keywords'stream' and 'endstream'.
///
///   All streams shall be indirect objects (see 7.3.10, "Indirect Objects") and the stream
///   dictionary shall be a direct object.
///
///   Beginning with PDF 1.5, indirect objects may reside in object streams (see 7.5.7, "Object
///   Streams"). They are referred to in the same way; however, their definition shall not
///   include the keywords obj and endobj, and their generation number shall be zero.
///
///   Filter:
///
///   an optional part of the specification of a stream object, indicating how the data in the
///   stream should be decoded before it is used
///
/// * `stream` - Optional pre-existing stream content (sequence of operator calls)
/// * `extra` - Optional extra dictionary entries
///
///   Stream Extent: Entries common to all stream dictionaries:
///
///   Length      integer              (Reqd) - The length of the stream in bytes.
///   Filter      name or array        (Opt)  - A filter or sequence of filters to be applied.
///   DecodeParms dictionary or array  (Opt)  - Parameters for the filter(s) in Filter.
///   F           file specification   (Opt)  - A file specification for the stream data.
///   FFilter     name or array        (Opt)  - A filter or sequence of filters to file data
///   FDecodeParms dictionary or array (Opt)  - Parameters for the filter(s) in FFilter.
///   DL          integer              (Opt)  - Non-negative length of the decoded stream in bytes.
///
///   Stream Filters:
///
///   Params   Ver  Data Type       Decode/Decompress
///   ASCIIHexDecode   no        binary          ASCII hex
///   ASCII85Decode    no        binary          ASCII base-85
///   LZWDecode        yes       text or binary  LZE (Lempel-Ziv-Welch) algorithm
///   FlateDecode      yes  1.2  text or binary  zlib/deflate compression
///   RunLengthDecode  no        text or binary  byte-oriented run-length encoding algorithm
///   CCITTFaxDecode   yes       image           CCITT facsimile standard. typ mono 1 bit/pixel
///   JBIG2Decode      yes  1.4  image           JBig2 standard -> mono or approx
///   DCTDecode        yes       image           Discrete Cosine Transform technique based on JPEG
///   JPXDecode        no   1.5  image           Wwavelet-based JPEG2000 standard
///   Crypt            yes  1.5  data            Data encrypted by a security handler
///
///

use std::io::Write as IoWrite;
use flate2::write::ZlibEncoder;
use flate2::Compression;

use crate::color::{Color, ColorSpace, CMYK, RGB};
use crate::encoding::{ascii85_encode, f_to_pdf_num};
use crate::error::{PdfError, PdfResult};
use crate::objects::pdf_object::PdfObj;
use crate::objects::string::encode_pdf_string;
pub use crate::util::{CompressionMethod, Dims, Matrix, Posn, StrokeOrFill, ToPdf, WindingRule};
use crate::PdfDictionaryObject;

//------------------------ PdfStreamObject -----------------------

#[derive(Clone)]
pub struct PdfStreamObject {
    pub dict: PdfDictionaryObject,
    pub content: Vec<u8>,
    pub object_number: Option<u64>,

    pub compression_method: CompressionMethod,
}

impl Default for PdfStreamObject {
    fn default() -> Self {
        Self {
            dict: PdfDictionaryObject::new(),
            content: Vec::new(),
            object_number: None,

            compression_method: CompressionMethod::None,
        }
    }
}

impl PdfStreamObject {
    pub fn new() -> Self {
        Self {
            compression_method: CompressionMethod::None,
            ..Default::default()
        }
    }

    pub fn compressed(mut self) -> Self {
        self.compression_method = CompressionMethod::Flate;

        self
    }

    pub fn with_data(mut self, stream: Vec<u8>, dict: PdfDictionaryObject) -> Self {
        self.content = stream;
        self.dict = dict;

        self
    }

    pub fn compression_method(&self) -> CompressionMethod {
        self.compression_method
    }

    pub fn add_content(&mut self, bytes: Vec<u8>) {
        self.content.extend(bytes);
    }

    fn push_op(&mut self, operands: &[&dyn ToPdf], operator: &str) {
        let mut cmd_parts: Vec<String> = operands.iter().map(|n| n.to_pdf()).collect();
        cmd_parts.push(operator.to_string());
        self.add_content(cmd_parts.join(" ").into_bytes());
    }

    pub fn serialise(&mut self) -> Result<Vec<u8>, PdfError> {
        let stream_bytes: Vec<u8> = match self.compression_method {
            CompressionMethod::None => self.content.clone(),
            CompressionMethod::Flate => {
                self.dict.add("Filter", PdfObj::name("FlateDecode"));
                let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
                encoder.write_all(&self.content)?;
                encoder.finish()?
            }
        };

        self.dict.add("Length", PdfObj::num(stream_bytes.len() as f64));

        let mut vec = self.dict.serialise()?;
        vec.push(b'\n');
        vec.extend(b"stream\n");
        vec.extend(&stream_bytes);
        vec.extend(b"endstream\n");

        Ok(vec)
    }

    fn cmd(&mut self, cmd: char) {
        self.content.push(cmd as u8);
    }

    fn windable_cmd(&mut self, cmd: char, even_odd: WindingRule) {
        let mut op_bytes = vec![cmd as u8];
        match even_odd {
            WindingRule::EvenOdd => op_bytes.push(b'*'),
            WindingRule::NonZero => op_bytes.push(b' '),
        }
        self.add_content(op_bytes);
    }

    fn float_cmd(&mut self, string: &str, value: f64) {
        self.add_content(format!("{} {}", f_to_pdf_num(value), string).into_bytes());
    }

    fn int_cmd(&mut self, string: &str, value: i32) {
        self.float_cmd(string, value as f64);
    }

    pub fn begin_marked_content(&mut self, tag: &str, property_list: Option<Vec<u8>>) {
        match property_list {
            None => {
                self.add_content(format!("/{tag} BMC").into_bytes());
            }

            Some(props) => {
                let mut cmd = format!("/{tag} ").into_bytes();
                cmd.extend(props);
                cmd.extend(b" BDC");
                self.add_content(cmd);
            }
        }
    }

    pub fn begin_text(&mut self) {
        self.add_content(b"BT".to_vec());
    }

    /// Use the nonzero winding number rule to determine which regions lie inside the clipping path by default.
    pub fn clip(&mut self, even_odd: WindingRule) {
        self.windable_cmd('W', even_odd);
    }

    pub fn close(&mut self) {
        self.cmd('h');
    }

    /// extend curve from `pos3` using `pos1` and `pos2` as Bézier control points.
    pub fn curve_to(&mut self, pos1: Posn, pos2: Posn, pos3: Posn) {
        self.push_op(&[&pos1, &pos2, &pos3], "c");
    }

    /// Extend curve to `pos3` using current point, and `pos2` as Bézier control points.
    pub fn curve_start_to(&mut self, pos2: Posn, pos3: Posn) {
        self.push_op(&[&pos2, &pos3], "v");
    }

    /// extend curve to `pos3` using `pos1`, and `pos3` as Bézier control points.
    pub fn curve_end_to(&mut self, pos1: Posn, pos3: Posn) {
        self.push_op(&[&pos1, &pos3], "y");
    }

    pub fn draw_x_object(&mut self, reference: &str) {
        self.add_content(format!("/{} Do", reference).into_bytes());
    }

    /// End path without filling or stroking.
    pub fn end(&mut self) {
        self.add_content(b"n".to_vec());
    }

    pub fn end_marked_content(&mut self) {
        self.add_content(b"EMC".to_vec());
    }

    pub fn end_text(&mut self) {
        self.add_content(b"ET".to_vec());
    }

    pub fn fill(&mut self, even_odd: WindingRule) {
        self.windable_cmd('f', even_odd);
    }

    pub fn fill_and_stroke(&mut self, even_odd: WindingRule) {
        self.windable_cmd('B', even_odd);
    }

    pub fn fill_stroke_and_close(&mut self, even_odd: WindingRule) {
        self.windable_cmd('b', even_odd);
    }

    pub fn inline_image(
        &mut self,
        width_pixels: u32,
        height_pixels: u32,
        color_space: ColorSpace,
        bits_per_component: u8, // typ 8
        raw_pixel_data: &[u8],
    ) -> PdfResult<()> {
        if width_pixels == 0 || height_pixels == 0 {
            return Err(PdfError::InvalidImage(format!(
                "Invalid image dimensions: {width_pixels} x {height_pixels} pixels",
            )));
        }

        let data_to_encode = match self.compression_method {
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
            "BI /W {} /H {} /BPC {} /CS /Device{} /F {} /L {} ID ",
            f_to_pdf_num(width_pixels as f64),
            f_to_pdf_num(height_pixels as f64),
            f_to_pdf_num(bits_per_component as f64),
            color_space,
            self.compression_method.to_string(),
            encoded_data.len()
        );

        let mut final_command_bytes = header_string.into_bytes();
        final_command_bytes.extend(encoded_data); // image data
        final_command_bytes.extend(b" EI"); // End Image marker

        self.add_content(final_command_bytes);

        Ok(())
    }

    /// image converted to RGB format and embedded.
    /// Use `push_state()` and `set_matrix()` before this call to position and scale the image.
    pub fn inline_image_from_file(&mut self, path: &str) -> PdfResult<()> {
        let img = image::open(path).map_err(|e| {
            PdfError::InvalidImage(format!("Failed to load image from {}: {}", path, e))
        })?;

        let rgb_img = img.to_rgb8();
        let (width_pixels, height_pixels) = rgb_img.dimensions();

        self.inline_image(
            width_pixels,
            height_pixels,
            ColorSpace::RGB,
            8,
            &rgb_img.into_raw(),
        )
    }

    pub fn line_to_x_y(&mut self, posn: Posn) {
        self.push_op(&[&posn], "l");
    }

    pub fn move_to_x_y(&mut self, posn: Posn) {
        self.push_op(&[&posn], "m");
    }

    pub fn move_text_to_next_line_at(&mut self, posn: Posn) {
        self.push_op(&[&posn], "T*");
    }

    pub fn paint_shading(&mut self, name: &str) {
        let mut cmd = b"/".to_vec();
        cmd.extend(name.as_bytes());
        cmd.extend(b" sh");
        self.add_content(cmd);
    }

    pub fn pop_state(&mut self) {
        self.add_content(b"Q".to_vec());
    }

    pub fn push_state(&mut self) {
        self.add_content(b"q".to_vec());
    }

    pub fn add_rectangle(&mut self, posn: Posn, size: Dims) {
        self.push_op(&[&posn, &size], "re");
    }

    pub fn set_color_rgb(&mut self, rgb: RGB, stroke: StrokeOrFill) {
        let operator = match stroke {
            StrokeOrFill::Stroke => "RG",
            StrokeOrFill::Fill => "rg",
        };
        self.push_op(&[&rgb], operator);
    }

    pub fn set_color_cmyk(&mut self, cmyk: CMYK, stroke: StrokeOrFill) {
        let operator = match stroke {
            StrokeOrFill::Stroke => "K",
            StrokeOrFill::Fill => "k",
        };
        self.push_op(&[&cmyk], operator);
    }

    pub fn set_color_grayscale(&mut self, grayscale: Color, stroke: StrokeOrFill) {
        let operator = match stroke {
            StrokeOrFill::Stroke => "G",
            StrokeOrFill::Fill => "g",
        };
        self.push_op(&[&grayscale], operator);
    }

    pub fn set_color_space(&mut self, space: &str, stroke: StrokeOrFill) {
        let operator = match stroke {
            StrokeOrFill::Stroke => "CS",
            StrokeOrFill::Fill => "cs",
        };
        self.add_content(format!("/ {space} {operator}").into_bytes());
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
        self.add_content(cmd_parts.join(" ").into_bytes());
    }

    pub fn set_dash_line_pattern(&mut self, dash_array: &[f64], dash_phase: i32) {
        // Build the [n n n] part directly
        let array_str: Vec<String> = dash_array.iter().map(|&n| f_to_pdf_num(n)).collect();

        // Build the entire command in one single allocation
        let cmd = format!("[{}] {} d", array_str.join(" "), dash_phase).into_bytes();

        self.add_content(cmd);
    }

    pub fn set_font_name_and_size(&mut self, font: &str, size: f64) {
        self.add_content(format!("/{} {} Tf", font, f_to_pdf_num(size)).into_bytes());
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
        self.add_content(format!("/{state_name} gs").into_bytes());
    }
    /// Set current text and text line transformation matrix.
    pub fn set_text_matrix(&mut self, matrix: Matrix) {
        self.push_op(&[&matrix], "Tm");
    }

    /// Set text position without scaling, rotation, or skewing.
    ///
    /// Convenience method equivalent to calling `set_text_matrix` with an identity matrix.
    pub fn set_text_position(&mut self, posn: Posn) {
        self.set_text_matrix(Matrix {
            a: 1.0,
            b: 0.0,
            c: 0.0,
            d: 1.0,
            e: posn.x,
            f: posn.y,
        });
    }

    pub fn show_text_strings(&mut self, text: &str) {
        self.add_content(format!("[{text}] TJ").into_bytes());
    }

    pub fn show_single_text_string(&mut self, text: &str) {
        let mut cmd = encode_pdf_string(text);
        cmd.push_str(" Tj");
        self.add_content(Vec::from(cmd));
    }

    pub fn stroke_path(&mut self) {
        self.add_content(b"S".to_vec());
    }

    pub fn stroke_and_close_path(&mut self) {
        self.add_content(b"s".to_vec());
    }

    pub fn add_rounded_rectangle(
        &mut self,
        posn: Posn,
        size: Dims,
        radius_top_left: f64,
        radius_top_right: f64,
        radius_bottom_right: f64,
        radius_bottom_left: f64,
    ) {
        const KAPPA: f64 = 0.5522847498307933; // makes cubic Bezier curve like circular arc

        let Posn { x, y } = posn;
        let Dims { width, height } = size;

        let draw_corner = |s: &mut PdfStreamObject, radius: f64, rel_corner_pos: Posn| {
            if radius < 0.0001 {
                return;
            }

            let Posn { x, y } = rel_corner_pos;
            s.curve_to(
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
            );
        };

        self.move_to_x_y(Posn {
            x: x + radius_top_left,
            y: y + height,
        });

        draw_corner(
            // top right
            self,
            radius_top_left,
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

        draw_corner(self, radius_top_right, Posn { x: width, y: 0.0 }); // bottom right

        self.line_to_x_y(Posn {
            // bottom
            x: x + width,
            y: y + radius_bottom_right,
        });

        draw_corner(self, radius_bottom_right, Posn { x: 0.0, y: 0.0 }); // bottom left

        self.line_to_x_y(Posn { x: x + width, y }); // left

        draw_corner(self, radius_bottom_left, Posn { x: 0.0, y: height }); // top left

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
