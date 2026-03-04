use crate::objects::base::PdfMetadata;

pub trait PdfObject {
    fn metadata(&self) -> &PdfMetadata;
    fn metadata_mut(&mut self) -> &mut PdfMetadata;
    fn data(&self) -> Vec<u8>;
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any; // Downcast to Any for type checking

    fn indirect(&self) -> Vec<u8> {
        let meta = self.metadata();
        let number = meta.number.unwrap_or(0);
        let header = format!("{} {} obj\n", number, meta.generation);
        let mut result = header.into_bytes();
        result.extend(self.data());
        result.extend(b"\nendobj");
        result
    }

    fn reference(&self) -> Vec<u8> {
        let meta = self.metadata();
        let number = meta.number.unwrap_or(0);
        format!("{} {} R", number, meta.generation).into_bytes()
    }

    /// Whether the object can be included in an object stream (PDF 1.5+).
    ///
    /// PDF spec: Only generation 0 objects can be compressed in object streams.
    /// Objects with generation > 0 (incremental updates) must be written directly.
    ///
    /// Note: Some object types (like Stream) override this to always return false.
    fn is_compressible(&self) -> bool {
        self.metadata().generation == 0
    }
}
