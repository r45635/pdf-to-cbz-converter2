# PDF to CBZ Converter

> **Convertisseur PDF ↔ CBZ/CBR haute performance avec interface graphique moderne**

[![License](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-1.75+-orange.svg)](https://www.rust-lang.org/)
[![TypeScript](https://img.shields.io/badge/TypeScript-5.0+-blue.svg)](https://www.typescriptlang.org/)
[![Version](https://img.shields.io/badge/Version-1.0.0-green.svg)](VERSION)

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
./dev.sh

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

### Interface Graphique

1. Lancez l'application : `./dev.sh`
2. **Glissez-déposez** votre fichier PDF ou CBZ
3. **Choisissez** le dossier de sortie
4. **Sélectionnez** la qualité (Lossless recommandé)
5. Cliquez sur **"Convertir"**

### Ligne de Commande

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

## 📊 Performances

| Opération | Temps (100 pages) | Qualité | Taille |
|-----------|------------------|---------|---------|
| PDF → CBZ (Lossless) | ~15s | 100% | ~50 MB |
| PDF → CBZ (Haute) | ~10s | 95% | ~30 MB |
| PDF → CBZ (Moyenne) | ~8s | 85% | ~15 MB |
| CBZ → PDF | ~5s | 100% | ~50 MB |

*Tests effectués sur MacBook Pro M1, 16GB RAM*

## 🐛 Résolution de Problèmes

### "Failed to load PDF"

La bibliothèque PDFium est incluse dans l'application. Si vous avez des problèmes :

```bash
# Vérifier que PDFium est présent
ls resources/pdfium/
```

### "unar not found" (pour fichiers CBR)

```bash
# Installer unar
brew install unar                # macOS
sudo apt-get install unar        # Linux
```

## 📚 Documentation

| Guide | Description |
|-------|-------------|
| 📘 [Guide Utilisateur](docs/GUIDE_UTILISATEUR.md) | Guide pas à pas pour débutants |
| 📗 [Guide CLI](docs/GUIDE_CLI.md) | Documentation complète ligne de commande |
| 📙 [Guide GUI](docs/GUIDE_GUI.md) | Utilisation interface graphique |
| 📕 [Architecture](docs/ARCHITECTURE.md) | Documentation technique |
| 📔 [Guide Développeur](docs/GUIDE_DEVELOPPEUR.md) | Contribution et développement |

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
│   └── Cargo.toml
├── src-lib/               # Bibliothèque partagée (Rust)
│   └── Cargo.toml
├── resources/pdfium/      # Bibliothèques PDFium (macOS, Windows, Linux)
└── docs/                  # Documentation utilisateur
```

### Stack Technique

| Composant | Technologies |
|-----------|-------------|
| **Backend** | Rust, pdfium-render, zip, image, printpdf |
| **Frontend** | React 18, TailwindCSS, Vite, Tauri v2 |
| **CLI** | Rust, clap, anyhow |

## 🛠️ Développement

```bash
# Cloner le projet
git clone https://github.com/r45635/pdf-to-cbz-converter2.git
cd pdf-to-cbz-converter2

# Installer les dépendances
pnpm install

# Développement GUI
./dev.sh

# Développement CLI
cd src-cli
cargo run -- convert test.pdf test.cbz
```

Voir le [Guide Développeur](docs/GUIDE_DEVELOPPEUR.md) pour plus de détails.

## 🤝 Contribution

Les contributions sont bienvenues ! 

1. Fork le projet
2. Créez votre branche (`git checkout -b feature/ma-feature`)
3. Commit vos changements (`git commit -m 'Ajout ma feature'`)
4. Push (`git push origin feature/ma-feature`)
5. Ouvrez une Pull Request

## 📝 Licence

MIT License - Voir [LICENSE](LICENSE)

## 📧 Support

- **Issues** : [GitHub Issues](https://github.com/r45635/pdf-to-cbz-converter2/issues)
- **Documentation** : Consultez le dossier [docs/](docs/)

---

**Made with ❤️ using Rust 🦀 and TypeScript**

