use crate::{Metadata, PdfDictionaryObject, PdfResult, PdfStreamObject};
use crate::objects::pdf_object::PdfObj;

/// XMP (Extensible Metadata Platform) (simplified)
pub struct XmpMetadata {
    pub xmp_packet: String,
}

impl XmpMetadata {
    /// Create XMP metadata from raw XML packet.
    pub fn from_packet(xmp_packet: String) -> Self {
        Self { xmp_packet }
    }

    /// Create a minimal Dublin Core XMP packet.
    pub fn from_document_info(info: &Metadata) -> Self {
        let mut xmp = String::from(
            r#"<?xpacket begin="" id="W5M0MpCehiHzreSzNTczkc9d"?>
<x:xmpmeta xmlns:x="adobe:ns:meta/">
<rdf:RDF xmlns:rdf="http://www.w3.org/1999/02/22-rdf-syntax-ns#">
<rdf:Description rdf:about="" xmlns:dc="http://purl.org/dc/elements/1.1/">"#,
        );

        if let Some(ref title) = info.get("Title") {
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
        let mut dict = PdfDictionaryObject::new().typed("Metadata")?;
        dict.add("SubType", PdfObj::make_name_obj("XML"))?;

        let stream = PdfStreamObject::new()
            .with_object_number(1u64)
            .with_data(self.xmp_packet.as_bytes().to_vec(), dict);

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
