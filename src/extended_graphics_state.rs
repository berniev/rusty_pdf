//! Extended Graphics State (ExtGState) for advanced rendering.
//!
//! ExtGState objects control advanced graphics rendering features like
//! transparency, blend modes, and rendering intent.

use crate::{DictionaryObject, NameObject, NumberObject, NumberType, PdfObject, Resource, ResourceCategory};
use std::any::Any;
use std::rc::Rc;

/// Blend mode for transparency groups.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BlendMode {
    /// Default: source over destination (normal painting).
    Normal,
    /// Multiply source and destination.
    Multiply,
    /// Screen (inverse multiply).
    Screen,
    /// Overlay.
    Overlay,
    /// Darken.
    Darken,
    /// Lighten.
    Lighten,
    /// Color dodge.
    ColorDodge,
    /// Color burn.
    ColorBurn,
    /// Hard light.
    HardLight,
    /// Soft light.
    SoftLight,
    /// Difference.
    Difference,
    /// Exclusion.
    Exclusion,
    /// Hue.
    Hue,
    /// Saturation.
    Saturation,
    /// Color.
    Color,
    /// Luminosity.
    Luminosity,
}

impl BlendMode {
    /// Get the PDF name for this blend mode.
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

/// Rendering intent for color management.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RenderingIntent {
    /// Absolute colorimetric.
    AbsoluteColorimetric,
    /// Relative colorimetric.
    RelativeColorimetric,
    /// Saturation.
    Saturation,
    /// Perceptual.
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
    /// Line width.
    pub line_width: Option<f64>,

    /// Line cap style (0=butt, 1=round, 2=projecting square).
    pub line_cap: Option<u8>,

    /// Line join style (0=miter, 1=round, 2=bevel).
    pub line_join: Option<u8>,

    /// Miter limit.
    pub miter_limit: Option<f64>,

    /// Stroke alpha (opacity, 0.0-1.0).
    pub stroke_alpha: Option<f64>,

    /// Fill alpha (opacity, 0.0-1.0).
    pub fill_alpha: Option<f64>,

    /// Blend mode.
    pub blend_mode: Option<BlendMode>,

    /// Rendering intent.
    pub rendering_intent: Option<RenderingIntent>,

    /// Overprint for stroking.
    pub overprint_stroke: Option<bool>,

    /// Overprint for filling.
    pub overprint_fill: Option<bool>,

    /// Overprint mode.
    pub overprint_mode: Option<u8>,

    /// Flatness tolerance.
    pub flatness: Option<f64>,

    /// Smoothness tolerance.
    pub smoothness: Option<f64>,

    /// Stroke adjustment.
    pub stroke_adjust: Option<bool>,

    /// Alpha is shape (not opacity).
    pub alpha_is_shape: Option<bool>,

    /// Text knockout.
    pub text_knockout: Option<bool>,
}

impl ExtGState {
    /// Create a new empty ExtGState.
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

    /// Create an ExtGState with transparency.
    pub fn with_alpha(stroke_alpha: f64, fill_alpha: f64) -> Self {
        Self {
            stroke_alpha: Some(stroke_alpha),
            fill_alpha: Some(fill_alpha),
            ..Self::new()
        }
    }

    /// Create an ExtGState with a blend mode.
    pub fn with_blend_mode(blend_mode: BlendMode) -> Self {
        Self {
            blend_mode: Some(blend_mode),
            ..Self::new()
        }
    }

    /// Set stroke alpha.
    pub fn set_stroke_alpha(mut self, alpha: f64) -> Self {
        self.stroke_alpha = Some(alpha.clamp(0.0, 1.0));
        self
    }

    /// Set fill alpha.
    pub fn set_fill_alpha(mut self, alpha: f64) -> Self {
        self.fill_alpha = Some(alpha.clamp(0.0, 1.0));
        self
    }

    /// Set blend mode.
    pub fn set_blend_mode(mut self, mode: BlendMode) -> Self {
        self.blend_mode = Some(mode);
        self
    }

    /// Set line width.
    pub fn set_line_width(mut self, width: f64) -> Self {
        self.line_width = Some(width);
        self
    }

    /// Convert to PDF dictionary.
    pub fn to_dict(&self) -> DictionaryObject {
        let mut dict = DictionaryObject::new(None);
        dict.set("Type", Rc::new(NameObject::new(Some("ExtGState".to_string()))));

        if let Some(lw) = self.line_width {
            dict.set("LW", Rc::new(NumberObject::new(NumberType::Real(lw))));
        }

        if let Some(lc) = self.line_cap {
            dict.set("LC", Rc::new(NumberObject::new(NumberType::Integer(lc as i64))));
        }

        if let Some(lj) = self.line_join {
            dict.set("LJ", Rc::new(NumberObject::new(NumberType::Integer(lj as i64))));
        }

        if let Some(ml) = self.miter_limit {
            dict.set("ML", Rc::new(NumberObject::new(NumberType::Real(ml))));
        }

        if let Some(ca) = self.stroke_alpha {
            dict.set("CA", Rc::new(NumberObject::new(NumberType::Real(ca))));
        }

        if let Some(ca) = self.fill_alpha {
            dict.set("ca", Rc::new(NumberObject::new(NumberType::Real(ca))));
        }

        if let Some(bm) = self.blend_mode {
            dict.set("BM", Rc::new(NameObject::new(Some(bm.as_str().to_string()))));
        }

        if let Some(ri) = self.rendering_intent {
            dict.set("RI", Rc::new(NameObject::new(Some(ri.as_str().to_string()))));
        }

        if let Some(op) = self.overprint_stroke {
            dict.set("OP", Rc::new(crate::BooleanObject::new(Some(op))));
        }

        if let Some(op) = self.overprint_fill {
            dict.set("op", Rc::new(crate::BooleanObject::new(Some(op))));
        }

        if let Some(opm) = self.overprint_mode {
            dict.set("OPM", Rc::new(NumberObject::new(NumberType::Integer(opm as i64))));
        }

        if let Some(fl) = self.flatness {
            dict.set("FL", Rc::new(NumberObject::new(NumberType::Real(fl))));
        }

        if let Some(sm) = self.smoothness {
            dict.set("SM", Rc::new(NumberObject::new(NumberType::Real(sm))));
        }

        if let Some(sa) = self.stroke_adjust {
            dict.set("SA", Rc::new(crate::BooleanObject::new(Some(sa))));
        }

        if let Some(ais) = self.alpha_is_shape {
            dict.set("AIS", Rc::new(crate::BooleanObject::new(Some(ais))));
        }

        if let Some(tk) = self.text_knockout {
            dict.set("TK", Rc::new(crate::BooleanObject::new(Some(tk))));
        }

        dict
    }

    /// Generate a unique identifier for resource deduplication.
    fn generate_id(&self) -> String {
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

    fn resource_id(&self) -> String {
        self.generate_id()
    }

    fn to_pdf_object(&self) -> Rc<dyn PdfObject> {
        Rc::new(self.to_dict())
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
        assert!(!gs.resource_id().is_empty());
    }

    #[test]
    fn test_alpha_clamping() {
        let gs = ExtGState::new()
            .set_stroke_alpha(1.5)
            .set_fill_alpha(-0.5);

        assert_eq!(gs.stroke_alpha, Some(1.0));
        assert_eq!(gs.fill_alpha, Some(0.0));
    }

    #[test]
    fn test_rendering_intent() {
        assert_eq!(RenderingIntent::Perceptual.as_str(), "Perceptual");
        assert_eq!(RenderingIntent::RelativeColorimetric.as_str(), "RelativeColorimetric");
    }
}
