# Guide CLI Avancé - PDF to CBZ Converter

> Documentation complète de l'interface ligne de commande

## 📋 Table des Matières

1. [Installation](#installation)
2. [Commandes de Base](#commandes-de-base)
3. [Options Avancées](#options-avancées)
4. [Scripts et Automatisation](#scripts-et-automatisation)
5. [Optimisation des Performances](#optimisation-des-performances)
6. [Exemples Pratiques](#exemples-pratiques)

## Installation

### Installation Rapide

```bash
cd pdf-to-cbz-converter2
./install-cli.sh
```

### Installation Manuelle

```bash
cd src-cli
cargo install --path .
```

### Vérification

```bash
pdf-to-cbz --version
pdf-to-cbz --help
```

## Commandes de Base

### Structure Générale

```bash
pdf-to-cbz <COMMANDE> [OPTIONS] <ARGUMENTS>
```

### Commandes Disponibles

#### 1. `convert` - Conversion PDF → CBZ

```bash
pdf-to-cbz convert <INPUT.pdf> <OUTPUT.cbz> [OPTIONS]
```

**Options** :
- `--quality <LEVEL>` : lossless, high, medium, low (défaut: high)
- `--threads <N>` : Nombre de threads (défaut: auto)
- `--verbose` : Mode verbeux

**Exemples** :
```bash
# Conversion standard
pdf-to-cbz convert livre.pdf livre.cbz

# Qualité lossless
pdf-to-cbz convert livre.pdf livre.cbz --quality lossless

# Avec 8 threads
pdf-to-cbz convert livre.pdf livre.cbz --threads 8

# Mode verbeux
pdf-to-cbz convert livre.pdf livre.cbz --verbose
```

#### 2. `extract` - Extraction CBZ/CBR → PDF

```bash
pdf-to-cbz extract <INPUT.cbz|cbr> <OUTPUT.pdf> [OPTIONS]
```

**Options** :
- `--verbose` : Mode verbeux

**Exemples** :
```bash
# Extraction CBZ
pdf-to-cbz extract archive.cbz livre.pdf

# Extraction CBR
pdf-to-cbz extract archive.cbr livre.pdf

# Mode verbeux
pdf-to-cbz extract archive.cbz livre.pdf --verbose
```

#### 3. `batch-convert` - Traitement par Lot

```bash
pdf-to-cbz batch-convert <INPUT_DIR> <OUTPUT_DIR> [OPTIONS]
```

**Options** :
- `--quality <LEVEL>` : Qualité (défaut: high)
- `--threads <N>` : Threads par fichier
- `--parallel <N>` : Fichiers en parallèle (défaut: 1)
- `--verbose` : Mode verbeux

**Exemples** :
```bash
# Convertir tous les PDFs d'un dossier
pdf-to-cbz batch-convert ./pdfs/ ./cbz/

# Avec qualité lossless
pdf-to-cbz batch-convert ./pdfs/ ./cbz/ --quality lossless

# 2 fichiers en parallèle
pdf-to-cbz batch-convert ./pdfs/ ./cbz/ --parallel 2
```

## Options Avancées

### Qualité d'Image

| Niveau | Paramètre | Compression | Taille | Usage |
|--------|-----------|-------------|---------|-------|
| **Lossless** | `--quality lossless` | Aucune | 100% | Archivage, original |
| **Haute** | `--quality high` | PNG optimisé | ~60% | Lecture tablette |
| **Moyenne** | `--quality medium` | JPEG 85% | ~30% | Lecture mobile |
| **Basse** | `--quality low` | JPEG 60% | ~10% | Partage web |

### Contrôle du Threading

```bash
# Utiliser tous les cœurs disponibles (défaut)
pdf-to-cbz convert input.pdf output.cbz

# Limiter à 4 threads
pdf-to-cbz convert input.pdf output.cbz --threads 4

# 1 seul thread (debug)
pdf-to-cbz convert input.pdf output.cbz --threads 1
```

### Mode Verbeux

Affiche des informations détaillées :
```bash
pdf-to-cbz convert input.pdf output.cbz --verbose
```

**Sortie typique** :
```
[INFO] Ouverture du PDF: input.pdf
[INFO] Nombre de pages: 142
[INFO] Qualité sélectionnée: High
[INFO] Threads utilisés: 8
[INFO] Traitement page 1/142...
[INFO] Traitement page 2/142...
...
[INFO] Création de l'archive CBZ...
[SUCCESS] Conversion terminée: output.cbz (42.3 MB)
[INFO] Temps total: 12.5s
```

## Scripts et Automatisation

### Script 1 : Conversion Batch Simple

```bash
#!/bin/bash
# convert_all.sh - Convertit tous les PDFs d'un dossier

INPUT_DIR="./pdfs"
OUTPUT_DIR="./cbz"

mkdir -p "$OUTPUT_DIR"

for pdf in "$INPUT_DIR"/*.pdf; do
    filename=$(basename "$pdf" .pdf)
    echo "Converting: $filename"
    pdf-to-cbz convert "$pdf" "$OUTPUT_DIR/$filename.cbz" --quality high
    echo "✓ Done: $filename.cbz"
done

echo "All conversions completed!"
```

### Script 2 : Traitement Sélectif

```bash
#!/bin/bash
# convert_large_only.sh - Convertit uniquement les PDFs > 10MB

for pdf in *.pdf; do
    size=$(stat -f%z "$pdf")
    if [ $size -gt 10485760 ]; then  # 10 MB
        echo "Converting large file: $pdf"
        pdf-to-cbz convert "$pdf" "${pdf%.pdf}.cbz" --quality medium
    fi
done
```

### Script 3 : Traitement avec Logs

```bash
#!/bin/bash
# convert_with_logs.sh - Conversion avec journalisation

LOG_FILE="conversion_$(date +%Y%m%d_%H%M%S).log"

for pdf in *.pdf; do
    echo "[$(date)] Converting: $pdf" | tee -a "$LOG_FILE"
    
    if pdf-to-cbz convert "$pdf" "${pdf%.pdf}.cbz" --quality high 2>&1 | tee -a "$LOG_FILE"; then
        echo "[$(date)] ✓ Success: $pdf" | tee -a "$LOG_FILE"
    else
        echo "[$(date)] ✗ Failed: $pdf" | tee -a "$LOG_FILE"
    fi
done
```

### Script 4 : Extraction Batch

```bash
#!/bin/bash
# extract_all.sh - Extrait tous les CBZ en PDF

for cbz in *.cbz; do
    pdf="${cbz%.cbz}.pdf"
    echo "Extracting: $cbz → $pdf"
    pdf-to-cbz extract "$cbz" "$pdf"
done
```

### Script 5 : Conversion Parallèle avec GNU Parallel

```bash
#!/bin/bash
# parallel_convert.sh - Utilise GNU parallel pour vitesse maximale

export -f pdf_to_cbz_wrapper
pdf_to_cbz_wrapper() {
    pdf-to-cbz convert "$1" "${1%.pdf}.cbz" --quality high
}

# Traite 4 fichiers simultanément
find . -name "*.pdf" | parallel -j 4 pdf_to_cbz_wrapper {}
```

### Script 6 : Validation Post-Conversion

```bash
#!/bin/bash
# validate_cbz.sh - Vérifie l'intégrité des CBZ créés

for cbz in *.cbz; do
    if unzip -t "$cbz" &> /dev/null; then
        echo "✓ Valid: $cbz"
    else
        echo "✗ Corrupt: $cbz"
    fi
done
```

## Optimisation des Performances

### Règles Générales

1. **Threading** :
   - Laissez l'auto-détection pour meilleure performance
   - Réduisez si vous manquez de RAM (1 thread ≈ 500 MB RAM)

2. **Qualité** :
   - **Lossless** : Le plus lent, meilleure qualité
   - **High** : Bon compromis vitesse/qualité
   - **Medium/Low** : Plus rapide, qualité réduite

3. **Fichiers volumineux** :
   - Divisez les PDFs > 500 pages
   - Utilisez `--threads` pour contrôler la RAM

### Benchmarks

**Configuration Test** : MacBook Pro M1, 16GB RAM

| Opération | Pages | Qualité | Threads | Temps | RAM |
|-----------|-------|---------|---------|-------|-----|
| PDF → CBZ | 100 | Lossless | 8 | 15s | 2GB |
| PDF → CBZ | 100 | High | 8 | 10s | 1.5GB |
| PDF → CBZ | 100 | Medium | 8 | 8s | 1GB |
| PDF → CBZ | 100 | Low | 8 | 5s | 800MB |
| CBZ → PDF | 100 | - | 8 | 5s | 500MB |

### Optimisations par Scénario

#### Scénario 1 : Maximum de Vitesse
```bash
# Utilisez qualité basse et tous les threads
pdf-to-cbz batch-convert ./input/ ./output/ \
  --quality low \
  --parallel 2
```

#### Scénario 2 : Maximum de Qualité
```bash
# Lossless, threads modérés pour stabilité
pdf-to-cbz convert input.pdf output.cbz \
  --quality lossless \
  --threads 4
```

#### Scénario 3 : Équilibre
```bash
# Qualité haute, threading auto
pdf-to-cbz batch-convert ./input/ ./output/ \
  --quality high
```

#### Scénario 4 : RAM Limitée
```bash
# 1 thread, qualité moyenne
pdf-to-cbz convert input.pdf output.cbz \
  --quality medium \
  --threads 1
```

## Exemples Pratiques

### Exemple 1 : Pipeline de Conversion

```bash
#!/bin/bash
# pipeline.sh - Convertit PDFs → CBZ → Validation

# Étape 1 : Conversion
pdf-to-cbz batch-convert ./raw_pdfs/ ./cbz/ --quality high

# Étape 2 : Validation
for cbz in ./cbz/*.cbz; do
    if unzip -t "$cbz" &> /dev/null; then
        echo "✓ $cbz"
    else
        echo "✗ $cbz - ERREUR"
    fi
done

# Étape 3 : Nettoyage
mv ./cbz/*.cbz ./validated/
```

### Exemple 2 : Conversion Conditionnelle

```bash
#!/bin/bash
# conditional_convert.sh - Convertit si CBZ n'existe pas déjà

for pdf in *.pdf; do
    cbz="${pdf%.pdf}.cbz"
    
    if [ ! -f "$cbz" ]; then
        echo "Converting: $pdf"
        pdf-to-cbz convert "$pdf" "$cbz" --quality high
    else
        echo "Skipping: $cbz already exists"
    fi
done
```

### Exemple 3 : Surveillance de Dossier

```bash
#!/bin/bash
# watch_and_convert.sh - Surveille un dossier et convertit automatiquement

WATCH_DIR="./inbox"
OUTPUT_DIR="./converted"

while true; do
    for pdf in "$WATCH_DIR"/*.pdf; do
        [ -e "$pdf" ] || continue
        
        filename=$(basename "$pdf" .pdf)
        echo "[$(date)] New file detected: $filename"
        
        pdf-to-cbz convert "$pdf" "$OUTPUT_DIR/$filename.cbz" --quality high
        
        # Déplacer le PDF traité
        mv "$pdf" "$WATCH_DIR/processed/"
    done
    
    sleep 10
done
```

### Exemple 4 : Statistiques de Conversion

```bash
#!/bin/bash
# stats.sh - Affiche des statistiques détaillées

total_pdfs=$(find . -name "*.pdf" | wc -l)
total_size=$(du -sh *.pdf | awk '{s+=$1} END {print s}')

echo "=== Statistiques ==="
echo "Nombre de PDFs: $total_pdfs"
echo "Taille totale: $total_size MB"
echo "===================="

start_time=$(date +%s)

pdf-to-cbz batch-convert ./pdfs/ ./cbz/ --quality high

end_time=$(date +%s)
duration=$((end_time - start_time))

echo "Temps total: ${duration}s"
echo "Vitesse moyenne: $((total_pdfs / duration)) fichiers/s"
```

### Exemple 5 : Intégration Cron

```bash
# Ajoutez à votre crontab
# crontab -e

# Convertit automatiquement chaque nuit à 2h
0 2 * * * /path/to/convert_all.sh >> /var/log/pdf-convert.log 2>&1
```

## Options Environnement

### Variables d'Environnement

```bash
# Définir la qualité par défaut
export PDF_TO_CBZ_QUALITY=high

# Nombre de threads par défaut
export PDF_TO_CBZ_THREADS=8

# Mode verbeux par défaut
export PDF_TO_CBZ_VERBOSE=1
```

### Configuration Bash

Ajoutez à `~/.bashrc` ou `~/.zshrc` :

```bash
# Alias utiles
alias p2c='pdf-to-cbz convert'
alias c2p='pdf-to-cbz extract'
alias p2c-batch='pdf-to-cbz batch-convert'

# Fonctions pratiques
convert_pdf() {
    pdf-to-cbz convert "$1" "${1%.pdf}.cbz" --quality high
}

extract_cbz() {
    pdf-to-cbz extract "$1" "${1%.cbz}.pdf"
}
```

## Dépannage

### Problème : "Command not found"

```bash
# Vérifier que le binaire est dans le PATH
echo $PATH | grep -q "$HOME/.cargo/bin" || echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc

# Recharger le shell
source ~/.bashrc
```

### Problème : Conversion très lente

```bash
# Vérifier l'utilisation CPU
pdf-to-cbz convert input.pdf output.cbz --verbose --threads 1

# Si le CPU n'est pas à 100%, augmentez les threads
pdf-to-cbz convert input.pdf output.cbz --threads 8
```

### Problème : Manque de mémoire

```bash
# Réduire le nombre de threads
pdf-to-cbz convert input.pdf output.cbz --threads 2

# Ou utiliser qualité plus basse
pdf-to-cbz convert input.pdf output.cbz --quality medium --threads 4
```

## Aller Plus Loin

- **[Guide Utilisateur](GUIDE_UTILISATEUR.md)** : Bases et interface graphique
- **[Architecture](ARCHITECTURE.md)** : Comprendre le fonctionnement interne
- **[Guide Développeur](GUIDE_DEVELOPPEUR.md)** : Contribuer au projet

---

**Bonne automatisation ! ⚡**
