use RustyPDF::cross_reference_table::CrossRefError;
use RustyPDF::generation::Generation;

/*#[test]
fn test_new_table_has_root_entry() {
    let table = CrossRefTable::new();
    let output = table.as_pdf().unwrap();

    // Should start with xref header
    assert!(output.starts_with("xref\r\n"));

    // Should have subsection starting at 0 with 1 entry
    assert!(output.contains("0 1\r\n"));

    // Should have root entry: 0000000000 65535 f
    let expected = format!("0000000000 {} f", Generation::Root.as_u16());
    assert!(output.contains(&expected));
}
*/
/*#[test]
fn test_entry_formatting() {
    let entry = CrossReferenceEntry::new(1, ObjectStatus::InUse, 12345, Generation::Normal);
    let formatted = entry.serialise();

    // Check format: 10-digit offset, 5-digit generation, status, CRLF
    assert_eq!(formatted, "0000012345 00000 n \r\n");
}
*/
/*#[test]
fn test_root_entry_formatting() {
    let entry = CrossReferenceEntry::new(0, ObjectStatus::Free, 0, Generation::Root);
    let formatted = entry.serialise();

    // Root entry should have generation 65535
    let expected = format!("0000000000 {} f \r\n", Generation::Root.as_u16());
    assert_eq!(formatted, expected);
}
*/
/*#[test]
fn test_add_multiple_entries() {
    let mut table = CrossRefTable::new();

    table.add_entry(CrossReferenceEntry::new(1, ObjectStatus::InUse, 100, Generation::Normal));
    table.add_entry(CrossReferenceEntry::new(2, ObjectStatus::InUse, 200, Generation::Normal));
    table.add_entry(CrossReferenceEntry::new(3, ObjectStatus::InUse, 300, Generation::Normal));

    let output = table.as_pdf().unwrap();

    // Should have 4 entries (root + 3 added)
    assert!(output.contains("0 4\r\n"));

    // Verify all entries present
    let expected = format!("0000000000 {} f", Generation::Root.as_u16());
    assert!(output.contains(&expected));
    assert!(output.contains("0000000100 00000 n"));
    assert!(output.contains("0000000200 00000 n"));
    assert!(output.contains("0000000300 00000 n"));
}
*/
#[test]
fn test_generation_enum() {
    assert_eq!(Generation::Root.as_u16(), Generation::ROOT_GENERATION);
    assert_eq!(Generation::Normal.as_u16(), 0);
}

#[test]
fn test_generation_equality() {
    assert_eq!(Generation::Root, Generation::Root);
    assert_eq!(Generation::Normal, Generation::Normal);
    assert_ne!(Generation::Root, Generation::Normal);
}

/*#[test]
fn test_large_offset_formatting() {
    let entry = CrossReferenceEntry::new(1, ObjectStatus::InUse, 9999999999, Generation::Normal);
    let formatted = entry.serialise();

    // Should handle 10-digit max value
    assert_eq!(formatted, "9999999999 00000 n \r\n");
}
*/
/*#[test]
fn test_free_entry_formatting() {
    // Free entry pointing to next free object at position 5
    let entry = CrossReferenceEntry::new(2, ObjectStatus::Free, 5, Generation::Normal);
    let formatted = entry.serialise();

    assert_eq!(formatted, "0000000005 00000 f \r\n");
}
*/
#[test]
fn test_cross_ref_error_types() {
    // Test that error enum has expected variants
    let err1 = CrossRefError::EmptyTable;
    let err2 = CrossRefError::InvalidRootEntry;

    assert_eq!(err1, CrossRefError::EmptyTable);
    assert_eq!(err2, CrossRefError::InvalidRootEntry);
    assert_ne!(err1, err2);
}

/*#[test]
fn test_pdf_spec_compliance() {
    let mut table = CrossRefTable::new();
    table.add_entry(CrossReferenceEntry::new(1, ObjectStatus::InUse, 18, Generation::Normal));
    table.add_entry(CrossReferenceEntry::new(2, ObjectStatus::InUse, 79, Generation::Normal));

    let output = table.as_pdf().unwrap();

    // Verify PDF spec format requirements:
    // 1. Starts with "xref"
    assert!(output.starts_with("xref\r\n"));

    // 2. Has subsection header with starting object number and count
    assert!(output.contains("0 3\r\n"));

    // 3. All lines end with CRLF
    for line in output.lines() {
        if !line.is_empty() {
            // Note: lines() strips line endings, so we check the raw bytes
        }
    }

    // Check raw bytes contain CRLF
    let bytes = output.as_bytes();
    let has_crlf = bytes.windows(2).any(|w| w == b"\r\n");
    assert!(has_crlf);
}
*/