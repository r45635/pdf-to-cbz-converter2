#!/bin/bash

# Script to download PDFium binaries for all platforms
# Uses bblanchon/pdfium-binaries releases

set -e

PDFIUM_VERSION="6721"  # Latest stable version
BASE_URL="https://github.com/bblanchon/pdfium-binaries/releases/download/chromium%2F${PDFIUM_VERSION}"

RESOURCES_DIR="$(cd "$(dirname "$0")/../resources/pdfium" && pwd)"

echo "📦 Downloading PDFium binaries v${PDFIUM_VERSION}..."
echo "Target directory: ${RESOURCES_DIR}"

cd /tmp

# Download Windows x64
if [ ! -f "${RESOURCES_DIR}/pdfium.dll" ]; then
    echo "⬇️  Downloading Windows x64..."
    curl -L "${BASE_URL}/pdfium-win-x64.tgz" -o pdfium-win-x64.tgz
    tar -xzf pdfium-win-x64.tgz
    cp bin/pdfium.dll "${RESOURCES_DIR}/"
    rm -rf bin include lib pdfium-win-x64.tgz
    echo "✅ Windows: pdfium.dll"
else
    echo "⏭️  Windows: pdfium.dll already exists"
fi

# Download Linux x64
if [ ! -f "${RESOURCES_DIR}/libpdfium.so" ]; then
    echo "⬇️  Downloading Linux x64..."
    curl -L "${BASE_URL}/pdfium-linux-x64.tgz" -o pdfium-linux-x64.tgz
    tar -xzf pdfium-linux-x64.tgz
    cp lib/libpdfium.so "${RESOURCES_DIR}/"
    rm -rf bin include lib pdfium-linux-x64.tgz
    echo "✅ Linux: libpdfium.so"
else
    echo "⏭️  Linux: libpdfium.so already exists"
fi

# macOS already exists
if [ -f "${RESOURCES_DIR}/libpdfium.dylib" ]; then
    echo "✅ macOS: libpdfium.dylib (already present)"
else
    echo "⚠️  macOS: libpdfium.dylib missing!"
fi

echo ""
echo "📋 Summary:"
ls -lh "${RESOURCES_DIR}"

echo ""
echo "✅ PDFium binaries ready!"
echo "🔧 Run 'git add resources/pdfium/' to commit them."
