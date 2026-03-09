// Main entry point for pydyf
// This is a simple example/test program

use pydyf::{Page, PDF};
use pydyf::page_size::PageSize;

fn main() {
    println!("PyDyf - PDF library for Rust");
    println!("Ported from Python pydyf library");

    // Example usage:
    let mut pdf = PDF::new();
    println!("Created new PDF with {} objects", pdf.objects.len());
    
    let mut page = Page::new(PageSize::A4);
    let contents: &[u8] = b"Hello, World!";
    page.set_contents(contents.to_vec());
    pdf.add_page(page);
    println!("Added page to PDF with {} objects", pdf.objects.len());
}
