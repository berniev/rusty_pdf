/*
The operands supplied to operators in a content stream shall only be direct objects; indirect
objects and object references shall not be permitted.

In some cases, an operator shall refer to a PDF object that is defined outside the content stream,
such as a font dictionary or a stream containing image data. This shall be accomplished by defining
such objects as named resources and referring to them by name from within the content stream.

Named resources shall be meaningful only in the context of a content stream.
The scope of a resource name shall be local to a particular content stream and shall be unrelated to
externally known identifiers for objects such as fonts. References from one object outside of
content streams to another outside of content streams shall be made by means of indirect object
references rather than named resources.

A content stream’s named resources shall be defined by a resource dictionary, which shall enumerate
the named resources needed by the operators in the content stream and the names by which they can
be referred to.

EXAMPLE 1 If a text operator appearing within the content stream needs a certain font, the content
stream’s resource dictionary can associate the name F42 with the corresponding font dictionary. The
text operator can use this name to refer to the font.

A resource dictionary shall be associated with a content stream in one of the following ways:
• For a content stream that is the value of a page’s Contents entry (or is an element of an array
  that is the value of that entry), the resource dictionary shall be designated by the page
  dictionary’s Resources or is inherited, as described under 7.7.3.4, "Inheritance of Page
  Attributes," from some ancestor node of the page object.
• For other content streams, a conforming writer shall include a Resources entry in the stream's
  dictionary specifying the resource dictionary which contains all the resources used by that
  content stream. This shall apply to content streams that define form XObjects, patterns, Type 3
  fonts, and annotation.
• PDF files written obeying earlier versions of PDF may have omitted the Resources entry in all form
  XObjects and Type 3 fonts used on a page. All resources that are referenced from those forms and
  fonts shall be inherited from the resource dictionary of the page on which they are used. This
  construct is obsolete and should not be used by conforming writers.

In the context of a given content stream, the term current resource dictionary refers to the
resource dictionary associated with the stream in one of the ways described above.
Each key in a resource dictionary shall be the name of a resource type, as shown in Table 33. The
corresponding values shall be as follows:
• For resource type ProcSet, the value shall be an array of procedure set names
• For all other resource types, the value shall be a subdictionary. Each key in the subdictionary
  shall be the name of a specific resource, and the corresponding value shall be a PDF object
  associated with the name.

Table 33 – Entries in a resource dictionary (all are optional)
==========  ==========  ============================================================================
Key         Type        Value
==========  ==========  ============================================================================
ExtGState   Dictionary  Resource names to graphics state parameter dictionaries
Pattern     Dictionary  Resource names to pattern objects
ColorSpace  Dictionary  Maps each resource name to either the name of a device-dependent colour
                        space or an array describing a colour space
Shading     Dictionary  Resource names to shading dictionaries
XObject     Dictionary  Resource names to external objects
Font        Dictionary  Resource names to font dictionaries
ProcSet     Array       An array of predefined procedure set names
Properties  Dictionary  Resource names to property list dictionaries for marked content
==========  ==========  ============================================================================

EXAMPLE 2 The following shows a resource dictionary containing procedure sets, fonts, and external
objects. The procedure sets are specified by an array, as described in 14.2, "Procedure Sets". The
fonts are specified with a subdictionary associating the names F5, F6, F7, and F8 with objects 6, 8,
10, and 12, respectively.
Likewise, the XObject subdict associates the names Im1 and Im2 with objects 13 and 15, respectively.
<</ProcSet [ /PDF /ImageB ]
/Font << /F5 6 0 R
/F6 8 0 R
/F7 10 0 R
/F8 12 0 R
>>
/XObject << /Im1 13 0 R
/Im2 15 0 R
>>
>>
*/

use crate::objects::pdf_object::PdfObj;
use crate::{PdfDictionaryObject, PdfError, PdfResult};
use std::collections::HashMap;
use crate::resource_category::STANDARD_RESOURCE_CATEGORIES;

#[derive(Clone)]
pub struct ResourceMap {
    categories: HashMap<String, HashMap<String, usize>>,
}

impl ResourceMap {
    pub fn new() -> Self {
        Self {
            categories: HashMap::new(),
        }
    }

    fn validate_resource_category(category: &str) -> PdfResult<()> {
        if STANDARD_RESOURCE_CATEGORIES.contains(&category) {
            Ok(())
        } else {
            Err(PdfError::StructureError(format!(
                "Invalid resource category: '{}'. Expected one of {:?}",
                category, STANDARD_RESOURCE_CATEGORIES
            )))
        }
    }

    pub fn implement(&mut self, category: &str) -> PdfResult<CategoryHandle<'_>> {
        Self::validate_resource_category(category)?;
        self.categories.entry(category.to_string()).or_default();

        Ok(CategoryHandle {
            dictionary: self,
            category: category.to_string(),
        })
    }

    pub fn add(&mut self, category: impl Into<String>, name: impl Into<String>, id: usize) {
        self.categories
            .entry(category.into())
            .or_default()
            .insert(name.into(), id);
    }

    /// Transforms the logical resources into a physical DictionaryObject.
    pub fn to_dict(&self) -> PdfDictionaryObject {
        let mut root_dict = PdfDictionaryObject::new();
        for (name, map) in &self.categories {
            let mut sub_dict = PdfDictionaryObject::new();
            for (name, &id) in map {
                sub_dict.add(name, PdfObj::num(id));
            }
            // Inlines the sub-dictionary directly into the Resources dictionary
            root_dict.add(name, PdfObj::dict(sub_dict));
        }

        root_dict
    }

    pub fn is_empty(&self) -> bool {
        self.categories.is_empty()
    }

    pub fn clear(&mut self) {
        self.categories.clear();
    }

    pub fn category_count(&self, cat: &str) -> usize {
        self.categories.get(cat).map_or(0, |m| m.len())
    }
}

/// A "Base Structure" that captures context to provide a simple add(name, id) API.
pub struct CategoryHandle<'a> {
    dictionary: &'a mut ResourceMap,
    category: String,
}

impl<'a> CategoryHandle<'a> {
    pub fn add(&mut self, name: &str, id: usize) {
        self.dictionary.add(&self.category, name, id)
    }

    pub fn count(&self) -> usize {
        self.dictionary.category_count(&self.category)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_resources() {
        let resources = ResourceMap::new();
        assert!(resources.is_empty());
        assert_eq!(resources.to_dict().len(), 0);
    }

    #[test]
    fn test_add_resources() {
        let mut resources = ResourceMap::new();

        {
            let mut extgstate = resources.implement("ExtGState").unwrap();

            extgstate.add("GS0", 5);
            assert_eq!(extgstate.count(), 1);
        }
        {
            let mut pattern = resources.implement("Pattern").unwrap();
            pattern.add("P0", 8);
            assert_eq!(pattern.count(), 1);
        }

        assert!(!resources.is_empty());

        let dict = resources.to_dict();
        assert!(dict.contains_key("ExtGState"));
        assert!(dict.contains_key("Pattern"));
    }
}
