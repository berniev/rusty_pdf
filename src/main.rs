// Main entry point for pydyf
// This is a simple example/test program

use std::rc::Rc;
use pydyf::{Page, PDF};
use pydyf::page_size::PageSize;

fn main() {
    println!("PyDyf - PDF library for Rust");
    println!("Ported from Python pydyf library");

    let mut pdf = PDF::new();
    println!("Created new PDF with {} objects", pdf.objects.len());
    
    let contents: &[u8] = b"Hello, World!";
    let page = Page::new(PageSize::A4).with_contents(Some(Rc::new(contents.to_vec())));
    pdf.add_page(page);
    println!("Added page to PDF with {} objects", pdf.objects.len());
}
