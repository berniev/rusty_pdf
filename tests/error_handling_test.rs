use pydyf::{Stream, PdfError};

#[test]
fn test_invalid_color_values() {
    let mut stream = Stream::new();

    let result = stream.set_color_rgb(1.5, 0.5, 0.5, false);
    assert!(result.is_err());
    match result {
        Err(PdfError::InvalidRGB { r, g: _, b: _ }) => {
            assert_eq!(r, 1.5);
        }
        _ => panic!("Expected InvalidColor error"),
    }

    let result = stream.set_color_rgb(-0.1, 0.5, 0.5, false);
    assert!(result.is_err());
}

#[test]
fn test_invalid_cmyk_values() {
    let mut stream = Stream::new();

    let result = stream.set_color_cmyk(1.5, 0.0, 0.0, 0.0, false);
    assert!(result.is_err());

    let result = stream.set_color_cmyk(0.0, -0.1, 0.0, 0.0, false);
    assert!(result.is_err());
}

#[test]
fn test_invalid_gray_values() {
    let mut stream = Stream::new();

    let result = stream.set_color_gray(1.5, false);
    assert!(result.is_err());

    let result = stream.set_color_gray(-0.1, false);
    assert!(result.is_err());
}

#[test]
fn test_invalid_image_dimensions() {
    let mut stream = Stream::new();
    let data = vec![255, 0, 0];

    let result = stream.inline_image(0, 100, "RGB", 8, &data);
    assert!(result.is_err());

    let result = stream.inline_image(100, 0, "RGB", 8, &data);
    assert!(result.is_err());
}

#[test]
fn test_invalid_color_space() {
    let mut stream = Stream::new();
    let data = vec![255, 0, 0];

    let result = stream.inline_image(1, 1, "INVALID", 8, &data);
    assert!(result.is_err());
    match result {
        Err(PdfError::InvalidImage(msg)) => {
            assert!(msg.contains("Invalid color space"));
        }
        _ => panic!("Expected InvalidImage error"),
    }
}
