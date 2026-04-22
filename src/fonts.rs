use crate::objects::pdf_object::PdfObj;
use crate::{PdfDictionaryObject, PdfError};
use crate::text::StandardFont;

const ALL_STANDARD_FONTS: [StandardFont; 12] = [
    StandardFont::Helvetica,
    StandardFont::HelveticaBold,
    StandardFont::HelveticaOblique,
    StandardFont::HelveticaBoldOblique,
    StandardFont::TimesRoman,
    StandardFont::TimesBold,
    StandardFont::TimesItalic,
    StandardFont::TimesBoldItalic,
    StandardFont::Courier,
    StandardFont::CourierBold,
    StandardFont::CourierOblique,
    StandardFont::CourierBoldOblique,
];

/// Build a Font resource dictionary containing all 12 standard Type 1 fonts.
pub fn standard_fonts_dict() -> Result<PdfDictionaryObject, PdfError> {
    let mut fonts_dict = PdfDictionaryObject::new();

    for font in ALL_STANDARD_FONTS {
        let name = font.pdf_name();
        let mut dict = PdfDictionaryObject::new().typed("Font")?;
        dict.add("Subtype", PdfObj::make_name_obj("Type1"))?;
        dict.add("BaseFont", PdfObj::make_name_obj(name))?;
        fonts_dict.add(name, dict)?;
    }

    Ok(fonts_dict)
}
