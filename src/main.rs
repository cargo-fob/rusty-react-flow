use std::fs;
use std::path::Path;

use serde::Serialize;
use swc_common::{sync::Lrc, SourceMap, FileName};
use swc_ecma_parser::{lexer::Lexer, Parser, StringInput, Syntax, TsSyntax};
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
struct Output {
    imports: Vec<ImportInfo>,
    exports: Vec<ExportInfo>,
}

fn main() {
    let cm: Lrc<SourceMap> = Default::default();

    let path = Path::new("test.tsx");
    let src = fs::read_to_string(path).expect("Failed to read file");

    let fm = cm.new_source_file(FileName::Real(path.to_path_buf()).into(), src);

    let syntax = Syntax::Typescript(TsSyntax {
        tsx: true,
        decorators: false,
        dts: false,
        no_early_errors: false,
        disallow_ambiguous_jsx_like: false,
    });

    let lexer = Lexer::new(syntax, Default::default(), StringInput::from(&*fm), None);
    let mut parser = Parser::new_from(lexer);
    let module = parser.parse_module().expect("Failed to parse module");

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
                for spec in &named.specifiers {
                    match spec {
                        ExportSpecifier::Named(named_spec) => {
                            let name = match &named_spec.exported {
                                Some(ModuleExportName::Ident(id)) => id.sym.to_string(),
                                _ => named_spec.orig.sym.to_string(),
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
            ModuleItem::ModuleDecl(ModuleDecl::ExportDefaultDecl(decl)) => match decl {
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
                _ => {
                    exports.push(ExportInfo {
                        name: "<other>".to_string(),
                        kind: "default".to_string(),
                    });
                }
            },
            _ => {}
        }
    }

    let output = Output { imports, exports };
    let json = serde_json::to_string_pretty(&output).unwrap();
    println!("{}", json);
}
