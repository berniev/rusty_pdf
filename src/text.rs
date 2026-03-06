/// Standard PDF fonts (built into all PDF readers).
///
/// These fonts don't need to be embedded in the PDF and are guaranteed
/// to be available in any PDF viewer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StandardFont {
    /// Helvetica (sans-serif)
    Helvetica,
    /// Helvetica Bold
    HelveticaBold,
    /// Helvetica Oblique (italic)
    HelveticaOblique,
    /// Helvetica Bold Oblique
    HelveticaBoldOblique,
    /// Times Roman (serif)
    TimesRoman,
    /// Times Bold
    TimesBold,
    /// Times Italic
    TimesItalic,
    /// Times Bold Italic
    TimesBoldItalic,
    /// Courier (monospace)
    Courier,
    /// Courier Bold
    CourierBold,
    /// Courier Oblique
    CourierOblique,
    /// Courier Bold Oblique
    CourierBoldOblique,
}

impl StandardFont {
    /// Get the PDF name for this font.
    ///
    /// Returns the name as used in PDF font dictionaries.
    pub fn pdf_name(&self) -> &'static str {
        match self {
            StandardFont::Helvetica => "Helvetica",
            StandardFont::HelveticaBold => "Helvetica-Bold",
            StandardFont::HelveticaOblique => "Helvetica-Oblique",
            StandardFont::HelveticaBoldOblique => "Helvetica-BoldOblique",
            StandardFont::TimesRoman => "Times-Roman",
            StandardFont::TimesBold => "Times-Bold",
            StandardFont::TimesItalic => "Times-Italic",
            StandardFont::TimesBoldItalic => "Times-BoldItalic",
            StandardFont::Courier => "Courier",
            StandardFont::CourierBold => "Courier-Bold",
            StandardFont::CourierOblique => "Courier-Oblique",
            StandardFont::CourierBoldOblique => "Courier-BoldOblique",
        }
    }

    /// Measure the approximate width of text in PDF points.
    ///
    /// This uses character width metrics for the standard fonts.
    /// The measurement is approximate and suitable for layout calculations.
    ///
    /// # Arguments
    ///
    /// * `text` - Text to measure
    /// * `size` - Font size in points
    ///
    /// # Returns
    ///
    /// Approximate width in PDF points
    pub fn measure_text(&self, text: &str, size: f64) -> f64 {
        // Average character width as a fraction of font size for each font
        let avg_width_factor = match self {
            StandardFont::Helvetica | StandardFont::HelveticaOblique => 0.5,
            StandardFont::HelveticaBold | StandardFont::HelveticaBoldOblique => 0.55,
            StandardFont::TimesRoman | StandardFont::TimesItalic => 0.46,
            StandardFont::TimesBold | StandardFont::TimesBoldItalic => 0.5,
            StandardFont::Courier
            | StandardFont::CourierBold
            | StandardFont::CourierOblique
            | StandardFont::CourierBoldOblique => 0.6, // Monospace is wider
        };

        text.len() as f64 * size * avg_width_factor
    }

    /// Get the font from family name, weight, and italic style.
    ///
    /// # Arguments
    ///
    /// * `family` - Font family ("helvetica", "times", "courier", "serif", "sans-serif", "monospace")
    /// * `weight` - Font weight (400 = normal, 700 = bold)
    /// * `italic` - Whether the font should be italic/oblique
    ///
    /// # Returns
    ///
    /// The matching standard font, defaulting to Helvetica if no match
    pub fn from_family(family: Option<&str>, weight: u16, italic: bool) -> Self {
        let is_bold = weight >= 700;

        let family_lower = family.map(|s| s.to_lowercase());
        match family_lower.as_deref() {
            Some(f) if f.contains("times") || f.contains("serif") => match (is_bold, italic) {
                (true, true) => StandardFont::TimesBoldItalic,
                (true, false) => StandardFont::TimesBold,
                (false, true) => StandardFont::TimesItalic,
                (false, false) => StandardFont::TimesRoman,
            },
            Some(f) if f.contains("courier") || f.contains("mono") || f.contains("console") => {
                match (is_bold, italic) {
                    (true, true) => StandardFont::CourierBoldOblique,
                    (true, false) => StandardFont::CourierBold,
                    (false, true) => StandardFont::CourierOblique,
                    (false, false) => StandardFont::Courier,
                }
            }
            _ => match (is_bold, italic) {
                (true, true) => StandardFont::HelveticaBoldOblique,
                (true, false) => StandardFont::HelveticaBold,
                (false, true) => StandardFont::HelveticaOblique,
                (false, false) => StandardFont::Helvetica,
            },
        }
    }
}

/// Text wrapping mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WrapMode {
    /// No wrapping - text continues on one line
    NoWrap,
    /// Wrap at word boundaries
    WordWrap,
    /// Wrap at character boundaries
    CharWrap,
}

/// Wrap text to fit within a maximum width.
///
/// Breaks text into multiple lines based on the specified wrap mode.
///
/// # Arguments
///
/// * `text` - Text to wrap
/// * `max_width` - Maximum width in PDF points
/// * `font` - Standard font to use for measurements
/// * `size` - Font size in points
/// * `mode` - Wrapping mode
///
/// # Returns
///
/// Vector of lines (strings)
///
/// # Example
///
/// ```rust
/// use pydyf::text::{wrap_text, StandardFont, WrapMode};
///
/// let lines = wrap_text(
///     "This is a long line of text that needs wrapping",
///     200.0,
///     StandardFont::Helvetica,
///     12.0,
///     WrapMode::WordWrap
/// );
/// ```
pub fn wrap_text(
    text: &str,
    max_width: f64,
    font: StandardFont,
    size: f64,
    mode: WrapMode,
) -> Vec<String> {
    let mut lines = Vec::new();
    let mut current_line = String::new();

    match mode {
        WrapMode::NoWrap => {
            lines.push(text.to_string());
        }
        WrapMode::WordWrap => {
            for word in text.split_whitespace() {
                let test_line = if current_line.is_empty() {
                    word.to_string()
                } else {
                    format!("{} {}", current_line, word)
                };

                let test_width = font.measure_text(&test_line, size);

                if test_width <= max_width {
                    current_line = test_line;
                } else {
                    if !current_line.is_empty() {
                        lines.push(current_line.clone());
                    }
                    current_line = word.to_string();
                }
            }

            if !current_line.is_empty() {
                lines.push(current_line);
            }
        }
        WrapMode::CharWrap => {
            for ch in text.chars() {
                let test_line = format!("{}{}", current_line, ch);
                let test_width = font.measure_text(&test_line, size);

                if test_width <= max_width {
                    current_line = test_line;
                } else {
                    if !current_line.is_empty() {
                        lines.push(current_line.clone());
                    }
                    current_line = ch.to_string();
                }
            }

            if !current_line.is_empty() {
                lines.push(current_line);
            }
        }
    }

    // Ensure at least one line
    if lines.is_empty() {
        lines.push(text.to_string());
    }

    lines
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_font_pdf_name() {
        assert_eq!(StandardFont::Helvetica.pdf_name(), "Helvetica");
        assert_eq!(StandardFont::TimesBold.pdf_name(), "Times-Bold");
        assert_eq!(StandardFont::Courier.pdf_name(), "Courier");
    }

    #[test]
    fn test_font_from_family() {
        assert_eq!(
            StandardFont::from_family(Some("Helvetica"), 400, false),
            StandardFont::Helvetica
        );
        assert_eq!(
            StandardFont::from_family(Some("Helvetica"), 700, false),
            StandardFont::HelveticaBold
        );
        assert_eq!(
            StandardFont::from_family(Some("Times"), 400, true),
            StandardFont::TimesItalic
        );
        assert_eq!(
            StandardFont::from_family(Some("Courier"), 700, true),
            StandardFont::CourierBoldOblique
        );
    }

    #[test]
    fn test_measure_text() {
        let width = StandardFont::Helvetica.measure_text("Hello", 12.0);
        assert!(width > 0.0);
        assert!(width < 100.0); // Sanity check
    }

    #[test]
    fn test_wrap_no_wrap() {
        let lines = wrap_text(
            "This is a test",
            100.0,
            StandardFont::Helvetica,
            12.0,
            WrapMode::NoWrap,
        );
        assert_eq!(lines.len(), 1);
        assert_eq!(lines[0], "This is a test");
    }

    #[test]
    fn test_wrap_word_wrap() {
        let lines = wrap_text(
            "This is a long line of text",
            50.0,
            StandardFont::Helvetica,
            12.0,
            WrapMode::WordWrap,
        );
        assert!(lines.len() > 1);
    }
}
