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
use crate::cross_reference_table::CrossRefTable;
use crate::file_identifier::FileIdentifierMode;
use crate::header::Header;
use crate::objects::pdf_object::PdfObj;
use crate::page::make_page_tree;
use crate::pdf_version::PdfVersion;
use crate::writer::write_legacy;
use crate::{PdfArrayObject, PdfDictionaryObject};

//--------------------------- Pdf -------------------------//

pub struct Pdf {
    header: Header,
    catalog_dict: PdfDictionaryObject,
    root_page_tree_dict: PdfDictionaryObject,
    trailer_dict: PdfDictionaryObject,
    xref_table: CrossRefTable,
    last_object_number: u64,
}

impl Pdf {
    //--------------------------- construction --------------------------

    pub fn new() -> Self {
        let mut pdf = Pdf {
            header: Header::new(),
            catalog_dict: PdfDictionaryObject::new().typed("Catalog"), // serialises into body
            root_page_tree_dict: PdfDictionaryObject::new(),
            trailer_dict: PdfDictionaryObject::new(), // not typed
            xref_table: CrossRefTable::new(), // buffers xref until body is complete, then appended
            last_object_number: 0, // 0 is in xref table as 'free'. is gen# 65535, else 0 for new
        };
        pdf.root_page_tree_dict = make_page_tree(pdf.next_object_number());

        pdf
    }

    pub fn version(mut self, version: PdfVersion) -> Self {
        self.header.set_version(version);

        self
    }

    pub fn encrypted(&mut self) -> &mut Self {
        let mut encryption_dict = PdfDictionaryObject::new(); // not typed, direct
        encryption_dict.add("Filter", PdfObj::name("Standard"));

        self.trailer_dict
            .add("Encrypt", PdfObj::dict(encryption_dict));

        let mut id_array = PdfArrayObject::new();
        id_array.push(PdfObj::string("1234567890"));
        id_array.push(PdfObj::string("0987654321"));

        self.trailer_dict.add("ID", PdfObj::array(id_array));

        self
    }

    //--------------------------- gets -----------------------------------

    pub fn catalog_dict_ref(&mut self) -> &mut PdfDictionaryObject {
        &mut self.catalog_dict
    }

    pub fn trailer_dict_ref(&mut self) -> &mut PdfDictionaryObject {
        &mut self.trailer_dict
    }

    pub fn root_page_tree_dict_ref(&mut self) -> &mut PdfDictionaryObject {
        &mut self.root_page_tree_dict
    }

    pub fn xref_table_ref(&mut self) -> &mut CrossRefTable {
        &mut self.xref_table
    }

    pub fn next_object_number(&mut self) -> u64 {
        self.last_object_number += 1;

        self.last_object_number
    }

    //------------------------------------------------------------

    // put it all together
    pub fn serialise<W: Write>(&mut self, output: W, id_mode: FileIdentifierMode) {
        write_legacy(|| self.next_object_number(), output, id_mode).unwrap();
    }
}
