# Guide Utilisateur - PDF to CBZ Converter

> Guide complet pour débutants et utilisateurs occasionnels

## 📋 Table des Matières

1. [Introduction](#introduction)
2. [Installation](#installation)
3. [Interface Graphique](#interface-graphique)
4. [Ligne de Commande Basique](#ligne-de-commande-basique)
5. [Cas d'Usage Courants](#cas-dusage-courants)
6. [FAQ](#faq)
7. [Résolution de Problèmes](#résolution-de-problèmes)

## Introduction

### Qu'est-ce que c'est ?

PDF to CBZ Converter est un outil qui permet de :
- **Convertir des PDFs en CBZ** (format de bandes dessinées numériques)
- **Extraire des CBZ/CBR vers PDF** pour archivage ou impression
- **Traiter plusieurs fichiers** en une seule opération

### Pourquoi l'utiliser ?

✅ **Pour les lecteurs de BD/Manga** : Les fichiers CBZ sont optimisés pour la lecture sur tablettes et liseuses
✅ **Pour l'archivage** : Compressez vos BDs scannées en préservant la qualité
✅ **Pour la compatibilité** : Convertissez entre différents formats selon vos besoins

## Installation

### Étape 1 : Vérifier les prérequis

**macOS** :
```bash
# Vérifier que vous avez les outils nécessaires
xcode-select --install
```

**Windows** :
- Installez [Visual Studio Build Tools](https://visualstudio.microsoft.com/downloads/)

**Linux (Ubuntu/Debian)** :
```bash
sudo apt update
sudo apt install build-essential
```

### Étape 2 : Installer Rust

```bash
# Sur macOS/Linux
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Puis redémarrez votre terminal
```

### Étape 3 : Installer l'application

**Interface Graphique (Recommandé pour débutants)** :
```bash
# Installer Node.js et pnpm d'abord
npm install -g pnpm

# Cloner et installer
git clone <url-du-repo>
cd pdf-to-cbz-converter2
pnpm install

# Lancer
pnpm tauri dev
```

**Ligne de Commande** :
```bash
cd pdf-to-cbz-converter2
./install-cli.sh
```

## Interface Graphique

### Première Utilisation

1. **Lancer l'application** :
   ```bash
   pnpm tauri dev
   ```

2. **Vous verrez** :
   - Une zone centrale pour glisser-déposer des fichiers
   - Des boutons pour sélectionner des fichiers/dossiers
   - Des options de qualité
   - Un bouton "Convertir"

### Conversion PDF → CBZ

#### Méthode 1 : Glisser-Déposer
1. **Glissez** votre fichier PDF dans la zone centrale
2. **Choisissez** le dossier de sortie (ou laissez par défaut)
3. **Sélectionnez** la qualité :
   - **Lossless** : Qualité maximale (recommandé, ~50 MB pour 100 pages)
   - **Haute** : Excellent compromis (~30 MB)
   - **Moyenne** : Plus léger (~15 MB)
   - **Basse** : Très compact (~5 MB)
4. **Cliquez** sur "Convertir"

#### Méthode 2 : Sélection Manuelle
1. **Cliquez** sur "Sélectionner un fichier"
2. **Choisissez** votre PDF
3. Suivez les étapes 2-4 ci-dessus

### Extraction CBZ/CBR → PDF

1. **Sélectionnez** votre fichier CBZ ou CBR
2. **Choisissez** "Extraire vers PDF" dans le menu
3. **Cliquez** sur "Convertir"

### Traitement par Lot

1. **Cliquez** sur "Traitement par lot"
2. **Sélectionnez** le dossier contenant vos PDFs
3. **Choisissez** le dossier de sortie
4. **Sélectionnez** la qualité
5. **Lancez** la conversion

L'application traitera tous les fichiers automatiquement.

### Barre de Progression

Pendant la conversion, vous verrez :
- **Pourcentage** de progression
- **Nom** du fichier en cours
- **Temps estimé** restant
- **Possibilité d'annuler** l'opération

## Ligne de Commande Basique

### Commandes Essentielles

#### Convertir un PDF en CBZ
```bash
pdf-to-cbz convert mon-livre.pdf mon-livre.cbz
```

#### Avec qualité spécifique
```bash
pdf-to-cbz convert mon-livre.pdf mon-livre.cbz --quality lossless
```

#### Extraire un CBZ en PDF
```bash
pdf-to-cbz extract ma-bd.cbz ma-bd.pdf
```

#### Convertir plusieurs fichiers
```bash
pdf-to-cbz batch-convert ./mes-pdfs/ ./mes-cbz/ --quality high
```

### Options de Qualité

| Option | Commande | Utilisation |
|--------|----------|-------------|
| Lossless | `--quality lossless` | Archives, conservation |
| Haute | `--quality high` | Lecture tablette |
| Moyenne | `--quality medium` | Lecture mobile |
| Basse | `--quality low` | Partage rapide |

## Cas d'Usage Courants

### 1. Convertir une Collection de BDs

**Interface Graphique** :
1. Cliquez sur "Traitement par lot"
2. Sélectionnez le dossier contenant vos PDFs
3. Choisissez la qualité "Lossless"
4. Lancez

**Ligne de Commande** :
```bash
pdf-to-cbz batch-convert ~/Documents/BDs/ ~/Documents/CBZ/ --quality lossless
```

### 2. Préparer pour Tablette

**Qualité recommandée** : Haute
```bash
pdf-to-cbz convert manga.pdf manga.cbz --quality high
```

### 3. Archiver une BD Scannée

**Qualité recommandée** : Lossless
```bash
pdf-to-cbz convert scan-bd.pdf archive-bd.cbz --quality lossless
```

### 4. Convertir pour Partage

**Qualité recommandée** : Moyenne ou Basse
```bash
pdf-to-cbz convert bd.pdf bd-partage.cbz --quality medium
```

## FAQ

### Quelle qualité choisir ?

| Situation | Qualité | Raison |
|-----------|---------|--------|
| **Archivage** | Lossless | Conservation parfaite |
| **Lecture tablette** | Haute | Bon équilibre qualité/taille |
| **Lecture smartphone** | Moyenne | Taille optimisée |
| **Partage en ligne** | Basse | Upload rapide |

### Combien de temps prend une conversion ?

Pour un PDF de 100 pages :
- **Lossless** : ~15 secondes
- **Haute** : ~10 secondes
- **Moyenne** : ~8 secondes
- **Basse** : ~5 secondes

*Sur MacBook Pro M1. Varie selon la machine.*

### Puis-je annuler une conversion ?

**Oui !**
- **Interface Graphique** : Cliquez sur "Annuler"
- **Ligne de Commande** : Appuyez sur `Ctrl+C`

### Quelle est la différence entre CBZ et CBR ?

- **CBZ** : Archive ZIP (standard, recommandé)
- **CBR** : Archive RAR (ancien format)

L'outil peut extraire les deux, mais crée uniquement des CBZ (plus universel).

### Puis-je convertir plusieurs PDFs en une seule fois ?

**Oui !**
- **GUI** : Utilisez le mode "Traitement par lot"
- **CLI** : Utilisez `batch-convert`

### Les métadonnées sont-elles préservées ?

- **PDF → CBZ** : L'ordre des pages est préservé
- **CBZ → PDF** : Les images sont conservées dans l'ordre

## Résolution de Problèmes

### L'installation échoue

**Problème** : Erreurs lors de `pnpm install`
**Solution** :
```bash
# Nettoyez et réinstallez
rm -rf node_modules
pnpm install --force
```

### La conversion est lente

**Causes possibles** :
1. **Machine ancienne** : Normal, attendez
2. **PDF très lourd** : Divisez en plusieurs fichiers
3. **Qualité Lossless** : Utilisez "Haute" pour gagner du temps

**Optimisations** :
```bash
# Utilisez qualité moyenne pour PDFs volumineux
pdf-to-cbz convert gros-pdf.pdf output.cbz --quality medium
```

### Le fichier CBZ est énorme

**Solution** : Réduisez la qualité
```bash
# Au lieu de lossless
pdf-to-cbz convert input.pdf output.cbz --quality high
```

### Erreur "Command not found"

**Problème** : `pdf-to-cbz` non trouvé
**Solution** :
```bash
# Ajoutez ~/.cargo/bin au PATH
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

### L'interface graphique ne démarre pas

**Solution** :
```bash
# Vérifiez les dépendances
pnpm install

# Rebuilder
pnpm tauri build
```

### Impossible d'ouvrir le CBZ créé

**Vérifications** :
1. Le fichier existe-t-il ?
2. L'extension est-elle `.cbz` ?
3. Utilisez un lecteur CBZ approprié (YACReader, CDisplay, etc.)

**Test manuel** :
```bash
# Un CBZ est juste un ZIP, testez :
unzip -t fichier.cbz
```

## Commandes Utiles

### Vérifier l'installation
```bash
pdf-to-cbz --version
pdf-to-cbz --help
```

### Lister les options d'une commande
```bash
pdf-to-cbz convert --help
pdf-to-cbz extract --help
pdf-to-cbz batch-convert --help
```

### Voir la progression détaillée
```bash
# Le CLI affiche automatiquement la progression
pdf-to-cbz convert input.pdf output.cbz
```

## Aller Plus Loin

- **[Guide CLI Avancé](GUIDE_CLI.md)** : Scripts, automatisation, options avancées
- **[Guide GUI Détaillé](GUIDE_GUI.md)** : Fonctionnalités avancées de l'interface
- **[Architecture](ARCHITECTURE.md)** : Comprendre comment ça marche

## Support

**Questions ?**
- Consultez d'abord cette documentation
- Cherchez dans les [Issues GitHub](https://github.com/votre-user/pdf-to-cbz-converter2/issues)
- Créez une nouvelle issue si besoin

---

**Bon courage avec vos conversions ! 🚀**
