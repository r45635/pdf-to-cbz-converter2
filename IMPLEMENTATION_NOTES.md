# PDF-to-CBZ Implementation: Stabilization & Diagnostics (3 Commits)

## Overview
This document describes the 3-commit implementation sequence to stabilize PDFium binding, implement direct image extraction with real coverage metrics, and establish regression testing with smoke tests.

---

## COMMIT 1: PDFium Stabilisation + Diagnostic Logging

### Objectives
- Single unified PDFium loader function for all code paths (CLI + Tauri)
- Comprehensive startup logging
- Page-1 diagnostic mode with structural analysis

### Changes

**New File: `src-cli/pdfium_loader.rs`**
```rust
pub fn bind_pdfium() -> Result<Pdfium, PdfiumError>
```
- Load library from `PDFIUM_LIB_DIR` env var if set
- Fallback to `./libpdfium.dylib` (macOS dev) or `./pdfium.dll` (Windows) or `./libpdfium.so` (Linux)
- Last resort: system library
- Log target architecture (arm64/x86_64) and OS at initialization

**Modified Files:**
- `src-cli/main.rs`:
  - Add `mod pdfium_loader`
  - Replace `Pdfium::default()` with `pdfium_loader::bind_pdfium()` in `self_check`
- `src-cli/pdf.rs`:
  - Replace all 3 occurrences of `Pdfium::default()`
  - Add import: `use crate::pdfium_loader`

**Enhanced Diagnostics (self_check command):**
```
PAGE 1 BOX ANALYSIS
- MediaBox, CropBox, TrimBox dimensions
- Page width/height

OBJECT DETECTION & BOUNDS ANALYSIS
- Iterate all page objects
- Compute UNION bounds of all objects
- Count by type: images, text, paths, forms
- Display coverage % vs page area
- Warn if < 25% coverage (possible offset/thumbnail issue)
```

**Build Status:** ✅ Clean compilation (2 warnings on unused code)

---

## COMMIT 2: Direct Extract Mode + Real Coverage Metrics

### Objectives
- Primary fast path for comic PDFs: direct image extraction
- Recursive traversal of page objects
- Real coverage computation (with CTM conceptually, though pdfium_render doesn't fully expose it)
- Smart fallback to rendering

### Changes

**New File: `src-cli/direct_extract.rs`**
```rust
pub struct ImageCandidate {
    pub object_index: usize,
    pub bounds: (f32, f32, f32, f32),
    pub coverage: f64,           // 0.0 to 1.0
    pub can_extract_raw: bool,
}

pub fn find_best_image_candidate(page: &PdfPage)
    -> Result<(Option<ImageCandidate>, (f32,f32,f32,f32))>

pub fn extract_image_bytes(page: &PdfPage, object_index: usize) -> Result<Vec<u8>>

pub fn log_page_diagnostic(page_num, candidate_opt, crop_bounds, fallback)
```

**Algorithm:**
1. Get CropBox (or MediaBox as fallback)
2. Iterate page.objects() to find image candidates
3. For each image: compute bounds and coverage = obj_area / cropbox_area
4. Select candidate with highest coverage if >= 60% (MIN_COVERAGE_FOR_DIRECT_EXTRACT)
5. Extraction:
   - Try `image_obj.get_raw_bitmap()` → encode as PNG (lossless)
   - If extraction fails → fallback to full-page render

**Modified Files:**
- `src-cli/main.rs`: Add `mod direct_extract`
- `src-cli/pdf.rs`:
  - Import `direct_extract`
  - Rewrite `extract_images_lossless()` to use new pipeline
  - Output PNG instead of JPEG

**Thresholds:**
```
MIN_COVERAGE_FOR_DIRECT_EXTRACT = 0.60 (60%)
```

**Build Status:** ✅ Clean (2 warnings on unused code)

---

## COMMIT 3: Lossless Redefinition + Smoke Test Framework

### Objectives
- Correct lossless definition: PNG at same DPI (not 72 DPI)
- Single-page regression testing with metrics
- Anti-regression thresholds: white_ratio and bbox_coverage

### Changes

**New CLI Command: `smoke_render`**
```bash
cargo run -- smoke_render <PDF> --page 1 --dpi 200
```

**Options:**
```
  --page <N>              Page to test (1-indexed, default: 1)
  --dpi <DPI>             DPI for render (default: 200)
  -o, --output <PATH>     Output PNG file (default: _smoke_page.png)
  --max-white-ratio <F>   Threshold (default: 0.95)
  --min-bbox-coverage <F> Threshold (default: 0.30)
```

**Metrics Reported:**
```
white_ratio:    % of near-white pixels (threshold: max_white_ratio)
                Fails if white_ratio > max_white_ratio

bbox_coverage:  union bounds area / page area (0.0-1.0)
                Fails if bbox_coverage < min_bbox_coverage

non-white:      percentage of non-white content (informational)
```

**Default Thresholds:**
```
max_white_ratio = 0.95    (fail if 95%+ white)
min_bbox_coverage = 0.30  (fail if < 30% content)
```

**Modified Command: `pdf-to-cbz`**
```bash
# Lossless mode: PNG at specified DPI (not 72)
cargo run -- pdf-to-cbz input.pdf --dpi 200 --lossless --max-pages 10

# Lossy mode: JPEG at DPI with quality
cargo run -- pdf-to-cbz input.pdf --dpi 300 --quality 85 --max-pages 10
```

**New Functions in `src-cli/pdf.rs`:**
```rust
pub fn extract_images_lossless_at_dpi(
    pdf_data: &[u8],
    dpi: u32,
    max_pages: u32
) -> Result<Vec<(String, Vec<u8>)>>
```
- Renders PNG at **same DPI** as lossy mode (DPI-agnostic)
- Supports max_pages limit
- Uses direct extract pipeline with fallback

**Key Semantic Changes:**
```
lossless=true  →  PNG at [dpi] DPI  (was: 72 DPI JPEG)
lossy=false    →  JPEG at [dpi] DPI with quality param
```

**Render Section Output Example:**
```
───────────────────────────────────────────────────────────
PAGE 1 BOX ANALYSIS
───────────────────────────────────────────────────────────
MediaBox: left=0.00, bottom=0.00, right=612.00, top=792.00 (8.5" x 11")
CropBox: NOT PRESENT
TrimBox: NOT PRESENT

page.width/height: 612.00 x 792.00 pt

───────────────────────────────────────────────────────────
OBJECT DETECTION & BOUNDS ANALYSIS
───────────────────────────────────────────────────────────
Total objects in page: 5

Object[0] type=IMAGE  bounds=(50.0,100.0,550.0,700.0) size=(500.0x600.0)
Object[1] type=TEXT   bounds=(10.0,10.0,600.0,90.0) size=(590.0x80.0)
...

Object summary:
  Images: 1
  Text:   1
  Paths:  0
  Forms:  0
  Other:  3

Content Bounding Box (union of all objects):
  left=10.00, bottom=10.00, right=600.00, top=700.00
  width=590.00 pt, height=690.00 pt
  coverage=0.756 (75.6% of page area)

Pipeline: Direct Extract (would attempt image extraction)

───────────────────────────────────────────────────────────
RENDERING PAGE 1
───────────────────────────────────────────────────────────
Using box: MediaBox
Box dimensions: 612.00 x 792.00 pt
Scale factor: 2.778 (DPI 200 / 72)
Target size: 1700 x 2200 px
Rendered image: 1700 x 2200 px

───────────────────────────────────────────────────────────
SANITY CHECK
───────────────────────────────────────────────────────────
Non-white pixels (64x64 thumbnail): 3456/4096 (84.4%)
White ratio: 0.156 (threshold: 0.950)
✓ White ratio: 0.156 <= 0.950 (OK)
✓ BBox coverage: 0.756 (75.6%) >= 0.300 (30.0%) (OK)

───────────────────────────────────────────────────────────
SMOKE TEST SUMMARY
───────────────────────────────────────────────────────────
Page:                1
Pipeline:            Direct Extract (fallback to render)
Rendered size:       1700 x 2200 px
Output file:         850.5 KB

Metrics:
  white_ratio:       0.156 / 0.950 ✓
  bbox_coverage:     0.756 / 0.300 ✓
  non-white pixels:  84.4% (OK)

Status:              ✓✓✓ PASS (all checks passed)
═══════════════════════════════════════════════════════════
```

**Build Status:** ✅ Clean (3 warnings on unused code)

---

## Command Examples

### 1. Smoke Test (Single Page Validation)
```bash
cd src-cli

# Test page 1 at 200 DPI with default thresholds
cargo run --release -- smoke_render "/path/to/Adler.pdf" --page 1 --dpi 200

# Fail if white_ratio > 0.90 OR bbox_coverage < 0.40
cargo run --release -- smoke_render "/path/to/Adler.pdf" \
  --page 1 --dpi 200 \
  --max-white-ratio 0.90 \
  --min-bbox-coverage 0.40
```

### 2. Full Conversion (10 Pages)
```bash
# PNG Lossless (direct extract or render at 200 DPI as PNG)
cargo run --release -- pdf-to-cbz "/path/to/Adler.pdf" \
  --lossless --dpi 200 --max-pages 10

# JPEG Lossy (render at 200 DPI with quality 85)
cargo run --release -- pdf-to-cbz "/path/to/Adler.pdf" \
  --dpi 200 --quality 85 --max-pages 10
```

### 3. CLI Diagnostic (All Pages Analysis)
```bash
# The smoke_render command at page 1 shows full structural diagnostics
cargo run -- smoke_render "/path/to/Adler.pdf" --page 1 --dpi 200
```

---

## Constants & Thresholds

**In `src-cli/direct_extract.rs`:**
```rust
const MIN_COVERAGE_FOR_DIRECT_EXTRACT: f64 = 0.60; // 60%
```

**In smoke_render defaults:**
```
max_white_ratio:    0.95  (95% white = fail)
min_bbox_coverage:  0.30  (< 30% content = fail)
```

---

## Implementation Architecture

```
CLI Entry: main()
│
├─ convert_pdf_to_cbz()
│  ├─ bind_pdfium() [pdfium_loader]
│  ├─ if lossless:
│  │  └─ extract_images_lossless_at_dpi() [pdf.rs]
│  │     ├─ find_best_image_candidate() [direct_extract]
│  │     ├─ extract_image_bytes() → PNG [direct_extract]
│  │     └─ fallback: render + PNG encode
│  └─ if !lossless:
│     └─ convert_pdf_to_images_parallel() → JPEG [pdf.rs]
│
└─ smoke_render()
   ├─ bind_pdfium() [pdfium_loader]
   ├─ Page 1 structural diagnostics
   ├─ Object iteration + bounds analysis
   ├─ Render at specified DPI
   ├─ Compute white_ratio & bbox_coverage
   └─ Exit non-zero if thresholds exceeded
```

---

## Testing Strategy

### 1. Unit Smoke Tests
```bash
cargo run -- smoke_render <PDF> --page 1 --dpi 200
# Should show:
# - ✓ PASS if white_ratio <= 0.95 AND bbox_coverage >= 0.30
# - ❌ FAIL otherwise
```

### 2. Full Conversions (With Limits)
```bash
# Lossless mode (PNG at 200 DPI)
cargo run -- pdf-to-cbz <PDF> --dpi 200 --lossless --max-pages 10

# Lossy mode (JPEG at 200 DPI, quality 85)
cargo run -- pdf-to-cbz <PDF> --dpi 200 --quality 85 --max-pages 10
```

### 3. Visual Inspection
- Compare output images page 1 and page 10
- Check file sizes: lossless PNG vs lossy JPEG at same DPI
- Verify PNG lossless files are larger (expected)

---

## Debugging

**Enable detailed logging:**
```bash
# stderr shows [PDFIUM] and [EXTRACT] diagnostics
cargo run -- smoke_render <PDF> --page 1 --dpi 200 2>&1 | head -50
```

**PDFium loader diagnostics (stderr):**
```
═══════════════════════════════════════════════════════════
PDFIUM INITIALIZATION
═══════════════════════════════════════════════════════════
[SYSTEM] Architecture: arm64
[SYSTEM] OS: macos
[SYSTEM] PDFIUM_LIB_DIR env var: not set
[PDFIUM] Attempting to load library from: /path/to/libpdfium.dylib
[PDFIUM] Successfully loaded library
═══════════════════════════════════════════════════════════
```

---

## Known Limitations & TODOs

1. **CTM Transformations**: pdfium_render does not fully expose the Current Transformation Matrix for objects. Bounds computation is post-transformation but we cannot extract the matrix itself.

2. **Form XObject Recursion**: Traversing nested Form XObjects not yet implemented (pdfium_render API limitation).

3. **JPEG Extraction**: Direct JPEG byte extraction from PDF objects is not exposed by pdfium_render. We render → PNG instead.

4. **Parallel PNG Encoding**: `extract_images_lossless_at_dpi` is sequential (PNG encoding not parallelized). Could use rayon for scaling.

---

## Files Modified

```
src-cli/
├── main.rs              (Refactored CLI commands, smoke_render, PDF-to-CBZ enhancements)
├── pdf.rs               (New extract_images_lossless_at_dpi, updated extract_images_lossless)
├── pdfium_loader.rs     (NEW: bind_pdfium() function)
└── direct_extract.rs    (NEW: find_best_image_candidate, extract_image_bytes, diagnostics)
```

**Commits:**
1. `7de498b` - PROMPT 1: Stabilise PDFium + diagnostics structuraux
2. `2ad2f30` - PROMPT 2: Extraction directe récursive + coverage réelle
3. `2e49aed` - PROMPT 3: Lossless correct + smoke tests avec regression checks

---

## Summary

This 3-commit sequence implements:
1. **Unified PDFium binding** with comprehensive diagnostics
2. **Direct image extraction pipeline** with real coverage metrics (60% threshold)
3. **Regression testing framework** via smoke_render command with dual thresholds (white_ratio ≤ 0.95, bbox_coverage ≥ 0.30)

All code compiles cleanly (release build tested). The CLI now provides both production conversion and diagnostic testing capabilities for validating PDF rendering quality.
