//! Extended Graphics State (ExtGState) for advanced rendering.
//!
//! ExtGState objects control advanced graphics rendering features like
//! transparency, blend modes, and rendering intent.

use crate::{NumberType, PdfBooleanObject, PdfDictionaryObject, PdfNameObject, PdfNumberObject, PdfObject, Resource, ResourceCategory};
use std::any::Any;
use std::rc::Rc;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BlendMode {
    Normal,
    Multiply,
    Screen,
    Overlay,
    Darken,
    Lighten,
    ColorDodge,
    ColorBurn,
    HardLight,
    SoftLight,
    Difference,
    Exclusion,
    Hue,
    Saturation,
    Color,
    Luminosity,
}

impl BlendMode {
    pub fn as_str(&self) -> &'static str {
        match self {
            BlendMode::Normal => "Normal",
            BlendMode::Multiply => "Multiply",
            BlendMode::Screen => "Screen",
            BlendMode::Overlay => "Overlay",
            BlendMode::Darken => "Darken",
            BlendMode::Lighten => "Lighten",
            BlendMode::ColorDodge => "ColorDodge",
            BlendMode::ColorBurn => "ColorBurn",
            BlendMode::HardLight => "HardLight",
            BlendMode::SoftLight => "SoftLight",
            BlendMode::Difference => "Difference",
            BlendMode::Exclusion => "Exclusion",
            BlendMode::Hue => "Hue",
            BlendMode::Saturation => "Saturation",
            BlendMode::Color => "Color",
            BlendMode::Luminosity => "Luminosity",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RenderingIntent {
    AbsoluteColorimetric,
    RelativeColorimetric,
    Saturation,
    Perceptual,
}

impl RenderingIntent {
    pub fn as_str(&self) -> &'static str {
        match self {
            RenderingIntent::AbsoluteColorimetric => "AbsoluteColorimetric",
            RenderingIntent::RelativeColorimetric => "RelativeColorimetric",
            RenderingIntent::Saturation => "Saturation",
            RenderingIntent::Perceptual => "Perceptual",
        }
    }
}

/// Extended Graphics State parameters.
///
/// Controls advanced rendering features like transparency, blend modes,
/// line caps, and overprint settings.
#[derive(Clone)]
pub struct ExtGState {
    pub line_width: Option<f64>,
    pub line_cap: Option<u8>,
    pub line_join: Option<u8>,
    pub miter_limit: Option<f64>,
    pub stroke_alpha: Option<f64>,
    pub fill_alpha: Option<f64>,
    pub blend_mode: Option<BlendMode>,
    pub rendering_intent: Option<RenderingIntent>,
    pub overprint_stroke: Option<bool>,
    pub overprint_fill: Option<bool>,
    pub overprint_mode: Option<u8>,
    pub flatness: Option<f64>,
    pub smoothness: Option<f64>,
    pub stroke_adjust: Option<bool>,
    pub alpha_is_shape: Option<bool>,
    pub text_knockout: Option<bool>,
}

impl ExtGState {
    pub fn new() -> Self {
        Self {
            line_width: None,
            line_cap: None,
            line_join: None,
            miter_limit: None,
            stroke_alpha: None,
            fill_alpha: None,
            blend_mode: None,
            rendering_intent: None,
            overprint_stroke: None,
            overprint_fill: None,
            overprint_mode: None,
            flatness: None,
            smoothness: None,
            stroke_adjust: None,
            alpha_is_shape: None,
            text_knockout: None,
        }
    }

    pub fn with_alpha(stroke_alpha: f64, fill_alpha: f64) -> Self {
        Self {
            stroke_alpha: Some(stroke_alpha),
            fill_alpha: Some(fill_alpha),
            ..Self::new()
        }
    }

    pub fn with_blend_mode(blend_mode: BlendMode) -> Self {
        Self {
            blend_mode: Some(blend_mode),
            ..Self::new()
        }
    }

    pub fn set_stroke_alpha(mut self, alpha: f64) -> Self {
        self.stroke_alpha = Some(alpha.clamp(0.0, 1.0));
        self
    }

    pub fn set_fill_alpha(mut self, alpha: f64) -> Self {
        self.fill_alpha = Some(alpha.clamp(0.0, 1.0));
        self
    }
    pub fn set_blend_mode(mut self, mode: BlendMode) -> Self {
        self.blend_mode = Some(mode);
        self
    }

    pub fn set_line_width(mut self, width: f64) -> Self {
        self.line_width = Some(width);
        self
    }

    pub fn to_dict(&self) -> PdfDictionaryObject {
        let mut dict = PdfDictionaryObject::new().typed("ExtGState");

        if let Some(lw) = self.line_width {
            dict.set("LW", PdfNumberObject::new(NumberType::from(lw)).boxed());
        }

        if let Some(lc) = self.line_cap {
            dict.set("LC", PdfNumberObject::new(NumberType::from(lc as i64)).boxed());
        }

        if let Some(lj) = self.line_join {
            dict.set("LJ", PdfNumberObject::new(NumberType::from(lj as i64)).boxed());
        }

        if let Some(ml) = self.miter_limit {
            dict.set("ML", PdfNumberObject::new(NumberType::from(ml)).boxed());
        }

        if let Some(ca) = self.stroke_alpha {
            dict.set("CA", PdfNumberObject::new(NumberType::from(ca)).boxed());
        }

        if let Some(ca) = self.fill_alpha {
            dict.set("ca", PdfNumberObject::new(NumberType::from(ca)).boxed());
        }

        if let Some(bm) = self.blend_mode {
            dict.set("BM", PdfNameObject::new(bm.as_str()).boxed());
        }

        if let Some(ri) = self.rendering_intent {
            dict.set("RI", PdfNameObject::new(ri.as_str()).boxed());
        }

        if let Some(op) = self.overprint_stroke {
            dict.set("OP", PdfBooleanObject::new(op).boxed());
        }

        if let Some(op) = self.overprint_fill {
            dict.set("op", PdfBooleanObject::new(op).boxed());
        }

        if let Some(opm) = self.overprint_mode {
            dict.set("OPM", PdfNumberObject::new(NumberType::from(opm as i64)).boxed());
        }

        if let Some(fl) = self.flatness {
            dict.set("FL", PdfNumberObject::new(NumberType::from(fl)).boxed());
        }

        if let Some(sm) = self.smoothness {
            dict.set("SM", PdfNumberObject::new(NumberType::from(sm)).boxed());
        }

        if let Some(sa) = self.stroke_adjust {
            dict.set("SA", PdfBooleanObject::new(sa).boxed());
        }

        if let Some(ais) = self.alpha_is_shape {
            dict.set("AIS", PdfBooleanObject::new(ais).boxed());
        }

        if let Some(tk) = self.text_knockout {
            dict.set("TK", PdfBooleanObject::new(tk).boxed());
        }

        dict
    }

    fn generate_unique_id(&self) -> String {
        format!(
            "extgs:lw={:?}:lc={:?}:lj={:?}:ml={:?}:ca={:?}:CA={:?}:bm={:?}",
            self.line_width,
            self.line_cap,
            self.line_join,
            self.miter_limit,
            self.fill_alpha,
            self.stroke_alpha,
            self.blend_mode.map(|b| b.as_str()),
        )
    }
}

impl Default for ExtGState {
    fn default() -> Self {
        Self::new()
    }
}

impl Resource for ExtGState {
    fn category(&self) -> ResourceCategory {
        ResourceCategory::ExtGState
    }

    fn resource_unique_id(&self) -> String {
        self.generate_unique_id()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blend_mode_names() {
        assert_eq!(BlendMode::Normal.as_str(), "Normal");
        assert_eq!(BlendMode::Multiply.as_str(), "Multiply");
        assert_eq!(BlendMode::Screen.as_str(), "Screen");
    }

    #[test]
    fn test_extgstate_with_alpha() {
        let gs = ExtGState::with_alpha(0.5, 0.8);
        assert_eq!(gs.stroke_alpha, Some(0.5));
        assert_eq!(gs.fill_alpha, Some(0.8));
    }

    #[test]
    fn test_extgstate_with_blend_mode() {
        let gs = ExtGState::with_blend_mode(BlendMode::Multiply);
        assert_eq!(gs.blend_mode, Some(BlendMode::Multiply));
    }

    #[test]
    fn test_extgstate_to_dict() {
        let gs = ExtGState::new()
            .set_stroke_alpha(0.5)
            .set_fill_alpha(0.8)
            .set_blend_mode(BlendMode::Screen);

        let dict = gs.to_dict();
        assert!(dict.contains_key("Type"));
        assert!(dict.contains_key("CA"));
        assert!(dict.contains_key("ca"));
        assert!(dict.contains_key("BM"));
    }

    #[test]
    fn test_extgstate_resource_trait() {
        let gs = ExtGState::with_alpha(0.5, 0.5);
        assert_eq!(gs.category(), ResourceCategory::ExtGState);
        assert!(!gs.resource_unique_id().is_empty());
    }

    #[test]
    fn test_alpha_clamping() {
        let gs = ExtGState::new().set_stroke_alpha(1.5).set_fill_alpha(-0.5);

        assert_eq!(gs.stroke_alpha, Some(1.0));
        assert_eq!(gs.fill_alpha, Some(0.0));
    }

    #[test]
    fn test_rendering_intent() {
        assert_eq!(RenderingIntent::Perceptual.as_str(), "Perceptual");
        assert_eq!(
            RenderingIntent::RelativeColorimetric.as_str(),
            "RelativeColorimetric"
        );
    }
}
