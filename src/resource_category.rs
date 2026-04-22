pub const STANDARD_RESOURCE_CATEGORIES: &[&str] = &[
    "ColorSpace",
    "ExtGState",
    "Font",
    "Pattern",
    "Properties",
    "Shading",
    "XObject",
    "ProcSet",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ResourceCategory {
    ColorSpace,
    ExtGState,
    Font,
    Pattern,
    Properties,
    Shading,
    XObject,
    ProcSet
}

impl ResourceCategory {
    pub fn as_str(&self) -> &'static str {
        match self {
            ResourceCategory::ColorSpace => "ColorSpace",
            ResourceCategory::ExtGState => "ExtGState",
            ResourceCategory::Font => "Font",
            ResourceCategory::Pattern => "Pattern",
            ResourceCategory::Properties => "Properties",
            ResourceCategory::Shading => "Shading",
            ResourceCategory::XObject => "XObject",
            ResourceCategory::ProcSet => "ProcSet",
        }
    }

    pub fn prefix(&self) -> &'static str {
        match self {
            ResourceCategory::ColorSpace => "CS",
            ResourceCategory::ExtGState => "GS",
            ResourceCategory::Font => "F",
            ResourceCategory::Pattern => "P",
            ResourceCategory::Properties => "Pr",
            ResourceCategory::Shading => "Sh",
            ResourceCategory::XObject => "Im",
            ResourceCategory::ProcSet => "PS",
        }
    }
}
/*
A resource dictionary shall be associated with a content stream in one of the following ways:
• For a content stream that is the value of a page’s Contents entry (or is an element of an array
  that is the value of that entry), the resource dictionary shall be designated by the page
  dictionary’s Resources or is inherited from some ancestor node of the page object.
• For other content streams, a conforming writer shall include a Resources entry in the stream's
  dictionary specifying the resource dictionary which contains all the resources used by that
  content stream. This shall apply to content streams that define form XObjects, patterns, Type 3
  fonts, and annotation.
• PDF files written obeying earlier versions of PDF may have omitted the Resources entry in all form
  XObjects and Type 3 fonts used on a page. All resources that are referenced from those forms and
  fonts shall be inherited from the resource dictionary of the page on which they are used. This
  construct is --obsolete-- and should not be used by conforming writers.

In the context of a given content stream, the term current resource dictionary refers to the
resource dictionary associated with the stream in one of the ways described above.

Each key in a resource dictionary shall be the name of a resource type. The
corresponding values shall be as follows:
• For resource type ProcSet, the value shall be an array of procedure set names
• For all other resource types, the value shall be a subdictionary. Each key in the subdictionary
  shall be the name of a specific resource, and the corresponding value shall be a PDF object
  associated with the name.

Resource Dictionary Entries (optional, * except for ProcSet)
==========  ==========  ============================================================================
Name        Type        Value
==========  ==========  ============================================================================
ColorSpace  Dictionary  Maps each resource name to either the name of a device-dependent colour
                        space or an array describing a colour space
ExtGState   Dictionary  Resource names to graphics state parameter dictionaries
Font        Dictionary  Resource names to font dictionaries
Pattern     Dictionary  Resource names to pattern objects
Properties  Dictionary  Resource names to property list dictionaries for marked content
Shading     Dictionary  Resource names to shading dictionaries
XObject     Dictionary  Resource names to external objects

ProcSet*    Array       An array of predefined procedure set names
====================================================================================================

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
