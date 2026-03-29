use std::rc::Rc;

use crate::color::RGBA;
use crate::objects::pdf_object::Pdf;
use crate::util::{Dims, Posn};
use crate::{PdfArrayObject, PdfDictionaryObject, PdfFile, PdfObject, PdfStreamObject};

//--------------------------- PDF Function ---------------------------//

/// Type 0: Sampled. Maps input to output via lookup table
/// `sample_data` The raw binary data containing the sample points (PDF Stream data).
/// `samples_per_dimension` The number of samples along each dimension (PDF /Size).
///                         For a 1D gradient, this is a single-element Vec like [256].
/// `bits_per_sample` The number of bits used to store each sample value (PDF /BitsPerSample).
/// `interpolation_order` The order of interpolation between samples (PDF /Order). Def 1 (Linear).
/// `input_encoding` Maps input domain values to integer indices of the sample table (PDF /Encode).
///                  Usually [0, samples_per_dimension - 1].
/// `output_decoding` raw sample values (e.g., 0-255 for 8-bit) back to output range (PDF /Decode).
///                   Usually matches the output_range.
/// `output_range` The valid range of output values (PDF /Range).
///                REQUIRED for Type 0 functions to clip values to the color space.
///
/// Type 2: Exponential. Interpolation between two points
/// `values_at_start` The output value at the start of the gradient.
/// `values_at_end` The output value at the end of the gradient.
/// `interpolation_exponent` The exponent controlling the curve (PDF /N). 1.0 is Linear.
///
/// Type 3: Stitching. Chains multiple functions together in sequence.
/// `sub_functions` The functions to combine.
/// `stitching_bounds` The input values where one sub-function ends and the next begins.
/// `encoding_ranges` How the input domain maps into each sub-function.
///
/// Type 4: PostScript. Calculated using a subset of the PostScript language.
pub enum PdfFunctionType {
    Sampled {
        sample_data: Vec<u8>,
        samples_per_dimension: Vec<u32>,
        bits_per_sample: u8,
        interpolation_order: u8,
        input_encoding: Vec<f64>,
        output_decoding: Vec<f64>,
        output_range: Vec<f64>,
    },
    Exponential {
        values_at_start: Vec<f64>,
        values_at_end: Vec<f64>,
        interpolation_exponent: f64,
    },
    Stitching {
        sub_functions: Vec<Rc<PdfFunctionType>>,
        stitching_bounds: Vec<f64>,
        encoding_ranges: Vec<f64>,
    },
    PostScript(String),
}

//--------------------------- Color Stop ---------------------------//

#[derive(Debug, Clone)]
pub struct ColorStop {
    pub offset: f32, // along the gradient (0.0 = start, 1.0 = end)
    pub rgba: RGBA,
}

impl ColorStop {
    pub fn new(offset: f32, rgba: RGBA) -> Self {
        ColorStop { offset, rgba }
    }
}

//--------------------------- GradientKind ---------------------------//

#[derive(Debug, Clone)]
pub enum GradientKind {
    Linear { angle: f32 }, // (CSS convention: 0° is north/up, clockwise).
    Radial,
}

//--------------------------- Gradient ---------------------------//

pub struct Gradient {
    pub stops: Vec<ColorStop>,
    pub kind: GradientKind,
}

impl Gradient {
    pub fn new(kind: GradientKind) -> Self {
        Gradient {
            stops: Vec::new(),
            kind,
        }
    }

    pub fn add_stop(&mut self, offset: f32, rgba: RGBA) {
        self.stops.push(ColorStop::new(offset, rgba));
    }

    /// Creates the PDF pattern and necessary resources (Shadings, Functions, Soft Masks).
    ///
    /// Returns a tuple of (Pattern Name, optional Graphics State Name for transparency).
    pub fn create_pattern(
        &self,
        pdf: &mut PdfFile,          // <== todo
        resource_counter: &mut u32, // <== todo
        posn: Posn,
        size: Dims,
        stroke_width: f64,
    ) -> Option<(String, Option<String>)> {
        if self.stops.len() < 2 {
            return None;
        }

        let (shading_type, coords) = self.get_shading_params(posn, size, stroke_width);

        let pattern_name = format!("P{}", *resource_counter);
        *resource_counter += 1; // <== todo

        let first = &self.stops[0].rgba;
        let last = &self.stops.last().unwrap().rgba;

        let color_func = create_interpolation_function_type_2(
            first.as_vec_64().to_vec(),
            last.as_vec_64().to_vec(),
            0.0,
        );

        let mut color_shading_dict = PdfDictionaryObject::new();
        color_shading_dict.add("ShadingType", Pdf::num(shading_type));
        color_shading_dict.add("ColorSpace", Pdf::name("DeviceRGB"));

        let make_coords = || {
            let mut arr = PdfArrayObject::new();
            for coord in coords.iter() {
                arr.push(Pdf::num(*coord));
            }
            arr
        };
        color_shading_dict.add("Coords", Pdf::array(make_coords()));

        let color_func_num = pdf.add_object(Box::new(color_func)); // <== todo
        color_shading_dict.add("Function", Pdf::indirect(color_func_num));

        let make_extend = || {
            let mut arr = PdfArrayObject::new();
            arr.push(Pdf::bool(true));
            arr.push(Pdf::bool(true));
            arr
        };
        
         color_shading_dict.add("Extend", Pdf::array(make_extend()));

        let shading_num = pdf.add_object(Box::new(color_shading_dict)); // <== todo

        let has_transparency = first.has_transparency() || last.has_transparency();
        let gs_name = if !has_transparency {
            None
        } else {
            let name = format!("GS{}", *resource_counter);
            *resource_counter += 1;

            let alpha_func = create_interpolation_function_type_2(
                vec![first.a().to_f64(), last.a().to_f64()],
                vec![last.a().to_f64()],
                0.0,
            );
            let alpha_func_num = pdf.add_object(Box::new(alpha_func)); // <== todo

            let mut alpha_shading = PdfDictionaryObject::new();
            alpha_shading.add("ShadingType", Pdf::num(shading_type as i64));
            alpha_shading.add("ColorSpace", Pdf::name("DeviceGray"));
            alpha_shading.add("Coords", Pdf::array(make_coords()));
            alpha_shading.add("Function", Pdf::indirect(alpha_func_num));
            alpha_shading.add("Extend", Pdf::array(make_extend()));

            let alpha_shading_num = pdf.add_object(Box::new(alpha_shading)); // <== todo

            create_soft_mask_for_shading(pdf, alpha_shading_num, size.width, size.height);

            Some(name)
        };

        let mut pattern_dict = PdfDictionaryObject::new().typed("Pattern");
        pattern_dict.add("PatternType", Pdf::num(2));
        pattern_dict.add("Shading", Pdf::indirect(shading_num));

        pdf.add_object(Box::new(pattern_dict)); // <== todo

        Some((pattern_name, gs_name))
    }

    /// Calculate geometry parameters (Type and Coords)
    fn get_shading_params(&self, posn: Posn, size: Dims, stroke_width: f64) -> (u8, Vec<f64>) {
        let Posn { x, y } = posn;
        let Dims { width, height } = size;
        match self.kind {
            GradientKind::Linear { angle } => {
                let math_angle = 90.0 - angle;
                let angle_rad = (math_angle as f64).to_radians();
                let cos = angle_rad.cos();
                let sin = angle_rad.sin();

                let cx = x + width / 2.0;
                let cy = y + height / 2.0;

                let half_len = (width * cos.abs() + height * sin.abs()) / 2.0 + stroke_width;

                let x0 = cx - cos * half_len;
                let y0 = cy + sin * half_len;
                let x1 = cx + cos * half_len;
                let y1 = cy - sin * half_len;

                (2, vec![x0, y0, x1, y1])
            }
            GradientKind::Radial => {
                let cx = x + width / 2.0;
                let cy = y + height / 2.0;
                let radius = width.min(height) * 1.5;
                // [x0 y0 r0 x1 y1 r1]

                (3, vec![cx, cy, 0.0, cx, cy, radius])
            }
        }
    }
}

//--------------------------- Helpers ---------------------------//

fn create_interpolation_function_type_2(
    c0: Vec<f64>,
    c1: Vec<f64>,
    exponent: f32,
) -> PdfDictionaryObject {
    let mut dict = PdfDictionaryObject::new();
    dict.add("FunctionType", Pdf::num(2));

    dict.add("Domain", Pdf::array(to_array(vec![0.0, 1.0])));
    dict.add("C0", Pdf::array(to_array(c0)));
    dict.add("C1", Pdf::array(to_array(c1)));
    dict.add("N", Pdf::num(exponent as f64)); // Linear interpolation

    dict
}

fn to_array(v: Vec<f64>) -> PdfArrayObject {
    let mut arr = PdfArrayObject::new();
    for val in v {
        arr.push(Pdf::num(val));
    }

    arr
}

fn create_soft_mask_for_shading(
    pdf: &mut PdfFile,
    alpha_shading_num: usize,
    width: f64,
    height: f64,
) {
    let mut xobj_dict = PdfDictionaryObject::new().typed("XObject");
    xobj_dict.add("Subtype", Pdf::name("Form"));
    xobj_dict.add("FormType", Pdf::num(1));
    xobj_dict.add("BBox", Pdf::array(to_array(vec![0.0, 0.0, width, height])));

    let mut group_dict = PdfDictionaryObject::new().typed("Group");
    group_dict.add("S", Pdf::name("Transparency"));
    group_dict.add("CS", Pdf::name("DeviceGray"));

    xobj_dict.add("Group", Pdf::dict(group_dict));

    let mut shading_res_dict = PdfDictionaryObject::new();
    shading_res_dict.add("Sh0", Pdf::indirect(alpha_shading_num));

    let mut resources_dict = PdfDictionaryObject::new();
    resources_dict.add("Shading", Pdf::dict(shading_res_dict));
    xobj_dict.add("Resources", Pdf::dict(resources_dict));

    let mut form_stream = PdfStreamObject::compressed();
    let mut cmd = b"/".to_vec();
    cmd.extend(b"Sh0");
    cmd.extend(b" sh");
    form_stream.add_to_content(cmd);

    let form_number = pdf.add_object(form_stream.boxed()); // <== todo

    let mut smask_dict = PdfDictionaryObject::new().typed("Mask");
    smask_dict.add("S", Pdf::name("Luminosity"));
    smask_dict.add("G", Pdf::indirect(form_number));
    let smask_number = pdf.add_object(smask_dict.boxed()); // <== todo

    let mut gs_dict = PdfDictionaryObject::new().typed("ExtGState");
    gs_dict.add("SMask", Pdf::indirect(smask_number));
    pdf.add_object(gs_dict.boxed()); // <== todo
}
