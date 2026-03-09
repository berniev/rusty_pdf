use std::rc::Rc;

use crate::{ArrayObject, DictionaryObject, NameObject, NumberObject, PdfObject};
use crate::util::Dims;

//--------------------------- Page Size ---------------------------//

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PageSize {
    A4,
    Letter,
    Legal,
    A3,
    Custom(Dims), // width, height in points
}

impl Default for PageSize {
    fn default() -> Self {
        PageSize::A4
    }
}

impl PageSize {
    /// Returns the Dims in PDF points (1 PDF point = 1/72 inch).
    /// Returns 0.0 for negative custom dimensions.
    pub fn dimensions(&self) -> Dims {
        match self {
            PageSize::A4 => Dims { width: 595.0, height: 842.0 },
            PageSize::Letter => Dims { width: 612.0, height: 792.0 },
            PageSize::Legal => Dims { width: 612.0, height: 1008.0 },
            PageSize::A3 => Dims { width: 842.0, height: 1191.0 },
            PageSize::Custom(dims) => Dims {
                width: dims.width.max(0.0),
                height: dims.height.max(0.0),
            },
        }
    }

    pub fn as_array(&self) -> ArrayObject {
        let dims = self.dimensions();
        ArrayObject::new(Some(vec![
            Rc::new(NumberObject::from(0.0)) as Rc<dyn PdfObject>,
            Rc::new(NumberObject::from(0.0)),
            Rc::new(NumberObject::from(dims.width)),
            Rc::new(NumberObject::from(dims.height)),
        ]))
    }
}
//--------------------------- Page ---------------------------//

/// Spec:
/// Page:
///     a dictionary specifying the attributes of a single page of the document.
/// Entries:
/// Key                   Ver             Type              Value
/// Type                       Reqd       name              "Page"
/// Parent                     Reqd       dictionary        indirect reference
/// LastModified               *          date              Reqd if PieceInfo
/// Resources                  Reqd  Inh  dictionary
/// MediaBox                   Reqd  Inh  rectangle
/// CropBox                    Opt   Inh  rectangle
/// BleedBox              1.3  Opt        rectangle
/// TrimBox               1.3  Opt        rectangle
/// ArtBox                1.3  Opt        rectangle
/// BoxColorInfo          1.4  Opt        dictionary
/// Contents                   Opt        stream or array
/// Rotate                     Opt   Inh  integer
/// Group                 1.4  Opt        dictionary
/// Thumb                      Opt        stream
/// B                     1.1  Opt        array
/// Dur                   1.1  Opt        number
/// Trans                      Opt        dictionary
/// Annots                     Opt        array
/// AA                    1.2  Opt        dictionary
/// Metadata              1.4  Opt        stream
/// PieceInfo             1.3  Opt        dictionary
/// StructParents         1.3  *          integer          Reqd if struct content items
/// ID byte               1.3  Opt        string
/// PZ                    1.3  Opt        number
/// SeparationInfo        1.3  Opt        dictionary
/// Tabs                  1.5  Opt        name
/// TemplateInstantiated  1.5  Opt        name
/// PresSteps             1.5  Opt        dictionary
/// UserUnit              1.6  Opt        number
/// VP                    1.6  Opt        dictionary
pub struct Page {
    pub size: PageSize,
    pub contents: Option<Rc<dyn PdfObject>>,
    pub resources: DictionaryObject,
    pub custom_dict: DictionaryObject, // For any other /Page entries
}

impl Page {
    pub fn new(size: PageSize) -> Self {
        Self {
            size,
            contents: None,
            resources: DictionaryObject::new(None),
            custom_dict: DictionaryObject::new(None),
        }
    }

    pub fn with_contents(mut self, contents: Option<Rc<dyn PdfObject>>) -> Self {
        self.contents = contents;

        self
    }

    pub fn set_contents(&mut self, contents: Option<Rc<dyn PdfObject>>) {
        self.contents = contents;
    }

    pub fn set_parent(&mut self, parent_id: usize) {
        self.custom_dict.set_indirect("Parent", parent_id);
    }

    pub fn into_dictionary(self) -> DictionaryObject {
        let mut dict = self.custom_dict;
        dict.set("Type", Rc::new(NameObject::new("Page".to_string())));
        dict.set("MediaBox", Rc::new(self.size.as_array()));

        if let Some(contents) = self.contents {
            dict.set("Contents", contents);
        }

        if !self.resources.values.is_empty() {
            dict.set("Resources", Rc::new(self.resources));
        }

        dict
    }
}
