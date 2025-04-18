use anyhow::{Error as AErr, Result};
use std::fs::{self, DirEntry};
use std::path::Path;
use std::{collections::HashMap, path::PathBuf};
fn get_files(path: &PathBuf) -> Result<Vec<DirEntry>, AErr> {
    Ok(fs::read_dir(path)?
        .filter_map(Result::ok)
        .collect::<Vec<_>>())
}

fn rename(path: impl Into<String>, file_extension: impl Into<String>) -> Result<(), AErr> {
    let file_ext = format!(".{}", file_extension.into());
    let path = PathBuf::from(path.into()).canonicalize()?;
    let files = get_files(&path)?;
    let mut name_counters: HashMap<String, usize> = HashMap::new();

    for entry in files {
        let file_name = match entry.file_name().to_str() {
            Some(name) => name.to_string(),
            None => {
                eprintln!("Invalid UTF-8 in filename, skipping {entry:?}");
                continue;
            }
        };

        if file_name.contains(&file_ext) {
            let base_name = file_name
                .replace(&file_ext, "")
                .to_lowercase()
                .replace(" ", "_")
                .split('-')
                .next()
                .unwrap_or("unknown")
                .to_string();

            let new_index = name_counters.entry(base_name.clone()).or_insert(0);
            let new_name = format!("{base_name}_{new_index}{file_ext}");
            *new_index += 1;

            let old_path = entry.path();
            let new_path = Path::new(&path).join(&new_name);

            if let Err(err) = fs::rename(&old_path, &new_path) {
                eprintln!("Failed to rename {} to {}: {}", file_name, new_name, err);
            } else {
                println!("Renamed: {} -> {}", file_name, new_name);
            }
        }
    }
    Ok(())
}
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let args: Vec<&str> = args.iter().map(String::as_str).collect();

    match args.len().cmp(&3) {
        std::cmp::Ordering::Equal => match rename(args[1], args[2]) {
            Ok(_) => println!("Finished renaming files"),
            Err(e) => eprintln!("Error: {e}"),
        },
        std::cmp::Ordering::Greater => {
            println!("Too many args. Usage:\n    renamer <path> <extension (no .)>")
        }
        std::cmp::Ordering::Less => {
            println!("Not enough args. Usage:\n    renamer <path> <extension (no .)>")
        }
    }
}
