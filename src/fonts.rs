use crate::objects::pdf_object::PdfObj;
use crate::{PdfDictionaryObject, PdfError};

#[allow(dead_code)]
pub(crate) struct Fonts {}

impl Fonts {
    #[allow(dead_code)]
    pub(crate) fn get_standard_fonts_dict() -> Result<PdfDictionaryObject, PdfError> {
        let fonts = [
            ("Helvetica", "Type1"),
            ("Helvetica-Bold", "Type1"),
            ("Courier", "Type1"),
        ];

        let mut fonts_dict = PdfDictionaryObject::new();
        for (name, subtype) in fonts {
            let mut dict = PdfDictionaryObject::new().typed("Font")?;
            dict.add("Subtype", PdfObj::make_name_obj(subtype))?;
            dict.add("BaseFont", PdfObj::make_name_obj(name))?;

            fonts_dict.add(name, dict)?;
        }

        Ok(fonts_dict)
    }
}
