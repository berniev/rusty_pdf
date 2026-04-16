use crate::{PdfDictionaryObject, PdfError};

pub fn add_font_resources(mut next_num_func: impl FnMut() -> u64) -> Result<u64, PdfError> {
    let mut resources_dict = PdfDictionaryObject::new();
    let next_num = next_num_func();
    let fonts_dict = PdfDictionaryObject::new().with_object_number(next_num);
    resources_dict.add("Font", fonts_dict)?;

    Ok(next_num)
}
