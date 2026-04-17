/*
Trailer entries

=======  ==========  =====  =====================================================================
Key      Type        Reqd   Value
=======  ==========  =====  =====================================================================
Size     Number      Reqd   The number of objects in the file.
Root     Object      Reqd   Indirect Ref. The object that is the root of the object hierarchy.
Info     Object      Opt    A dictionary that contains information about the document.
ID       Array       Reqd*  If Encrypt entry present, else opt, but recommended.
                            A two-element array that uniquely identifies the document.
Encrypt  Dictionary  Reqd*  If doc is encrypted. Specifies how the document is encrypted.
*/
use std::fs::File;
use std::io::Write;
use crate::file_identifier::FileIdentifierMode;
use crate::objects::pdf_object::PdfObj;
use crate::string_functions::encode_pdf_string;
use crate::{PdfArrayObject, PdfDictionaryObject, PdfError, PdfObject};
use crate::xref_ops::XRefOps;

pub struct Trailer {
    dict: PdfDictionaryObject,
}

impl Trailer {
    pub fn new() -> Self {
        Trailer {
            dict: PdfDictionaryObject::new(),
        }
    }

    pub fn with_size(mut self, size: u64) -> Result<Self, PdfError> {
        self.dict.add("Size", size)?;

        Ok(self)
    }

    pub fn with_root(mut self, root: u64) -> Result<Self, PdfError> {
        self.dict.add("Root", PdfObj::make_reference_obj(root))?;

        Ok(self)
    }

    pub fn encrypted(&mut self) -> Result<&mut Self, PdfError> {
        let mut encryption_dict = PdfDictionaryObject::new(); // not typed, direct
        encryption_dict.add("Filter", PdfObj::make_name_obj("Standard"))?;

        self.dict.add("Encrypt", encryption_dict)?;

        let mut id_array = PdfArrayObject::new();
        id_array.push(PdfObj::make_string_obj("1234567890"));
        id_array.push(PdfObj::make_string_obj("0987654321"));

        self.dict.add("ID", id_array)?;

        Ok(self)
    }

    pub fn serialise(&self, xref:&mut XRefOps, file:&mut File) -> Result<(), PdfError> {
        let mut bytes :Vec<u8>= vec![];
        bytes.extend(b"\ntrailer\n");
        bytes.extend(self.dict.encode()?);
        bytes.extend(format!("startxref\n{}\n%%EOF\n", xref.position).as_bytes());

    file.write_all(&bytes)?;

        Ok(())
    }

    /// Formats two byte arrays into a PDF ID array string.
    fn format_id_array(first_id: &[u8], second_id: &[u8]) -> Vec<u8> {
        let s1 = encode_pdf_string(&String::from_utf8_lossy(first_id));
        let s2 = encode_pdf_string(&String::from_utf8_lossy(second_id));
        format!("/ID [{} {}]", s1, s2).into_bytes()
    }

    /// Computes MD5 hash of all non-free objects and returns both hex string and bytes.
    fn compute_data_hash(_objects: &[PdfObject]) -> (String, Vec<u8>) {
        let context = md5::Context::new();
        /*       for obj in objects {
                    /*if obj.metadata().status != ObjectStatus::Free {
                        context.consume(obj.serialise());
                    }*/
                }
        */
        let hash_result = context.finalize().0;
        let data_hash_hex: String = hash_result.iter().map(|b| format!("{:02x}", b)).collect();
        let data_hash_bytes = data_hash_hex.as_bytes().to_vec();
        (data_hash_hex, data_hash_bytes)
    }

    fn get_id_bytes<'a>(
        identifier_mode: &'a FileIdentifierMode,
        data_hash_bytes: &'a [u8],
    ) -> &'a [u8] {
        match identifier_mode {
            FileIdentifierMode::Custom(bytes) => bytes.as_slice(),
            _ => data_hash_bytes,
        }
    }

    #[allow(dead_code)]
    fn format_identifier(
        objects: &[PdfObject],
        identifier_mode: &FileIdentifierMode,
    ) -> Option<Vec<u8>> {
        match identifier_mode {
            FileIdentifierMode::None => None,
            FileIdentifierMode::AutoMD5 | FileIdentifierMode::Custom(_) => {
                let (_data_hash_hex, data_hash_bytes) = Self::compute_data_hash(objects);
                let id_bytes = Self::get_id_bytes(identifier_mode, &data_hash_bytes);
                Some(Self::format_id_array(id_bytes, &data_hash_bytes))
            }
        }
    }
}
