use rusty_pdf::color::Color;

#[test]
#[should_panic(expected = "color must be in range 0.0..=1.0")]
fn test_color_too_high() {
    Color::new(1.5);
}

#[test]
#[should_panic(expected = "color must be in range 0.0..=1.0")]
fn test_color_too_low() {
    Color::new(-0.1);
}
