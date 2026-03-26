pub trait PdfObject where Self: 'static {
    fn data(&mut self) -> Vec<u8>;
    
    fn boxed(self) -> Box<dyn PdfObject> where Self: Sized {
        Box::new(self)
    }
}
