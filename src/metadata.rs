//! PDF metadata framework (XMP and document information).
//!
//! Provides structures for embedding metadata in PDF documents, including
//! both the legacy Info dictionary and modern XMP metadata streams.
/// Document information dictionary (legacy PDF metadata).
///
/// This is the traditional way of storing document metadata in PDF,
/// predating XMP. Still widely supported and used.
///
use crate::objects::pdf_object::PdfObj;
use crate::{PdfDictionaryObject, PdfResult, PdfStreamObject};

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

#[derive(Clone, Debug, Default)]
pub struct DocumentInfo {
    pub title: Option<String>,
    pub author: Option<String>,
    pub subject: Option<String>,
    pub keywords: Option<String>,
    pub creator: Option<String>,
    pub producer: Option<String>,
    pub creation_date: Option<String>,
    pub mod_date: Option<String>,
    pub trapped: Option<TrappedState>,
}

impl DocumentInfo {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_title(mut self, title: String) -> Self {
        self.title = Some(title);
        self
    }

    pub fn with_author(mut self, author: String) -> Self {
        self.author = Some(author);
        self
    }

    pub fn with_subject(mut self, subject: String) -> Self {
        self.subject = Some(subject);
        self
    }

    pub fn with_keywords(mut self, keywords: String) -> Self {
        self.keywords = Some(keywords);
        self
    }

    pub fn with_creator(mut self, creator: String) -> Self {
        self.creator = Some(creator);
        self
    }

    pub fn with_producer(mut self, producer: String) -> Self {
        self.producer = Some(producer);
        self
    }

    pub fn with_creation_date(mut self, date: String) -> Self {
        self.creation_date = Some(date);
        self
    }

    pub fn with_mod_date(mut self, date: String) -> Self {
        self.mod_date = Some(date);
        self
    }

    pub fn with_trapped(mut self, trapped: TrappedState) -> Self {
        self.trapped = Some(trapped);
        self
    }

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

    pub fn to_dict(&self) -> PdfDictionaryObject {
        let mut dict = PdfDictionaryObject::new();

        if let Some(ref title) = self.title {
            dict.add("Title", PdfObj::string(title.as_str()));
        }

        if let Some(ref author) = self.author {
            dict.add("Author", PdfObj::string(author.as_str()));
        }

        if let Some(ref subject) = self.subject {
            dict.add("Subject", PdfObj::string(subject.as_str()));
        }

        if let Some(ref keywords) = self.keywords {
            dict.add("Keywords", PdfObj::string(keywords.as_str()));
        }

        if let Some(ref creator) = self.creator {
            dict.add("Creator", PdfObj::string(creator.as_str()));
        }

        if let Some(ref producer) = self.producer {
            dict.add("Producer", PdfObj::string(producer.as_str()));
        }

        if let Some(ref creation_date) = self.creation_date {
            dict.add("CreationDate", PdfObj::string(creation_date.as_str()));
        }

        if let Some(ref mod_date) = self.mod_date {
            dict.add("ModDate", PdfObj::string(mod_date.as_str()));
        }

        if let Some(trapped) = self.trapped {
            dict.add("Trapped", PdfObj::string(trapped.as_name()));
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

    pub fn to_stream(&self) -> PdfResult<PdfStreamObject> {
        let mut dict = PdfDictionaryObject::new().typed("Metadata");
        dict.add("SubType", PdfObj::name("XML"));

        let stream = PdfStreamObject::new().with_data(self.xmp_packet.as_bytes().to_vec(), dict);

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
        let mut stream = xmp.to_stream().unwrap();

        assert!(!stream.serialise().expect("REASON").is_empty());
    }
}
