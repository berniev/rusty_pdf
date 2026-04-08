//! PDF validation tests using external tools
//! Requires qpdf to be installed. Tests skip gracefully if not available.

use std::fs;
use std::path::Path;
use std::process::Command;

/// Helper function to check if a PDF validation tool is available
fn check_validator(tool: &str) -> bool {
    Command::new(tool)
        .arg("--version")
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

/// Helper function to validate a PDF file with a tool
fn validate_pdf_file(path: &str, tool: &str, args: &[&str]) -> Result<(), String> {
    let output = Command::new(tool)
        .args(args)
        .arg(path)
        .output()
        .map_err(|e| format!("Failed to run {}: {}", tool, e))?;

    if !output.status.success() {
        return Err(format!(
            "{} validation failed:\nstdout: {}\nstderr: {}",
            tool,
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    Ok(())
}

#[test]
fn validate_output_pdf() {
    if !check_validator("qpdf") {
        eprintln!("Skipping validation - qpdf not installed");
        eprintln!("  macOS: brew install qpdf");
        eprintln!("  Linux: apt install qpdf / dnf install qpdf");
        eprintln!("  Windows: choco install qpdf");
        return;
    }

    validate_pdf_file("output.pdf", "qpdf", &["--check"]).expect("output.pdf should be valid");
}

#[test]
fn validate_all_generated_pdfs() {
    if !check_validator("qpdf") {
        eprintln!("Skipping validation - qpdf not installed");
        eprintln!("  macOS: brew install qpdf");
        eprintln!("  Linux: apt install qpdf / dnf install qpdf");
        eprintln!("  Windows: choco install qpdf");
        return;
    }

    let test_dir = Path::new("/tmp/pydyf_test");
    if !test_dir.exists() {
        eprintln!(
            "Test directory /tmp/pydyf_test does not exist. Run other tests first to generate PDFs."
        );
        return;
    }

    let mut pdf_files = Vec::new();

    // PDFs that are intentionally broken/invalid for testing
    let skip_files = ["break_no_pages.pdf"];

    // Collect all PDF files
    if let Ok(entries) = fs::read_dir(test_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("pdf") {
                // Skip intentionally broken PDFs
                if let Some(filename) = path.file_name().and_then(|n| n.to_str())
                    && skip_files.contains(&filename)
                {
                    println!("⊘ {} (intentionally invalid, skipped)", filename);
                    continue;
                }
                pdf_files.push(path);
            }
        }
    }

    if pdf_files.is_empty() {
        eprintln!("No PDF files found in /tmp/pydyf_test. Run other tests first.");
        return;
    }

    println!("Found {} PDF files to validate", pdf_files.len());

    let mut passed = 0;
    let mut failed = 0;

    for pdf_path in pdf_files {
        let path_str = pdf_path.to_string_lossy();
        match validate_pdf_file(&path_str, "qpdf", &["--check"]) {
            Ok(_) => {
                println!("✓ {}", pdf_path.file_name().unwrap().to_string_lossy());
                passed += 1;
            }
            Err(e) => {
                eprintln!(
                    "✗ {}: {}",
                    pdf_path.file_name().unwrap().to_string_lossy(),
                    e
                );
                failed += 1;
            }
        }
    }

    println!("\nValidation results: {} passed, {} failed", passed, failed);
    assert_eq!(failed, 0, "{} PDF(s) failed validation", failed);
}

#[test]
fn validate_with_multiple_tools() {
    let tools = vec![("qpdf", vec!["--check"]), ("pdfinfo", vec![])];

    for (tool, args) in tools {
        if check_validator(tool) {
            let args_refs: Vec<&str> = args.iter().map(|s| s.as_ref()).collect();
            match validate_pdf_file("output.pdf", tool, &args_refs) {
                Ok(_) => println!("✓ Validated with {}", tool),
                Err(e) => eprintln!("✗ {} validation: {}", tool, e),
            }
        } else {
            eprintln!("⊘ {} not installed", tool);
        }
    }
}
