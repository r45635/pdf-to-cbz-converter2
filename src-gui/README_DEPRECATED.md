# ⚠️ DEPRECATED - DO NOT USE

This directory (`src-gui/`) contains a **legacy GUI implementation** that is **no longer maintained**.

## Why deprecated?

- Uses minimal HTML/JS + calls CLI via `.output()` which blocks and cannot show progress
- May fail due to missing file paths in WebView
- No reliable logging or error handling
- Cannot provide real-time feedback to users

## What to use instead?

**✅ Use the modern GUI:**

```bash
# From repository root:
npm install
npm run tauri:dev
```

The modern implementation is in:
- Frontend: `src/` (React + TypeScript + Vite)
- Backend: `src-tauri/` (Rust + Tauri with proper IPC)

## Features of modern GUI

- Real-time progress events via Tauri IPC
- Proper error handling and logging
- Multi-file batch processing
- Live conversion feedback
- Professional UI with progress bars

---

**This directory is kept for reference only. Do not modify or use it.**
