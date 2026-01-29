# Guide Interface Graphique - PDF to CBZ Converter

> Documentation complète de l'application de bureau

## 📋 Table des Matières

1. [Démarrage](#démarrage)
2. [Interface Principale](#interface-principale)
3. [Conversion PDF → CBZ](#conversion-pdf--cbz)
4. [Extraction CBZ/CBR → PDF](#extraction-cbzcbr--pdf)
5. [Traitement par Lot](#traitement-par-lot)
6. [Paramètres et Configuration](#paramètres-et-configuration)
7. [Astuces et Raccourcis](#astuces-et-raccourcis)

## Démarrage

### Première Utilisation

1. **Lancer l'application** :
   ```bash
   cd pdf-to-cbz-converter2
   npm run tauri dev
   # ou
   npm run tauri build  # puis ouvrir l'app compilée
   ```

2. **Interface de bienvenue** :
   - Zone centrale de glisser-déposer
   - Onglets Conversion/Extraction
   - Barre de réglages en haut

### Navigation Générale

```
┌────────────────────────────────────────┐
│  [Conversion] [Extraction] [Paramètres]│
├────────────────────────────────────────┤
│                                        │
│     📁 Glissez vos fichiers ici       │
│        ou cliquez pour parcourir      │
│                                        │
├────────────────────────────────────────┤
│  Qualité: [Haute ▼]  Threads: [Auto ▼]│
│                                        │
│        [🚀 Lancer la conversion]       │
└────────────────────────────────────────┘
```

## Interface Principale

### Zones de l'Interface

#### 1. Barre d'Onglets

**Onglets disponibles** :
- **Conversion** : PDF → CBZ
- **Extraction** : CBZ/CBR → PDF
- **Paramètres** : Configuration globale

**Navigation** :
- Cliquez sur un onglet pour changer de mode
- Raccourci : `Cmd+1` (Conversion), `Cmd+2` (Extraction), `Cmd+3` (Paramètres)

#### 2. Zone de Glisser-Déposer

**Fonctionnement** :
- Glissez des fichiers depuis le Finder
- Ou cliquez pour ouvrir un sélecteur de fichiers
- Accepte plusieurs fichiers simultanément

**Fichiers acceptés** :
- **Mode Conversion** : `.pdf`
- **Mode Extraction** : `.cbz`, `.cbr`

**Indication visuelle** :
- 📁 Normal : Zone grise avec icône
- 🟢 Survol : Zone verte (fichier compatible)
- 🔴 Survol : Zone rouge (fichier incompatible)

#### 3. Panneau de Configuration

**Paramètres rapides** :
- **Qualité** : Sélecteur déroulant (Lossless, Haute, Moyenne, Basse)
- **Threads** : Sélecteur (Auto, 1, 2, 4, 8, 16)
- **Dossier de sortie** : Bouton de parcours

#### 4. Bouton d'Action

**États** :
- ⚪ Gris : Aucun fichier sélectionné
- 🟢 Vert : Prêt à traiter
- 🟡 Jaune : Traitement en cours
- ✅ Vert foncé : Traitement terminé

#### 5. Zone de Progression

**Informations affichées** :
- Barre de progression générale
- Fichier en cours de traitement
- Temps écoulé / temps estimé
- Vitesse de traitement

## Conversion PDF → CBZ

### Conversion Simple

#### Étape 1 : Sélectionner le PDF

**Méthode 1 - Glisser-déposer** :
1. Ouvrez le Finder
2. Glissez le fichier PDF vers l'application
3. Relâchez dans la zone de dépôt

**Méthode 2 - Parcourir** :
1. Cliquez sur la zone de dépôt
2. Sélectionnez votre PDF dans le navigateur
3. Cliquez sur "Ouvrir"

#### Étape 2 : Configurer

**Qualité** :
- **Lossless** : 
  - ✅ Qualité maximale, aucune perte
  - ❌ Fichiers volumineux (100% taille originale)
  - 🎯 Usage : Archivage, impression
  
- **Haute** :
  - ✅ Excellente qualité visuelle
  - ✅ Taille réduite (~60%)
  - 🎯 Usage : Lecture sur tablette/PC
  
- **Moyenne** :
  - ✅ Bonne qualité
  - ✅ Taille compacte (~30%)
  - 🎯 Usage : Lecture sur mobile
  
- **Basse** :
  - ⚠️ Qualité réduite
  - ✅ Taille minimale (~10%)
  - 🎯 Usage : Partage web, aperçu

**Threads** :
- **Auto** : Recommandé (utilise tous les cœurs)
- **Nombre manuel** : Pour limiter l'usage CPU/RAM

**Dossier de sortie** :
- Par défaut : Même dossier que le PDF
- Personnalisé : Cliquez sur "Parcourir"

#### Étape 3 : Lancer

1. Cliquez sur **🚀 Lancer la conversion**
2. Attendez la fin du traitement
3. Le CBZ est créé dans le dossier de sortie

**Progression affichée** :
```
┌────────────────────────────────────────┐
│ Conversion de: mon_livre.pdf          │
│ ████████████████░░░░░░░░  75%         │
│ Page 75/100 - 5s restant               │
└────────────────────────────────────────┘
```

### Conversion Multiple

#### Sélection de Plusieurs Fichiers

**Méthode 1 - Glisser-déposer multiple** :
1. Sélectionnez plusieurs PDFs dans le Finder
2. Glissez-les tous ensemble vers l'application

**Méthode 2 - Sélection groupée** :
1. Cliquez sur la zone de dépôt
2. Maintenez `Cmd` et cliquez sur chaque PDF
3. Cliquez sur "Ouvrir"

#### Traitement par Lot

**Interface de lot** :
```
┌────────────────────────────────────────┐
│ 3 fichiers sélectionnés :              │
│ ✓ livre1.pdf (142 pages)               │
│ ✓ livre2.pdf (89 pages)                │
│ ✓ livre3.pdf (201 pages)               │
├────────────────────────────────────────┤
│ Traitement parallèle: [1 ▼]            │
│                                        │
│     [🚀 Convertir tous les fichiers]   │
└────────────────────────────────────────┘
```

**Paramètre "Traitement parallèle"** :
- **1** : Traite un fichier à la fois (recommandé)
- **2+** : Traite plusieurs fichiers simultanément (nécessite plus de RAM)

**Progression de lot** :
```
┌────────────────────────────────────────┐
│ Fichier 2/3                            │
│ ✓ livre1.pdf → livre1.cbz (terminé)    │
│ ⏳ livre2.pdf → livre2.cbz (en cours)   │
│   livre3.pdf → livre3.cbz (en attente) │
│                                        │
│ ████████████░░░░░░░░░░░░  65%         │
│ Temps total: 2m 15s / ~3m 30s          │
└────────────────────────────────────────┘
```

### Options Avancées

#### Prévisualisation

**Activer** :
1. Cochez "Afficher aperçu" dans Paramètres
2. Un panneau de prévisualisation apparaît à droite

**Fonctionnalités** :
- Vignettes des pages PDF
- Navigation rapide
- Zoom avant/arrière
- Sélection de plage de pages (futur)

#### Nommage Personnalisé

**Pattern de sortie** :
- `{nom}` : Nom du fichier original
- `{date}` : Date du jour
- `{qualite}` : Niveau de qualité

**Exemples** :
- `{nom}_cbz` → `livre_cbz.cbz`
- `{nom}_{qualite}` → `livre_high.cbz`
- `Archive_{date}` → `Archive_2024-01-15.cbz`

## Extraction CBZ/CBR → PDF

### Interface d'Extraction

```
┌────────────────────────────────────────┐
│  [Conversion] [Extraction] [Paramètres]│
├────────────────────────────────────────┤
│                                        │
│     📦 Glissez votre CBZ/CBR ici      │
│        ou cliquez pour parcourir      │
│                                        │
├────────────────────────────────────────┤
│  Dossier de sortie: [Parcourir...]    │
│                                        │
│        [📄 Extraire en PDF]            │
└────────────────────────────────────────┘
```

### Extraction Simple

#### Étape 1 : Sélectionner l'Archive

**Formats supportés** :
- `.cbz` (Comic Book ZIP)
- `.cbr` (Comic Book RAR)

**Sélection** :
1. Changez vers l'onglet **Extraction**
2. Glissez le fichier CBZ/CBR
3. Ou cliquez pour parcourir

#### Étape 2 : Choisir la Destination

**Options** :
- **Dossier par défaut** : Même emplacement que le CBZ/CBR
- **Dossier personnalisé** : Cliquez sur "Parcourir..."

#### Étape 3 : Extraire

1. Cliquez sur **📄 Extraire en PDF**
2. L'application :
   - Extrait les images du CBZ/CBR
   - Les convertit en pages PDF
   - Crée le fichier PDF final

**Progression** :
```
┌────────────────────────────────────────┐
│ Extraction: archive.cbz → livre.pdf    │
│ ████████████████████  95%              │
│ Image 95/100                           │
└────────────────────────────────────────┘
```

### Extraction Multiple

**Fonctionnement** :
1. Glissez plusieurs fichiers CBZ/CBR
2. Configurez le dossier de sortie
3. Cliquez sur "Extraire tous"

**Résultat** :
- Un PDF par CBZ/CBR
- Nommés automatiquement d'après les archives

## Traitement par Lot

### Mode Batch Avancé

#### Activer le Mode Batch

1. Cliquez sur l'icône ⚙️ Paramètres
2. Activez "Mode traitement par lot"
3. Retournez à l'onglet Conversion

#### Interface Batch

```
┌────────────────────────────────────────┐
│ MODE BATCH ACTIVÉ                      │
├────────────────────────────────────────┤
│ Dossier source:  [Parcourir...]        │
│ Dossier sortie:  [Parcourir...]        │
│                                        │
│ Fichiers trouvés: 24 PDFs              │
│                                        │
│ Options:                               │
│ ☑ Créer sous-dossiers par date         │
│ ☑ Conserver structure de dossiers      │
│ ☐ Supprimer PDFs après conversion     │
│                                        │
│     [🚀 Lancer le traitement batch]    │
└────────────────────────────────────────┘
```

#### Dossier Source

**Sélection** :
1. Cliquez sur "Parcourir..." (Dossier source)
2. Choisissez le dossier contenant vos PDFs
3. L'app scanne récursivement tous les sous-dossiers

**Affichage** :
- Nombre total de PDFs trouvés
- Taille totale estimée
- Liste des fichiers (expandable)

#### Options de Traitement

**Créer sous-dossiers par date** :
- Organise les CBZ par date de conversion
- Structure : `YYYY/MM/DD/fichier.cbz`

**Conserver structure de dossiers** :
- Reproduit l'arborescence source
- Exemple :
  ```
  Source: ./Livres/SF/Asimov/Foundation.pdf
  Sortie: ./CBZ/Livres/SF/Asimov/Foundation.cbz
  ```

**Supprimer PDFs après conversion** :
- ⚠️ Option destructive
- Demande confirmation
- Supprime uniquement les PDFs convertis avec succès

#### Progression Batch

**Vue d'ensemble** :
```
┌────────────────────────────────────────┐
│ TRAITEMENT BATCH - 24 fichiers         │
├────────────────────────────────────────┤
│ Progression globale:                   │
│ ████████████░░░░░░░░░░░░  50% (12/24)  │
│                                        │
│ Fichier en cours:                      │
│ 📄 Science-Fiction/Dune.pdf            │
│ ████████████████████  95%              │
│                                        │
│ Statistiques:                          │
│ ✓ Réussis: 11                          │
│ ⏳ En cours: 1                          │
│ ⏸ En attente: 12                       │
│ ✗ Erreurs: 0                           │
│                                        │
│ Temps écoulé: 5m 32s                   │
│ Temps restant: ~5m 30s                 │
│                                        │
│     [⏸ Pause] [❌ Annuler]             │
└────────────────────────────────────────┘
```

**Liste détaillée** :
```
✓ Foundation.pdf → Foundation.cbz (42 MB)
✓ Dune.pdf → Dune.cbz (38 MB)
⏳ Neuromancer.pdf → Neuromancer.cbz (en cours...)
  Hyperion.pdf → Hyperion.cbz (en attente)
  Snow_Crash.pdf → Snow_Crash.cbz (en attente)
```

#### Gestion des Erreurs

**Si une conversion échoue** :
- L'erreur est enregistrée dans un log
- Le traitement continue avec le fichier suivant
- À la fin, un rapport détaillé est affiché

**Rapport d'erreurs** :
```
┌────────────────────────────────────────┐
│ TRAITEMENT TERMINÉ AVEC ERREURS        │
├────────────────────────────────────────┤
│ ✓ Réussis: 22/24                       │
│ ✗ Erreurs: 2/24                        │
│                                        │
│ Fichiers en erreur:                    │
│ ✗ corrupted.pdf                        │
│   Raison: PDF corrompu                 │
│                                        │
│ ✗ huge_file.pdf                        │
│   Raison: Mémoire insuffisante         │
│                                        │
│ Log complet: ~/conversion_errors.log   │
│                                        │
│     [📋 Copier le rapport]             │
│     [🔄 Réessayer les erreurs]         │
│     [✓ Fermer]                         │
└────────────────────────────────────────┘
```

## Paramètres et Configuration

### Onglet Paramètres

```
┌────────────────────────────────────────┐
│  [Conversion] [Extraction] [Paramètres]│
├────────────────────────────────────────┤
│                                        │
│ ⚙️ CONFIGURATION GÉNÉRALE               │
│                                        │
│ Qualité par défaut:                    │
│ ○ Lossless  ● Haute  ○ Moyenne  ○ Basse│
│                                        │
│ Threading:                             │
│ ● Auto  ○ Manuel: [8 ▼]                │
│                                        │
│ Dossier de sortie par défaut:          │
│ [~/Documents/CBZ]  [Parcourir...]      │
│                                        │
│ ☑ Ouvrir le dossier après conversion   │
│ ☑ Émettre un son à la fin              │
│ ☐ Mode sombre                          │
│                                        │
│ ─────────────────────────────          │
│                                        │
│ 🔧 OPTIONS AVANCÉES                     │
│                                        │
│ ☐ Activer logs verbeux                 │
│ ☐ Prévisualisation automatique         │
│ ☑ Vérifier les mises à jour            │
│                                        │
│     [💾 Sauvegarder] [🔄 Réinitialiser]│
└────────────────────────────────────────┘
```

### Paramètres Détaillés

#### Qualité par Défaut

**Définit la qualité utilisée automatiquement pour toutes les conversions.**

**Changement** :
1. Cliquez sur un bouton radio
2. La modification est instantanée
3. Affecte uniquement les conversions futures

#### Threading

**Auto** (recommandé) :
- Détecte automatiquement le nombre de cœurs
- Utilise tous les cœurs disponibles
- Meilleure performance générale

**Manuel** :
- Permet de limiter l'usage CPU
- Utile si vous travaillez simultanément
- Réduit aussi l'usage RAM

**Équivalence RAM** :
- 1 thread ≈ 500 MB RAM
- 8 threads ≈ 4 GB RAM
- Auto ≈ (Nombre de cœurs × 500 MB)

#### Dossier de Sortie

**Par défaut** :
- Les CBZ sont créés au même endroit que les PDFs
- Ou dans `~/Documents/CBZ` si configuré

**Personnalisation** :
1. Cliquez sur "Parcourir..."
2. Sélectionnez un dossier
3. Cliquez sur "Sauvegarder"

#### Actions Post-Conversion

**Ouvrir le dossier après conversion** :
- ✅ Activé : Ouvre le Finder automatiquement
- ❌ Désactivé : Affiche uniquement une notification

**Émettre un son à la fin** :
- ✅ Activé : Joue un son de notification
- ❌ Désactivé : Notification silencieuse

**Mode sombre** :
- Interface claire/sombre
- Suit les préférences système si non défini

#### Options Avancées

**Logs verbeux** :
- Affiche des informations techniques détaillées
- Utile pour le débogage
- Ralentit légèrement l'interface

**Prévisualisation automatique** :
- Affiche automatiquement les vignettes
- Consomme plus de mémoire
- Ralentit le chargement initial

**Vérifier les mises à jour** :
- Vérifie automatiquement au démarrage
- Notifie si une nouvelle version existe

## Astuces et Raccourcis

### Raccourcis Clavier

#### Navigation

| Raccourci | Action |
|-----------|--------|
| `Cmd+1` | Onglet Conversion |
| `Cmd+2` | Onglet Extraction |
| `Cmd+3` | Onglet Paramètres |
| `Cmd+W` | Fermer l'application |
| `Cmd+Q` | Quitter |

#### Actions

| Raccourci | Action |
|-----------|--------|
| `Cmd+O` | Ouvrir un fichier |
| `Cmd+D` | Glisser-déposer (focus zone) |
| `Cmd+Enter` | Lancer la conversion |
| `Cmd+.` | Annuler l'opération |

#### Édition

| Raccourci | Action |
|-----------|--------|
| `Cmd+Z` | Annuler |
| `Cmd+Shift+Z` | Rétablir |
| `Cmd+A` | Tout sélectionner |
| `Cmd+C` | Copier |
| `Cmd+V` | Coller |

### Astuces d'Utilisation

#### Astuce 1 : Glisser-Déposer Direct depuis le Bureau

Vous pouvez glisser des fichiers directement depuis le Bureau macOS sans ouvrir le Finder.

#### Astuce 2 : Utiliser Quick Look

Avant de convertir, utilisez `Espace` sur un PDF pour le prévisualiser avec Quick Look.

#### Astuce 3 : Traitement Rapide

Pour une conversion ultra-rapide d'un fichier unique :
1. Glissez le PDF
2. Appuyez immédiatement sur `Cmd+Enter`
3. Le fichier est traité avec les paramètres par défaut

#### Astuce 4 : Organisation Automatique

Créez des dossiers de sortie par catégorie dans les Paramètres :
- `~/Documents/CBZ/Manga`
- `~/Documents/CBZ/BD`
- `~/Documents/CBZ/Comics`

Puis changez rapidement le dossier de sortie selon vos besoins.

#### Astuce 5 : Mode Nuit

Si vous travaillez tard le soir, activez le mode sombre dans Paramètres pour réduire la fatigue oculaire.

### Workflow Optimisés

#### Workflow 1 : Conversion Quotidienne

```
1. Ouvrir l'app (1x par jour)
2. Glisser tous les nouveaux PDFs
3. Cmd+Enter
4. Continuer à travailler
5. Notification de fin → ouvrir le dossier
```

#### Workflow 2 : Archivage de Collection

```
1. Activer Mode Batch
2. Sélectionner dossier racine de la collection
3. Qualité → Lossless
4. Options → Conserver structure
5. Lancer et laisser tourner
```

#### Workflow 3 : Partage Web

```
1. Glisser les PDFs à partager
2. Qualité → Basse (fichiers légers)
3. Dossier de sortie → ~/Downloads
4. Convertir
5. Uploader directement depuis Downloads
```

## Dépannage Interface

### L'application ne démarre pas

**Solutions** :
1. Vérifier les prérequis :
   ```bash
   node --version  # doit être >= 18
   pnpm --version  # doit être >= 8
   ```

2. Réinstaller les dépendances :
   ```bash
   pnpm install
   npm run tauri dev
   ```

### Le glisser-déposer ne fonctionne pas

**Causes possibles** :
- Fichier d'un type non supporté
- Permissions insuffisantes
- Application en arrière-plan

**Solutions** :
1. Vérifier l'extension du fichier (`.pdf`, `.cbz`, `.cbr`)
2. Mettre l'application au premier plan
3. Utiliser "Parcourir" à la place

### La conversion est bloquée à 0%

**Solutions** :
1. Vérifier que le PDF n'est pas corrompu :
   ```bash
   pdf-to-cbz convert fichier.pdf test.cbz --verbose
   ```

2. Vérifier l'espace disque disponible

3. Redémarrer l'application

### Interface figée / non-responsive

**Solutions** :
1. Réduire le nombre de threads dans Paramètres
2. Fermer d'autres applications gourmandes
3. Redémarrer l'ordinateur

## Aller Plus Loin

- **[Guide Utilisateur](GUIDE_UTILISATEUR.md)** : Bases et démarrage rapide
- **[Guide CLI](GUIDE_CLI.md)** : Interface ligne de commande
- **[Architecture](ARCHITECTURE.md)** : Fonctionnement technique

---

**Bonne conversion ! 🎨**
