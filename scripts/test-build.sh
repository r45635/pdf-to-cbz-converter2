#!/bin/bash

# Script to test Tauri build locally
# This will show where Tauri copies resources

set -e

echo "🔨 Building Tauri app locally..."
echo ""

cd "$(dirname "$0")/.."

# Build with verbose output
pnpm tauri build --verbose

echo ""
echo "📦 Build complete!"
echo ""
echo "🔍 Checking for PDFium libraries in bundle:"
echo ""

# Find the built app
if [[ "$OSTYPE" == "darwin"* ]]; then
    BUNDLE_PATH="src-tauri/target/release/bundle/macos/PDF to CBZ Converter.app"
    if [ -d "$BUNDLE_PATH" ]; then
        echo "macOS App Bundle: $BUNDLE_PATH"
        echo ""
        echo "Contents:"
        find "$BUNDLE_PATH" -name "*.dylib" -o -name "*.dll" -o -name "*.so"
        echo ""
        echo "Resources directory:"
        ls -la "$BUNDLE_PATH/Contents/Resources/" 2>/dev/null || echo "No Resources directory"
        echo ""
        echo "MacOS directory:"
        ls -la "$BUNDLE_PATH/Contents/MacOS/" 2>/dev/null || echo "No MacOS directory"
    fi
elif [[ "$OSTYPE" == "msys" ]] || [[ "$OSTYPE" == "win32" ]]; then
    BUNDLE_PATH="src-tauri/target/release"
    if [ -d "$BUNDLE_PATH" ]; then
        echo "Windows build directory: $BUNDLE_PATH"
        echo ""
        find "$BUNDLE_PATH" -name "*.dll" -maxdepth 2
    fi
else
    BUNDLE_PATH="src-tauri/target/release"
    if [ -d "$BUNDLE_PATH" ]; then
        echo "Linux build directory: $BUNDLE_PATH"
        echo ""
        find "$BUNDLE_PATH" -name "*.so" -maxdepth 2
    fi
fi

echo ""
echo "✅ Check complete!"
