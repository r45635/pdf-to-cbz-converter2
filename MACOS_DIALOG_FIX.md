# macOS Dialog Crash Fix

## Problem

When launching the app in dev mode, you may encounter this error:

```
thread 'main' panicked at NSOpenPanel.rs:127:5:
unexpected NULL returned from +[NSOpenPanel openPanel]
```

This is a known issue with Tauri 2.x on macOS Sequoia (15.x) and Sonoma (14.x).

## Root Cause

macOS sandboxing prevents the app from opening system dialogs without proper entitlements.

## Solutions

### Option 1: Grant Full Disk Access (Recommended for Development)

1. Open **System Settings** → **Privacy & Security** → **Full Disk Access**
2. Click the **+** button
3. Navigate to and add your terminal app (e.g., Terminal.app, iTerm.app, or VS Code)
4. Restart the terminal
5. Run `npm run tauri:dev` again

### Option 2: Use Production Build (No crash)

```bash
npm run tauri:build
open src-tauri/target/release/bundle/macos/PDF\ to\ CBZ\ Converter.app
```

Production builds have proper code signing and entitlements.

### Option 3: Disable Sandbox (Not Recommended)

Edit `src-tauri/tauri.conf.json`:

```json
{
  "app": {
    "macOS": {
      "sandbox": false
    }
  }
}
```

**Warning:** This reduces security and is not recommended for distribution.

## Verification

After applying a fix, the app should launch without crashes. Test by:

1. Launch: `npm run tauri:dev`
2. Click "Select PDF" button
3. File dialog should open without crashes

## macOS Version Compatibility

- ✅ macOS 10.13+ (High Sierra) - Minimum supported
- ⚠️ macOS 14.x (Sonoma) - May require Full Disk Access
- ⚠️ macOS 15.x (Sequoia) - Requires Full Disk Access or production build

## Additional Notes

- This issue **only affects development mode** (`npm run tauri:dev`)
- Production builds (`npm run tauri:build`) work correctly
- The crash occurs when trying to open `NSOpenPanel` (file picker)
- Related Tauri issue: https://github.com/tauri-apps/tauri/issues/8631

## Alternative: Use Drag & Drop

If dialogs continue to crash, you can still use the app via drag & drop:

1. Launch the app
2. Drag PDF/CBZ files directly onto the window
3. Conversion will work normally

The app supports drag & drop as a fallback when file dialogs fail.
