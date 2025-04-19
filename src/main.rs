use clap::Parser;
use rusty_react_flow::cli::Cli;
use rusty_react_flow::run_app;

fn main() {
    let cli = Cli::parse();

    if let Err(e) = run_app(cli) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}