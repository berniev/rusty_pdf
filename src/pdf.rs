use std::io::Write;
use crate::page::{PageObject, PageTreeNode};

use crate::{DictionaryObject, NameObject, PdfObject};
use std::rc::Rc;
use crate::cross_ref::CrossRefTable;

//--------------------------- PDF -------------------------

/// Spec:
/// Object:
///     a basic data structure from which PDF files are constructed and includes these types:
///     array, boolean, dictionary, integer, name, null, real, stream and string
/// Object Reference:
///     an object value used to allow one object to refer to another: “<n> <m> R”
///     where <n> is an indirect object number, <m> is its version number and R is the uppercase R
/// Object stream:
///     a stream that contains a sequence of PDF objects
/// File Structure:
///     Header: One line identifying pdf version
///     Body: containing the objects that make up the document
///     Cross-Reference Table: (xreft) information about the indirect objects in the file
///     Trailer: location of the xreft and of certain special objects within the body of the file

//--------------------------- Version -------------------------

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub enum PdfVersion {
    #[default]
    Auto,
    V1_4,
    V1_5,
}

impl PdfVersion {
    pub fn as_str(&self) -> &str {
        match self {
            PdfVersion::Auto => "Auto",
            PdfVersion::V1_4 => "1.4",
            PdfVersion::V1_5 => "1.5",
        }
    }
}

//----------------------- Identifier -----------------------

/// for trailer
pub enum FileIdentifierMode {
    None,
    AutoMD5,
    Custom(Vec<u8>),
}

//--------------------------- PDF -------------------------

pub struct PDF {
    pub version: PdfVersion,
    pub objects: Vec<Box<dyn PdfObject>>,
    pub catalog: DictionaryObject,
    pub page_tree: PageTreeNode,
    pub cross_ref_table: CrossRefTable,
    pub last_num: usize,
}

impl PDF {
    pub fn new() -> Self {
        let mut pdf = PDF {
            version: PdfVersion::Auto,
            objects: Vec::new(),
            catalog: DictionaryObject::typed("Catalog"),
            page_tree: PageTreeNode::new(None),
            cross_ref_table: CrossRefTable::new(),
            last_num: 0,
        };

        pdf
    }

    pub fn with_version(mut self, version: PdfVersion) -> Self {
        self.version = version;

        self
    }

    fn next_num(&mut self) -> usize {
        self.last_num += 1;

        self.last_num
    }

    pub fn add_object(&mut self, mut object: Box<dyn PdfObject>) -> usize {
        let number = self.objects.len();
        object.metadata_mut().object_number = Some(number);
        self.objects.push(object);

        number
    }

    pub fn add_page(&mut self, mut page: PageObject) {
        page.set_id(self.next_num());
        self.page_tree.add_page(page);
    }

    fn get_standard_fonts_dict() -> DictionaryObject {
        let mut font_dict = DictionaryObject::new(None);
        let fonts = [
            ("Helvetica", "Type1"),
            ("Helvetica-Bold", "Type1"),
            ("Courier", "Type1"),
        ];

        for (name, subtype) in fonts {
            let mut f = DictionaryObject::typed("Font");
            f.set("Subtype", Rc::new(NameObject::new(subtype.to_string())));
            f.set("BaseFont", Rc::new(NameObject::new(name.to_string())));
            font_dict.set(name, Rc::new(f));
        }

        font_dict
    }

    fn get_standard_fonts() -> Vec<u8> {
        let mut font_dict = String::from("<<");
        let fonts = [
            ("Helvetica", "Type1"),
            ("Helvetica-Bold", "Type1"),
            ("Courier", "Type1"),
        ];
        for (name, subtype) in fonts {
            font_dict.push_str(&format!(
                " /{} << /Type /Font /Subtype /{} /BaseFont /{} >>",
                name, subtype, name
            ));
        }
        font_dict.push_str(" >>");

        font_dict.into_bytes()
    }

pub fn write<W: Write>(
        &mut self,
        output: &mut W,
        version: Option<&[u8]>,
        id_mode: FileIdentifierMode,
        compress: bool,
    ) -> std::io::Result<()> {
        let version = version.unwrap_or(b"1.7");

        let font_resources = Self::get_standard_fonts();
        let resources_number = self.objects.len();
        let mut resources = DictionaryObject::new(None);
        resources.metadata.number = Some(resources_number);
        resources
            .values
            .insert("Font".to_string(), font_resources.clone());
        self.objects.push(Box::new(resources));

        if self.page_tree.metadata.number.is_none() {
            let pages_number = self.objects.len();
            self.page_tree.metadata.number = Some(pages_number);
            let pages_ref = format!("{} 0 R", pages_number).into_bytes();
            let res_ref = format!("{} 0 R", resources_number).into_bytes();

            for obj in &mut self.objects {
                if let Some(page) = obj.as_any_mut().downcast_mut::<Page>() {
                    // Set Parent on Page by injecting into its 'other' map
                    page.other.insert("Parent".to_string(), pages_ref.clone());
                    // Merge or set Resources
                    if let Some(resources) = page.resources.clone() {
                        let res_bytes = resources.data();
                        let res_str = String::from_utf8_lossy(&res_bytes);
                        if res_str.starts_with("<<") {
                            let mut merged = res_str.trim_end_matches(">>").to_string();
                            merged.push_str(" /Font ");
                            merged.push_str(&String::from_utf8_lossy(&font_resources));
                            merged.push_str(" >>");
                            page.other
                                .insert("Resources".to_string(), merged.into_bytes());
                        }
                    } else {
                        page.other.insert("Resources".to_string(), res_ref.clone());
                    }
                }
            }
            let pages_copy = self.page_tree.clone();
            self.objects.push(Box::new(pages_copy));
        }

        if self.catalog.metadata.number.is_none() {
            let catalog_number = self.objects.len();
            self.catalog.metadata.number = Some(catalog_number);
            let pages_ref = self.page_tree.reference();
            self.catalog.values.insert("Pages".to_string(), pages_ref);
            let catalog_copy = self.catalog.clone();
            self.objects.push(Box::new(catalog_copy));
        }

        // Add Info if needed
        if !self.info.values.is_empty() && self.info.metadata.number.is_none() {
            self.info.metadata.number = Some(self.objects.len());
            let info_copy = self.info.clone();
            self.objects.push(Box::new(info_copy));
        }

        // --- SCOPE 2: PDF Printing Phase (The Writing Phase) ---
        self.write_line(
            &format!("%PDF-{}", String::from_utf8_lossy(version)).into_bytes(),
            output,
        )?;
        self.write_line(b"%\xf0\x9f\x96\xa4", output)?;

        if version >= b"1.5" && compress {
            self.write_compressed(output)?;
        } else {
            Self::write_legacy_objects(&mut self.objects, &mut self.current_position, output)?;

            // Re-calculate positions for Catalog and Info since they were pushed to objects
            // The original logic used self.catalog.reference(), but that only works if catalog.number is set.
            // In our refactor, we push a CLONE of self.catalog, so self.catalog itself might not have the ID.

            self.write_legacy_xref_and_trailer(output, id_mode)?;
        }

        self.write_line(b"startxref", output)?;
        self.write_line(
            self.xref_position.unwrap_or(0).to_string().as_bytes(),
            output,
        )?;
        self.write_line(b"%%EOF", output)?;

        Ok(())
    }
}
