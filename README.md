# PyDyf - PDF Library for Rust

A low-level PDF generation library ported from python at [pydyf](https://github.com/CourtBouillon/pydyf).

## Features

- **Graphics**: Rectangles, lines, curves, paths, and shapes
- **Text**: Multiple fonts, positioning, transformations
- **Colors**: RGB, CMYK, and grayscale color spaces
- **Images**: Inline images and external image file loading (PNG, JPEG)
- **Compression**: Optional stream compression with flate
- **Error Handling**: Comprehensive validation with custom error types

## Quick Start

```rust
use pydyf::{PDF, Stream, Page, PageSize};
use std::fs::File;

fn main() {
    // Create a new PDF document
    let mut pdf = PDF::new(PageSize::A4);

    // Create a content stream
    let mut stream = Stream::new();

    // Draw a red rectangle
    stream.set_color_rgb(1.0, 0.0, 0.0, false).unwrap();
    stream.rectangle(100.0, 100.0, 200.0, 150.0);
    stream.fill(false);

    // Add text
    stream.begin_text();
    stream.set_font_size("Helvetica", 24.0);
    stream.set_text_matrix(1.0, 0.0, 0.0, 1.0, 100.0, 300.0);
    stream.show_text_string("Hello, PDF!");
    stream.end_text();

    // Add stream to PDF
    pdf.add_object(Box::new(stream));

    // Create page
    let content_ref = format!("{} 0 R", pdf.objects.len() - 1).into_bytes();
    let mut page = Page::new();
    page.set_contents(content_ref);

    pdf.add_page(page);

    // Write to file
    let mut file = File::create("output.pdf").unwrap();
    pdf.write(&mut file, Some(b"1.7"), pydyf::Identifier::AutoMD5, false).unwrap();
}
```

## Color Spaces

### RGB Colors

```rust
// RGB values from 0.0 to 1.0
stream.set_color_rgb(1.0, 0.0, 0.0, false).unwrap(); // Red fill
stream.set_color_rgb(0.0, 1.0, 0.0, true).unwrap();  // Green stroke
```

### CMYK Colors

```rust
// CMYK values from 0.0 to 1.0
stream.set_color_cmyk(1.0, 0.0, 0.0, 0.0, false).unwrap(); // Cyan
stream.set_color_cmyk(0.0, 1.0, 0.0, 0.0, false).unwrap(); // Magenta
stream.set_color_cmyk(0.0, 0.0, 1.0, 0.0, false).unwrap(); // Yellow
stream.set_color_cmyk(0.0, 0.0, 0.0, 1.0, false).unwrap(); // Black (key)
```

### Grayscale

```rust
// Gray value from 0.0 (black) to 1.0 (white)
stream.set_color_gray(0.0, false).unwrap();  // Black
stream.set_color_gray(0.5, false).unwrap();  // 50% gray
stream.set_color_gray(1.0, false).unwrap();  // White
```

## Images

### Load from File

```rust
// Load PNG or JPEG image from file
stream.push_state();
stream.set_matrix(200.0, 0.0, 0.0, 200.0, 50.0, 500.0); // Scale to 200x200 at position (50, 500)
stream.inline_image_from_file("image.png").unwrap();
stream.pop_state();
```

### Inline Image Data

```rust
// Raw RGB pixel data
let image_data = vec![
    255, 0, 0,    // Red pixel
    0, 255, 0,    // Green pixel
    0, 0, 255,    // Blue pixel
    255, 255, 0,  // Yellow pixel
];

stream.push_state();
stream.set_matrix(100.0, 0.0, 0.0, 100.0, 50.0, 500.0);
stream.inline_image(2, 2, "RGB", 8, &image_data).unwrap();
stream.pop_state();
```

## Graphics

### Rectangles

```rust
stream.rectangle(x, y, width, height);
stream.fill(false); // false = non-zero winding rule
```

### Lines and Paths

```rust
stream.move_to(100.0, 100.0);
stream.line_to(200.0, 200.0);
stream.stroke();
```

### Curves

```rust
stream.move_to(100.0, 100.0);
stream.curve_to(150.0, 200.0, 200.0, 200.0, 250.0, 100.0);
stream.stroke();
```

## Text

### Basic Text

```rust
stream.begin_text();
stream.set_font_size("Helvetica", 18.0);
stream.set_text_matrix(1.0, 0.0, 0.0, 1.0, 100.0, 700.0); // Position at (100, 700)
stream.show_text_string("Hello, World!");
stream.end_text();
```

### Available Standard Fonts

- `Helvetica`
- `Helvetica-Bold`
- `Courier`
- And other PDF standard fonts

## Compression

Enable stream compression with the third parameter:

```rust
let stream = Stream::new(None, None, true); // true = enable flate compression
```

## Error Handling

Most operations return `Result<()>` for validation:

```rust
// Color validation (must be 0.0-1.0)
match stream.set_color_rgb(1.5, 0.0, 0.0, false) {
    Ok(_) => println!("Success"),
    Err(e) => println!("Error: {}", e),
}

// Image validation
match stream.inline_image(0, 100, "RGB", 8, &data) {
    Ok(_) => println!("Success"),
    Err(e) => println!("Error: Invalid dimensions"),
}
```

## Page Sizes

Common page sizes (in points, 1 point = 1/72 inch):

- **Letter**: `[0 0 612 792]` (8.5" x 11")
- **A4**: `[0 0 595 842]` (210mm x 297mm)
- **Legal**: `[0 0 612 1008]` (8.5" x 14")

## Building Documentation

```bash
cargo doc --no-deps --open
```

## Running Tests

```bash
# Run all tests
cargo test

# Run specific test suite
cargo test --test visual_pdf_test
cargo test --test error_handling_test
cargo test --test color_spaces_test
```

## License

Ported from Python's pydyf library.
