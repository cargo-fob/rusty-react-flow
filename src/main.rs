use std::fs;
use std::path::{Path, PathBuf};
use std::collections::HashMap;

use clap::{Parser, ArgAction};
use inquire::{Select, MultiSelect};
use walkdir::WalkDir;
use serde::Serialize;
use swc_common::{sync::Lrc, SourceMap, FileName};
use swc_ecma_parser::{lexer::Lexer, Parser as SwcParser, StringInput, Syntax, TsSyntax};
use swc_ecma_ast::*;

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct ImportInfo {
    name: String,
    source: String,
    kind: String,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct ExportInfo {
    name: String,
    kind: String,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct FileAnalysis {
    file_path: String,
    imports: Vec<ImportInfo>,
    exports: Vec<ExportInfo>,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Output {
    files: Vec<FileAnalysis>,
    summary: Summary,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Summary {
    total_files: usize,
    total_imports: usize,
    total_exports: usize,
    most_imported: Vec<String>,
    most_exported: Vec<String>,
}

// 새로운 clap 구문을 사용한 CLI 설정
#[derive(Parser, Debug)]
#[command(author, version, about = "Analyzes TypeScript/JavaScript modules for imports and exports")]
struct Cli {
    /// Directory path to analyze (default: current directory)
    #[arg(short, long, value_name = "PATH", default_value = ".")]
    path: String,

    /// Run in interactive mode
    #[arg(short, long, action = ArgAction::SetTrue)]
    interactive: bool,

    /// Output JSON file (default: print to stdout)
    #[arg(short, long, value_name = "FILE")]
    output: Option<String>,
}

fn is_typescript_file(path: &Path) -> bool {
    if let Some(ext) = path.extension() {
        let ext_str = ext.to_string_lossy().to_lowercase();
        return ext_str == "ts" || ext_str == "tsx" || ext_str == "js" || ext_str == "jsx";
    }
    false
}

fn analyze_file(file_path: &Path) -> Option<FileAnalysis> {
    let cm: Lrc<SourceMap> = Default::default();

    let src = match fs::read_to_string(file_path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Failed to read file {}: {}", file_path.display(), e);
            return None;
        }
    };
    println!("Reading file: {}", src);

    let fm = cm.new_source_file(FileName::Real(file_path.to_path_buf()).into(), src);
    println!("Creating source file: {:?}", fm);

    // Determine if the file is TSX/JSX based on extension
    let is_tsx = if let Some(ext) = file_path.extension() {
        let ext_str = ext.to_string_lossy().to_lowercase();
        ext_str == "tsx" || ext_str == "jsx"
    } else {
        false
    };

    let syntax = Syntax::Typescript(TsSyntax {
        tsx: is_tsx,
        decorators: true,
        dts: false,
        no_early_errors: false,
        disallow_ambiguous_jsx_like: false,
    });

    let lexer = Lexer::new(syntax, Default::default(), StringInput::from(&*fm), None);
    let mut parser = SwcParser::new_from(lexer);
    println!("Creating parser: {:?}", parser.parse_module());


    let module = match parser.parse_module() {
        Ok(module) => module,
        Err(e) => {
            eprintln!("Failed to parse {}: {:?}", file_path.display(), e);
            return None;
        }
    };

    let mut imports = Vec::new();
    let mut exports = Vec::new();

    for item in &module.body {
        if let ModuleItem::ModuleDecl(ModuleDecl::Import(import)) = item {
            let from = import.src.value.to_string();
            for specifier in &import.specifiers {
                match specifier {
                    ImportSpecifier::Named(named) => {
                        let name = named.local.sym.to_string();
                        imports.push(ImportInfo {
                            name,
                            source: from.clone(),
                            kind: "named".to_string(),
                        });
                    }
                    ImportSpecifier::Default(default) => {
                        imports.push(ImportInfo {
                            name: default.local.sym.to_string(),
                            source: from.clone(),
                            kind: "default".to_string(),
                        });
                    }
                    ImportSpecifier::Namespace(ns) => {
                        imports.push(ImportInfo {
                            name: ns.local.sym.to_string(),
                            source: from.clone(),
                            kind: "namespace".to_string(),
                        });
                    }
                }
            }
        }

        match item {
            ModuleItem::ModuleDecl(ModuleDecl::ExportNamed(named)) => {
                println!("Exporting named: {:?}", named.specifiers);

                for spec in &named.specifiers {
                    match spec {
                        ExportSpecifier::Named(named_spec) => {
                            let name = if let Some(exported) = &named_spec.exported {
                                match exported {
                                    ModuleExportName::Ident(id) => id.sym.to_string(),
                                    ModuleExportName::Str(s) => s.value.to_string(),
                                }
                            } else {
                                named_spec.orig.atom().to_string()
                            };
                            exports.push(ExportInfo {
                                name,
                                kind: "named".to_string(),
                            });
                        }
                        _ => {}
                    }
                }
            }
            ModuleItem::ModuleDecl(ModuleDecl::ExportDecl(decl)) => match &decl.decl {
                Decl::Var(var_decl) => {
                    for decl in &var_decl.decls {
                        if let Pat::Ident(ident) = &decl.name {
                            exports.push(ExportInfo {
                                name: ident.id.sym.to_string(),
                                kind: "variable".to_string(),
                            });
                        }
                    }
                }
                Decl::Fn(f) => {
                    exports.push(ExportInfo {
                        name: f.ident.sym.to_string(),
                        kind: "function".to_string(),
                    });
                }
                Decl::Class(c) => {
                    exports.push(ExportInfo {
                        name: c.ident.sym.to_string(),
                        kind: "class".to_string(),
                    });
                }
                Decl::TsInterface(i) => {
                    exports.push(ExportInfo {
                        name: i.id.sym.to_string(),
                        kind: "interface".to_string(),
                    });
                }
                _ => {}
            },
            ModuleItem::ModuleDecl(ModuleDecl::ExportDefaultExpr(_)) => {
                exports.push(ExportInfo {
                    name: "<expression>".to_string(),
                    kind: "default".to_string(),
                });
            }
            ModuleItem::ModuleDecl(ModuleDecl::ExportDefaultDecl(decl)) => {
                match &decl.decl {
                    DefaultDecl::Class(c) => {
                        let name = c
                            .ident
                            .as_ref()
                            .map(|id| id.sym.to_string())
                            .unwrap_or_else(|| "<anonymous>".to_string());
                        exports.push(ExportInfo {
                            name,
                            kind: "default-class".to_string(),
                        });
                    }
                    DefaultDecl::Fn(f) => {
                        let name = f
                            .ident
                            .as_ref()
                            .map(|id| id.sym.to_string())
                            .unwrap_or_else(|| "<anonymous>".to_string());
                        exports.push(ExportInfo {
                            name,
                            kind: "default-function".to_string(),
                        });
                    }
                    DefaultDecl::TsInterfaceDecl(_) => {
                        exports.push(ExportInfo {
                            name: "<interface>".to_string(),
                            kind: "default".to_string(),
                        });
                    }
                }
            },
            _ => {}
        }
    }

    Some(FileAnalysis {
        file_path: file_path.to_string_lossy().to_string(),
        imports,
        exports,
    })
}

fn generate_summary(file_analyses: &[FileAnalysis]) -> Summary {
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

    // Get top 5 most imported sources
    let mut import_vec: Vec<(String, usize)> = import_sources.into_iter().collect();
    import_vec.sort_by(|a, b| b.1.cmp(&a.1));
    let most_imported = import_vec.into_iter()
        .take(5)
        .map(|(source, _)| source)
        .collect();

    // Get top 5 most exported names
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

fn main() {
    // 새로운 clap 구문으로 CLI 파싱
    let cli = Cli::parse();

    let path = cli.path;
    let interactive = cli.interactive;
    let output_file = cli.output;

    let target_dir = if interactive {
        let current_dir = std::env::current_dir().expect("Failed to get current directory");
        let mut dirs = get_subdirectories(&current_dir);

        if dirs.is_empty() {
            println!("No subdirectories found in current directory.");
            current_dir
        } else {
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
    } else {
        PathBuf::from(path)
    };

    println!("Analyzing directory: {}", target_dir.display());

    // Collect TypeScript/JavaScript files
    let mut ts_files = Vec::new();

    for entry in WalkDir::new(&target_dir).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() && is_typescript_file(path) {
            ts_files.push(path.to_path_buf());
        }
    }

    println!("Found {} TypeScript/JavaScript files", ts_files.len());

    // If interactive, ask user to select specific files
    let selected_files = if interactive && !ts_files.is_empty() {
        let file_options: Vec<String> = ts_files.iter()
            .map(|p| p.strip_prefix(&target_dir).unwrap_or(p).to_string_lossy().to_string())
            .collect();

        let selected = MultiSelect::new("Select files to analyze (Space to select, Enter to confirm):", file_options)
            .prompt()
            .unwrap_or_else(|_| Vec::new());

        if selected.is_empty() {
            println!("No files selected, analyzing all files");
            ts_files
        } else {
            selected.into_iter()
                .map(|s| target_dir.join(s))
                .collect()
        }
    } else {
        ts_files
    };

    // Analyze files
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

    // Generate summary
    let summary = generate_summary(&file_analyses);

    // Create output
    let output = Output {
        files: file_analyses,
        summary,
    };

    // Output result
    let json = serde_json::to_string_pretty(&output).unwrap();

    match output_file {
        Some(file) => {
            fs::write(&file, &json).expect("Failed to write output file");
            println!("Analysis written to {}", file);
        },
        None => {
            println!("{}", json);
        }
    }
}

fn get_subdirectories(dir: &Path) -> Vec<PathBuf> {
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