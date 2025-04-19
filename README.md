# rusty react flow

A powerful command-line tool to analyze TypeScript/JavaScript modules for imports and exports.

[![Crates.io](https://img.shields.io/crates/v/typescript-analyzer)](https://crates.io/crates/rusty-react-flow)
[![npm](https://img.shields.io/npm/v/typescript-analyzer)](https://www.npmjs.com/package/rusty-react-flow)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## Features

- Analyze TypeScript/JavaScript files for imports and exports
- Supports TS, TSX, JS, and JSX file extensions
- Generate detailed reports on module dependencies
- Interactive mode for selecting directories and files
- Export analysis to JSON or display in console
- Get summary statistics about most imported/exported modules

## Installation

### From Cargo

```bash
cargo install typescript-analyzer
```

### From NPM

```bash
npm install -g typescript-analyzer
```

## Usage

Basic usage:

```bash
typescript-analyzer
```

This will analyze the current directory and print the results to the console.

### Options

```
USAGE:
    typescript-analyzer [OPTIONS]

OPTIONS:
    -p, --path <PATH>      Directory path to analyze [default: .]
    -i, --interactive      Run in interactive mode
    -o, --output <FILE>    Output JSON file (default: print to stdout)
    -h, --help             Print help information
    -V, --version          Print version information
```

### Examples

Analyze a specific directory:

```bash
typescript-analyzer --path ./src
```

Run in interactive mode:

```bash
typescript-analyzer --interactive
```

Save analysis to a file:

```bash
typescript-analyzer --output analysis.json
```

## Output Format

The tool generates a JSON output with the following structure:

```json
{
  "files": [
    {
      "filePath": "src/components/App.tsx",
      "imports": [
        {
          "name": "React",
          "source": "react",
          "kind": "default"
        },
        // ...
      ],
      "exports": [
        {
          "name": "App",
          "kind": "default-function"
        },
        // ...
      ]
    },
    // ...
  ],
  "summary": {
    "totalFiles": 10,
    "totalImports": 45,
    "totalExports": 15,
    "mostImported": [
      "react",
      "@material-ui/core",
      // ...
    ],
    "mostExported": [
      "utils",
      "types",
      // ...
    ]
  }
}
```

## Advanced Usage

### Programmatic API

TypeScript Analyzer can also be used as a library in your Rust projects:

```rust
use typescript_analyzer::{run_app, cli::Cli};

fn main() {
    let cli = Cli {
        path: String::from("./src"),
        interactive: false,
        output: Some(String::from("output.json")),
    };
    
    run_app(cli).expect("Failed to analyze");
}
```

## Development

### Prerequisites

- Rust 1.56.0 or later
- Cargo

### Building from Source

```bash
git clone https://github.com/cargo-fob/rusty-react-flow.git
cd rusty-react-flow
cargo build --release
```

## How It Works

TypeScript Analyzer uses [swc](https://github.com/swc-project/swc) to parse TypeScript and JavaScript files. It extracts import and export declarations from the AST and generates a comprehensive report.

The analysis process includes:
1. Recursively finding all TS/JS files in the specified directory
2. Parsing each file with swc
3. Extracting import and export information
4. Generating summary statistics
5. Outputting results in JSON format

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- [swc](https://github.com/swc-project/swc) - The JavaScript/TypeScript compiler used for parsing
- [clap](https://github.com/clap-rs/clap) - Command-line argument parsing
- [inquire](https://github.com/mikaelmello/inquire) - Interactive CLI interface