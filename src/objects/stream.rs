use crate::PdfDictionaryObject;
use crate::error::PdfError;
use crate::objects::pdf_object::PdfObj;
pub use crate::util::{CompressionMethod, Dims, Matrix, Posn, StrokeOrFill, ToPdf, WindingRule};
use flate2::Compression;
use flate2::write::ZlibEncoder;
/// PDF content stream
///
/// Content streams most commonly define page content, e.g.
/// - Graphics: paths, rectangles, curves
/// - Text: fonts, positioning, display
/// - Colors: RGB, CMYK, grayscale
/// - Images: inline images
/// - Transformations: matrices, state management
/// but have other uses as well.
///
/// A stream object is a (potentially very long) sequence of bytes. Objects with potentially large
/// amounts of data, such as images and page descriptions, shall be represented as streams.
///
/// A stream shall be an indirect object and consist of a direct dictionary object (known as the
/// Stream Extent) followed by zero or more bytes bracketed between the keywords'stream' and
/// 'endstream'.
/// ```
///     5 0 obj          ← object number + generation number
///     <<
///       /Length 42
///       /Filter /FlateDecode
///     >>
///     stream
///     ...bytes...
///     endstream
///     endobj
///```
/// ```
/// Stream dictionary (Stream Extent) Entries:
/// ===========================================================================
/// Name          Type     Reqd Description
/// ============  ==========  = ===============================================
/// Length        int         R The length of the stream in bytes
/// DL            int         O Non-negative len of the decoded stream in bytes
/// Filter        nam or arr  O A filter or sequence of filters to be applied
/// DecodeParms   dic or arr  O Parameters for the filter(s) in Filter
///
/// F             filespec    O A file specification for the stream data
/// FFilter       nam or arr  O A filter or sequence of filters to file data
/// FDecodeParms  dic or arr  O Parameters for the filter(s) in FFilter
/// ===========================================================================
/// ```
/// Stream Filters:
/// Indicate how the data in the stream should be decoded before it is used. Used in "Filter" and
/// "FFilter" dict entries.
/// ```
/// Stream Filters:
/// =============================================================================
/// Name            P V Type    Decode/Decompress
/// =============== = = ======= =================================================
/// DCTDecode       y 5 image   Discrete Cosine Transform technique based on JPEG
/// JPXDecode       n 5 image   Wwavelet-based JPEG2000 standard
/// JBIG2Decode     y 4 image   JBig2 standard -> mono or approx
/// ASCIIHexDecode  n   binary  ASCII hex
/// ASCII85Decode   n   binary  ASCII base-85
/// LZWDecode       y   txt/bin LZE (Lempel-Ziv-Welch) algorithm
/// FlateDecode     y 2 txt/bin zlib/deflate compression
/// RunLengthDecode n   txt/bin byte-oriented run-length encoding algorithm
/// CCITTFaxDecode  y   image   CCITT facsimile standard. typ mono 1 bit/pixel
/// JBIG2Decode     y 4 image   JBig2 standard -> mono or approx
/// DCTDecode       y   image   Discrete Cosine Transform technique based on JPEG
/// JPXDecode       n 5 image   Wwavelet-based JPEG2000 standard
/// Crypt           y 5 data    Data encrypted by a security handler
/// =============================================================================
/// ```
/// Beginning with PDF 1.5, indirect objects may reside in object streams.
/// They are referred to in the same way; however, their definition shall not include the keywords
/// obj and endobj, and their generation number shall be zero.
///
use std::io::Write as IoWrite;

#[derive(Clone)]
pub struct PdfStreamObject {
    pub(crate) dict: PdfDictionaryObject,
    pub(crate) content: Vec<u8>,
    pub(crate) object_number: Option<u64>,
    pub(crate) generation_number: Option<u16>,

    pub(crate) compression_method: CompressionMethod,
}

impl PdfStreamObject {
    pub fn new() -> Self {
        Self {
            dict: PdfDictionaryObject::new(),
            content: Vec::new(),
            object_number: None, // all streams objects are indirect
            generation_number: None,
            compression_method: CompressionMethod::None,
        }
    }

    pub fn with_dict_and_content(mut self, dict: PdfDictionaryObject, content: Vec<u8>) -> Self {
        self.dict = dict;
        self.content = content;

        self
    }

    pub fn with_object_number(mut self, value: u64) -> Self {
        self.object_number = Some(value);

        self
    }

    pub fn with_generation_number(mut self, value: u16) -> Self {
        self.generation_number = Some(value);

        self
    }

    pub fn compressed(mut self) -> Result<Self, PdfError> {
        self.compression_method = CompressionMethod::Flate;
        self.dict
            .add("Filter", PdfObj::make_name_obj("FlateDecode"))?;

        Ok(self)
    }

    pub fn with_data(mut self, data: Vec<u8>, dict: PdfDictionaryObject) -> Self {
        self.content = data;
        self.dict = dict;

        self
    }

    pub fn compression_method(&self) -> CompressionMethod {
        self.compression_method
    }

    pub fn add(&mut self, bytes: Vec<u8>) {
        self.content.extend(bytes);
    }

    pub fn encode(&self) -> Result<Vec<u8>, PdfError> {
        let stream_bytes: Vec<u8> = match self.compression_method {
            CompressionMethod::None => self.content.clone(),
            CompressionMethod::Flate => {
                let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
                encoder.write_all(&self.content)?;
                encoder.finish()?
            }
        };

        let mut dict = self.dict.clone();
        dict.add("Length", stream_bytes.len() as f64)?;

        let mut vec = vec![];
        vec.extend(dict.encode()?);
        vec.extend(b"stream\n");
        vec.extend(stream_bytes);
        vec.extend(b"\nendstream\n");

        Ok(vec)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_empty_stream() {
        let stream = PdfStreamObject::new().with_object_number(1);
        let output = String::from_utf8(stream.encode().unwrap()).unwrap();
        assert!(output.contains("/Length 0"));
        assert!(output.contains("stream\n"));
        assert!(output.contains("\nendstream\n"));
    }

    #[test]
    fn encode_stream_with_content() {
        let mut stream = PdfStreamObject::new().with_object_number(1);
        stream.add(b"some data".to_vec());
        let output = String::from_utf8(stream.encode().unwrap()).unwrap();
        assert!(output.contains("/Length 9"));
        assert!(output.contains("stream\nsome data\nendstream\n"));
        stream.add(b"BT /F1 12 Tf ET".to_vec());
        let output = String::from_utf8(stream.encode().unwrap()).unwrap();
        assert!(output.contains("/Length 15"));
        assert!(output.contains("stream\nBT /F1 12 Tf ET\nendstream\n"));
    }

    #[test]
    fn encode_stream_length_matches_content() {
        let mut stream = PdfStreamObject::new().with_object_number(1);
        let content = b"q 1 0 0 1 100 200 cm Q";
        stream.add(content.to_vec());
        let output = String::from_utf8(stream.encode().unwrap()).unwrap();
        assert!(output.contains(&format!("/Length {}", content.len())));
    }

    #[test]
    fn encode_compressed_stream_has_filter() {
        let stream = PdfStreamObject::new().with_object_number(1).compressed();
        let encoded = stream.expect("REASON").encode().unwrap();
        let contains = |needle: &[u8]| encoded.windows(needle.len()).any(|w| w == needle);
        assert!(contains(b"/Filter /FlateDecode"));
        assert!(contains(b"stream\n"));
        assert!(contains(b"\nendstream\n"));
    }

    #[test]
    fn encode_stream_with_dict_entries() {
        let mut stream = PdfStreamObject::new().with_object_number(1);
        stream
            .dict
            .add("Type", PdfObj::make_name_obj("XObject"))
            .expect("fail");
        stream.add(b"some data".to_vec());
        let output = String::from_utf8(stream.encode().unwrap()).unwrap();
        assert!(output.contains("/Type /XObject"));
        assert!(output.contains("/Length 9"));
    }
}
