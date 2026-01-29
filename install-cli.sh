#!/bin/bash

# Installation script for PDF to CBZ Converter CLI
# Downloads the required Pdfium library for the current platform

set -e

# Detect platform
OS="$(uname -s)"
ARCH="$(uname -m)"

# Pdfium version
PDFIUM_VERSION="chromium%2F7543"

echo "🔍 Detecting platform..."
echo "OS: $OS"
echo "Architecture: $ARCH"

# Determine download URL
case "$OS" in
    Darwin)
        if [ "$ARCH" = "arm64" ]; then
            PDFIUM_URL="https://github.com/bblanchon/pdfium-binaries/releases/download/${PDFIUM_VERSION}/pdfium-mac-arm64.tgz"
            LIB_NAME="libpdfium.dylib"
        else
            PDFIUM_URL="https://github.com/bblanchon/pdfium-binaries/releases/download/${PDFIUM_VERSION}/pdfium-mac-x64.tgz"
            LIB_NAME="libpdfium.dylib"
        fi
        ;;
    Linux)
        if [ "$ARCH" = "x86_64" ]; then
            PDFIUM_URL="https://github.com/bblanchon/pdfium-binaries/releases/download/${PDFIUM_VERSION}/pdfium-linux-x64.tgz"
            LIB_NAME="libpdfium.so"
        elif [ "$ARCH" = "aarch64" ]; then
            PDFIUM_URL="https://github.com/bblanchon/pdfium-binaries/releases/download/${PDFIUM_VERSION}/pdfium-linux-arm64.tgz"
            LIB_NAME="libpdfium.so"
        else
            echo "❌ Unsupported Linux architecture: $ARCH"
            exit 1
        fi
        ;;
    MINGW*|MSYS*|CYGWIN*)
        if [ "$ARCH" = "x86_64" ]; then
            PDFIUM_URL="https://github.com/bblanchon/pdfium-binaries/releases/download/${PDFIUM_VERSION}/pdfium-win-x64.tgz"
            LIB_NAME="pdfium.dll"
        else
            echo "❌ Unsupported Windows architecture: $ARCH"
            exit 1
        fi
        ;;
    *)
        echo "❌ Unsupported operating system: $OS"
        exit 1
        ;;
esac

echo ""
echo "📦 Downloading Pdfium library..."
echo "URL: $PDFIUM_URL"

# Download
if command -v curl &> /dev/null; then
    curl -L -o pdfium.tgz "$PDFIUM_URL"
elif command -v wget &> /dev/null; then
    wget -O pdfium.tgz "$PDFIUM_URL"
else
    echo "❌ Neither curl nor wget is available. Please install one of them."
    exit 1
fi

echo "📂 Extracting archive..."
tar -xzf pdfium.tgz

echo "📋 Copying library to project root..."
if [ -f "lib/$LIB_NAME" ]; then
    cp "lib/$LIB_NAME" .
    echo "✅ $LIB_NAME copied to project root"
else
    echo "❌ Library file not found in extracted archive"
    exit 1
fi

echo "🧹 Cleaning up..."
rm -f pdfium.tgz

echo ""
echo "✅ Installation complete!"
echo ""
echo "📝 Next steps:"
echo "1. Build the CLI:"
echo "   cd src-cli && cargo build --release"
echo ""
echo "2. Run the CLI:"
echo "   ./src-cli/target/release/pdf-to-cbz --help"
echo ""

# Install unar if on macOS and Homebrew is available
if [ "$OS" = "Darwin" ]; then
    if command -v brew &> /dev/null; then
        if ! command -v unar &> /dev/null; then
            echo "📦 Installing unar for CBR support..."
            brew install unar
        else
            echo "✅ unar is already installed"
        fi
    else
        echo "⚠️  For CBR support, install unar manually: brew install unar"
    fi
elif [ "$OS" = "Linux" ]; then
    echo "⚠️  For CBR support, install unar: sudo apt-get install unar (Ubuntu/Debian)"
fi
