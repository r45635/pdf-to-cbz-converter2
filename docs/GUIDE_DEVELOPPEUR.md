# Guide Développeur - PDF to CBZ Converter

> Documentation pour contribuer au projet

## 📋 Table des Matières

1. [Environnement de Développement](#environnement-de-développement)
2. [Structure du Code](#structure-du-code)
3. [Workflow de Développement](#workflow-de-développement)
4. [Standards de Code](#standards-de-code)
5. [Tests](#tests)
6. [Contribution](#contribution)
7. [Déploiement](#déploiement)

## Environnement de Développement

### Prérequis

#### Système d'Exploitation

- **macOS** : 12.0+ (Monterey ou plus récent)
- **Linux** : Ubuntu 20.04+, Fedora 36+
- **Windows** : 10/11 (support expérimental)

#### Outils Requis

1. **Rust** :
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   rustup default stable
   rustup update
   ```
   Version minimale : `1.75.0`

2. **Node.js et pnpm** :
   ```bash
   # Via nvm (recommandé)
   curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.0/install.sh | bash
   nvm install 18
   nvm use 18
   
   # Installer pnpm
   npm install -g pnpm
   ```
   Versions minimales : Node `18.0.0`, pnpm `8.0.0`

3. **Git** :
   ```bash
   git --version  # Doit être >= 2.30
   ```

4. **Tauri CLI** :
   ```bash
   cargo install tauri-cli
   ```

### Installation du Projet

#### 1. Cloner le Repository

```bash
git clone https://github.com/votre-org/pdf-to-cbz-converter2.git
cd pdf-to-cbz-converter2
```

#### 2. Installer les Dépendances

**Frontend** :
```bash
pnpm install
```

**Backend** :
```bash
# Dépendances Rust (automatique lors du build)
cargo build
```

#### 3. Configuration

**Fichier `.env`** (optionnel) :
```bash
# Copier le template
cp .env.example .env

# Éditer les valeurs
# RUST_LOG=debug
# TAURI_DEBUG=true
```

#### 4. Vérification

```bash
# Frontend
pnpm run dev

# CLI
cd src-cli
cargo run -- --version

# GUI (Tauri)
npm run tauri dev
```

### Outils Recommandés

#### Éditeurs

**VS Code** (recommandé) :
```bash
# Extensions essentielles
code --install-extension rust-lang.rust-analyzer
code --install-extension tauri-apps.tauri-vscode
code --install-extension dbaeumer.vscode-eslint
code --install-extension esbenp.prettier-vscode
```

**Configuration VS Code** (`.vscode/settings.json`) :
```json
{
  "rust-analyzer.cargo.features": "all",
  "rust-analyzer.checkOnSave.command": "clippy",
  "editor.formatOnSave": true,
  "[rust]": {
    "editor.defaultFormatter": "rust-lang.rust-analyzer"
  },
  "[typescript]": {
    "editor.defaultFormatter": "esbenp.prettier-vscode"
  }
}
```

**Autres Éditeurs** :
- **IntelliJ IDEA** : Plugin Rust
- **Vim/Neovim** : rust-analyzer + coc.nvim
- **Emacs** : rust-mode + lsp-mode

#### CLI Tools

**Cargo Utilities** :
```bash
# Formateur de code
cargo install rustfmt

# Linter
cargo install clippy

# Watch mode
cargo install cargo-watch

# Tests coverage
cargo install cargo-tarpaulin
```

**Node Utilities** :
```bash
# TypeScript compiler
pnpm add -D typescript

# Linter
pnpm add -D eslint @typescript-eslint/parser @typescript-eslint/eslint-plugin

# Formatter
pnpm add -D prettier
```

## Structure du Code

### Organisation des Modules

```
src-lib/src/
├── lib.rs              # Point d'entrée, exports publics
├── pdfium.rs           # Wrapper PDFium
├── converter.rs        # Logique PDF → CBZ
├── extractor.rs        # Logique CBZ → PDF
├── image.rs            # Manipulation images
├── archive.rs          # Opérations ZIP
├── quality.rs          # Niveaux de qualité
└── error.rs            # Types d'erreurs
```

### Conventions de Nommage

#### Rust

**Fichiers** :
- Modules : `snake_case.rs` (ex: `pdf_converter.rs`)
- Tests : `nom_module_test.rs` ou `tests/integration.rs`

**Code** :
```rust
// Structs/Enums : PascalCase
pub struct PdfDocument { }
pub enum Quality { Lossless, High }

// Fonctions/méthodes : snake_case
pub fn convert_pdf_to_cbz() { }

// Constants : SCREAMING_SNAKE_CASE
const MAX_PAGE_SIZE: usize = 4096;

// Modules : snake_case
mod pdf_converter;
```

#### TypeScript

**Fichiers** :
- Composants React : `PascalCase.tsx` (ex: `FileDropZone.tsx`)
- Utilitaires : `camelCase.ts` (ex: `formatters.ts`)
- Types : `PascalCase.d.ts` (ex: `ConversionTypes.d.ts`)

**Code** :
```typescript
// Interfaces/Types : PascalCase
interface ConversionOptions { }
type Quality = 'lossless' | 'high' | 'medium' | 'low';

// Fonctions/variables : camelCase
const convertPDF = () => { };
let currentFile: File;

// Constants : SCREAMING_SNAKE_CASE
const MAX_FILE_SIZE = 1024 * 1024 * 100; // 100 MB

// React Components : PascalCase
function FileDropZone() { }
```

### Documentation du Code

#### Rust

**Documentation Publique** :
```rust
/// Convertit un PDF en archive CBZ.
///
/// # Arguments
///
/// * `pdf_path` - Chemin vers le fichier PDF source
/// * `cbz_path` - Chemin de sortie pour le CBZ
/// * `options` - Options de conversion
///
/// # Exemples
///
/// ```
/// use pdf_to_cbz::convert_pdf_to_cbz;
///
/// let options = ConversionOptions::default();
/// convert_pdf_to_cbz("input.pdf", "output.cbz", &options)?;
/// ```
///
/// # Erreurs
///
/// Retourne une erreur si :
/// - Le fichier PDF n'existe pas ou est corrompu
/// - Permissions insuffisantes pour écrire le CBZ
/// - Mémoire insuffisante pour le traitement
pub fn convert_pdf_to_cbz(
    pdf_path: &Path,
    cbz_path: &Path,
    options: &ConversionOptions,
) -> Result<()> {
    // ...
}
```

**Documentation Interne** :
```rust
// Rend une page PDF en bitmap RGB.
// Utilise PDFium avec résolution 150 DPI.
fn render_page(doc: &PdfDocument, page_num: usize) -> Vec<u8> {
    // ...
}
```

#### TypeScript

**JSDoc** :
```typescript
/**
 * Convertit un PDF en CBZ via l'API Tauri.
 *
 * @param pdfPath - Chemin absolu du fichier PDF
 * @param cbzPath - Chemin de sortie du CBZ
 * @param options - Options de conversion
 * @returns Promise résolue avec le message de succès
 * @throws {Error} Si la conversion échoue
 *
 * @example
 * ```typescript
 * await convertPDF('/path/to/input.pdf', '/path/to/output.cbz', {
 *   quality: 'high',
 *   threads: 8,
 * });
 * ```
 */
async function convertPDF(
  pdfPath: string,
  cbzPath: string,
  options: ConversionOptions
): Promise<string> {
  // ...
}
```

## Workflow de Développement

### Branches

**Structure** :
```
main              # Production-ready code
├── develop       # Branche de développement
│   ├── feature/nom-feature
│   ├── fix/nom-bug
│   └── refactor/nom-refactor
└── release/v1.x  # Branches de release
```

**Conventions de Nommage** :
- `feature/description` : Nouvelle fonctionnalité
- `fix/description` : Correction de bug
- `refactor/description` : Refactoring
- `docs/description` : Documentation
- `test/description` : Tests

### Cycle de Développement

#### 1. Créer une Branche

```bash
# Depuis develop
git checkout develop
git pull origin develop
git checkout -b feature/nouvelle-fonctionnalite
```

#### 2. Développer

**Watch mode** :
```bash
# Frontend (auto-reload)
pnpm run dev

# Backend CLI (recompile à chaque changement)
cd src-cli
cargo watch -x run

# GUI Tauri (auto-reload frontend + backend)
npm run tauri dev
```

**Commits Fréquents** :
```bash
git add .
git commit -m "feat: ajoute support format CBR"
```

#### 3. Tests

**Avant chaque commit** :
```bash
# Rust
cargo test
cargo clippy
cargo fmt --check

# TypeScript
pnpm run lint
pnpm run type-check
pnpm run test
```

#### 4. Pull Request

```bash
git push origin feature/nouvelle-fonctionnalite
```

Puis créer une PR sur GitHub :
- Titre clair : `feat: Support format CBR`
- Description détaillée
- Lier les issues associées
- Demander review

### Débogage

#### Rust

**Logs** :
```rust
use log::{debug, info, warn, error};

pub fn convert_pdf(path: &Path) -> Result<()> {
    info!("Début conversion: {:?}", path);
    debug!("Nombre de pages: {}", page_count);
    
    if page_count == 0 {
        warn!("PDF vide détecté");
        return Err(Error::EmptyPdf);
    }
    
    // ...
}
```

**Exécution avec logs** :
```bash
RUST_LOG=debug cargo run
```

**Debugger** :
```bash
# VS Code : Ajouter breakpoints et F5

# CLI
rust-lldb target/debug/pdf-to-cbz
# ou
rust-gdb target/debug/pdf-to-cbz
```

#### TypeScript

**Console** :
```typescript
console.log('Info:', data);
console.warn('Attention:', warning);
console.error('Erreur:', error);
console.debug('Debug:', detail);
```

**Chrome DevTools** :
- Ouvrir Tauri app
- `Cmd+Option+I` (macOS) pour ouvrir DevTools
- Onglet Console, Sources, Network

**VS Code Debugger** :
```json
// .vscode/launch.json
{
  "version": "0.2.0",
  "configurations": [
    {
      "type": "chrome",
      "request": "launch",
      "name": "Tauri Dev",
      "url": "http://localhost:1420",
      "webRoot": "${workspaceFolder}/src"
    }
  ]
}
```

## Standards de Code

### Rust

#### Style Guide

**Suit le Rust Style Guide officiel** :
```bash
cargo fmt
```

**Principes** :
- Lignes max 100 caractères
- Indentation 4 espaces
- Pas de trailing whitespace

**Exemple** :
```rust
// ✅ BON
pub fn convert_pdf_to_cbz(
    pdf_path: &Path,
    cbz_path: &Path,
    options: &ConversionOptions,
) -> Result<()> {
    let pdf = PdfDocument::open(pdf_path)?;
    let page_count = pdf.page_count();
    
    info!("Converting {} pages", page_count);
    
    // ...
    
    Ok(())
}

// ❌ MAUVAIS
pub fn convert_pdf_to_cbz(pdf_path: &Path,cbz_path: &Path,options: &ConversionOptions)->Result<()>{
let pdf=PdfDocument::open(pdf_path)?;
let page_count=pdf.page_count();
// ...
}
```

#### Clippy

**Activer tous les lints** :
```bash
cargo clippy -- -D warnings
```

**Règles strictes** :
```rust
#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![allow(clippy::missing_errors_doc)] // Temporaire
```

**Exemples de corrections** :
```rust
// ❌ Clippy warning: use of `unwrap()`
let value = option.unwrap();

// ✅ Correction
let value = option.expect("Description de l'erreur");

// ❌ Clippy warning: needless borrow
some_fn(&my_string);

// ✅ Correction
some_fn(my_string);
```

### TypeScript

#### ESLint Configuration

```json
// .eslintrc.json
{
  "extends": [
    "eslint:recommended",
    "plugin:@typescript-eslint/recommended",
    "plugin:react/recommended",
    "plugin:react-hooks/recommended"
  ],
  "rules": {
    "@typescript-eslint/no-explicit-any": "error",
    "@typescript-eslint/explicit-function-return-type": "warn",
    "react/prop-types": "off",
    "no-console": ["warn", { "allow": ["warn", "error"] }]
  }
}
```

#### Prettier Configuration

```json
// .prettierrc
{
  "semi": true,
  "singleQuote": true,
  "tabWidth": 2,
  "trailingComma": "es5",
  "printWidth": 80
}
```

#### Type Safety

**Éviter `any`** :
```typescript
// ❌ MAUVAIS
function process(data: any) {
  return data.value;
}

// ✅ BON
interface Data {
  value: string;
}

function process(data: Data): string {
  return data.value;
}
```

**Utiliser `unknown` si type inconnu** :
```typescript
function parse(json: string): unknown {
  return JSON.parse(json);
}

const data = parse('{"value": 42}');

// Type guard
if (typeof data === 'object' && data !== null && 'value' in data) {
  console.log(data.value);
}
```

## Tests

### Tests Unitaires (Rust)

#### Structure

```
src-lib/
├── src/
│   ├── converter.rs
│   └── converter_test.rs  # Tests inline
└── tests/
    ├── integration.rs      # Tests d'intégration
    └── fixtures/
        └── sample.pdf
```

#### Tests Inline

```rust
// converter.rs
pub fn convert_pdf_to_cbz(/* ... */) -> Result<()> {
    // ...
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_convert_simple_pdf() {
        let pdf_path = Path::new("tests/fixtures/sample.pdf");
        let cbz_path = Path::new("/tmp/output.cbz");
        let options = ConversionOptions::default();
        
        let result = convert_pdf_to_cbz(pdf_path, cbz_path, &options);
        
        assert!(result.is_ok());
        assert!(cbz_path.exists());
    }
    
    #[test]
    #[should_panic(expected = "File not found")]
    fn test_convert_missing_file() {
        let pdf_path = Path::new("nonexistent.pdf");
        let cbz_path = Path::new("/tmp/output.cbz");
        let options = ConversionOptions::default();
        
        convert_pdf_to_cbz(pdf_path, cbz_path, &options).unwrap();
    }
}
```

#### Tests d'Intégration

```rust
// tests/integration.rs
use pdf_to_cbz::{convert_pdf_to_cbz, ConversionOptions, Quality};
use std::path::Path;

#[test]
fn test_full_conversion_workflow() {
    // Arrange
    let pdf = Path::new("tests/fixtures/sample.pdf");
    let cbz = Path::new("/tmp/integration_test.cbz");
    
    let options = ConversionOptions {
        quality: Quality::High,
        threads: 4,
    };
    
    // Act
    let result = convert_pdf_to_cbz(pdf, cbz, &options);
    
    // Assert
    assert!(result.is_ok());
    assert!(cbz.exists());
    
    // Vérifier contenu
    let size = std::fs::metadata(cbz).unwrap().len();
    assert!(size > 1000); // Au moins 1 KB
    
    // Cleanup
    std::fs::remove_file(cbz).unwrap();
}
```

#### Exécution

```bash
# Tous les tests
cargo test

# Tests d'un module
cargo test converter

# Tests verbeux
cargo test -- --nocapture

# Tests avec coverage
cargo tarpaulin --out Html
```

### Tests Frontend (TypeScript)

#### Vitest Configuration

```typescript
// vite.config.ts
import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';

export default defineConfig({
  plugins: [react()],
  test: {
    globals: true,
    environment: 'jsdom',
    setupFiles: './src/test/setup.ts',
  },
});
```

#### React Testing Library

```typescript
// FileDropZone.test.tsx
import { render, screen, fireEvent } from '@testing-library/react';
import { FileDropZone } from './FileDropZone';

describe('FileDropZone', () => {
  it('renders drop zone', () => {
    render(<FileDropZone onDrop={() => {}} />);
    
    const dropZone = screen.getByText(/Glissez vos fichiers/i);
    expect(dropZone).toBeInTheDocument();
  });
  
  it('calls onDrop when file is dropped', async () => {
    const onDrop = vi.fn();
    render(<FileDropZone onDrop={onDrop} />);
    
    const file = new File(['content'], 'test.pdf', { type: 'application/pdf' });
    const dropZone = screen.getByTestId('drop-zone');
    
    fireEvent.drop(dropZone, {
      dataTransfer: { files: [file] },
    });
    
    expect(onDrop).toHaveBeenCalledWith([file]);
  });
});
```

#### Exécution

```bash
# Tous les tests
pnpm run test

# Watch mode
pnpm run test:watch

# Coverage
pnpm run test:coverage
```

### Tests End-to-End (E2E)

**Playwright** (futur) :
```typescript
// e2e/conversion.spec.ts
import { test, expect } from '@playwright/test';

test('convertir un PDF en CBZ', async ({ page }) => {
  await page.goto('http://localhost:1420');
  
  // Upload file
  const fileInput = page.locator('input[type="file"]');
  await fileInput.setInputFiles('tests/fixtures/sample.pdf');
  
  // Set quality
  await page.selectOption('select#quality', 'high');
  
  // Convert
  await page.click('button:has-text("Lancer")');
  
  // Wait for completion
  await expect(page.locator('.success-message')).toBeVisible();
});
```

## Contribution

### Processus de Contribution

#### 1. Fork et Clone

```bash
# Fork sur GitHub
# Puis cloner votre fork
git clone https://github.com/votre-username/pdf-to-cbz-converter2.git
cd pdf-to-cbz-converter2

# Ajouter upstream
git remote add upstream https://github.com/org/pdf-to-cbz-converter2.git
```

#### 2. Créer une Issue

Avant de coder, créez une issue pour discuter :
- **Bug** : Décrire le problème, steps to reproduce
- **Feature** : Décrire le besoin, cas d'usage
- **Refactor** : Justifier les changements

#### 3. Développer

```bash
# Créer une branche
git checkout -b feature/ma-feature

# Coder, tester, commiter
git commit -m "feat: ma nouvelle feature"

# Rester à jour
git fetch upstream
git rebase upstream/develop
```

#### 4. Pull Request

**Template** :
```markdown
## Description
Courte description des changements.

## Type de changement
- [ ] Bug fix
- [ ] Nouvelle feature
- [ ] Refactoring
- [ ] Documentation

## Checklist
- [ ] Tests ajoutés/mis à jour
- [ ] Documentation mise à jour
- [ ] `cargo test` passe
- [ ] `cargo clippy` sans warnings
- [ ] `pnpm run lint` passe

## Screenshots (si UI)
[Ajouter captures d'écran]
```

### Convention de Commits

**Format** : `<type>(<scope>): <message>`

**Types** :
- `feat`: Nouvelle fonctionnalité
- `fix`: Correction de bug
- `refactor`: Refactoring
- `docs`: Documentation
- `test`: Tests
- `chore`: Maintenance (deps, config)
- `perf`: Performance

**Exemples** :
```bash
git commit -m "feat(cli): ajoute option --parallel"
git commit -m "fix(gui): corrige bug glisser-déposer"
git commit -m "refactor(lib): simplifie logique conversion"
git commit -m "docs: met à jour README avec exemples"
git commit -m "test(converter): ajoute tests qualité lossless"
git commit -m "chore: bump dependencies"
git commit -m "perf(pdfium): optimise rendu pages"
```

## Déploiement

### Build de Production

#### CLI

```bash
cd src-cli
cargo build --release

# Binaire disponible dans:
# target/release/pdf-to-cbz (Linux/macOS)
# target/release/pdf-to-cbz.exe (Windows)
```

#### GUI (Tauri)

```bash
npm run tauri build

# Artefacts dans:
# src-tauri/target/release/bundle/
# - dmg (macOS)
# - deb (Linux)
# - msi (Windows)
```

### Release

#### Versioning (SemVer)

**Format** : `MAJOR.MINOR.PATCH`

- `MAJOR` : Breaking changes
- `MINOR` : Nouvelles features (backward-compatible)
- `PATCH` : Bug fixes

**Exemple** :
```bash
# Version actuelle: 1.2.3
# Bug fix → 1.2.4
# Nouvelle feature → 1.3.0
# Breaking change → 2.0.0
```

#### Processus de Release

1. **Update VERSION** :
   ```bash
   echo "1.3.0" > VERSION
   ```

2. **Update Cargo.toml** :
   ```toml
   [package]
   version = "1.3.0"
   ```

3. **Update package.json** :
   ```json
   {
     "version": "1.3.0"
   }
   ```

4. **Create Tag** :
   ```bash
   git tag -a v1.3.0 -m "Release v1.3.0"
   git push origin v1.3.0
   ```

5. **GitHub Release** :
   - Créer une release sur GitHub
   - Uploader binaires
   - Ajouter changelog

### CI/CD

**GitHub Actions** (`.github/workflows/ci.yml`) :
```yaml
name: CI

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - run: cargo test
      - run: cargo clippy -- -D warnings

  build:
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        os: [ubuntu-latest, macos-latest, windows-latest]
    steps:
      - uses: actions/checkout@v3
      - run: npm run tauri build
      - uses: actions/upload-artifact@v3
        with:
          name: app-${{ matrix.os }}
          path: src-tauri/target/release/bundle/
```

## Ressources

### Documentation

- **Rust** : https://doc.rust-lang.org/
- **Tauri** : https://tauri.app/
- **React** : https://react.dev/
- **PDFium** : https://pdfium.googlesource.com/pdfium/

### Community

- **Discord** : [Lien Discord du projet]
- **GitHub Discussions** : [Lien Discussions]
- **Issues** : [Lien Issues]

---

**Bon développement ! 🚀**
