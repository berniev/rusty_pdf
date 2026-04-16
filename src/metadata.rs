use crate::objects::pdf_object::PdfObj;
use crate::{PdfDictionaryObject, PdfError, PdfResult, PdfStreamObject};

//--------------------------TrappedState-------------------------------//

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrappedState {
    True,
    False,
    Unknown,
}

impl TrappedState {
    pub fn as_name(&self) -> &'static str {
        match self {
            TrappedState::True => "True",
            TrappedState::False => "False",
            TrappedState::Unknown => "Unknown",
        }
    }
}

//--------------------------DocumentInfo-------------------------------//

#[derive(Clone)]
pub struct Metadata {
    pub dictionary: PdfDictionaryObject,
}

impl Metadata {
    pub fn new() -> Result<Self, PdfError> {
        Ok(Self {
            dictionary: PdfDictionaryObject::new(),
        })
    }

    fn add(mut self, key: &str, value: &str) -> Result<Self, PdfError> {
        self.dictionary.add(key, PdfObj::make_string_obj(value))?;

        Ok(self)
    }

    pub fn with_title(self, title: &str) -> Result<Self, PdfError> {
        Ok(self.add("Title", title)?)
    }

    pub fn with_author(self, author: &str) -> Result<Self, PdfError> {
        Ok(self.add("Author", author)?)
    }

    pub fn with_subject(self, subject: &str) -> Result<Self, PdfError> {
        Ok(self.add("Subject", subject)?)
    }

    pub fn with_keywords(self, keywords: &str) -> Result<Self, PdfError> {
        Ok(self.add("Keywords", keywords)?)
    }

    pub fn with_creator(self, creator: &str) -> Result<Self, PdfError> {
        Ok(self.add("Creator", creator)?)
    }

    pub fn with_producer(self, producer: &str) -> Result<Self, PdfError> {
        Ok(self.add("Producer", producer)?)
    }

    pub fn with_creation_date(self, date: &str) -> Result<Self, PdfError> {
        Ok(self.add("CreationDate", date)?)
    }

    pub fn with_mod_date(self, date: &str) -> Result<Self, PdfError> {
        Ok(self.add("ModDate", date)?)
    }

    pub fn with_trapped(self, trapped: TrappedState) -> Result<Self, PdfError> {
        Ok(self.add("Trapped", trapped.as_name())?)
    }

    pub fn is_empty(&self) -> bool {
        self.dictionary.len() == 0
     }
}
