mod config;
use config::Config;
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
    src: Option<String>,
    /// Destination directory to write organized files
    #[clap(long)]
    dest: Option<String>,
    /// Path to Sled DB file
    #[clap(long)]
    db: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Work {
    file: String,
    language: String,
    quality: String,
    national: bool,
    content_length: usize,  // Added to help with scoring
}

#[derive(Debug)]
struct ScoredOpportunity {
    filename: String,
    score: f64,
    details: WorkDetails,
}

#[derive(Debug)]
struct WorkDetails {
    is_java: bool,
    is_uruguayan: bool,
    is_high_quality: bool,
    content_length: usize,
}

impl ScoredOpportunity {
    fn calculate_score(&mut self) {
        let mut score = 0.0;
        
        // Base scores for each criterion
        if self.details.is_java { score += 3.0; }
        if self.details.is_uruguayan { score += 2.0; }
        if self.details.is_high_quality { score += 1.0; }
        
        // Bonus for content length (normalized between 0 and 1)
        let length_score = (self.details.content_length as f64 / 5000.0).min(1.0);
        score += length_score;
        
        self.score = score;
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    
    // Use command line args if provided, otherwise use default config
    let config = if let (Some(src), Some(dest), Some(db)) = (args.src, args.dest, args.db) {
        Config::from_args(&src, &dest, &db)
    } else {
        Config::new()
    };

    let dest_path = &config.destination_dir;
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
    
    let db = sled::open(&config.db_path)?;
    db.clear()?;
    let java_re = Regex::new(r"(?i)\bjava\b").unwrap();
    
    // scan all files and classify
    for entry in WalkDir::new(&config.source_dir).into_iter().filter_map(|e| e.ok()) {
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
                content_length: content.len(),
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
        let rel_path = src_path.strip_prefix(&config.source_dir).unwrap();
        // determine base group directory
        let mut group_dir = PathBuf::from(&config.destination_dir);
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

    // Create or update top_opportunities.md
    update_top_opportunities(&config)?;
    
    Ok(())
}

fn update_top_opportunities(config: &Config) -> Result<(), Box<dyn Error>> {
    let db = sled::open(&config.db_path)?;
    let mut opportunities = Vec::new();

    // Process all files in the database
    for item in db.iter() {
        let (_, val) = item?;
        if let Ok(work) = serde_json::from_slice::<Work>(&val) {
            let filename = Path::new(&work.file)
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("unknown")
                .to_string();

            let mut scored = ScoredOpportunity {
                filename,
                score: 0.0,
                details: WorkDetails {
                    is_java: work.language == "java",
                    is_uruguayan: work.national,
                    is_high_quality: work.quality == "good",
                    content_length: work.content_length,
                },
            };
            
            scored.calculate_score();
            opportunities.push(scored);
        }
    }

    // Sort opportunities by score in descending order
    opportunities.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));

    // Create markdown content
    let mut content = String::from("# Top Job Opportunities\n\n");
    content.push_str("This document lists job opportunities sorted by importance. Importance is calculated based on:\n");
    content.push_str("- Java-based positions (3 points)\n");
    content.push_str("- Uruguayan companies/organizations (2 points)\n");
    content.push_str("- High quality job descriptions (1 point)\n");
    content.push_str("- Content length bonus (up to 1 point)\n\n");
    content.push_str("## Opportunities (Sorted by Importance)\n\n");

    if opportunities.is_empty() {
        content.push_str("No opportunities found.\n");
    } else {
        for (index, opp) in opportunities.iter().enumerate() {
            content.push_str(&format!("### {}. {} (Score: {:.2})\n", 
                index + 1, 
                opp.filename,
                opp.score
            ));
            content.push_str("**Criteria met:**\n");
            if opp.details.is_java { content.push_str("- Java position\n"); }
            if opp.details.is_uruguayan { content.push_str("- Uruguayan company\n"); }
            if opp.details.is_high_quality { content.push_str("- High quality description\n"); }
            content.push_str(&format!("- Content length: {} characters\n\n", opp.details.content_length));
        }
    }

    content.push_str("\n## Last Updated\n\n");
    content.push_str(&format!("Generated on: {}\n", chrono::Local::now().format("%Y-%m-%d %H:%M:%S")));

    // Write to file
    fs::write(config.get_top_opportunities_path(), content)?;
    Ok(())
}
