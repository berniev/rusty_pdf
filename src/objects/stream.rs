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

use crate::PdfDictionaryObject;
use crate::error::PdfError;
use crate::objects::pdf_object::PdfObj;
pub use crate::util::{CompressionMethod, Dims, Matrix, Posn, StrokeOrFill, ToPdf, WindingRule};

//------------------------ PdfStreamObject -----------------------

#[derive(Clone)]
pub struct PdfStreamObject {
    pub(crate) dict: PdfDictionaryObject,
    pub(crate) content: Vec<u8>,
    pub(crate) object_number: Option<u64>,
    pub(crate) generation_number: Option<u16>,

    pub(crate) compression_method: CompressionMethod,
}

impl PdfStreamObject {
    //-------------------------- Constructors --------------------------
    pub fn new(object_number: u64) -> Self {
        Self {
            dict: PdfDictionaryObject::new(),
            content: Vec::new(),
            object_number: Some(object_number), // all streams objects are indirect
            generation_number: None,
            compression_method: CompressionMethod::None,
        }
    }

    pub fn compressed(mut self) -> Self {
        self.compression_method = CompressionMethod::Flate;
        self.dict.add("Filter", PdfObj::make_name_obj("FlateDecode"));
        self
    }

    pub fn with_data(mut self, stream: Vec<u8>, dict: PdfDictionaryObject) -> Self {
        self.content = stream;
        self.dict = dict;

        self
    }

    //----------------------------------------------------------------

    pub fn compression_method(&self) -> CompressionMethod {
        self.compression_method
    }

    pub fn add(&mut self, bytes: Vec<u8>) {
        self.content.extend(bytes);
    }

    pub fn serialise(&self) -> Result<Vec<u8>, PdfError> {
        let stream_bytes: Vec<u8> = match self.compression_method {
            CompressionMethod::None => self.content.clone(),
            CompressionMethod::Flate => {
                let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
                encoder.write_all(&self.content)?;
                encoder.finish()?
            }
        };

        let mut dict = self.dict.clone(); // else self must be mut, which it can't be
        dict.add("Length", stream_bytes.len() as f64);

        let mut vec = dict.serialise()?; // direct object

        vec.push(b'\n');
        vec.extend(b"stream\n");
        vec.extend(&stream_bytes);
        vec.extend(b"endstream\n");

        Ok(vec)
    }
}
