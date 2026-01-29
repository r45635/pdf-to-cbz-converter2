# PDF to CBZ Converter - GUI v3.0

> Lightweight, user-friendly GUI that wraps the optimized CLI

## 🎯 Architecture

```
GUI (Tauri - 50MB)
  ↓
  ├─ HTML/CSS/JS (simple interface)
  └─ Calls subprocess → CLI binary (src-cli)
       └─ 495-line optimized converter
```

### Benefits
- ✅ **User-friendly**: Drag-drop, buttons, visual feedback
- ✅ **Lightweight**: ~50-60 MB (just Tauri + wrappers)
- ✅ **Efficient**: Calls optimized CLI, no processing overhead
- ✅ **Simple**: No React, no complex frameworks

---

## 🚀 Installation & Build

### Prerequisites
- Rust 1.70+
- Build tools (Xcode, MSVC, build-essential)
- libpdfium (installed via ./install.sh from src-cli)

### Build
```bash
cd src-gui
cargo build --release
```

**Binary location:** `target/release/pdf-to-cbz-gui` (~50-60 MB)

---

## 📋 Features

### User Interface
- **Drag & Drop Zone** - Drop PDF or CBZ files
- **Mode Selector** - Switch between PDF→CBZ and CBZ→PDF
- **File Selection** - Click to browse
- **DPI Selector** (PDF mode) - Choose quality level
- **Progress Bar** - Real-time conversion status
- **Status Messages** - Success/error feedback

### Supported Conversions
- ✅ **PDF → CBZ** with custom DPI
- ✅ **CBZ → PDF**
- ✅ **CBR → PDF** (requires unar)

### Quality Presets (PDF→CBZ)
- **Low** (150 DPI) - Fast, small file
- **Standard** (300 DPI) - Balanced (default)
- **High** (600 DPI) - Better quality
- **Maximum** (1200 DPI) - Lossless
- **Custom** - Enter any DPI (72-1200)

---

## 💻 Usage

### Running
```bash
./target/release/pdf-to-cbz-gui
```

### Converting PDF to CBZ
1. Select "PDF → CBZ" mode
2. Drag PDF file to window OR click "Select File"
3. Choose quality (DPI)
4. Click "Convert"
5. File saves to Downloads folder

### Converting CBZ/CBR to PDF
1. Select "CBZ/CBR → PDF" mode
2. Drag CBZ/CBR file to window OR click "Select File"
3. Click "Convert"
4. File saves to Downloads folder

---

## 🛠️ Technical Details

### Files Structure
```
src-gui/
├── src/
│   └── main.rs              # Tauri commands (subprocess calls)
├── index.html               # Simple HTML interface
├── app.js                   # Client-side logic (no framework)
├── style.css                # Styling (responsive design)
├── tauri.conf.json          # Tauri configuration
└── Cargo.toml               # Rust dependencies (minimal)
```

### How It Works
1. **GUI receives file** → User drops or selects file
2. **Validates input** → Checks file extension and size
3. **Shows progress** → Displays progress bar
4. **Calls CLI subprocess** → Invokes `/pdf-to-cbz pdf-to-cbz ...`
5. **Captures result** → Displays success/error message
6. **Saves output** → File automatically saved to Downloads

### Tauri Commands
```rust
convert_pdf_to_cbz(input_path, output_path, dpi)
convert_cbz_to_pdf(input_path, output_path)
get_save_path(filename)
open_file_dialog()
```

---

## 📊 Size Comparison

| Component | Size | Purpose |
|-----------|------|---------|
| CLI binary | 5 MB | Core converter (src-cli) |
| Tauri framework | 40-50 MB | Window + system integration |
| GUI assets | <1 MB | HTML/CSS/JS |
| **Total** | **~50-60 MB** | Full application |

vs. v2.0 Tauri+React: 150+ MB

---

## 🔧 Dependencies (Minimal)

### Tauri
- `tauri` - Desktop framework
- `tauri-plugin-dialog` - File dialogs
- `tauri-plugin-fs` - File operations

### Utilities
- `serde` + `serde_json` - Serialization

**Total: 5 dependencies** (vs 16+ in v2.0)

---

## ⚡ Performance

### Startup
- v2.0 Tauri+React: 2-3 seconds
- v3.0 GUI: ~1 second (just Tauri)
- Improvement: 2-3x faster

### Conversion
- Same speed as CLI (calls same binary)
- No GUI processing overhead
- Progress bar updates every second

---

## 🎨 UI/UX

### Design
- **Modern gradient background** - Purple to violet
- **Clean white container** - Good contrast
- **Responsive layout** - Works on mobile displays
- **Color-coded messages** - Green (success), Red (error), Blue (info)

### Interactions
- **Hover effects** - Visual feedback
- **Drag-drop highlighting** - Clear drop zone
- **Progress bar** - Real-time feedback
- **Status messages** - Clear what's happening

---

## 🐛 Troubleshooting

### "CLI binary not found"
Make sure the CLI binary is installed:
```bash
cd src-cli
cargo build --release
cp target/release/pdf-to-cbz /usr/local/bin/
```

### "File conversion fails"
Check system requirements:
- libpdfium installed: `brew install pdfium` (macOS)
- For CBR: `brew install unar` (macOS)

### "GUI won't start"
Verify Rust installation:
```bash
rustc --version
cargo --version
```

---

## 📦 Distribution

### macOS
```bash
cargo build --release
# Creates: target/release/pdf-to-cbz-gui
```

### Linux
```bash
cargo build --release
# Creates: target/release/pdf-to-cbz-gui (AppImage coming)
```

### Windows
```bash
cargo build --release
# Creates: target/release/pdf-to-cbz-gui.exe
```

---

## 🔄 How It Works: Flow Chart

```
User Interface (HTML/CSS/JS)
        ↓
File Selection (Drag-drop or Browse)
        ↓
Validation (Check file type)
        ↓
Show Options (DPI for PDF mode)
        ↓
Convert Button Click
        ↓
Subprocess Call (spawn: pdf-to-cbz CLI)
        ↓
Progress Display
        ↓
Result Message (success/error)
        ↓
Save to Downloads
```

---

## 📝 Code Examples

### Converting from Python
```python
import subprocess
import json

# Call the GUI in headless mode (not implemented, but CLI works)
result = subprocess.run(['pdf-to-cbz', 'pdf-to-cbz', 'input.pdf', '--dpi', '300'])
```

### Converting from Shell
```bash
# Run GUI
./target/release/pdf-to-cbz-gui &

# Or use CLI directly
pdf-to-cbz pdf-to-cbz input.pdf --dpi 300
```

---

## 🚀 Roadmap

### Completed ✅
- [x] Basic GUI with drag-drop
- [x] Mode selector (PDF/CBZ)
- [x] DPI selector
- [x] File selection dialog
- [x] Progress bar
- [x] Status messages

### Possible Future
- [ ] Batch conversion UI
- [ ] Conversion history
- [ ] Custom presets
- [ ] Theme selector
- [ ] Keyboard shortcuts

---

## 💡 Tips

### For Best Performance
1. Install CLI first: `./install.sh` in src-cli
2. GUI automatically finds CLI binary
3. Keep source PDF/CBZ files

### For Better Quality
- Use 600 DPI for magazines/scans
- Use 300 DPI for standard PDFs
- Custom DPI for specific needs

### For Batch Processing
- Use the CLI directly for multiple files
- Or create shell script wrapper

---

## 🤝 Contributing

The GUI code is simple and well-organized. To modify:

1. **UI Changes** → Edit `index.html` and `style.css`
2. **Logic Changes** → Edit `app.js`
3. **Tauri Commands** → Edit `src/main.rs`
4. **Styling** → Edit `style.css`

---

## 📄 License

MIT License - Same as main project

---

**v3.0 GUI** ✅ | **Simple** 🎨 | **Fast** ⚡ | **User-Friendly** 👍
