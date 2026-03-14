// Main entry point for pydyf
// This is a simple example/test program
// TODO: Update this to use the new API

// use std::rc::Rc;
// use pydyf::{PageObject, PDF};
// use pydyf::page::PageSize;

fn main() {
    println!("PyDyf - PDF library for Rust");
    println!("Ported from Python pydyf library");
    println!("TODO: Update main.rs to use the refactored API");

    // let mut pdf = PDF::new();
    // println!("Created new PDF with {} objects", pdf.objects.len());

    // TODO: PageObject API has changed - set_media_box now modifies in place
    // and with_contents method doesn't exist anymore
    // let page = PageObject::new(0.into());
    // page.set_media_box(PageSize::A4);
    // pdf.add_page(page);
    // println!("Added page to PDF with {} objects", pdf.objects.len());
}
