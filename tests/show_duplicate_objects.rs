/// Show exactly what each duplicate object contains

#[test]
fn show_what_duplicate_object_5_contains() {
    use pydyf::{FileIdentifierMode, PageObject, StreamObject, PDF};
    use pydyf::page::PageSize;

    let mut pdf = PDF::new();
    let stream = StreamObject::new();
    pdf.add_object(Box::new(stream));

    let next_num = pdf.objects.len() - 1;
    let mut page = PageObject::new(next_num.into());
    page.set_media_box(PageSize::A4);
    pdf.add_page(page);

    let mut output = Vec::new();
    pdf.write_compressed(&mut output, FileIdentifierMode::None).unwrap();

    // Parse and show ALL occurrences of each object
    let pdf_bytes = output.as_slice();

    use std::collections::HashMap;
    let mut objects: HashMap<usize, Vec<String>> = HashMap::new();

    let pdf_str = String::from_utf8_lossy(pdf_bytes);
    let lines: Vec<&str> = pdf_str.lines().collect();

    let mut i = 0;
    while i < lines.len() {
        let line = lines[i];
        if line.ends_with(" obj") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 3 && parts[1] == "0" && parts[2] == "obj" {
                if let Ok(obj_num) = parts[0].parse::<usize>() {
                    // Collect content until endobj
                    let mut content = String::new();
                    i += 1;
                    while i < lines.len() && !lines[i].contains("endobj") {
                        content.push_str(lines[i]);
                        content.push('\n');
                        i += 1;
                    }
                    objects.entry(obj_num).or_insert_with(Vec::new).push(content);
                }
            }
        }
        i += 1;
    }

    // Show duplicates
    println!("\n=== All objects and their contents ===\n");
    for (obj_num, contents) in objects.iter() {
        println!("Object {} appears {} time(s):", obj_num, contents.len());
        for (occurrence, content) in contents.iter().enumerate() {
            let preview = if content.len() > 100 {
                format!("{}...", &content[..100])
            } else {
                content.clone()
            };
            println!("  Occurrence #{}: {}", occurrence + 1, preview.trim());
        }
        println!();
    }

    // Specifically check for duplicates
    let duplicates: Vec<_> = objects.iter()
        .filter(|(_, contents)| contents.len() > 1)
        .map(|(num, _)| num)
        .collect();

    if !duplicates.is_empty() {
        println!("DUPLICATES FOUND: {:?}", duplicates);
        for obj_num in &duplicates {
            println!("\n=== Full content of duplicate object {} ===", obj_num);
            for (i, content) in objects[obj_num].iter().enumerate() {
                println!("\n--- Occurrence #{} ---", i + 1);
                println!("{}", content);
            }
        }
    }

    assert!(duplicates.is_empty(), "Found duplicate objects: {:?}", duplicates);
}
