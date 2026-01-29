# PDF to CBZ Converter

> **Convertisseur PDF ↔ CBZ/CBR haute performance avec interface graphique moderne**

[![License](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-1.75+-orange.svg)](https://www.rust-lang.org/)
[![TypeScript](https://img.shields.io/badge/TypeScript-5.0+-blue.svg)](https://www.typescriptlang.org/)

## 📋 Présentation

Application professionnelle de conversion de fichiers PDF en archives CBZ (Comic Book Archive), optimisée pour les bandes dessinées et mangas. Disponible en deux versions :

- **🖥️ Interface Graphique (GUI)** - Application Tauri moderne avec prévisualisation temps réel
- **⚡ Ligne de Commande (CLI)** - Outil ultra-rapide pour automatisation et traitement batch

### ✨ Fonctionnalités

- ✅ **Conversion bidirectionnelle** PDF ↔ CBZ/CBR haute qualité
- ✅ **Multithreading** optimisé pour performances maximales
- ✅ **Qualité configurable** (lossless, haute, moyenne, basse)
- ✅ **Traitement batch** pour fichiers multiples
- ✅ **Interface moderne** avec glisser-déposer et prévisualisation
- ✅ **Cross-platform** macOS, Windows, Linux
- Multi-page support

## 📖 Quick Links

**Choose your path:**

- **[CLI-README.md](CLI-README.md)** - Full feature documentation
- **[USAGE-GUIDE.md](USAGE-GUIDE.md)** - Practical examples and scripts
- **[INSTALLATION.md](INSTALLATION.md)** - System-specific setup
- **[MIGRATION-SUMMARY.md](MIGRATION-SUMMARY.md)** - What changed (for existing users)
- **[REFACTORING.md](REFACTORING.md)** - Technical details of the refactoring

## ⚡ Quick Start

### Installation (macOS/Linux)

```bash
# One command - handles everything
./install.sh

# Or manual build
cd src-cli
cargo build --release
```

### Usage

## 🚀 Installation Rapide

### Prérequis
- **Rust 1.75+** - [Installer Rust](https://rustup.rs/)
- **Node.js 18+** - [Installer Node.js](https://nodejs.org/)
- **pnpm 8+** - `npm install -g pnpm`

### Installation GUI (Interface Graphique)

```bash
# Installer les dépendances
pnpm install

# Lancer en mode développement
pnpm tauri dev

# Build pour production
pnpm tauri build
```

### Installation CLI (Ligne de Commande)

```bash
# Installation automatique
./install-cli.sh

# Ou installation manuelle
cd src-cli
cargo install --path .

# L'exécutable sera disponible dans ~/.cargo/bin/pdf-to-cbz
```

## 💡 Utilisation

### Interface Graphique (Débutants)

1. Lancez l'application : `pnpm tauri dev`
2. **Glissez-déposez** votre fichier PDF ou CBZ
3. **Choisissez** le dossier de sortie
4. **Sélectionnez** la qualité (Lossless recommandé)
5. Cliquez sur **"Convertir"**

Voir [Guide GUI détaillé](docs/GUIDE_GUI.md)

### Ligne de Commande (Avancé)

```bash
# Conversion PDF → CBZ (qualité lossless)
pdf-to-cbz convert mybook.pdf mybook.cbz --quality lossless

# Conversion CBZ → PDF
pdf-to-cbz extract archive.cbz output.pdf

# Conversion CBR → PDF
pdf-to-cbz extract archive.cbr output.pdf

# Traitement batch (tous les PDFs d'un dossier)
pdf-to-cbz batch-convert ./input/ ./output/ --quality high

# Afficher l'aide
pdf-to-cbz --help
```

Voir [Guide CLI complet](docs/GUIDE_CLI.md)

## 📊 Performances

| Opération | Temps (100 pages) | Qualité | Taille |
|-----------|------------------|---------|---------|
| PDF → CBZ (Lossless) | ~15s | 100% | ~50 MB |
| PDF → CBZ (Haute) | ~10s | 95% | ~30 MB |
| PDF → CBZ (Moyenne) | ~8s | 85% | ~15 MB |
| CBZ → PDF | ~5s | 100% | ~50 MB |

*Tests effectués sur MacBook Pro M1, 16GB RAM*

See [USAGE-GUIDE.md](USAGE-GUIDE.md) for more examples.

## 🐛 Troubleshooting

### "Failed to load PDF"
```bash
# Install libpdfium
brew install pdfium              # macOS
sudo apt-get install libpdfium0-dev  # Linux
```

### "unar not found" (for CBR files)
```bash
# Install unar
brew install unar                # macOS
sudo apt-get install unar        # Linux
```

See [INSTALLATION.md](INSTALLATION.md) for full setup guide.

## 🚀 Getting Started


## 📚 Documentation Complète

- 📘 **[Guide Utilisateur](docs/GUIDE_UTILISATEUR.md)** - Guide pas à pas pour débutants
- 📗 **[Guide CLI](docs/GUIDE_CLI.md)** - Documentation complète ligne de commande
- 📙 **[Guide GUI](docs/GUIDE_GUI.md)** - Utilisation interface graphique
- 📕 **[Architecture](docs/ARCHITECTURE.md)** - Documentation technique
- 📔 **[Guide Développeur](docs/GUIDE_DEVELOPPEUR.md)** - Contribution et développement

## 🏗️ Architecture

```
pdf-to-cbz-converter/
├── src/                    # Frontend React/TypeScript
│   ├── components/         # Composants UI
│   ├── lib/               # Utilitaires et client Tauri
│   └── pages/             # Pages principales
├── src-tauri/             # Backend Tauri (Rust)
│   ├── src/
│   │   ├── commands/      # Commandes IPC
│   │   └── utils/         # Utilitaires de conversion
│   └── Cargo.toml
├── src-cli/               # Application CLI (Rust)
│   ├── main.rs           # Point d'entrée CLI
│   ├── pdf.rs            # Traitement PDF
│   ├── archive.rs        # Gestion archives
│   └── Cargo.toml
└── src-lib/               # Bibliothèque partagée (Rust)
    ├── src/
    │   └── conversion.rs  # Logique de conversion
    └── Cargo.toml
```

### Stack Technique

**Backend (Rust)** :
- `pdfium-render` - Rendu PDF haute performance
- `zip` - Création/extraction archives CBZ
- `image` - Traitement d'images
- `printpdf` - Génération PDF

**Frontend (TypeScript)** :
- React 18 - Framework UI
- TailwindCSS - Styling
- Vite - Build tool
- Tauri - Desktop framework

**CLI** :
- `clap` - Parsing arguments
- `anyhow` - Gestion erreurs

## 🛠️ Développement

```bash
# Cloner le projet
git clone https://github.com/votre-user/pdf-to-cbz-converter2.git
cd pdf-to-cbz-converter2

# Installer les dépendances
pnpm install

# Build la bibliothèque partagée
cd src-lib && cargo build --release && cd ..

# Développement GUI
pnpm tauri dev

# Développement CLI
cd src-cli
cargo run -- convert test.pdf test.cbz
```

Voir le [Guide Développeur](docs/GUIDE_DEVELOPPEUR.md) pour plus de détails.

## 🤝 Contribution

Les contributions sont bienvenues ! Consultez :
1. [Guide Développeur](docs/GUIDE_DEVELOPPEUR.md)
2. [Architecture](docs/ARCHITECTURE.md)
3. Créez une issue ou pull request

## 📝 Licence

MIT License - Voir [LICENSE](LICENSE)

## 📧 Support

- **Issues** : [GitHub Issues](https://github.com/votre-user/pdf-to-cbz-converter2/issues)
- **Documentation** : Consultez le dossier [docs/](docs/)

---

**Made with ❤️ using Rust 🦀 and TypeScript**

**Total dependencies: 6** (down from 16+)

## 📊 Metrics Summary

| Metric | v2.0 | v3.0 | Improvement |
|--------|------|------|------------|

