use clap::Parser;
use walkdir::WalkDir;
use serde::{Serialize, Deserialize};
use regex::Regex;
use std::fs;
use std::path::{Path, PathBuf};
use std::error::Error;

#[derive(Parser)]
#[clap(author, version, about)]
struct Args {
    /// Source directory containing text files
    #[clap(long)]
    src: String,
    /// Destination directory to write organized files
    #[clap(long)]
    dest: String,
    /// Path to Sled DB file
    #[clap(long, default_value = "organizer.db")]
    db: String,
}

#[derive(Serialize, Deserialize)]
struct Work {
    file: String,
    language: String,
    quality: String,
    national: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let dest_path = Path::new(&args.dest);
    // clear contents of dest without removing mountpoint
    if dest_path.exists() {
        for entry in fs::read_dir(dest_path)? {
            let entry = entry?;
            let p = entry.path();
            if p.is_dir() {
                fs::remove_dir_all(&p)?;
            } else {
                fs::remove_file(&p)?;
            }
        }
    } else {
        fs::create_dir_all(dest_path).expect("Failed to create destination");
    }
    fs::create_dir_all(dest_path).expect("Failed to create destination");
    let db = sled::open(&args.db)?;
    db.clear()?;
    let java_re = Regex::new(r"(?i)\bjava\b").unwrap();
    // scan all files and classify
    for entry in WalkDir::new(&args.src).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            let path = entry.path();
            // process all files regardless of extension
            let content = fs::read_to_string(path).unwrap_or_default();
            let language = if java_re.is_match(&content) {"java"} else {"other"};
            let quality = if content.len() > 1000 {"good"} else {"low"};
            let national = content.contains("Uruguay");
            let work = Work {
                file: path.to_string_lossy().into_owned(),
                language: language.to_string(),
                quality: quality.to_string(),
                national,
            };
            db.insert(work.file.as_bytes(), serde_json::to_vec(&work)?)?;
        }
    }

    db.flush()?;
    // load and copy stored works preserving relative paths
    for item in db.iter() {
        let (_, val) = item?;
        let work: Work = serde_json::from_slice(&val)?;
        let src_path = Path::new(&work.file);
        // compute relative path from source root
        let rel_path = src_path.strip_prefix(&args.src).unwrap();
        // determine base group directory
        let mut group_dir = PathBuf::from(&args.dest);
        if work.language == "java" {
            group_dir.push("Works in java language");
        } else {
            group_dir.push("Works in other languages");
        }
        let subgroup = if work.national {
            "National (uruguayan) works"
        } else if work.quality == "good" {
            "Good quality works"
        } else {
            "Low quality works"
        };
        group_dir.push(subgroup);
        // create output path including subdirectories
        let out_path = group_dir.join(rel_path);
        if let Some(parent) = out_path.parent() {
            fs::create_dir_all(parent).expect("Failed to create output directories");
        }
        fs::copy(src_path, &out_path).expect("Failed to copy file");
    }
    Ok(())
}
