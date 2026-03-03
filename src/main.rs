#![allow(dead_code)]

// Main entry point for pydyf
// This is a simple example/test program

use pydyf::PDF;
use pydyf::PageSize;

fn main() {
    println!("PyDyf - PDF library for Rust");
    println!("Ported from Python pydyf library");

    // Example usage:
    let pdf = PDF::new(PageSize::A4);
    println!("Created new PDF with {} objects", pdf.objects.len());
}
