/*#[test]
fn test_xref_stream_basic_structure() {
    // Test that xref stream has required entries per PDF spec Table 3.15
    use rusty_pdf::{PdfFile};

    let mut pdf = PdfFile::new();
    let mut output = Vec::new();

    pdf.write_compressed(&mut output, FileIdentifierMode::None).unwrap();

    let pdf_str = String::from_utf8_lossy(&output);

    assert!(pdf_str.contains("/Type /XRef"), "XRef stream must have /Type /XRef");

    assert!(pdf_str.contains("/Size"), "XRef stream must have /Size entry");

    // Must have /W entry (array of field widths)
    assert!(pdf_str.contains("/W ["), "XRef stream must have /W array");

    // Must have /Root entry pointing to catalog
    assert!(pdf_str.contains("/Root"), "XRef stream must have /Root entry");
}
*/
/*#[test]
fn test_xref_entries_for_minimal_pdf() {
    // PDF Reference 1.7, Section 3.4.7:
    // Entry format: type field2 field3
    // Type 0: free entry (field2=next free, field3=generation)
    // Type 1: uncompressed (field2=byte offset, field3=generation)
    // Type 2: compressed (field2=object stream number, field3=index)

    use rusty_pdf::{FileIdentifierMode, PdfFile};

    let mut pdf = PdfFile::new();
    let mut output = Vec::new();

    pdf.write_compressed(&mut output, FileIdentifierMode::None).unwrap();

    // Extract xref stream data
    let pdf_bytes = &output[..];
    let xref_marker = b"/Type /XRef";
    let xref_start = pdf_bytes.windows(xref_marker.len())
        .position(|window| window == xref_marker)
        .expect("No /Type /XRef found");

    let stream_start = output[xref_start..].iter().position(|&b| b == b's')
        .and_then(|pos| {
            if output[xref_start + pos..].starts_with(b"stream\n") {
                Some(xref_start + pos + 7)
            } else {
                None
            }
        })
        .expect("No stream keyword");

    let stream_end = output[stream_start..].iter().position(|&b| b == b'\n')
        .and_then(|pos| {
            if output[stream_start + pos..].starts_with(b"\nendstream") {
                Some(stream_start + pos)
            } else {
                None
            }
        })
        .expect("No endstream keyword");

    let xref_data = &output[stream_start..stream_end];

    // W array should be [1, 2, 2] for most PDFs (type=1 byte, field2=2 bytes, field3=2 bytes)
    let bytes_per_entry = 5; // 1 + 2 + 2
    let num_entries = xref_data.len() / bytes_per_entry;

    println!("\n=== XRef Stream Analysis ===");
    println!("XRef data length: {} bytes", xref_data.len());
    println!("Bytes per entry: {}", bytes_per_entry);
    println!("Number of entries: {}", num_entries);

    // PDF spec: "The value of the Size entry shall be 1 greater than the highest
    // object number used in the file"
    // So if we have objects 0-N, Size should be N+1, and we need N+1 xref entries

    for i in 0..num_entries {
        let entry = &xref_data[i * bytes_per_entry..(i + 1) * bytes_per_entry];
        let typ = entry[0];
        let field2 = (entry[1] as usize) << 8 | entry[2] as usize;
        let field3 = (entry[3] as u16) << 8 | entry[4] as u16;

        println!("\nEntry {}: type={}, field2={}, field3={}", i, typ, field2, field3);

        match typ {
            0 => println!("  -> Free entry: next_free={}, generation={}", field2, field3),
            1 => println!("  -> Uncompressed: offset={}, generation={}", field2, field3),
            2 => println!("  -> Compressed: objstm_num={}, index={}", field2, field3),
            _ => panic!("Invalid xref entry type: {}", typ),
        }
    }

    // Validate: xref_entries[i] should describe object i
    // Check by reading the actual PDF to see what object numbers exist
    let pdf_str = String::from_utf8_lossy(&output);
    println!("\n=== Objects in PDF ===");

    for line in pdf_str.lines() {
        if line.ends_with(" obj") && line.chars().nth(0).map(|c| c.is_numeric()).unwrap_or(false) {
            println!("{}", line);
        }
    }
}
*/
/*#[test]
fn test_object_zero_handling() {
    // PDF Reference 1.7, Section 3.4.3:
    // "Object number 0 shall always be free and shall have a generation
    // number of 65,535; it is the head of the linked list of free objects."

    use rusty_pdf::{FileIdentifierMode, PdfFile};

    let mut pdf = PdfFile::new();
    let mut output = Vec::new();

    pdf.write_compressed(&mut output, FileIdentifierMode::None).unwrap();

    // Extract first xref entry (for object 0)
    let pdf_bytes = &output[..];
    let xref_marker = b"/Type /XRef";
    let xref_start = pdf_bytes.windows(xref_marker.len())
        .position(|window| window == xref_marker)
        .expect("No /Type /XRef found");

    let stream_start = output[xref_start..].iter().position(|&b| b == b's')
        .and_then(|pos| {
            if output[xref_start + pos..].starts_with(b"stream\n") {
                Some(xref_start + pos + 7)
            } else {
                None
            }
        })
        .expect("No stream keyword");

    let stream_end = output[stream_start..].iter().position(|&b| b == b'\n')
        .and_then(|pos| {
            if output[stream_start + pos..].starts_with(b"\nendstream") {
                Some(stream_start + pos)
            } else {
                None
            }
        })
        .expect("No endstream keyword");

    let xref_data = &output[stream_start..stream_end];

    // First entry (object 0)
    let entry0 = &xref_data[0..5];
    let typ = entry0[0];
    let field2 = (entry0[1] as usize) << 8 | entry0[2] as usize;
    let field3 = (entry0[3] as u16) << 8 | entry0[4] as u16;

    println!("\nObject 0 xref entry:");
    println!("  Type: {}", typ);
    println!("  Field2: {}", field2);
    println!("  Field3: {}", field3);

    // Per PDF spec, object 0 must be free with generation 65535
    assert_eq!(typ, 0, "Object 0 must be free (type 0)");
    assert_eq!(field3, Generation::Root.as_u16(), "Object 0 generation must be 65535");

    // Check that no "0 0 obj" exists in the PDF
    let pdf_str = String::from_utf8_lossy(&output);
    assert!(!pdf_str.contains("0 0 obj"),
        "PDF should not contain '0 0 obj' - object 0 must be free per PDF spec");
}
*/
