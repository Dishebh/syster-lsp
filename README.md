# Syster LSP

Language Server Protocol implementation for SysML v2 and KerML.

## Components

- `crates/syster-lsp` - Rust LSP server binary

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

```bash
cargo build --release -p syster-lsp
```

## Usage

The LSP server binary can be used with any editor that supports the Language Server Protocol.

For VS Code integration, see the [vscode-lsp extension](https://github.com/jade-codes/syster/tree/main/editors/vscode-lsp).

## License

MIT

## Development

### DevContainer Setup (Recommended)

This project includes a DevContainer configuration for a consistent development environment.

**Using VS Code:**
1. Install the [Dev Containers extension](https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.remote-containers)
2. Open this repository in VS Code
3. Click "Reopen in Container" when prompted (or use Command Palette: "Dev Containers: Reopen in Container")

**What's included:**
- Rust 1.x with toolchain
- rust-analyzer, clippy
- GitHub CLI
- All VS Code extensions pre-configured

### Manual Setup

If not using DevContainer:

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Build the LSP server
cargo build --release -p syster-lsp

# Install VS Code extension dependencies
cd editors/vscode
npm install
```
