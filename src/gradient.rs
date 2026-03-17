use std::rc::Rc;

use crate::color::RGBA;
use crate::util::{Dims, Posn};
use crate::{
    ArrayObject, BooleanObject, DictionaryObject, NameObject, NumberObject, NumberType, PDF, PdfObject,
    StreamObject,
};
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

//--------------------------- Gradient Kind ---------------------------//

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
        pdf: &mut PDF,
        resource_counter: &mut u32,
        posn: Posn<f64>,
        size: Dims,
        stroke_width: f64,
    ) -> Option<(String, Option<String>)> {
        if self.stops.len() < 2 {
            return None;
        }

        // 1. Determine Geometry Strategy
        let (shading_type, coords) = self.get_shading_params(posn, size, stroke_width);

        let pattern_name = format!("P{}", *resource_counter);
        *resource_counter += 1;

        let first = &self.stops[0].rgba;
        let last = &self.stops.last().unwrap().rgba;

        let extend: Rc<dyn PdfObject> = Rc::new(ArrayObject::new(Some(vec![
            Rc::new(BooleanObject::new(Option::from(true))) as Rc<dyn PdfObject>,
            Rc::new(BooleanObject::new(Option::from(true))),
        ])));

        // 2. Create Color Function (Type 2 - Exponential Interpolation)
        let color_func = create_interpolation_function_type_2(
            vec![first.red.color as f64, first.green.color as f64, first.blue.color as f64],
            vec![last.red.color as f64, last.green.color as f64, last.blue.color as f64],
            0.0,
        );
        let color_func_num = pdf.add_object(Box::new(color_func));

        // 3. Create Color Shading Dictionary
        let mut shading_dict = DictionaryObject::new(None);
        shading_dict.set_number("ShadingType", NumberType::Integer(shading_type as i64));
        shading_dict.set(
            "ColorSpace",
            Rc::new(NameObject::new(Option::from("DeviceRGB".to_string()))),
        );
        shading_dict.set("Coords", to_array(coords.clone()));
        shading_dict.set_indirect("Function", color_func_num);
        shading_dict.set("Extend", extend.clone());

        let shading_num = pdf.add_object(Box::new(shading_dict));

        // 4. Handle Transparency (Soft Mask)
        let has_transparency = first.alpha < 1.0 || last.alpha < 1.0;
        let gs_name = if has_transparency {
            let name = format!("GS{}", *resource_counter);
            *resource_counter += 1;

            // Alpha Interpolation Function
            let alpha_func = create_interpolation_function_type_2(
                vec![first.alpha.color as f64, last.alpha.color as f64],
                vec![last.alpha.color as f64],
                0.0,
            );
            let alpha_func_num = pdf.add_object(Box::new(alpha_func));

            // Alpha Shading (DeviceGray)
            let mut alpha_shading = DictionaryObject::new(None);
            alpha_shading.set_number("ShadingType", NumberType::Integer(shading_type as i64));
            alpha_shading.set(
                "ColorSpace",
                Rc::new(NameObject::new(Option::from("DeviceGray".to_string()))),
            );
            alpha_shading.set("Coords", to_array(coords));
            alpha_shading.set_indirect("Function", alpha_func_num);
            alpha_shading.set("Extend", extend);

            let alpha_shading_num = pdf.add_object(Box::new(alpha_shading));

            // Create the Soft Mask group and ExtGState
            create_soft_mask_for_shading(pdf, alpha_shading_num, size.width, size.height);

            Some(name)
        } else {
            None
        };

        // 5. Create Pattern Dictionary
        let mut pattern_dict = DictionaryObject::typed("Pattern");
        pattern_dict.set_number("PatternType", 2);
        pattern_dict.set_indirect("Shading", shading_num);

        pdf.add_object(Box::new(pattern_dict));

        Some((pattern_name, gs_name))
    }

    /// Calculates geometry parameters (Type and Coords) based on gradient kind.
    fn get_shading_params(&self, posn: Posn<f64>, size: Dims, stroke_width: f64) -> (u8, Vec<f64>) {
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
    exponent: f64,
) -> DictionaryObject {
    let mut dict = DictionaryObject::new(None);
    dict.set_number("FunctionType", 2);
    dict.set("Domain", to_array(vec![0.0, 1.0]));
    dict.set("C0", to_array(c0));
    dict.set("C1", to_array(c1));
    dict.set_number("N", exponent); // Linear interpolation
    dict
}

fn to_array(v: Vec<f64>) -> Rc<dyn PdfObject> {
    Rc::new(ArrayObject::new(Some(
        v.into_iter()
            .map(|v| Rc::new(NumberObject::from(v)) as Rc<dyn PdfObject>)
            .collect(),
    )))
}

/// Soft Mask (/SMask) object graph.
fn create_soft_mask_for_shading(pdf: &mut PDF, alpha_shading_num: usize, width: f64, height: f64) {
    // 1. Create Form XObject (Transparency Group)
    let mut xobj = DictionaryObject::typed("XObject");
    xobj.set(
        "Subtype",
        Rc::new(NameObject::new(Option::from("Form".to_string()))),
    );
    xobj.set_number("FormType", NumberType::Integer(1));
    xobj.set("BBox", to_array(vec![0.0, 0.0, width, height]));

    let mut group_dict = DictionaryObject::new(None);
    group_dict.set(
        "Type",
        Rc::new(NameObject::new(Option::from("Group".to_string()))),
    );
    group_dict.set(
        "S",
        Rc::new(NameObject::new(Option::from("Transparency".to_string()))),
    );
    group_dict.set(
        "CS",
        Rc::new(NameObject::new(Option::from("DeviceGray".to_string()))),
    );

    xobj.set("Group", Rc::new(group_dict));

    let mut shading_res = DictionaryObject::new(None);
    shading_res.set_indirect("Sh0", alpha_shading_num);

    let mut resources = DictionaryObject::new(None);
    resources.set("Shading", Rc::new(shading_res));
    xobj.set("Resources", Rc::new(resources));

    let mut form_stream = StreamObject::compressed();
    form_stream.paint_shading("Sh0");
    form_stream.extra = xobj.values;

    let form_number = pdf.add_object(Box::new(form_stream));

    // 2. Create Mask Dictionary
    let mut smask_dict = DictionaryObject::typed("Mask");
    smask_dict.set(
        "S",
        Rc::new(NameObject::new(Option::from("Luminosity".to_string()))),
    );
    smask_dict.set_indirect("G", form_number);

    let smask_number = pdf.add_object(Box::new(smask_dict));

    // 3. Create ExtGState with the SMask
    let mut gs_dict = DictionaryObject::typed("ExtGState");
    gs_dict.set_indirect("SMask", smask_number);
    pdf.add_object(Box::new(gs_dict));
}
