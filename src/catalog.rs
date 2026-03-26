/**
 * Spec:
 * Document Catalog:
 *     The primary dictionary object containing references directly or indirectly to all other
 *     objects in the document with the exception that there may be objects in the trailer that
 *     are not referred to by the Catalog
 *
 *  Catalog
 *          Page Tree
 *                           Page
 *                                          Content Stream
 *                                          Thumbnail Image
 *                                          Annotations
 *                                    ...
 *                           Page
 *          Outline Hierachy
 *                           Outline Entry
 *                                ...
 *                           Outline Entry
 *          Article Threads
 *                           Thread
 *                                          Bead <--> Bead
 *                               ...
 *                           Thread
 *          Named Destinations
 *          Interactive form
 * Entries:
 *     Type               name           Reqd          "Catalog"
 *     Version            name           Opt     1.4
 *     Extensions         dictionary     Opt
 *     Pages              dictionary     Reqd          shall be indirect ref
 *     PageLabels         number tree    Opt     1.3
 *     Names              dictionary     Opt     1.2
 *     Dests              dictionary     Opt     1.1   indirect reference
 *     ViewerPreferences  dictionary     Opt     1.2
 *     PageLayout         name           Opt
 *         SinglePage (def)
 *         OneColumn
 *         TwoColumnLeft
 *         TwoColumnRight
 *         TwoPageLeft
 *         TwoPageRight
 *     PageMode           name           Opt
 *          UseNone (def)
 *          UseOutlines
 *          UseThumbs
 *          FullScreen
 *          UseOC
 *          UseAttachments
 *     Outlines            dictionary     Opt         indirect reference
 *     Threads             array          Opt    1.1  indirect reference
 *     OpenAction          array or dict  Opt    1.1
 *     AA                  dictionary     Opt    1.4
 *     URI                 dictionary     Opt    1.1
 *     AcroForm            dictionary     Opt    1.2
 *     Metadata            dictionary     Opt    1.4
 *     StructTreeRoot      dictionary     Opt    1.3
 *     MarkInfo            dictionary     Opt    1.4
 *     Lang                text string    Opt    1.4
 *     SpiderInfo          dictionary     Opt    1.3
 *     OutputIntents       array          Opt    1.4
 *     PieceInfo           dictionary     Opt    1.4
 *     OCProperties        dictionary     Opt    1.5
 *     Perms               dictionary     Opt    1.5
 *     Legal               dictionary     Opt    1.5
 *     Requirements        array          Opt    1.7
 *     Collection          dictionary     Opt    1.7
 *     NeedsRendering      boolean        Opt    1.7
 */

use once_cell::sync::Lazy;
use std::collections::HashMap;

use crate::page::PageTree;
use crate::{PdfArrayObject, PdfBooleanObject, PdfIndirectObject, PdfNameObject};
use crate::{PdfDictionaryObject, PdfMetadata, PdfObject};

//--------------------------- CatalogError -------------------------

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CatalogError {
    UnsupportedEntry(String),
    UnsupportedVersion { entry: String },
}

//--------------------------- Catalog Entry Metadata -------------------------

#[derive(Debug, Clone, Copy)]
pub struct Info {
    pub pdf_ver: f32,
    pub ent_type: Type,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Dictionary,
    Array,
    Name,
    Boolean,
    IndirectRef,
}

#[allow(dead_code)]
#[rustfmt::skip]
static SUPPORTED_CATALOG_ENTRIES: Lazy<HashMap<&'static str, Info>> = Lazy::new(|| {
    HashMap::from([
        ("Type",              Info {  pdf_ver: 1.0, ent_type: Type::Name }),
        ("Version",           Info {  pdf_ver: 1.4, ent_type: Type::Name }),
        ("Extensions",        Info {  pdf_ver: 1.3, ent_type: Type::Dictionary }),
        ("Pages",             Info {  pdf_ver: 1.0, ent_type: Type::IndirectRef }),
        ("PageLabels",        Info {  pdf_ver: 1.3, ent_type: Type::Dictionary }),
        ("Names",             Info {  pdf_ver: 1.2, ent_type: Type::Dictionary }),
        ("Dests",             Info {  pdf_ver: 1.1, ent_type: Type::Dictionary }),
        ("ViewerPreferences", Info {  pdf_ver: 1.2, ent_type: Type::Dictionary }),
        ("PageLayout",        Info {  pdf_ver: 1.0, ent_type: Type::Name }),
        ("PageMode",          Info {  pdf_ver: 1.0, ent_type: Type::Name }),
        ("Outlines",          Info {  pdf_ver: 1.1, ent_type: Type::IndirectRef }),
        ("Threads",           Info {  pdf_ver: 1.1, ent_type: Type::Array }),
        ("OpenAction",        Info {  pdf_ver: 1.1, ent_type: Type::Array }),
        ("AA",                Info {  pdf_ver: 1.4, ent_type: Type::Dictionary }),
        ("URI",               Info {  pdf_ver: 1.1, ent_type: Type::Dictionary }),
        ("AcroForm",          Info {  pdf_ver: 1.2, ent_type: Type::Dictionary }),
        ("Metadata",          Info {  pdf_ver: 1.4, ent_type: Type::IndirectRef }),
        ("StructTreeRoot",    Info {  pdf_ver: 1.3, ent_type: Type::Dictionary }),
        ("MarkInfo",          Info {  pdf_ver: 1.4, ent_type: Type::Dictionary }),
        ("Lang",              Info {  pdf_ver: 1.4, ent_type: Type::Name }),
        ("SpiderInfo",        Info {  pdf_ver: 1.3, ent_type: Type::Dictionary }),
        ("OutputIntents",     Info {  pdf_ver: 1.4, ent_type: Type::Array }),
        ("PieceInfo",         Info {  pdf_ver: 1.4, ent_type: Type::Dictionary }),
        ("OCProperties",      Info {  pdf_ver: 1.5, ent_type: Type::Dictionary }),
        ("Perms",             Info {  pdf_ver: 1.5, ent_type: Type::Dictionary }),
        ("Legal",             Info {  pdf_ver: 1.5, ent_type: Type::Dictionary }),
        ("Requirements",      Info {  pdf_ver: 1.7, ent_type: Type::Array }),
        ("Collection",        Info {  pdf_ver: 1.7, ent_type: Type::Dictionary }),
        ("NeedsRendering",    Info {  pdf_ver: 1.7, ent_type: Type::Boolean }),
    ])
});

//--------------------------- Catalog -------------------------

#[allow(dead_code)]
struct Catalog {
    metadata: PdfMetadata,
    pages: Option<PageTree>,
}

#[allow(dead_code)]
impl Catalog {
    pub fn new(pages: Option<PageTree>) -> Self {
        Self {
            metadata: Default::default(),
            pages,
        }
    }

    /// Validate the catalog entry name is supported
    pub fn lookup_catalog_entry(&self, name: &str) -> Result<&Info, CatalogError> {
        let info = SUPPORTED_CATALOG_ENTRIES
            .get(name)
            .ok_or_else(|| CatalogError::UnsupportedEntry(name.to_string()))?;

        Ok(info)
    }

    pub fn make_catalog_item(&self, name: &str) -> Result<Box<dyn PdfObject>, CatalogError> {
        let info = self.lookup_catalog_entry(name)?;

        let res: Box<dyn PdfObject> = match info.ent_type {
            Type::Dictionary => PdfDictionaryObject::new().typed(name).boxed(),
            Type::Array => PdfArrayObject::new().boxed(),
            Type::Boolean => PdfBooleanObject::new().boxed(),
            Type::Name => PdfNameObject::new().boxed(),
            Type::IndirectRef => PdfIndirectObject::new().boxed(),
        };

        Ok(res)
    }
}
