use std::path::{Path, PathBuf};
use std::fs;
use clap::{Parser, ArgAction};
use inquire::{Select, MultiSelect};

#[derive(Parser, Debug)]
#[command(author, version, about = "Analyzes TypeScript/JavaScript modules for imports and exports")]
pub struct Cli {
    /// Directory path to analyze (default: current directory)
    #[arg(short, long, value_name = "PATH", default_value = ".")]
    pub path: String,

    /// Run in interactive mode
    #[arg(short, long, action = ArgAction::SetTrue)]
    pub interactive: bool,

    /// Output JSON file (default: print to stdout)
    #[arg(short, long, value_name = "FILE")]
    pub output: Option<String>,
}

pub fn get_subdirectories(dir: &Path) -> Vec<PathBuf> {
    let mut subdirs = Vec::new();

    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.filter_map(|e| e.ok()) {
            let path = entry.path();
            if path.is_dir() {
                subdirs.push(path);
            }
        }
    }

    subdirs
}

pub fn select_directory(interactive: bool, path: &str) -> PathBuf {
    if !interactive {
        return PathBuf::from(path);
    }

    let current_dir = std::env::current_dir().expect("Failed to get current directory");
    let mut dirs = get_subdirectories(&current_dir);

    if dirs.is_empty() {
        println!("No subdirectories found in current directory.");
        return current_dir;
    }

    dirs.insert(0, current_dir.clone());

    let options: Vec<String> = dirs.iter()
        .map(|p| p.to_string_lossy().to_string())
        .collect();

    let default_option = options[0].clone();

    let selection = Select::new("Select directory to analyze:", options)
        .prompt()
        .unwrap_or_else(|_| default_option);

    PathBuf::from(selection)
}

pub fn select_files(interactive: bool, target_dir: &Path, all_files: Vec<PathBuf>) -> Vec<PathBuf> {
    if !interactive || all_files.is_empty() {
        return all_files;
    }

    let file_options: Vec<String> = all_files.iter()
        .map(|p| p.strip_prefix(target_dir).unwrap_or(p).to_string_lossy().to_string())
        .collect();

    let selected = MultiSelect::new(
        "Select files to analyze (Space to select, Enter to confirm):",
        file_options
    )
        .prompt()
        .unwrap_or_else(|_| Vec::new());

    if selected.is_empty() {
        println!("No files selected, analyzing all files");
        all_files
    } else {
        selected.into_iter()
            .map(|s| target_dir.join(s))
            .collect()
    }
}