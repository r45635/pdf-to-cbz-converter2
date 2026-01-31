#!/bin/bash
# Script to run Tauri dev with PDFium library configured
cd "$(dirname "$0")"

# Ensure PDFium library is in project root for development
if [ ! -f "libpdfium.dylib" ] && [ -f "resources/pdfium/libpdfium.dylib" ]; then
    echo "📦 Copying PDFium library to project root for development..."
    cp resources/pdfium/libpdfium.dylib .
fi

# Set environment for development
export DYLD_LIBRARY_PATH="$(pwd):/usr/local/lib:$DYLD_LIBRARY_PATH"
export DYLD_FALLBACK_LIBRARY_PATH="$(pwd):/usr/local/lib:/usr/lib"

echo "🚀 Starting Tauri dev with PDFium library configured..."
echo "   DYLD_LIBRARY_PATH=$DYLD_LIBRARY_PATH"
echo "   PDFium library: $(ls -la libpdfium.dylib 2>/dev/null || echo 'NOT FOUND')"

npm run tauri:dev
