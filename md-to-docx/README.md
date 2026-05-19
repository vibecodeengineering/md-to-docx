# md-to-docx

A fast, simple CLI tool written in Rust to convert Markdown files to Microsoft Word (.docx) format.

## Features

- ✅ Headings (H1, H2, H3)
- ✅ Bold and italic text
- ✅ Inline code and code blocks
- ✅ Horizontal rules
- ✅ Simple lists
- ✅ Fast conversion

## Installation

### From Source

```bash
git clone https://github.com/vibecodeengineering/md-to-docx.git
cd md-to-docx
cargo build --release
```

The binary will be at `target/release/md-to-docx`.

### Prerequisites

- Rust 1.70+ installed

## Cross-compilation

### Building for Windows on Linux

You can cross-compile the binary for Windows from a Linux machine using Docker:

```bash
./build-windows.sh
```

This will create a Docker image that builds the Windows binary (`md-to-docx.exe`) and outputs it to the current directory.

**Requirements:**
- Docker installed and running
- Linux host (tested on Debian Trixie)

The resulting binary will be available as `md-to-docx.exe` in the current directory.

## Usage

```bash
# Basic usage - converts input.md to input.docx
md-to-docx input.md

# Specify custom output file
md-to-docx input.md -o output.docx

# Or use long form
md-to-docx input.md --output output.docx

# Get help
md-to-docx --help
```

## Examples

```bash
# Convert README.md to README.docx
md-to-docx README.md

# Convert to a specific location
md-to-docx document.md -o ~/Desktop/my-document.docx
```

## Supported Markdown

| Markdown | DOCX Output |
|----------|-------------|
| `# Heading` | Heading 1 |
| `## Heading` | Heading 2 |
| `### Heading` | Heading 3 |
| `**bold**` | **Bold** |
| `*italic*` | *Italic* |
| `` `code` `` | Monospace |
| ````code block```` | Code block (Courier New) |
| `---` | Horizontal rule |

## License

MIT
