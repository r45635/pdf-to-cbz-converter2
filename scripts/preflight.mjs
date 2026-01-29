#!/usr/bin/env node

/**
 * Preflight check to prevent accidental use of legacy GUI
 */

const args = process.argv.slice(2).join(' ');
const cwd = process.cwd();

// Check if we're trying to run anything from src-gui
if (cwd.includes('src-gui') || args.includes('src-gui')) {
  console.error('\n❌ ERROR: Legacy GUI is deprecated!\n');
  console.error('The src-gui/ directory is no longer maintained.');
  console.error('It uses outdated patterns and cannot show conversion progress.\n');
  console.error('✅ Use the modern GUI instead:\n');
  console.error('   cd /path/to/repo/root');
  console.error('   npm install');
  console.error('   npm run tauri:dev\n');
  console.error('📖 See src-gui/README_DEPRECATED.md for details.\n');
  process.exit(1);
}

// All good
process.exit(0);
