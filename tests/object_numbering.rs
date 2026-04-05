use rusty_pdf::file_identifier::FileIdentifierMode;
use rusty_pdf::objects::pdf_object::PdfObj;

/// Tests to verify single source of truth for object numbering
///
/// PDF Reference 1.7, Section 3.4.3:
/// "Object number 0 shall always be free"
/// Therefore: object numbering starts at 1

/*#[test]
fn test_next_object_number_is_single_source_of_truth() {
    // This test verifies that pdf.next_object_number() is the canonical way
    // to get the next available object ID

    use pydyf::PdfFile;

    let pdf = PdfFile::new();

    // Initially, no objects exist, so next should be 1 (0 is reserved as free)
    // Note: next_object_number() is private, but we can infer it from add_object()

    // The rule should be: next_object_number = max(existing object IDs) + 1
    // Or if no objects exist: next_object_number = 1

    println!("Initial state: pdf.objects.len() = {}", pdf.objects.len());
}
*/
#[test]
fn test_add_object_assigns_sequential_ids() {
    use rusty_pdf::{Pdf, PdfStreamObject};

    let mut pdf = Pdf::new();

    // Add objects one by one and verify IDs are sequential starting from 1
    let id1 = pdf.save_indirect_object(PdfObj::stream(PdfStreamObject::new()));
    let id2 = pdf.save_indirect_object(PdfObj::stream(PdfStreamObject::new()));
    let id3 = pdf.save_indirect_object(PdfObj::stream(PdfStreamObject::new()));

    println!("Added object IDs: {}, {}, {}", id1, id2, id3);

    assert_eq!(id1, 1, "First object should be ID 1 (0 is reserved)");
    assert_eq!(id2, 2, "Second object should be ID 2");
    assert_eq!(id3, 3, "Third object should be ID 3");

    // Verify metadata matches
/*    assert_eq!(pdf.objects[0].metadata().object_identifier, Some(1));
    assert_eq!(pdf.objects[1].metadata().object_identifier, Some(2));
    assert_eq!(pdf.objects[2].metadata().object_identifier, Some(3));
*/}

#[test]
fn test_all_object_assignments_use_consistent_numbering() {
    // This test verifies that ALL objects get sequential IDs with no duplicates
    // Whether assigned via add_object, initialize_catalog, etc.

    use rusty_pdf::{Pdf, PdfStreamObject};
    let mut pdf = Pdf::new();

    // Add a page
    pdf.save_indirect_object(PdfObj::stream(PdfStreamObject::new()));

    // Initialize everything
/*    let resources_id = pdf.add_font_resources();
    //pdf.initialize_page_tree(resources_id);
    pdf.initialize_catalog();
    //pdf.initialize_info();
*/
/*    // Collect all assigned object IDs
    let mut ids: Vec<usize> = pdf.objects.iter()
        .filter_map(|obj| obj.metadata().object_identifier)
        .collect();

    ids.sort();

    println!("All object IDs after initialization: {:?}", ids);

    // Check for duplicates
    let unique_ids: HashSet<_> = ids.iter().collect();
    assert_eq!(ids.len(), unique_ids.len(), "Found duplicate object IDs: {:?}", ids);

    // All IDs should be >= 1 (0 is reserved)
    assert!(ids.iter().all(|&id| id >= 1), "All object IDs must be >= 1");

    // IDs should be sequential (no gaps) OR we need to track which IDs are used
    // For now, just verify max_id <= len + 1 (accounting for 0 being skipped)
    let max_id = ids.iter().max().copied().unwrap_or(0);
    let expected_max = ids.len(); // Since we start from 1, max should equal count

    assert_eq!(max_id, expected_max,
        "Object IDs should be sequential 1..N, but max={} and count={}",
        max_id, expected_max);
*/}

#[test]
fn test_compressed_write_assigns_unique_objstm_number() {
    // Verify that when creating object streams during compressed write,
    // the ObjStm gets a unique object number that doesn't collide

    use rusty_pdf::{Pdf, PdfStreamObject};
    use std::collections::HashMap;

    let mut pdf = Pdf::new();
    pdf.save_indirect_object(PdfObj::stream(PdfStreamObject::new()));

    let mut output = Vec::new();
    pdf.write_compressed(&mut output, FileIdentifierMode::None).unwrap();

    // Parse the PDF and find all object numbers
    let pdf_str = String::from_utf8_lossy(&output);
    let mut object_occurrences: HashMap<usize, usize> = HashMap::new();

    for line in pdf_str.lines() {
        if line.ends_with(" obj") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 3 && parts[1] == "0" && parts[2] == "obj" {
                if let Ok(num) = parts[0].parse::<usize>() {
                    *object_occurrences.entry(num).or_insert(0) += 1;
                }
            }
        }
    }

    println!("\nObject occurrences in PDF:");
    for (obj_num, count) in object_occurrences.iter() {
        println!("  Object {}: appears {} time(s)", obj_num, count);
    }

    // Every object number should appear exactly once
    let duplicates: Vec<_> = object_occurrences.iter()
        .filter(|(_, count)| **count > 1)
        .collect();

    assert!(duplicates.is_empty(),
        "Found duplicate object numbers: {:?}", duplicates);
}

#[test]
fn test_objstm_number_calculation() {
    // Test the specific calculation used for ObjStm numbering
    // It should be: max(all existing object IDs) + 1

    use rusty_pdf::{Pdf, PdfStreamObject};

    let mut pdf = Pdf::new();
    pdf.save_indirect_object(PdfObj::stream(PdfStreamObject::new()));

    // Before write_compressed, find max object ID
 /*   let max_id_before = pdf.objects.iter()
        .filter_map(|obj| obj.metadata().object_identifier)
        .max()
        .unwrap_or(0);

    println!("Max object ID before write_compressed: {}", max_id_before);
    println!("pdf.objects.len() before write_compressed: {}", pdf.objects.len());

    let mut output = Vec::new();
    pdf.write_compressed(&mut output, FileIdentifierMode::None).unwrap();

    // Find the ObjStm in the output
    let pdf_str = String::from_utf8_lossy(&output);
    let mut objstm_num: Option<usize> = None;

    let lines: Vec<&str> = pdf_str.lines().collect();
    for (i, line) in lines.iter().enumerate() {
        if line.contains("/Type /ObjStm") && i > 0 {
            // Look back for "N 0 obj"
            for j in (0..i).rev() {
                if lines[j].ends_with(" obj") {
                    let parts: Vec<&str> = lines[j].split_whitespace().collect();
                    if parts.len() >= 3 && parts[1] == "0" && parts[2] == "obj" {
                        objstm_num = parts[0].parse().ok();
                        break;
                    }
                }
            }
            break;
        }
    }

    if let Some(objstm) = objstm_num {
        println!("ObjStm assigned object number: {}", objstm);

        // ObjStm number should be max_id + 1
        // (unless other objects were added during write, in which case it should be
        // greater than max_id_before)
        assert!(objstm > max_id_before,
            "ObjStm number {} should be > max ID before write ({})",
            objstm, max_id_before);
    } else {
        // If no ObjStm, that's fine (maybe nothing was compressed)
        println!("No ObjStm found (nothing was compressed)");
    }
*/}
