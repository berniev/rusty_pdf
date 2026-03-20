use crate::generation::Generation;

pub trait PdfObject {
    fn data(&self) -> String;

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any; // Downcast to Any for type checking

    fn metadata(&self) -> &crate::objects::metadata::PdfMetadata;

    fn metadata_mut(&mut self) -> &mut crate::objects::metadata::PdfMetadata;

    fn indirect(&self) -> String {
        let meta = self.metadata();
        let number = meta.object_identifier.unwrap_or(0);
        format!(
            "{} {} obj\n{}\nendobj",
            number,
            meta.generation_number,
            self.data()
        )
    }

    fn reference(&self) -> String {
        let meta = self.metadata();
        format!(
            "{} {} R",
            meta.object_identifier.unwrap_or(0),
            meta.generation_number
        )
    }

    /// Whether the object can be included in an object stream (PDF 1.5+).
    ///
    /// PDF spec: Only generation 0 objects can be compressed in object streams.
    /// Objects with generation > 0 (incremental updates) must be written directly.
    ///
    /// Note: Some object types (like Stream) override this to always return false.
    fn is_compressible(&self) -> bool {
        self.metadata().generation_number == Generation::Normal
    }
}
