# Installation Guide

## CLI Version 3.0 (Recommended)

### macOS

```bash
# Install dependencies
brew install pdfium

# Build from source
git clone <repo>
cd pdf-to-cbz-converter2/src-cli
cargo build --release

# Binary at: target/release/pdf-to-cbz
# Copy to PATH for easy access:
cp target/release/pdf-to-cbz /usr/local/bin/pdf-to-cbz
```

### Linux (Ubuntu/Debian)

```bash
# Install dependencies
sudo apt-get update
sudo apt-get install libpdfium0-dev

# Build
cd src-cli
cargo build --release

# Install
sudo cp target/release/pdf-to-cbz /usr/local/bin/
```

### Linux (Fedora/RHEL)

```bash
# Install dependencies
sudo dnf install pdfium-devel

# Build
cd src-cli
cargo build --release

# Install
sudo cp target/release/pdf-to-cbz /usr/local/bin/
```

### Windows

1. Download and install Rust from https://rustup.rs/
2. Download pdfium library or use vcpkg:
   ```bash
   vcpkg install pdfium:x64-windows
   ```
3. Build:
   ```bash
   cd src-cli
   cargo build --release
   ```
4. Binary at: `target/release/pdf-to-cbz.exe`

## Requirements

- **Rust 1.70+** - [Install](https://rustup.rs/)
- **C++ build tools**
  - macOS: Xcode Command Line Tools (`xcode-select --install`)
  - Linux: `build-essential` or equivalent
  - Windows: Visual Studio or MSVC build tools

## Optional Dependencies

For CBR (RAR) format support:

- **macOS**: `brew install unar`
- **Linux**: `sudo apt-get install unar` (Debian/Ubuntu) or `sudo dnf install unar` (Fedora)
- **Windows**: Download from http://theunarchiver.com/command-line (unar.exe)

## Verify Installation

```bash
pdf-to-cbz --version
pdf-to-cbz --help
```

## Size Comparison

| Version | Binary Size | Dependencies | Memory Usage |
|---------|------------|--------------|------------|
| v2.0 (Tauri) | 150+ MB | 16+ | High |
| v3.0 (CLI) | ~40-50 MB* | 5 | Low |

*Includes linked pdfium library
