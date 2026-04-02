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

/*pub struct Trailer{
    dataframe: Vec<u8>,
}

impl Trailer{
    pub fn new() -> Self {
        Trailer{
            dataframe: vec![],
        }
    }
}

impl Trailer {
/*    pub fn write(&self, stream: &&mut PdfStream) -> Result<(), std::io::Result<()>>{  // Write trailer
        stream.write_line(b"trailer")?;
        stream.write_line(b"<<")?;
        stream.write_line(format!("/Size {}", pdf.object_count()).as_bytes())?;
        stream.write_line(
            format!(
                "/Root {} 0 R",
                pdf.catalog.metadata.object_identifier.unwrap()
            )
                .as_bytes(),
        )?;

        if !pdf.info.values.is_empty() {
            stream.write_line(
                &format!("/Info {} 0 R", pdf.info.metadata.object_identifier.unwrap()).into_bytes(),
            )?;
        }

        if let Some(id_line) = Self::format_identifier(&pdf.objects, id_mode) {
            stream.write_line(&id_line)?;
        }

        stream.write_line(b">>")?;

        Ok(())
    }
*/}
*/