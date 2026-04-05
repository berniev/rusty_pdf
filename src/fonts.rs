use crate::PdfDictionaryObject;
use crate::objects::pdf_object::PdfObj;


#[allow(dead_code)]
pub(crate) struct Fonts {}

impl Fonts {
    #[allow(dead_code)]
    pub(crate) fn get_standard_fonts_dict() -> PdfDictionaryObject {
        let fonts = [
            ("Helvetica", "Type1"),
            ("Helvetica-Bold", "Type1"),
            ("Courier", "Type1"),
        ];

        let mut fonts_dict = PdfDictionaryObject::new();
        for (name, subtype) in fonts {
            let mut dict = PdfDictionaryObject::new().typed("Font");
            dict.add("Subtype", PdfObj::name(subtype));
            dict.add("BaseFont", PdfObj::name(name));

            fonts_dict.add(name, dict);
        }

        fonts_dict
    }
}
