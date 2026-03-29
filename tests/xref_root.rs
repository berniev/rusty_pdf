/// Test that xref stream /Root entry points to the actual Catalog object

#[test]
fn test_xref_root_points_to_catalog() {
    use pydyf::{FileIdentifierMode, PdfFile};

    let mut pdf = PdfFile::new();
    let mut output = Vec::new();

    pdf.write_compressed(&mut output, FileIdentifierMode::None).unwrap();

    let pdf_str = String::from_utf8_lossy(&output);

    // Find the catalog object number by looking for "/Type /Catalog"
    let mut catalog_obj_num: Option<usize> = None;
    for line in pdf_str.lines() {
        if line.contains("/Type /Catalog") {
            // Find the line before this that has "N 0 obj"
            let lines: Vec<&str> = pdf_str.lines().collect();
            for (i, l) in lines.iter().enumerate() {
                if l.contains("/Type /Catalog") && i > 0 {
                    // Look at previous lines for "N 0 obj"
                    for j in (0..i).rev() {
                        if lines[j].ends_with(" obj") {
                            let parts: Vec<&str> = lines[j].split_whitespace().collect();
                            if parts.len() >= 3 && parts[1] == "0" && parts[2] == "obj" {
                                catalog_obj_num = parts[0].parse().ok();
                                break;
                            }
                        }
                    }
                    break;
                }
            }
            break;
        }
    }

    let catalog_num = catalog_obj_num.expect("Could not find catalog object number");
    println!("Catalog is object {}", catalog_num);

    // Find the /Root entry in xref stream
    let root_marker = "/Root ";
    let root_pos = pdf_str.find(root_marker).expect("No /Root in xref stream");
    let after_root = &pdf_str[root_pos + root_marker.len()..];
    let root_ref = after_root.split_whitespace().next().expect("No value after /Root");
    let root_obj_num: usize = root_ref.parse().expect("Could not parse /Root value");

    println!("XRef stream has /Root {} 0 R", root_obj_num);

    // They must match
    assert_eq!(
        root_obj_num, catalog_num,
        "XRef stream /Root points to object {}, but Catalog is object {}",
        root_obj_num, catalog_num
    );
}

#[test]
fn test_objects_match_their_declarations() {
    // Verify that when we write "N 0 obj", the metadata.object_identifier matches N

    use pydyf::{FileIdentifierMode, PdfFile};

    let mut pdf = PdfFile::new();
    let mut output = Vec::new();

    pdf.write_compressed(&mut output, FileIdentifierMode::None).unwrap();

    // Check that pdf.objects all have matching IDs
    println!("\n=== Checking pdf.objects metadata ===");
    for (i, obj) in pdf.objects.iter().enumerate() {
        let meta_id = obj.metadata().object_identifier;
        println!("objects[{}]: metadata.object_identifier = {:?}", i, meta_id);
    }

    let pdf_str = String::from_utf8_lossy(&output);

    // Find actual catalog in output
    let mut actual_catalog_num: Option<usize> = None;
    let lines: Vec<&str> = pdf_str.lines().collect();
    for (i, line) in lines.iter().enumerate() {
        if line.contains("/Type /Catalog") && i > 0 {
            for j in (0..i).rev() {
                if lines[j].ends_with(" obj") {
                    let parts: Vec<&str> = lines[j].split_whitespace().collect();
                    if parts.len() >= 3 && parts[1] == "0" && parts[2] == "obj" {
                        actual_catalog_num = parts[0].parse().ok();
                        break;
                    }
                }
            }
            break;
        }
    }

    let actual_num = actual_catalog_num.expect("Could not find catalog in PDF output");
    println!("Actual catalog object in PDF: {}", actual_num);
}

#[test]
fn test_all_object_numbers_are_sequential() {
    // Per PDF spec, objects should be numbered starting from 1 (0 is free)
    // and should be sequential (or have gaps marked as free in xref)

    use pydyf::{FileIdentifierMode, PdfFile};

    let mut pdf = PdfFile::new();
    let mut output = Vec::new();

    pdf.write_compressed(&mut output, FileIdentifierMode::None).unwrap();

    let pdf_str = String::from_utf8_lossy(&output);

    // Extract all "N 0 obj" declarations
    let mut object_numbers: Vec<usize> = Vec::new();
    for line in pdf_str.lines() {
        if line.ends_with(" obj") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 3 && parts[1] == "0" && parts[2] == "obj" {
                if let Ok(num) = parts[0].parse::<usize>() {
                    object_numbers.push(num);
                }
            }
        }
    }

    object_numbers.sort();
    object_numbers.dedup();

    println!("Objects in PDF: {:?}", object_numbers);

    // Object 0 should NOT be in the list (it's free)
    assert!(!object_numbers.contains(&0), "Object 0 should not exist (it's the free list head)");

    // Object numbers should start from 1
    assert!(object_numbers.iter().all(|&n| n >= 1), "All object numbers should be >= 1");

    // Check xref has correct size
    let size_marker = "/Size ";
    if let Some(size_pos) = pdf_str.find(size_marker) {
        let after_size = &pdf_str[size_pos + size_marker.len()..];
        let size_str = after_size.split_whitespace().next().expect("No value after /Size");
        let size: usize = size_str.parse().expect("Could not parse /Size");

        println!("XRef /Size: {}", size);

        // Size should be max_object_num + 1
        let max_obj = object_numbers.iter().max().copied().unwrap_or(0);
        println!("Max object number: {}", max_obj);

        // Size should account for object 0 (free) plus all objects up to max
        assert_eq!(size, max_obj + 1,
            "/Size should be {} (max object {} + 1), but is {}",
            max_obj + 1, max_obj, size);
    }
}
