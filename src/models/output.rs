use serde::Serialize;
use super::{ImportInfo, ExportInfo};

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FileAnalysis {
    pub file_path: String,
    pub imports: Vec<ImportInfo>,
    pub exports: Vec<ExportInfo>,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Output {
    pub files: Vec<FileAnalysis>,
    pub summary: Summary,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Summary {
    pub total_files: usize,
    pub total_imports: usize,
    pub total_exports: usize,
    pub most_imported: Vec<String>,
    pub most_exported: Vec<String>,
}