use crate::objects::pdf_object::Pdf;
use crate::PdfDictionaryObject;

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
            dict.add("Subtype", Pdf::name(subtype));
            dict.add("BaseFont", Pdf::name(name));
            
            fonts_dict.add(name, Pdf::dict(dict));
        }

        fonts_dict
    }
}
