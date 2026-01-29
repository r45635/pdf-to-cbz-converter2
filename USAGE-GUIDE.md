# PDF to CBZ Converter v3.0 - Usage Guide

Quick start guide for common scenarios.

## Installation

```bash
# Clone or download repository
git clone <repo>
cd pdf-to-cbz-converter2

# Run installation script (handles dependencies)
./install.sh

# Or manual build
cd src-cli
cargo build --release
./target/release/pdf-to-cbz --help
```

## Basic Usage

### Convert PDF to CBZ (Standard Quality)

```bash
pdf-to-cbz pdf-to-cbz mybook.pdf
# Creates: mybook.cbz (at 300 DPI)
```

### Convert PDF to CBZ (Specific Output Path)

```bash
pdf-to-cbz pdf-to-cbz mybook.pdf --output ~/Comics/mybook.cbz
# Creates: ~/Comics/mybook.cbz
```

### Convert PDF to CBZ (Custom DPI)

```bash
# Low quality, small file (fast)
pdf-to-cbz pdf-to-cbz mybook.pdf --dpi 150

# High quality, larger file (slow)
pdf-to-cbz pdf-to-cbz mybook.pdf --dpi 600

# Maximum quality for scanning
pdf-to-cbz pdf-to-cbz scan.pdf --dpi 1200
```

### Convert CBZ to PDF

```bash
# Basic conversion
pdf-to-cbz cbz-to-pdf comic.cbz

# With output path
pdf-to-cbz cbz-to-pdf comic.cbz --output comic.pdf
```

### Convert CBR to PDF

```bash
# RAR-based comics (requires unar)
pdf-to-cbz cbz-to-pdf comic.cbr --output comic.pdf
```

## DPI Reference

Choose DPI based on your use case:

| DPI | Quality | File Size | Use Case |
|-----|---------|-----------|----------|
| 72 | Screen | Very small | Screen reading only |
| 150 | Low | Small | Fast conversions, mobile |
| **300** | **Standard** | **Medium** | **Recommended default** |
| 600 | High | Large | Printed scans, magazines |
| 1200 | Lossless | Very large | High-quality originals |

## Batch Processing

### Convert Multiple PDFs (Shell)

```bash
# Convert all PDFs in a directory
for file in *.pdf; do
    pdf-to-cbz pdf-to-cbz "$file" --dpi 300
done

# Convert with custom output path
for file in *.pdf; do
    output="${file%.pdf}.cbz"
    pdf-to-cbz pdf-to-cbz "$file" --output "$output" --dpi 300
done
```

### Batch with Different DPI

```bash
# Low quality for quick preview
pdf-to-cbz pdf-to-cbz *.pdf --dpi 150

# High quality for archives (each file gets its own conversion)
for file in *.pdf; do
    pdf-to-cbz pdf-to-cbz "$file" --dpi 600
done
```

## Script Examples

### Create Comics Directory Structure

```bash
#!/bin/bash
# convert_comics.sh - Convert PDF books to CBZ

SOURCE_DIR="${1:-.}"
OUTPUT_DIR="${2:-.}"
DPI="${3:-300}"

echo "Converting PDFs from $SOURCE_DIR to CBZ at $DPI DPI"

mkdir -p "$OUTPUT_DIR"

for pdf in "$SOURCE_DIR"/*.pdf; do
    if [ -f "$pdf" ]; then
        filename=$(basename "$pdf" .pdf)
        output="$OUTPUT_DIR/$filename.cbz"

        echo "Converting: $pdf → $output"
        pdf-to-cbz pdf-to-cbz "$pdf" --output "$output" --dpi "$DPI"

        if [ $? -eq 0 ]; then
            echo "✓ Success: $output"
        else
            echo "✗ Failed: $pdf"
        fi
    fi
done

echo "Conversion complete!"
```

Usage:
```bash
chmod +x convert_comics.sh
./convert_comics.sh ./pdfs ./cbz_output 300
```

### Parallel Batch Processing (GNU Parallel)

```bash
# Convert multiple PDFs in parallel
ls *.pdf | parallel pdf-to-cbz pdf-to-cbz {} --dpi 300

# With progress
ls *.pdf | parallel --bar pdf-to-cbz pdf-to-cbz {} --dpi 300
```

### Convert and Organize

```bash
#!/bin/bash
# Script: organize_comics.sh

for cbz in *.cbz; do
    # Create directory based on filename prefix
    dir=$(echo "$cbz" | cut -d_ -f1)
    mkdir -p "$dir"
    mv "$cbz" "$dir/"
    echo "Organized: $cbz → $dir/"
done
```

## Advanced Scenarios

### Convert with File Size Optimization

```bash
#!/bin/bash
# Find optimal DPI for target file size

TARGET_SIZE=5  # 5 MB target

for dpi in 150 300 600; do
    output="test_dpi${dpi}.cbz"
    pdf-to-cbz pdf-to-cbz input.pdf --output "$output" --dpi "$dpi"

    size=$(ls -lh "$output" | awk '{print $5}')
    echo "DPI $dpi: $size"
done

# Choose the DPI that gives you the desired size
```

### Watch Directory and Convert

```bash
#!/bin/bash
# Watch for new PDFs and auto-convert

INPUT_DIR="./input"
OUTPUT_DIR="./output"

while true; do
    find "$INPUT_DIR" -name "*.pdf" -type f | while read pdf; do
        output="$OUTPUT_DIR/$(basename "$pdf" .pdf).cbz"

        if [ ! -f "$output" ]; then
            echo "Converting: $pdf"
            pdf-to-cbz pdf-to-cbz "$pdf" --output "$output" --dpi 300
            mv "$pdf" "$pdf.processed"
        fi
    done

    sleep 5
done
```

## Troubleshooting

### Error: "Failed to load PDF"

```
Converting PDF to CBZ: "myfile.pdf"
thread 'main' panicked at ... dlopen(libpdfium.dylib)
```

**Solution**: Install pdfium
```bash
# macOS
brew install pdfium

# Linux (Debian/Ubuntu)
sudo apt-get install libpdfium0-dev

# Linux (Fedora/RHEL)
sudo dnf install pdfium-devel
```

### Error: "No images found in archive"

**Cause**: CBR file, but `unar` not installed

**Solution**:
```bash
# macOS
brew install unar

# Linux
sudo apt-get install unar
```

### Error: "Failed to read PDF file"

**Cause**: File doesn't exist or no read permissions

**Solution**:
```bash
# Check file exists
ls -l myfile.pdf

# Check permissions
chmod 644 myfile.pdf
```

### Slow Conversion

**Cause**: High DPI setting

**Solution**: Reduce DPI
```bash
# Instead of:
pdf-to-cbz pdf-to-cbz huge.pdf --dpi 1200

# Try:
pdf-to-cbz pdf-to-cbz huge.pdf --dpi 300
```

## Performance Tips

1. **For Speed**: Use DPI 150-300
2. **For Quality**: Use DPI 600 (good balance)
3. **For Archives**: Keep originals, convert copies
4. **For Batch**: Use parallel processing (see examples above)

## Getting Help

```bash
# General help
pdf-to-cbz --help

# Subcommand help
pdf-to-cbz pdf-to-cbz --help
pdf-to-cbz cbz-to-pdf --help

# Version
pdf-to-cbz --version
```

## Integration with Other Tools

### Convert to Different Archive Format

```bash
# Create CBZ (default)
pdf-to-cbz pdf-to-cbz input.pdf --output temp.cbz

# CBZ is just a ZIP file - rename if needed
cp temp.cbz comic.zip
```

### Convert with ImageMagick Post-Processing

```bash
# Convert PDF to CBZ
pdf-to-cbz pdf-to-cbz input.pdf --output output.cbz

# Post-process images if needed
unzip output.cbz -d images/
mogrify -resize 800x600 images/*.png  # Resize if needed
cd images && zip -r ../output_resized.cbz * && cd ..
```

## Tips & Tricks

### Create Reading List

```bash
# Convert everything in a directory
ls *.pdf | while read f; do
    pdf-to-cbz pdf-to-cbz "$f"
done

# Then organize with your comic reader
```

### Backup Original Quality

```bash
# Keep high-quality version
pdf-to-cbz pdf-to-cbz book.pdf --output book_hq.cbz --dpi 600

# Create mobile-friendly version
pdf-to-cbz pdf-to-cbz book.pdf --output book_mobile.cbz --dpi 150
```

### Check Archive Content

```bash
# List images in CBZ (it's a ZIP file)
unzip -l comic.cbz

# Extract for inspection
unzip -l comic.cbz | head -20
```

## FAQ

**Q: Why is the CLI slower than the GUI?**
A: It's not! The CLI is faster because it has no startup overhead.

**Q: Can I convert CBR directly to CBZ?**
A: Yes, via PDF:
```bash
pdf-to-cbz cbz-to-pdf input.cbr --output temp.pdf
pdf-to-cbz pdf-to-cbz temp.pdf --output output.cbz
```

**Q: What's the maximum file size?**
A: Depends on your RAM. Typically works with files up to 1-2 GB.

**Q: Can I use this in Python/Node.js?**
A: Yes, use `subprocess`:
```python
import subprocess
subprocess.run(['pdf-to-cbz', 'pdf-to-cbz', 'input.pdf', '--dpi', '300'])
```

**Q: Is there a GUI version?**
A: No, but you can create shell scripts or use with your file manager.

## Next Steps

- Read [REFACTORING.md](REFACTORING.md) for technical details
- See [INSTALLATION.md](INSTALLATION.md) for system-specific setup
- Check [CLI-README.md](CLI-README.md) for full documentation
