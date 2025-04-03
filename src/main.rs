use std::fs;
use std::path::Path;

use swc_common::{sync::Lrc, SourceMap, FileName};
use swc_ecma_parser::{lexer::Lexer, Parser, StringInput, Syntax, TsSyntax};
use swc_ecma_ast::{ModuleItem, ModuleDecl};

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

    let lexer = Lexer::new(
        syntax,
        Default::default(),
        StringInput::from(&*fm),
        None,
    );

    let mut parser = Parser::new_from(lexer);
    let module = parser.parse_module().expect("Failed to parse module");

    println!("== Imports ==");
    for item in &module.body {
        if let ModuleItem::ModuleDecl(ModuleDecl::Import(import)) = item {
            println!("import from \"{}\"", import.src.value);
        }
    }

    println!("\n== Exports ==");
    for item in &module.body {
        match item {
            ModuleItem::ModuleDecl(ModuleDecl::ExportNamed(_)) => println!("ExportNamed"),
            ModuleItem::ModuleDecl(ModuleDecl::ExportDefaultDecl(_)) => println!("ExportDefaultDecl"),
            ModuleItem::ModuleDecl(ModuleDecl::ExportDefaultExpr(_)) => println!("ExportDefaultExpr"),
            _ => {}
        }
    }
}
