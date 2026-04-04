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
        }
    }

    pub fn category_prefix(category: ResourceCategory) -> &'static str {
        match category {
            ResourceCategory::ColorSpace => "CS",
            ResourceCategory::ExtGState => "GS",
            ResourceCategory::Font => "F",
            ResourceCategory::Pattern => "P",
            ResourceCategory::Properties => "Pr",
            ResourceCategory::Shading => "Sh",
            ResourceCategory::XObject => "Im",
        }
    }
}
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
  construct is --obsolete-- and should not be used by conforming writers.

In the context of a given content stream, the term current resource dictionary refers to the
resource dictionary associated with the stream in one of the ways described above.
Each key in a resource dictionary shall be the name of a resource type, as shown in Table 33. The
corresponding values shall be as follows:
• For resource type ProcSet, the value shall be an array of procedure set names
• For all other resource types, the value shall be a subdictionary. Each key in the subdictionary
  shall be the name of a specific resource, and the corresponding value shall be a PDF object
  associated with the name.

Optional entries in a resource dictionary (none reqd)
==========  ==========  ============================================================================
Key         Type        Value
==========  ==========  ============================================================================
ColorSpace  Dictionary  Maps each resource name to either the name of a device-dependent colour
                        space or an array describing a colour space
ExtGState   Dictionary  Resource names to graphics state parameter dictionaries
Font        Dictionary  Resource names to font dictionaries
Pattern     Dictionary  Resource names to pattern objects
Properties  Dictionary  Resource names to property list dictionaries for marked content
Shading     Dictionary  Resource names to shading dictionaries
XObject     Dictionary  Resource names to external objects

ProcSet     Array       An array of predefined procedure set names
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
