use crate::object_ops::PdfObject;
use crate::{PdfArrayObject, PdfDictionaryObject, PdfError};

pub struct FileSpecification {
    dict: PdfDictionaryObject,
}
impl FileSpecification {
    pub fn new() -> Result<Self, PdfError> {
        let dict = PdfDictionaryObject::new().typed("FileSpec")?;
        Ok(Self { dict })
    }

    pub fn with_name(mut self, name: &str) -> Result<Self, PdfError> {
        self.dict.add("FS", PdfObject::name(name))?;
        Ok(self)
    }

    pub fn with_spec_string(mut self, spec: &str) -> Result<Self, PdfError> {
        self.dict.add("F", spec)?;
        Ok(self)
    }

    pub fn with_doc_encoding(mut self, encoding: &str) -> Result<Self, PdfError> {
        self.dict.add("UF", encoding)?;
        Ok(self)
    }

    pub fn with_dos_name(mut self, dos_name: &str) -> Result<Self, PdfError> {
        self.dict.add("EF", dos_name)?;
        Ok(self)
    }

    pub fn with_mac_name(mut self, mac_name: &str) -> Result<Self, PdfError> {
        self.dict.add("Mac", mac_name)?;
        Ok(self)
    }

    pub fn with_unix_name(mut self, unix_name: &str) -> Result<Self, PdfError> {
        self.dict.add("Unix", unix_name)?;
        Ok(self)
    }

    pub fn with_id(mut self, id1: &str, id2: &str) -> Result<Self, PdfError> {
        let mut arr = PdfArrayObject::new();
        arr.push(id1);
        arr.push(id2);
        self.dict.add("ID", arr)?;

        Ok(self)
    }

    pub fn with_volatile(mut self, volatile: bool) -> Result<Self, PdfError> {
        self.dict.add("V", volatile)?;
        Ok(self)
    }

    pub fn with_embedded_file_streams(
        mut self,
        embedded_file_streams: EmbeddedFileStreams,
    ) -> Result<Self, PdfError> {
        self.dict.add("EF", embedded_file_streams.dict)?;
        Ok(self)
    }

    pub fn with_related_file_streams(
        mut self,
        related_file_streams: RelatedFileStreams,
    ) -> Result<Self, PdfError> {
        self.dict.add("RF", related_file_streams.dict)?;
        Ok(self)
    }

    pub fn with_description(mut self, description: &str) -> Result<Self, PdfError> {
        self.dict.add("Desc", description)?;
        Ok(self)
    }

    pub fn with_collection_items(
        mut self,
        collection_items: CollectionItems,
    ) -> Result<Self, PdfError> {
        self.dict.add("Collection", collection_items.dict)?;
        Ok(self)
    }
}

pub struct EmbeddedFileStreams {
    pub dict: PdfDictionaryObject,
}

pub struct RelatedFileStreams {
    pub dict: PdfDictionaryObject,
}

pub struct CollectionItems {
    pub dict: PdfDictionaryObject,
}
