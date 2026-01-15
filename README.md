# Syster LSP

Language Server Protocol implementation for SysML v2 and KerML.

## Components

- `crates/syster-lsp` - Rust LSP server binary
- `editors/vscode` - VS Code extension

## Features

- Syntax highlighting
- Code completion
- Go to definition
- Find references
- Hover documentation
- Document outline
- Code formatting
- Semantic tokens
- Inlay hints
- Folding ranges
- Diagram support

## Building

### LSP Server

```bash
cargo build --release -p syster-lsp
```

### VS Code Extension

```bash
cd editors/vscode
npm install
npm run compile
npm run package
```

## Installation

Install the VS Code extension from the marketplace or build from source.

## License

MIT
