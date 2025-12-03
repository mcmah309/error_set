use std::fs;
use std::path::{Path, PathBuf};

use ignore::WalkBuilder;

/// Combines all uses of the `error_set_part!` macro in `src/` (respecting `.gitignore`) into one
/// `error_set!` macro at the generated file `src/error_set.rs`
pub fn combine_error_set_parts() {
    let mut parts = String::with_capacity(1024);
    for file_path in find_rust_files("src") {
        if let Ok(content) = fs::read_to_string(&file_path) {
            extract_error_set_parts(&content, &mut parts, &file_path);
        }
    }
    if parts.is_empty() {
        return;
    }
    let output_path = "src/error_set.rs";
    if let Err(e) = fs::write(
        output_path,
        format!(
            "// This file is auto-generated\n\nerror_set::error_set! {{\n{parts}\n}}"
        ),
    ) {
        panic!("Failed to write to {}: {}", output_path, e);
    }
    // if let Err(e) = std::process::Command::new("rustfmt")
    //     .arg(output_path)
    //     .status()
    // {
    //     println!("cargo:warning=Failed to format {:?}: {}", output_path, e);
    // }
}

#[inline]
fn find_rust_files<P: AsRef<Path>>(dir: P) -> impl Iterator<Item = PathBuf> {
    WalkBuilder::new(dir)
        .build()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().and_then(|s| s.to_str()) == Some("rs"))
        .map(|e| e.path().to_path_buf())
}

#[inline]
fn extract_error_set_parts(content: &str, error_set_body: &mut String, current_file: &Path) {
    let needle = "error_set_part!";
    let mut pos = 0;
    
    while let Some(relative_idx) = content[pos..].find(needle) {
        let statement_start = pos + relative_idx;
        let mut brace_pos = statement_start + needle.len();
        
        // Skip whitespace
        let skip = content[brace_pos..].chars()
            .take_while(|c| c.is_whitespace())
            .map(|c| c.len_utf8())
            .sum::<usize>();
        brace_pos += skip;
        
        if brace_pos < content.len() {
            let end = extract_balanced_braces(content, brace_pos, error_set_body, current_file);
            pos = end;
        } else {
            break;
        }
    }
}

#[inline]
fn extract_balanced_braces(content: &str, brace_start_pos: usize, error_set_body: &mut String, current_file: &Path) -> usize {
    let mut chars = content[brace_start_pos..].chars();

    let next = chars.next();
    if let Some(next) = next {
        if next != '{' {
            panic!("Expected '{{' after error_set_part! macro in {}", current_file.display());
        }
    } else {
        panic!("Unexpected end of input after error_set_part! macro in {}", current_file.display());
    }

    let mut depth = 1;
    let mut end = brace_start_pos + 1;

    for (i, ch) in chars.enumerate() {
        match ch {
            '{' => depth += 1,
            '}' => {
                depth -= 1;
                if depth == 0 {
                    end = brace_start_pos + i + 2; // +1 for opening brace, +1 for current closing brace
                    break;
                }
            }
            _ => {}
        }
    }

    if depth == 0 {
        error_set_body.push_str(&format!("\t// From `{}::{brace_start_pos}`\n", current_file.display()));
        error_set_body.push_str(&content[brace_start_pos + 1..end - 1].trim_start_matches('\n'));
        error_set_body.push_str("\n");
        end
    } else {
        panic!("Unmatched braces in error_set_part! macro in {}", current_file.display());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_error_set_parts() {
        let code = r#"
            error_set_part! {
                MyError1,
                MyError2,
            }
            
            fn some_function() {}
            
            error_set_part! { AnotherError }
        "#;
        let mut parts = String::new();
        extract_error_set_parts(code, &mut parts, &PathBuf::from("test"));
        assert!(parts.contains("MyError1"));
        assert!(parts.contains("AnotherError"));
    }
}
