#!/bin/bash
# PDF to CBZ Converter v3.0 - Installation Script

set -e

echo "📦 PDF to CBZ Converter v3.0 - Installation"
echo "============================================"

# Detect OS
OS=$(uname -s)

case "$OS" in
    Darwin)
        echo "🍎 Detected macOS"
        echo "Installing dependencies with Homebrew..."

        if ! command -v brew &> /dev/null; then
            echo "❌ Homebrew not found. Install from https://brew.sh"
            exit 1
        fi

        # Install pdfium for PDF processing
        if ! brew list pdfium &>/dev/null; then
            echo "Installing pdfium..."
            # Note: pdfium may not be available in main brew repo
            # Users may need to install from custom tap
            brew install pdfium || echo "⚠️  Could not install pdfium via brew"
        fi

        # Optional: unar for CBR support
        if ! brew list unar &>/dev/null; then
            echo "Installing unar (for CBR support)..."
            brew install unar
        fi
        ;;

    Linux)
        echo "🐧 Detected Linux"
        distro=$(lsb_release -si 2>/dev/null || echo "unknown")

        case "$distro" in
            Ubuntu|Debian)
                echo "Installing dependencies via apt..."
                sudo apt-get update
                sudo apt-get install -y libpdfium0-dev unar
                ;;
            Fedora|RHEL|CentOS)
                echo "Installing dependencies via dnf/yum..."
                sudo dnf install -y pdfium-devel unar || sudo yum install -y pdfium-devel unar
                ;;
            *)
                echo "⚠️  Unknown Linux distribution"
                echo "Please install: libpdfium-dev (or pdfium-devel) and unar"
                ;;
        esac
        ;;

    MINGW*|MSYS*|CYGWIN*)
        echo "🪟 Detected Windows"
        echo "Please install pdfium via vcpkg or Visual Studio"
        echo "vcpkg install pdfium:x64-windows"
        ;;

    *)
        echo "❌ Unsupported OS: $OS"
        exit 1
        ;;
esac

# Check Rust installation
if ! command -v cargo &> /dev/null; then
    echo "❌ Rust not found"
    echo "Install from https://rustup.rs/"
    exit 1
fi

echo ""
echo "📦 Building PDF to CBZ Converter..."
cd "$(dirname "$0")/src-cli"

# Clean previous build
cargo clean --release 2>/dev/null || true

# Build in release mode
cargo build --release

BINARY="target/release/pdf-to-cbz"

if [ ! -f "$BINARY" ]; then
    echo "❌ Build failed"
    exit 1
fi

echo ""
echo "✅ Build successful!"
echo ""
echo "📊 Binary information:"
ls -lh "$BINARY"
file "$BINARY"

echo ""
echo "🚀 Installation options:"
echo ""
echo "Option 1: Copy to /usr/local/bin (requires sudo)"
echo "  sudo cp $BINARY /usr/local/bin/pdf-to-cbz"
echo "  sudo chmod +x /usr/local/bin/pdf-to-cbz"
echo ""
echo "Option 2: Copy to ~/.local/bin (no sudo needed)"
echo "  mkdir -p ~/.local/bin"
echo "  cp $BINARY ~/.local/bin/pdf-to-cbz"
echo "  # Make sure ~/.local/bin is in your PATH"
echo ""
echo "Option 3: Add current directory to PATH"
echo "  export PATH=\"\$PATH:$(pwd)/target/release\""
echo ""
echo "Testing the binary:"
echo "  $(pwd)/$BINARY --version"
echo ""
