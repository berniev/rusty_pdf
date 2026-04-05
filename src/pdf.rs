use crate::cross_reference_table::CrossRefTable;
use crate::header::Header;
use crate::page::make_page_tree;
use crate::version::Version;
use crate::trailer::Trailer;
use crate::{PdfDictionaryObject, PdfError};
use std::fs::File;
use std::io::{Seek, Write};
//--------------------------- Pdf -------------------------//

pub struct Pdf {
    header: Header,
    catalog_dict: PdfDictionaryObject,
    root_page_tree_dict: PdfDictionaryObject,
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
            xref_table: CrossRefTable::new(), // buffers xref until body is complete, then appended
            last_object_number: 0, // 0 is in xref table as 'free'. is gen# 65535, else 0 for new
        };
        pdf.root_page_tree_dict = make_page_tree(pdf.next_object_number());

        pdf
    }

    pub fn version(mut self, version: Version) -> Self {
        self.header.set_version(version);

        self
    }

    //--------------------------- gets -----------------------------------

    pub fn catalog_dict_ref(&mut self) -> &mut PdfDictionaryObject {
        &mut self.catalog_dict
    }

    pub fn root_page_tree_dict_ref(&mut self) -> &mut PdfDictionaryObject {
        &mut self.root_page_tree_dict
    }

    pub fn xref_table_ref(&mut self) -> &mut CrossRefTable {
        &mut self.xref_table
    }

    pub fn last_object_number(&self) -> u64 {
        self.last_object_number
    }

    pub fn next_object_number(&mut self) -> u64 {
        self.last_object_number += 1;

        self.last_object_number
    }

    //------------------------------ finalise ----------------------------------

    pub fn finalise(&mut self, path: &str) -> Result<(), PdfError> {
        let mut file = File::create(path)?;

        file.write(&*self.header.serialise()).map_err(|e| PdfError::Io(e))?;

        // writer.write_data(self.catalog_dict.serialise());

        let posn = file.stream_position()?;
        let trailer = Trailer::new()
            .with_size(self.last_object_number + 1)
            .with_root(self.root_page_tree_dict_ref().object_number.unwrap());

        let trailer_bytes = trailer.serialise(posn)?;
        file.write(&trailer_bytes).map_err(PdfError::Io)?;

        Ok(())
    }
}

pub enum Strategy {
    Legacy,
    Compressed,
}
