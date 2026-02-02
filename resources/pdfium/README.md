# PDFium Libraries

This directory contains the PDFium native libraries required for PDF processing.

## Required Files

- **macOS**: `libpdfium.dylib` ✅ (included)
- **Windows**: `pdfium.dll` ✅ (included)
- **Linux**: `libpdfium.so` ✅ (included)

## Download PDFium Libraries

You can download pre-compiled PDFium libraries from:

### Official Google PDFium
https://github.com/bblanchon/pdfium-binaries/releases

Download the appropriate version for each platform:
- **Windows x64**: `pdfium-win-x64.tgz`
- **Linux x64**: `pdfium-linux-x64.tgz`
- **macOS universal**: `pdfium-mac-universal.tgz` (already included)

### Installation

1. Download the appropriate archive for each platform
2. Extract the library file:
   - Windows: Extract `bin/pdfium.dll`
   - Linux: Extract `lib/libpdfium.so`
3. Copy the file to this `resources/pdfium/` directory

## Current Status

- ✅ macOS: `libpdfium.dylib` (5.5 MB)
- ✅ Windows: `pdfium.dll` (5.8 MB)
- ✅ Linux: `libpdfium.so` (5.7 MB)

## Note

These files are platform-specific native libraries and must be included in the repository for cross-platform builds to work correctly.
