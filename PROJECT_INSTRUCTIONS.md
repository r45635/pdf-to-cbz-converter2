# Project Instructions - PDF to CBZ Converter

## Architecture Overview

This repository contains a **modern Tauri + React GUI** for PDF ↔ CBZ conversion.

### Directory Structure

```
.
├── src/                    # ✅ Modern React frontend (TypeScript + Vite)
├── src-tauri/              # ✅ Modern Rust backend (Tauri IPC)
├── src-cli/                # ✅ Standalone CLI binary
├── src-gui/                # ❌ DEPRECATED - Legacy GUI (do not use)
└── scripts/                # Build and maintenance scripts
```

## Running the Application

### Development Mode

```bash
# From repository root:
npm install
npm run tauri:dev
```

This will:
- Start Vite dev server on http://localhost:1420
- Launch Tauri window with hot-reload
- Show real-time conversion progress
- Display detailed logs

### Production Build

```bash
npm run tauri:build
```

Output: `src-tauri/target/release/bundle/`

## Cleaning the Project

### Clean build artifacts (keep node_modules)
```bash
npm run clean
```

### Clean everything including node_modules
```bash
npm run clean:all
```

### Clean only Rust targets
```bash
npm run clean:rust
```

### Clean only JavaScript build outputs
```bash
npm run clean:js
```

## Progress Events Architecture

The modern GUI uses **Tauri IPC events** for real-time progress updates:

### Backend (Rust)
File: `src-tauri/src/commands/conversion.rs`

```rust
// Emits progress events during conversion
window.emit("conversion-progress", serde_json::json!({
    "percentage": 50,
    "message": "Processing page 135/270..."
}));
```

### Frontend (React)
File: `src/pages/page.tsx`

```typescript
// Listens to progress events
useEffect(() => {
  const unlisten = listen('conversion-progress', (event: any) => {
    const { percentage, message } = event.payload;
    // Update UI with progress
  });
  return () => { unlisten.then(fn => fn()); };
}, []);
```

## Legacy GUI Warning

The `src-gui/` directory is **DEPRECATED** and should not be used:

- ❌ Uses minimal HTML/JS with blocking CLI calls
- ❌ Cannot show real-time progress
- ❌ Poor error handling
- ❌ No proper logging

**Always use `npm run tauri:dev` instead.**

See `src-gui/README_DEPRECATED.md` for details.

## Scripts

All scripts are in `package.json`:

| Script | Purpose |
|--------|---------|
| `npm run dev` | Start Vite dev server only |
| `npm run tauri:dev` | Start full Tauri app (recommended) |
| `npm run tauri:build` | Build production app |
| `npm run clean` | Clean build artifacts |
| `npm run clean:all` | Clean everything (needs `npm install` after) |

## Preflight Checks

The project includes automatic preflight checks that run before `tauri:dev` and `tauri:build`:

- ✅ Prevents accidental use of legacy GUI
- ✅ Ensures running from correct directory
- ✅ Shows helpful error messages if misconfigured

## Development Workflow

1. **Start development:**
   ```bash
   npm install
   npm run tauri:dev
   ```

2. **Make changes:**
   - Frontend: Edit files in `src/`
   - Backend: Edit files in `src-tauri/src/`
   - Hot reload applies automatically

3. **View logs:**
   - Frontend: Browser DevTools console
   - Backend: Terminal output (prefixed with `[GUI]`, `[PARALLEL]`, etc.)

4. **Build production:**
   ```bash
   npm run tauri:build
   ```

5. **Clean when needed:**
   ```bash
   npm run clean
   ```

## Common Issues

### Issue: "Legacy GUI is deprecated" error
**Solution:** You're trying to run from `src-gui/`. Use `npm run tauri:dev` from repository root instead.

### Issue: Progress bar not moving
**Solution:** Check that:
- Frontend is listening to `conversion-progress` events (see `src/pages/page.tsx`)
- Backend is emitting events (see `src-tauri/src/commands/conversion.rs`)

### Issue: Build artifacts taking up space
**Solution:** Run `npm run clean` or `npm run clean:all`

## Multi-Threading Performance

The backend uses **Rayon** for parallel processing:

- Sequential PDF rendering (pdfium limitation)
- Parallel image scaling and JPEG encoding
- ~2-3x speedup on multi-core systems

Example: 270-page PDF at 200 DPI
- Sequential: ~60 seconds
- Parallel: ~25 seconds

## Code Quality

- TypeScript strict mode enabled
- Rust warnings suppressed for unused imports (in progress)
- All core functionality tested with real-world PDFs (850MB, 270 pages)

## Next Steps

- [ ] Wire progress events to UI progress bar display
- [ ] Add error recovery for IPC timeouts
- [ ] Implement chunked file transfer for large CBZ files (>30MB)
- [ ] Complete removal of `src-gui/` after migration confirmation
