//! PDF metadata framework (XMP and document information).
//!
//! Provides structures for embedding metadata in PDF documents, including
//! both the legacy Info dictionary and modern XMP metadata streams.

use crate::{DictionaryObject, NameObject, PdfResult, StreamObject, StringObject};

#[cfg(test)]
use crate::PdfObject;

/// Document information dictionary (legacy PDF metadata).
///
/// This is the traditional way of storing document metadata in PDF,
/// predating XMP. Still widely supported and used.
#[derive(Clone, Debug, Default)]
pub struct DocumentInfo {
    /// Document title.
    pub title: Option<String>,

    /// Name of person who created the document.
    pub author: Option<String>,

    /// Subject of the document.
    pub subject: Option<String>,

    /// Keywords associated with the document.
    pub keywords: Option<String>,

    /// Name of application that created the original document.
    pub creator: Option<String>,

    /// Name of application that converted to PDF.
    pub producer: Option<String>,

    /// Document creation date (PDF date format).
    pub creation_date: Option<String>,

    /// Document modification date (PDF date format).
    pub mod_date: Option<String>,

    /// PDF trapping state.
    pub trapped: Option<TrappedState>,
}

/// PDF trapping state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrappedState {
    /// Document has been trapped.
    True,
    /// Document has not been trapped.
    False,
    /// Unknown trapping state.
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

impl DocumentInfo {
    /// Create a new empty document info.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the title.
    pub fn with_title(mut self, title: String) -> Self {
        self.title = Some(title);
        self
    }

    /// Set the author.
    pub fn with_author(mut self, author: String) -> Self {
        self.author = Some(author);
        self
    }

    /// Set the subject.
    pub fn with_subject(mut self, subject: String) -> Self {
        self.subject = Some(subject);
        self
    }

    /// Set keywords.
    pub fn with_keywords(mut self, keywords: String) -> Self {
        self.keywords = Some(keywords);
        self
    }

    /// Set the creator application.
    pub fn with_creator(mut self, creator: String) -> Self {
        self.creator = Some(creator);
        self
    }

    /// Set the producer application.
    pub fn with_producer(mut self, producer: String) -> Self {
        self.producer = Some(producer);
        self
    }

    /// Set creation date (PDF date format: D:YYYYMMDDHHmmSSOHH'mm).
    pub fn with_creation_date(mut self, date: String) -> Self {
        self.creation_date = Some(date);
        self
    }

    /// Set modification date (PDF date format: D:YYYYMMDDHHmmSSOHH'mm).
    pub fn with_mod_date(mut self, date: String) -> Self {
        self.mod_date = Some(date);
        self
    }

    /// Set trapping state.
    pub fn with_trapped(mut self, trapped: TrappedState) -> Self {
        self.trapped = Some(trapped);
        self
    }

    /// Check if any metadata is set.
    pub fn is_empty(&self) -> bool {
        self.title.is_none()
            && self.author.is_none()
            && self.subject.is_none()
            && self.keywords.is_none()
            && self.creator.is_none()
            && self.producer.is_none()
            && self.creation_date.is_none()
            && self.mod_date.is_none()
            && self.trapped.is_none()
    }

    /// Convert to PDF dictionary.
    pub fn to_dict(&self) -> DictionaryObject {
        let mut dict = DictionaryObject::new(None);

        if let Some(ref title) = self.title {
            dict.set("Title", StringObject::make_pdf_obj(title.clone()));
        }

        if let Some(ref author) = self.author {
            dict.set("Author", StringObject::make_pdf_obj(author.clone()));
        }

        if let Some(ref subject) = self.subject {
            dict.set("Subject", StringObject::make_pdf_obj(subject.clone()));
        }

        if let Some(ref keywords) = self.keywords {
            dict.set("Keywords", StringObject::make_pdf_obj(keywords.clone()));
        }

        if let Some(ref creator) = self.creator {
            dict.set("Creator", StringObject::make_pdf_obj(creator.clone()));
        }

        if let Some(ref producer) = self.producer {
            dict.set("Producer", StringObject::make_pdf_obj(producer.clone()));
        }

        if let Some(ref creation_date) = self.creation_date {
            dict.set(
                "CreationDate",
                StringObject::make_pdf_obj(creation_date.clone()),
            );
        }

        if let Some(ref mod_date) = self.mod_date {
            dict.set("ModDate", StringObject::make_pdf_obj(mod_date.clone()));
        }

        if let Some(trapped) = self.trapped {
            dict.set("Trapped", NameObject::make_pdf_obj(trapped.as_name()));
        }

        dict
    }
}

/// XMP (Extensible Metadata Platform) metadata.
///
/// XMP is the modern, XML-based metadata format used in PDFs.
/// This is a simplified interface; full XMP support would require XML generation.
pub struct XmpMetadata {
    /// Raw XMP packet (XML).
    pub xmp_packet: String,
}

impl XmpMetadata {
    /// Create XMP metadata from raw XML packet.
    pub fn from_packet(xmp_packet: String) -> Self {
        Self { xmp_packet }
    }

    /// Create a basic XMP packet from document info.
    ///
    /// This generates a minimal Dublin Core XMP packet.
    pub fn from_document_info(info: &DocumentInfo) -> Self {
        let mut xmp = String::from(
            r#"<?xpacket begin="" id="W5M0MpCehiHzreSzNTczkc9d"?>
<x:xmpmeta xmlns:x="adobe:ns:meta/">
<rdf:RDF xmlns:rdf="http://www.w3.org/1999/02/22-rdf-syntax-ns#">
<rdf:Description rdf:about="" xmlns:dc="http://purl.org/dc/elements/1.1/">"#,
        );

        if let Some(ref title) = info.title {
            xmp.push_str(&format!(
                r#"
  <dc:title>
    <rdf:Alt>
      <rdf:li xml:lang="x-default">{}</rdf:li>
    </rdf:Alt>
  </dc:title>"#,
                Self::escape_xml(title)
            ));
        }

        if let Some(ref author) = info.author {
            xmp.push_str(&format!(
                r#"
  <dc:creator>
    <rdf:Seq>
      <rdf:li>{}</rdf:li>
    </rdf:Seq>
  </dc:creator>"#,
                Self::escape_xml(author)
            ));
        }

        if let Some(ref subject) = info.subject {
            xmp.push_str(&format!(
                r#"
  <dc:description>
    <rdf:Alt>
      <rdf:li xml:lang="x-default">{}</rdf:li>
    </rdf:Alt>
  </dc:description>"#,
                Self::escape_xml(subject)
            ));
        }

        xmp.push_str(
            r#"
</rdf:Description>
</rdf:RDF>
</x:xmpmeta>
<?xpacket end="w"?>"#,
        );

        Self { xmp_packet: xmp }
    }

    /// Convert to PDF metadata stream object.
    pub fn to_stream(&self) -> PdfResult<StreamObject> {
        // Add required dictionary entries
        let dict_entries = vec![
            ("Type".to_string(), NameObject::make_pdf_obj("Metadata")),
            ("Subtype".to_string(), NameObject::make_pdf_obj("XML")),
        ];

        let stream = StreamObject::new().with_data(
            Some(vec![self.xmp_packet.as_bytes().to_vec()]),
            Some(dict_entries),
        );

        Ok(stream)
    }

    fn escape_xml(s: &str) -> String {
        s.replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;")
            .replace('"', "&quot;")
            .replace('\'', "&apos;")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_document_info_empty() {
        let info = DocumentInfo::new();
        assert!(info.is_empty());
    }

    #[test]
    fn test_document_info_with_metadata() {
        let info = DocumentInfo::new()
            .with_title("My Document".to_string())
            .with_author("John Doe".to_string())
            .with_subject("Testing".to_string());

        assert!(!info.is_empty());
        assert_eq!(info.title, Some("My Document".to_string()));
        assert_eq!(info.author, Some("John Doe".to_string()));
    }

    #[test]
    fn test_document_info_to_dict() {
        let info = DocumentInfo::new()
            .with_title("Test".to_string())
            .with_author("Author".to_string());

        let dict = info.to_dict();
        assert!(dict.contains_key("Title"));
        assert!(dict.contains_key("Author"));
    }

    #[test]
    fn test_trapped_state() {
        assert_eq!(TrappedState::True.as_name(), "True");
        assert_eq!(TrappedState::False.as_name(), "False");
        assert_eq!(TrappedState::Unknown.as_name(), "Unknown");
    }

    #[test]
    fn test_xmp_from_document_info() {
        let info = DocumentInfo::new()
            .with_title("Test Document".to_string())
            .with_author("Test Author".to_string());

        let xmp = XmpMetadata::from_document_info(&info);
        assert!(xmp.xmp_packet.contains("Test Document"));
        assert!(xmp.xmp_packet.contains("Test Author"));
        assert!(xmp.xmp_packet.contains("<?xpacket"));
    }

    #[test]
    fn test_xmp_xml_escaping() {
        let info = DocumentInfo::new().with_title("<Test & \"Special\" Characters>".to_string());

        let xmp = XmpMetadata::from_document_info(&info);
        assert!(xmp.xmp_packet.contains("&lt;"));
        assert!(xmp.xmp_packet.contains("&amp;"));
        assert!(xmp.xmp_packet.contains("&quot;"));
    }

    #[test]
    fn test_xmp_to_stream() {
        let xmp = XmpMetadata::from_packet("<xml>test</xml>".to_string());
        let stream = xmp.to_stream().unwrap();

        // Check that stream was created (extra entries are internal)
        // Full validation would require checking the generated data() output
        assert!(!stream.data().is_empty());
    }
}
