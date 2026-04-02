/// File Structure
///
/// =====================  =====================================================================
/// Header                 One line identifying pdf version
/// Body                   The objects that make up the document
/// Cross-Reference Table  Information about the __indirect__ objects in the file
/// Trailer                Location of the xref tbl and of certain special objects in the file body
/// ============================================================================================
///

/**
space lines are optional
```
%PDF-1.4                    ← header
%âãÏÓ                       ← comment in the body, not required nowadays but spec does say 'shall'
1 0 obj                     ← first actual body object
...
endobj
...

xref                        ← cross-reference table
0 9
0000000000 65535 f\r\n
...

trailer                     ← trailer
<<
  /Size 9
  /Root 1 0 R
>>
startxref
1234                        ← byte offset of xref
%%EOF
```
*/

use std::io::Write;

use crate::{PdfDictionaryObject, PdfArrayObject};
use crate::cross_reference_table::CrossRefTable;
use crate::file_identifier::FileIdentifierMode;
use crate::fonts::Fonts;
use crate::header::Header;
use crate::objects::pdf_object::PdfObj;
use crate::page::make_page_tree;
use crate::pdf_version::PdfVersion;
use crate::writer::{CompressedStrategy, LegacyStrategy, PdfWriter};

//--------------------------- PDF -------------------------//

pub struct Pdf {
    header: Header,
    catalog_dict: PdfDictionaryObject,
    page_tree_dict: PdfDictionaryObject,
    trailer_dict: PdfDictionaryObject,
    xref_table: CrossRefTable,
    last_object_number: u64,
}

impl Pdf {
    //--------------------------- construction --------------------------//
    pub fn new() -> Self {
        let mut pdf = Pdf {
            header: Header::new(),
            catalog_dict: PdfDictionaryObject::new().typed("Catalog"), // serialises into body
            page_tree_dict: PdfDictionaryObject::new(),
            trailer_dict: PdfDictionaryObject::new(), // not typed
            xref_table: CrossRefTable::new(), // buffers xref until body is complete, then appended
            last_object_number: 0,
        };
        pdf.page_tree_dict = make_page_tree(pdf.next_object_number());

        pdf
    }

    pub fn version(mut self, version: PdfVersion) -> Self {
        self.header.set_version(version);

        self
    }

    pub fn encrypted(&mut self) -> &mut Self {
        let mut encryption_dict = PdfDictionaryObject::new(); // direct
        encryption_dict
            .add("Filter", PdfObj::name("Standard"));

        self.trailer_dict
            .add("Encrypt", PdfObj::dict(encryption_dict));
        
        let mut id_array = PdfArrayObject::new();
        id_array.push(PdfObj::string("1234567890"));
        id_array.push(PdfObj::string("0987654321"));
        
        self.trailer_dict
            .add("ID", PdfObj::array(id_array));

        self
    }

    //-------------------------------------------------------

    pub fn get_catalog_dict(&mut self) -> &mut PdfDictionaryObject {
        &mut self.catalog_dict
    }

    pub fn get_trailer_dict(&mut self) -> &mut PdfDictionaryObject {
        &mut self.trailer_dict
    }

    pub fn get_xref_table_dict(&mut self) -> &mut CrossRefTable {
        &mut self.xref_table
    }

    pub fn next_object_number(&mut self) -> u64 {
        self.last_object_number += 1;

        self.last_object_number
    }

    // put it all together
    pub fn serialise() {}

    fn write_common(&mut self) {
        let _resources_number = self.add_font_resources();
        //self.initialize_catalog();
    }

    pub fn write_legacy<W: Write>(
        &mut self,
        output: W,
        id_mode: FileIdentifierMode,
    ) -> std::io::Result<()> {
        self.write_common();
        PdfWriter::new(output, LegacyStrategy::default(), id_mode).perform(self)
    }

    pub fn write_compressed<W: Write>(
        &mut self,
        output: W,
        id_mode: FileIdentifierMode,
    ) -> std::io::Result<()> {
        self.write_common();
        PdfWriter::new(output, CompressedStrategy::default(), id_mode).perform(self)
    }

    pub fn add_font_resources(&mut self) -> usize {
        let mut resources_dict = PdfDictionaryObject::new();
        resources_dict.add("Font", PdfObj::dict(Fonts::get_standard_fonts_dict()));

        //self.indirect_pdf_objects.push(resources_dict.boxed());

        let resources_number = 0; // self.allocate_object_id();
        //resources_dict.metadata.object_identifier = Some(resources_number);

        resources_number
    }
}
