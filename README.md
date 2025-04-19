# 🚀 rusty-react-flow

A powerful command-line tool to analyze TypeScript/JavaScript modules for imports and exports.

[![Crates.io](https://img.shields.io/crates/v/rusty-react-flow)](https://crates.io/crates/rusty-react-flow)  [![npm](https://img.shields.io/npm/v/rusty-react-flow)](https://www.npmjs.com/package/rusty-react-flow)  [![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

---

## ✨ Features

- 🔍 Analyze `.ts`, `.tsx`, `.js`, and `.jsx` files for imports and exports
- 📊 Generate detailed reports on module dependencies
- 🎛 Interactive mode for selecting directories and files
- 📁 Export analysis to JSON or display in console
- 📈 Summary statistics for most imported/exported modules

---

## 📦 Installation

### As a library dependency

```bash
cargo add rusty-react-flow
```

### Global CLI (Cargo)

```bash
cargo install rusty-react-flow
```

### Node.js projects (NPM)

```bash
npm install -D rusty-react-flow
```

Invoke via `npx`:

```bash
npx rusty-react-flow [OPTIONS]
```

---

## ⚙️ Usage

Run without arguments to analyze the current directory:

```bash
rusty-react-flow
# or via npm:
# npx rusty-react-flow
```

### ❯ CLI Options

| Option                   | Description                                 | Default |
| ------------------------ | ------------------------------------------- | ------- |
| `-p`, `--path <PATH>`    | Directory path to analyze                   | `.`     |
| `-i`, `--interactive`     | Run in interactive mode                     | —       |
| `-o`, `--output <FILE>`  | Write output JSON to file                   | stdout  |
| `--help`                 | Print help information                      | —       |
| `--version`              | Print version information                   | —       |

### ❯ Examples

- **Analyze `src` folder:**
  ```bash
  rusty-react-flow --path ./src
  ```

- **Interactive mode:**
  ```bash
  rusty-react-flow --interactive
  ```

- **Save JSON output:**
  ```bash
  rusty-react-flow --output report.json
  ```

- **All combined (with npx):**
  ```bash
  npx rusty-react-flow --path ./lib --interactive --output deps.json
  ```

---

## 📄 Output Format

The JSON output has this structure:

```json
{
  "files": [
    {
      "filePath": "src/App.tsx",
      "imports": [
        { "name": "React", "source": "react", "kind": "default" }
      ],
      "exports": [
        { "name": "App", "kind": "default-function" }
      ]
    }
  ],
  "summary": {
    "totalFiles": 1,
    "totalImports": 1,
    "totalExports": 1,
    "mostImported": ["react"],
    "mostExported": ["App"]
  }
}
```

---

## 🛠️ Development

### Prerequisites

- Rust 1.85.1 or later
- Cargo

### Build from source

```bash
git clone https://github.com/cargo-fob/rusty-react-flow.git
cd rusty-react-flow
cargo build --release
```

---

## 🤝 Contributing

Contributions are welcome:

1. Fork the repo
2. Create a branch (`git checkout -b feature/X`)
3. Commit your changes (`git commit -m "Add feature X"`)
4. Push (`git push origin feature/X`)
5. Open a Pull Request

---

## 📝 License

MIT © Jaeha Lee

