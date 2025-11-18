use anyhow::{Context, Result};
use std::collections::HashMap;
use std::env;
use std::path::Path;
use std::process;
use walkdir::WalkDir;

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}

fn run() -> Result<()> {
    // Parse command-line arguments
    let args: Vec<String> = env::args().collect();
    let directory = args.get(1).map(|s| s.as_str()).unwrap_or(".");

    // Validate directory path
    let path = Path::new(directory);
    if !path.exists() {
        eprintln!("Error: Directory '{}' does not exist.", directory);
        process::exit(1);
    }
    if !path.is_dir() {
        eprintln!("Error: '{}' is not a directory.", directory);
        process::exit(1);
    }

    // Scan directory and collect file type sizes
    let file_type_sizes = scan_directory(path)?;

    // Display results
    display_results(&file_type_sizes);

    Ok(())
}

/// Recursively scans a directory and aggregates file sizes by type
fn scan_directory(directory: &Path) -> Result<HashMap<String, u64>> {
    let mut magika = magika::Session::new().context("Failed to initialize Magika")?;
    let mut file_type_sizes: HashMap<String, u64> = HashMap::new();

    for entry in WalkDir::new(directory)
        .follow_links(false)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();

        // Skip directories and symlinks
        if !path.is_file() || path.is_symlink() {
            continue;
        }

        // Get file size
        let size = match path.metadata() {
            Ok(metadata) => metadata.len(),
            Err(e) => {
                eprintln!("Warning: Could not access {}: {}", path.display(), e);
                continue;
            }
        };

        // Identify file type using Magika
        let file_type = match magika.identify_file_sync(path) {
            Ok(result) => result.info().label.to_string(),
            Err(e) => {
                eprintln!(
                    "Warning: Could not identify file type for {}: {}",
                    path.display(),
                    e
                );
                continue;
            }
        };

        // Aggregate size by file type
        *file_type_sizes.entry(file_type).or_insert(0) += size;
    }

    Ok(file_type_sizes)
}

/// Formats bytes into human-readable size (B, KB, MB, GB, TB, PB)
fn format_size(size_bytes: u64) -> String {
    let units = ["B", "KB", "MB", "GB", "TB", "PB"];
    let mut size = size_bytes as f64;
    let mut unit_index = 0;

    while size >= 1024.0 && unit_index < units.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }

    format!("{:.2} {}", size, units[unit_index])
}

/// Displays results in a formatted table
fn display_results(file_type_sizes: &HashMap<String, u64>) {
    if file_type_sizes.is_empty() {
        println!("No files found.");
        return;
    }

    // Calculate total size
    let total_size: u64 = file_type_sizes.values().sum();

    // Sort file types by size (descending)
    let mut sorted_types: Vec<(&String, &u64)> = file_type_sizes.iter().collect();
    sorted_types.sort_by(|a, b| b.1.cmp(a.1));

    // Print table
    let separator = "=".repeat(70);

    println!();
    println!("{}", separator);
    println!("{:<30} {:<20} {:>10}", "File Type", "Size", "Percentage");
    println!("{}", separator);

    for (file_type, size) in sorted_types {
        let percentage = (*size as f64 / total_size as f64) * 100.0;
        println!(
            "{:<30} {:<20} {:>9.2}%",
            file_type,
            format_size(*size),
            percentage
        );
    }

    println!("{}", separator);
    println!(
        "{:<30} {:<20} {:>9.2}%",
        "Total",
        format_size(total_size),
        100.0
    );
    println!("{}", separator);
    println!();
}
