use anyhow::{Error as AErr, Result};
use std::fs::{self};
use std::path::Path;
use std::{collections::HashMap, path::PathBuf};

fn rename(path: impl Into<String>, file_extension: impl Into<String>) -> Result<(), AErr> {
    let file_ext = format!(".{}", file_extension.into());
    let path = PathBuf::from(path.into()).canonicalize()?;
    let files = match fs::read_dir(&path) {
        Ok(entries) => entries.filter_map(Result::ok).collect::<Vec<_>>(),
        Err(err) => {
            eprintln!("Error reading directory: {}", err);
            return Ok(());
        }
    };

    let mut name_counters: HashMap<String, usize> = HashMap::new();

    for entry in files {
        let file_name = entry.file_name().to_str().expect("crap").to_string();
        if file_name.contains(&file_ext) {
            let file_name = file_name.replace(&file_ext, "");
            let lowercased = file_name.to_lowercase();

            let lowercased = lowercased.replace(" ", "_");

            let base_name = lowercased.split('-').next().unwrap_or("").to_string();

            let new_index = name_counters.entry(base_name.clone()).or_insert(0);
            let new_name = if lowercased.contains('-') {
                format!("{base_name}_{new_index}{file_ext}")
            } else {
                format!("{base_name}{file_ext}")
            };
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
fn main() -> Result<(), AErr> {
    let args: Vec<String> = std::env::args().collect();
    let args: Vec<&str> = args.iter().map(String::as_str).collect();

    match args.len().cmp(&3) {
        std::cmp::Ordering::Equal => rename(args[1], args[2])?,
        std::cmp::Ordering::Greater => println!("too many args"),
        std::cmp::Ordering::Less => {
            println!("you have to provide a path and an extension:\nrename ./assets glb")
        }
    }

    Ok(())
}
