pub mod cli;
pub mod analyzer;
pub mod models;

use std::fs;
use walkdir::WalkDir;

use crate::cli::{Cli, select_directory, select_files};
use crate::analyzer::{is_typescript_file, analyze_file, generate_summary};
use crate::models::Output;

pub fn run_app(cli: Cli) -> Result<(), Box<dyn std::error::Error>> {
    let target_dir = select_directory(cli.interactive, &cli.path);
    println!("Analyzing directory: {}", target_dir.display());

    let mut ts_files = Vec::new();

    for entry in WalkDir::new(&target_dir).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() && is_typescript_file(path) {
            ts_files.push(path.to_path_buf());
        }
    }

    println!("Found {} TypeScript/JavaScript files", ts_files.len());

    let selected_files = select_files(cli.interactive, &target_dir, ts_files);

    let mut file_analyses = Vec::new();

    for file_path in &selected_files {
        print!("Analyzing {}... ", file_path.display());

        if let Some(analysis) = analyze_file(file_path) {
            println!("Done");
            file_analyses.push(analysis);
        } else {
            println!("Failed");
        }
    }

    let summary = generate_summary(&file_analyses);

    let output = Output {
        files: file_analyses,
        summary,
    };

    let json = serde_json::to_string_pretty(&output)?;

    match cli.output {
        Some(file) => {
            fs::write(&file, &json)?;
            println!("Analysis written to {}", file);
        },
        None => {
            println!("{}", json);
        }
    }

    Ok(())
}