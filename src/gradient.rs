use crate::DictionaryObject;
use crate::objects::stream::StreamObject;
use crate::pdf::PDF;
use crate::util::{PosnXY, Size, RGBA};
use std::collections::HashMap;

fn common(size:Size) {
    let gs_name = if self.stops.len() >= 2 {
        let first = &self.stops[0].rgba;
        let last = &self.stops[self.stops.len() - 1].rgba;

        // Check if gradient has transparency
        let has_transparency = first.alpha < 1.0 || last.alpha < 1.0;

        let function_str = format!(
            "<< /FunctionType 2 /Domain [0 1] /C0 [{} {} {}] /C1 [{} {} {}] /N 1 >>",
            first.red, first.green, first.blue, last.red, last.green, last.blue
        );
        shading_values.insert("Function".to_string(), function_str.into_bytes());

        // If transparent, create soft mask for alpha channel
        if has_transparency {
            let gs_name = format!("GS{}", *resource_counter);
            *resource_counter += 1;

            // Create alpha shading (DeviceGray for opacity)
            let mut alpha_shading_values = HashMap::new();
            alpha_shading_values.insert("ShadingType".to_string(), b"3".to_vec());
            alpha_shading_values.insert("ColorSpace".to_string(), b"/DeviceGray".to_vec());
            alpha_shading_values.insert(
                "Coords".to_string(),
                format!("[{} {} 0 {} {} {}]", cx, cy, cx, cy, radius).into_bytes(),
            );

            let alpha_function_str = format!(
                "<< /FunctionType 2 /Domain [0 1] /C0 [{}] /C1 [{}] /N 1 >>",
                first.alpha, last.alpha
            );
            alpha_shading_values
                .insert("Function".to_string(), alpha_function_str.into_bytes());
            alpha_shading_values.insert("Extend".to_string(), b"[true true]".to_vec());

            let alpha_shading_dict = DictionaryObject::new(Some(alpha_shading_values));
            let alpha_shading_num = pdf.objects.len();
            pdf.add_object(Box::new(alpha_shading_dict));

            // Create soft mask form XObject
            create_soft_mask_for_shading(pdf, alpha_shading_num, size.width, size.height, &gs_name);

            Some(gs_name)
        } else {
            None
        }
    } else {
        None
    };
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

//--------------------------- Linear Gradient ---------------------------//

pub struct LinearGradient {
    angle: f32, // (CSS convention: 0° = north/up, 90° = east/right)
    stops: Vec<ColorStop>,
}

impl LinearGradient {
    pub fn new(angle: f32) -> Self {
        LinearGradient {
            angle,
            stops: Vec::new(),
        }
    }

    pub fn add_stop(&mut self, offset: f32, rgba: RGBA) {
        self.stops.push(ColorStop::new(offset, rgba));
    }

    pub fn create_pattern(
        &self,
        pdf: &mut PDF,
        resource_counter: &mut u32,
        posn:PosnXY,
        size:Size,
        stroke_width: f64,
    ) -> Option<(String, Option<String>)> {
        // Check if gradient ends in transparent - skip rendering (can't represent in PDF)
        if self.stops.len() >= 2 && self.stops.last().unwrap().rgba.alpha == 0.0 {
            return None;
        }

        if self.stops.is_empty() {
            return None;
        }

        let pattern_name = format!("P{}", *resource_counter);
        *resource_counter += 1;

        // Create shading dictionary for linear gradient (Type 2 - Axial)
        let mut shading_values = HashMap::new();
        shading_values.insert("ShadingType".to_string(), b"2".to_vec());
        shading_values.insert("ColorSpace".to_string(), b"/DeviceRGB".to_vec());

        // Get gradient angle and convert to coordinates
        // CSS/Slint: 0° = up/north, clockwise. Convert to math convention: 0° = right/east, counter-clockwise
        let math_angle = 90.0 - self.angle;
        let angle_rad = math_angle * std::f32::consts::PI / 180.0;
        let cos = angle_rad.cos() as f64;
        let sin = angle_rad.sin() as f64;

        // Define coords in absolute PDF page space
        let cx = posn.x + size.width / 2.0;
        let cy = posn.y + size.height / 2.0;

        // Gradient half-length: distance from center to edge in gradient direction
        // Add stroke_width to extend gradient beyond path for strokes
        let half_len = (size.width * cos.abs() + size.height * sin.abs()) / 2.0 + stroke_width;

        // Direction vector: cos points right, sin points up (in math coords)
        // PDF Y-axis is inverted, so reverse Y direction to match CSS gradient
        let x0 = cx - cos * half_len;
        let y0 = cy + sin * half_len; // Start of gradient (top for 0deg)
        let x1 = cx + cos * half_len;
        let y1 = cy - sin * half_len; // End of gradient (bottom for 0deg)

        shading_values.insert(
            "Coords".to_string(),
            format!("[{} {} {} {}]", x0, y0, x1, y1).into_bytes(),
        );

        common();

        shading_values.insert("Extend".to_string(), b"[true true]".to_vec());

        let shading_dict = DictionaryObject::new(Some(shading_values));
        let shading_num = pdf.objects.len();
        pdf.add_object(Box::new(shading_dict));

        // Create pattern dictionary
        let mut pattern_values = HashMap::new();
        pattern_values.insert("Type".to_string(), b"/Pattern".to_vec());
        pattern_values.insert("PatternType".to_string(), b"2".to_vec());
        pattern_values.insert(
            "Shading".to_string(),
            format!("{} 0 R", shading_num).into_bytes(),
        );

        let pattern_dict = DictionaryObject::new(Some(pattern_values));
        pdf.add_object(Box::new(pattern_dict));

        Some((pattern_name, gs_name))
    }
}
//--------------------------- Radial Gradient ---------------------------//

pub struct RadialGradient {
    stops: Vec<ColorStop>, // Color stops along the gradient
}

impl RadialGradient {
    pub fn new() -> Self {
        RadialGradient { stops: Vec::new() }
    }

    pub fn add_stop(&mut self, offset: f32, rgba: RGBA) {
        self.stops.push(ColorStop::new(offset, rgba));
    }

    pub fn create_pattern(
        &self,
        pdf: &mut PDF,
        resource_counter: &mut u32,
        posn:PosnXY,
        size:Size,
    ) -> Option<(String, Option<String>)> {
        // Check if gradient ends in transparent - skip rendering (can't represent in PDF)
        if self.stops.len() >= 2 && self.stops.last().unwrap().rgba.alpha == 0.0 {
            return None;
        }

        if self.stops.is_empty() {
            return None;
        }

        let pattern_name = format!("P{}", *resource_counter);
        *resource_counter += 1;

        // Create shading dictionary for radial gradient (Type 3)
        let mut shading_values = HashMap::new();
        shading_values.insert("ShadingType".to_string(), b"3".to_vec());
        shading_values.insert("ColorSpace".to_string(), b"/DeviceRGB".to_vec());

        // Center point and radius in absolute PDF page space
        // Extend radius to cover paths that may extend beyond their nominal box
        let cx = posn.x + size.width / 2.0;
        let cy = posn.y + size.height / 2.0; // PDF y-axis
        let radius = size.width.min(size.height) * 1.5; // Use 1.5x size to cover extended paths

        common();
        
        // Coords: [x0 y0 r0 x1 y1 r1] for circles from center
        shading_values.insert(
            "Coords".to_string(),
            format!("[{} {} 0 {} {} {}]", cx, cy, cx, cy, radius).into_bytes(),
        );

        // Build color function from gradient stops
 
        shading_values.insert("Extend".to_string(), b"[true true]".to_vec());

        let shading_dict = DictionaryObject::new(Some(shading_values));
        let shading_num = pdf.objects.len();
        pdf.add_object(Box::new(shading_dict));

        // Create pattern dictionary
        let mut pattern_values = HashMap::new();
        pattern_values.insert("Type".to_string(), b"/Pattern".to_vec());
        pattern_values.insert("PatternType".to_string(), b"2".to_vec());
        pattern_values.insert(
            "Shading".to_string(),
            format!("{} 0 R", shading_num).into_bytes(),
        );

        let pattern_dict = DictionaryObject::new(Some(pattern_values));
        pdf.add_object(Box::new(pattern_dict));

        Some((pattern_name, gs_name))
    }
}

impl Default for RadialGradient {
    fn default() -> Self {
        Self::new()
    }
}

//--------------------------- Conic Gradient ---------------------------//

fn create_soft_mask_for_shading(
    pdf: &mut PDF,
    alpha_shading_num: usize,
    width: f64,
    height: f64,
    _gs_name: &str,
) {
    // Create form XObject for the soft mask (transparency group)
    let mut form_extra = HashMap::new();
    form_extra.insert("Type".to_string(), b"/XObject".to_vec());
    form_extra.insert("Subtype".to_string(), b"/Form".to_vec());
    form_extra.insert("FormType".to_string(), b"1".to_vec());
    form_extra.insert(
        "BBox".to_string(),
        format!("[0 0 {} {}]", width, height).into_bytes(),
    );
    // Mark as transparency group for soft mask
    form_extra.insert(
        "Group".to_string(),
        b"<< /Type /Group /S /Transparency /CS /DeviceGray >>".to_vec(),
    );

    // Create resources dict with shading
    let resources_str = format!("<< /Shading << /Sh0 {} 0 R >> >>", alpha_shading_num);
    form_extra.insert("Resources".to_string(), resources_str.into_bytes());

    // Stream content that paints the shading
    let stream_content = b"/Sh0 sh".to_vec();
    let form_stream = StreamObject::new().with_data(Some(vec![stream_content]), Some(form_extra));

    let form_number = pdf.objects.len();
    pdf.add_object(Box::new(form_stream));

    // Create SMask dictionary
    let mut smask_values = HashMap::new();
    smask_values.insert("Type".to_string(), b"/Mask".to_vec());
    smask_values.insert("S".to_string(), b"/Luminosity".to_vec());
    smask_values.insert("G".to_string(), format!("{} 0 R", form_number).into_bytes());
    let smask_dict = DictionaryObject::new(Some(smask_values));
    let smask_number = pdf.objects.len();
    pdf.add_object(Box::new(smask_dict));

    // Create ExtGState with SMask
    let mut gs_values = HashMap::new();
    gs_values.insert("Type".to_string(), b"/ExtGState".to_vec());
    gs_values.insert(
        "SMask".to_string(),
        format!("{} 0 R", smask_number).into_bytes(),
    );
    let gs_dict = DictionaryObject::new(Some(gs_values));
    pdf.add_object(Box::new(gs_dict));
}
