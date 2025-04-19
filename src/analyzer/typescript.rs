use std::fs;
use std::path::Path;
use swc_common::{sync::Lrc, SourceMap, FileName};
use swc_ecma_parser::{lexer::Lexer, Parser as SwcParser, StringInput, Syntax, TsSyntax};
use swc_ecma_ast::*;

use crate::models::{FileAnalysis, ImportInfo, ExportInfo};

/// 파일이 TypeScript/JavaScript 파일인지 확인합니다.
pub fn is_typescript_file(path: &Path) -> bool {
    if let Some(ext) = path.extension() {
        let ext_str = ext.to_string_lossy().to_lowercase();
        return ext_str == "ts" || ext_str == "tsx" || ext_str == "js" || ext_str == "jsx";
    }
    false
}

/// TypeScript/JavaScript 파일을 분석합니다.
pub fn analyze_file(file_path: &Path) -> Option<FileAnalysis> {
    let cm: Lrc<SourceMap> = Default::default();

    let src = match fs::read_to_string(file_path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Failed to read file {}: {}", file_path.display(), e);
            return None;
        }
    };

    let fm = cm.new_source_file(FileName::Real(file_path.to_path_buf()).into(), src);

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

    let module = match parser.parse_module() {
        Ok(module) => module,
        Err(e) => {
            eprintln!("Failed to parse {}: {:?}", file_path.display(), e);
            return None;
        }
    };

    let mut imports = Vec::new();
    let mut exports = Vec::new();

    // Import 정보 추출
    extract_imports(&module, &mut imports);

    // Export 정보 추출
    extract_exports(&module, &mut exports);

    Some(FileAnalysis {
        file_path: file_path.to_string_lossy().to_string(),
        imports,
        exports,
    })
}

/// 모듈에서 import 정보를 추출합니다.
fn extract_imports(module: &Module, imports: &mut Vec<ImportInfo>) {
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
    }
}

/// 모듈에서 export 정보를 추출합니다.
fn extract_exports(module: &Module, exports: &mut Vec<ExportInfo>) {
    for item in &module.body {
        match item {
            ModuleItem::ModuleDecl(ModuleDecl::ExportNamed(named)) => {
                for spec in &named.specifiers {
                    if let ExportSpecifier::Named(named_spec) = spec {
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
}