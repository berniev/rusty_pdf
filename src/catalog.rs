use std::collections::HashMap;
use once_cell::sync::Lazy;
use crate::pdf::PdfVersion;
use crate::{DictionaryObject};
use crate::page::{ObjectId, PageTreeNode};
use crate::objects::array::ArrayObject;

//--------------------------- DirectRef -------------------------

#[derive(Default)]
struct DirectRef {
    object_id: ObjectId,
}

impl DirectRef {
    pub fn new(object_id: ObjectId) -> Self {
        Self { object_id }
    }

    fn reference(&self) -> String {
        format!("{} 0 R", self.object_id)
    }
}

//--------------------------- IndirectRef -------------------------

#[derive(Default)]
struct IndirectRef {
    object_id: ObjectId,
}

impl IndirectRef {
    pub fn new(object_id: ObjectId) -> Self {
        Self { object_id }
    }

    pub fn reference(&self) -> String {
        format!("{} 0 R", self.object_id)
    }
}

//--------------------------- Catalog -------------------------

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

//--------------------------- CatalogError -------------------------

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CatalogError {
    UnsupportedEntry(String),
    UnsupportedVersion { entry: String, required: f32, current: f32 },
}

//--------------------------- Catalog Entry Metadata -------------------------

#[derive(Debug, Clone, Copy)]
pub struct CatalogEntryInfo {
    pub pdf_version: f32,
    pub entry_type: CatalogEntryType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CatalogEntryType {
    Dictionary,
    Array,
    Name,
    Boolean,
    IndirectRef,
}

#[rustfmt::skip]
static SUPPORTED_CATALOG_ENTRIES: Lazy<HashMap<&'static str, CatalogEntryInfo>> = Lazy::new(|| {
    HashMap::from([
        ("Type",              CatalogEntryInfo {  pdf_version: 1.0, entry_type: CatalogEntryType::Name }),
        ("Version",           CatalogEntryInfo {  pdf_version: 1.4, entry_type: CatalogEntryType::Name }),
        ("Extensions",        CatalogEntryInfo {  pdf_version: 1.3, entry_type: CatalogEntryType::Dictionary }),
        ("Pages",             CatalogEntryInfo {  pdf_version: 1.0, entry_type: CatalogEntryType::IndirectRef }),
        ("PageLabels",        CatalogEntryInfo {  pdf_version: 1.3, entry_type: CatalogEntryType::Dictionary }),
        ("Names",             CatalogEntryInfo {  pdf_version: 1.2, entry_type: CatalogEntryType::Dictionary }),
        ("Dests",             CatalogEntryInfo {  pdf_version: 1.1, entry_type: CatalogEntryType::Dictionary }),
        ("ViewerPreferences", CatalogEntryInfo {  pdf_version: 1.2, entry_type: CatalogEntryType::Dictionary }),
        ("PageLayout",        CatalogEntryInfo {  pdf_version: 1.0, entry_type: CatalogEntryType::Name }),
        ("PageMode",          CatalogEntryInfo {  pdf_version: 1.0, entry_type: CatalogEntryType::Name }),
        ("Outlines",          CatalogEntryInfo {  pdf_version: 1.1, entry_type: CatalogEntryType::IndirectRef }),
        ("Threads",           CatalogEntryInfo {  pdf_version: 1.1, entry_type: CatalogEntryType::Array }),
        ("OpenAction",        CatalogEntryInfo {  pdf_version: 1.1, entry_type: CatalogEntryType::Array }),
        ("AA",                CatalogEntryInfo {  pdf_version: 1.4, entry_type: CatalogEntryType::Dictionary }),
        ("URI",               CatalogEntryInfo {  pdf_version: 1.1, entry_type: CatalogEntryType::Dictionary }),
        ("AcroForm",          CatalogEntryInfo {  pdf_version: 1.2, entry_type: CatalogEntryType::Dictionary }),
        ("Metadata",          CatalogEntryInfo {  pdf_version: 1.4, entry_type: CatalogEntryType::IndirectRef }),
        ("StructTreeRoot",    CatalogEntryInfo {  pdf_version: 1.3, entry_type: CatalogEntryType::Dictionary }),
        ("MarkInfo",          CatalogEntryInfo {  pdf_version: 1.4, entry_type: CatalogEntryType::Dictionary }),
        ("Lang",              CatalogEntryInfo {  pdf_version: 1.4, entry_type: CatalogEntryType::Name }),
        ("SpiderInfo",        CatalogEntryInfo {  pdf_version: 1.3, entry_type: CatalogEntryType::Dictionary }),
        ("OutputIntents",     CatalogEntryInfo {  pdf_version: 1.4, entry_type: CatalogEntryType::Array }),
        ("PieceInfo",         CatalogEntryInfo {  pdf_version: 1.4, entry_type: CatalogEntryType::Dictionary }),
        ("OCProperties",      CatalogEntryInfo {  pdf_version: 1.5, entry_type: CatalogEntryType::Dictionary }),
        ("Perms",             CatalogEntryInfo {  pdf_version: 1.5, entry_type: CatalogEntryType::Dictionary }),
        ("Legal",             CatalogEntryInfo {  pdf_version: 1.5, entry_type: CatalogEntryType::Dictionary }),
        ("Requirements",      CatalogEntryInfo {  pdf_version: 1.7, entry_type: CatalogEntryType::Array }),
        ("Collection",        CatalogEntryInfo {  pdf_version: 1.7, entry_type: CatalogEntryType::Dictionary }),
        ("NeedsRendering",    CatalogEntryInfo {  pdf_version: 1.7, entry_type: CatalogEntryType::Boolean }),
    ])
});

//--------------------------- Catalog factories -------------------------

/// Validates that the catalog entry is supported and checks PDF version compatibility
pub fn validate_catalog_entry(name: &str, pdf_version: f32) -> Result<&CatalogEntryInfo, CatalogError> {
    let info = SUPPORTED_CATALOG_ENTRIES
        .get(name)
        .ok_or_else(|| CatalogError::UnsupportedEntry(name.to_string()))?;

    if info.pdf_version > pdf_version {
        return Err(CatalogError::UnsupportedVersion {
            entry: name.to_string(),
            required: info.pdf_version,
            current: pdf_version,
        });
    }

    Ok(info)
}

/// Creates a catalog dictionary entry with validation
pub fn make_catalog_dict(name: &str, pdf_version: f32) -> Result<DictionaryObject, CatalogError> {
    let info = validate_catalog_entry(name, pdf_version)?;

    if info.entry_type != CatalogEntryType::Dictionary {
        return Err(CatalogError::UnsupportedEntry(
            format!("{} is not a dictionary type", name)
        ));
    }

    Ok(DictionaryObject::typed(name))
}

/// Creates a catalog array entry with validation
pub fn make_catalog_array(name: &str, pdf_version: f32) -> Result<ArrayObject, CatalogError> {
    let info = validate_catalog_entry(name, pdf_version)?;

    if info.entry_type != CatalogEntryType::Array {
        return Err(CatalogError::UnsupportedEntry(
            format!("{} is not an array type", name)
        ));
    }

    Ok(ArrayObject::new(None))
}

/// Returns metadata for a catalog entry if it exists
pub fn get_catalog_entry_info(name: &str) -> Option<&CatalogEntryInfo> {
    SUPPORTED_CATALOG_ENTRIES.get(name)
}

/// Returns all required catalog entries
pub fn get_required_entries() -> Vec<&'static str> {
    SUPPORTED_CATALOG_ENTRIES
        .iter()
        .filter(|(_, info)| info.required)
        .map(|(name, _)| *name)
        .collect()
}

//--------------------------- Catalog -------------------------

struct Catalog {
    pages: PageTreeNode,
    version: Option<PdfVersion>,
}

impl Catalog {
    pub fn new(pages: PageTreeNode, version: Option<PdfVersion>) -> Self {
        Self {
            pages,
            version,
        }
    }
}