use crate::{PdfDictionaryObject, PdfNameObject, PdfObject};

pub(crate) struct Fonts {}

impl Fonts {
    pub(crate) fn get_standard_fonts_dict() -> PdfDictionaryObject {
        let fonts = [
            ("Helvetica", "Type1"),
            ("Helvetica-Bold", "Type1"),
            ("Courier", "Type1"),
        ];

        let mut fonts_dict = PdfDictionaryObject::new();
        for (name, subtype) in fonts {
            let mut dict = PdfDictionaryObject::new().typed("Font");
            dict.set("Subtype", Box::new(PdfNameObject::new(subtype)).boxed());
            dict.set("BaseFont", Box::new(PdfNameObject::new(name)).boxed());
            
            fonts_dict.set(name, dict.boxed());
        }

        fonts_dict
    }
}
