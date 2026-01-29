#!/usr/bin/env node

import { rm, access } from 'fs/promises';
import { join, resolve } from 'path';
import { fileURLToPath } from 'url';
import { dirname } from 'path';

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);
const ROOT = resolve(__dirname, '..');

const args = process.argv.slice(2);
const flags = {
  all: args.includes('--all'),
  rust: args.includes('--rust'),
  js: args.includes('--js'),
};

// Auto-select all if no specific flags
const cleanAll = flags.all || (!flags.rust && !flags.js);

const targets = {
  rust: ['src-tauri/target', 'src-cli/target'],
  js: ['dist', '.next', 'out', 'build'],
  logs: ['*.log', 'npm-debug.log*', 'yarn-debug.log*', 'yarn-error.log*'],
  nodeModules: ['node_modules'], // Only with --all
};

async function exists(path) {
  try {
    await access(path);
    return true;
  } catch {
    return false;
  }
}

async function removeIfExists(path) {
  const fullPath = join(ROOT, path);
  if (await exists(fullPath)) {
    console.log(`🗑️  Removing: ${path}`);
    await rm(fullPath, { recursive: true, force: true });
    return true;
  }
  return false;
}

async function clean() {
  console.log('🧹 Cleaning PDF to CBZ Converter...\n');

  let removed = 0;

  // Clean Rust targets
  if (cleanAll || flags.rust) {
    console.log('📦 Rust build artifacts:');
    for (const target of targets.rust) {
      if (await removeIfExists(target)) removed++;
    }
    console.log();
  }

  // Clean JS build outputs
  if (cleanAll || flags.js) {
    console.log('📦 JavaScript build outputs:');
    for (const target of targets.js) {
      if (await removeIfExists(target)) removed++;
    }
    console.log();
  }

  // Clean logs (always)
  if (cleanAll) {
    console.log('📝 Log files:');
    for (const pattern of targets.logs) {
      // Simple pattern matching - just check common log files
      const logFiles = ['npm-debug.log', 'yarn-debug.log', 'yarn-error.log'];
      for (const log of logFiles) {
        if (await removeIfExists(log)) removed++;
      }
    }
    console.log();
  }

  // Clean node_modules (only with --all)
  if (flags.all) {
    console.log('📦 Node modules:');
    if (await removeIfExists('node_modules')) removed++;
    console.log();
  }

  if (removed === 0) {
    console.log('✨ Nothing to clean - already clean!');
  } else {
    console.log(`✅ Cleaned ${removed} item(s)`);
  }

  if (flags.all) {
    console.log('\n💡 Run `npm install` to reinstall dependencies');
  }
}

clean().catch((err) => {
  console.error('❌ Error during cleanup:', err.message);
  process.exit(1);
});
