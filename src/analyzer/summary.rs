use std::collections::HashMap;
use crate::models::{FileAnalysis, Summary};

/// 파일 분석 결과를 바탕으로 요약 정보를 생성합니다.
pub fn generate_summary(file_analyses: &[FileAnalysis]) -> Summary {
    let total_files = file_analyses.len();

    let mut total_imports = 0;
    let mut total_exports = 0;

    let mut import_sources = HashMap::new();
    let mut exported_names = HashMap::new();

    for analysis in file_analyses {
        total_imports += analysis.imports.len();
        total_exports += analysis.exports.len();

        for import in &analysis.imports {
            *import_sources.entry(import.source.clone()).or_insert(0) += 1;
        }

        for export in &analysis.exports {
            *exported_names.entry(export.name.clone()).or_insert(0) += 1;
        }
    }

    // 가장 많이 import된 소스 상위 5개 가져오기
    let mut import_vec: Vec<(String, usize)> = import_sources.into_iter().collect();
    import_vec.sort_by(|a, b| b.1.cmp(&a.1));
    let most_imported = import_vec.into_iter()
        .take(5)
        .map(|(source, _)| source)
        .collect();

    // 가장 많이 export된 이름 상위 5개 가져오기
    let mut export_vec: Vec<(String, usize)> = exported_names.into_iter().collect();
    export_vec.sort_by(|a, b| b.1.cmp(&a.1));
    let most_exported = export_vec.into_iter()
        .take(5)
        .map(|(name, _)| name)
        .collect();

    Summary {
        total_files,
        total_imports,
        total_exports,
        most_imported,
        most_exported,
    }
}