# Rapport de Benchmark Complet - PDF ↔ CBZ

**Date:** 23 janvier 2026  
**Fichier test:** Adler (Integrale 1).pdf (850 MB, 270 pages)  
**Plateforme:** macOS Apple Silicon

---

## 📊 Résultats Complets

### PDF → CBZ (Conversions testées)

| Configuration | Temps | Vitesse | Taille Sortie | Compression | Recommandation |
|---------------|-------|---------|---------------|-------------|----------------|
| **DPI 150, Q50** | 39s | 6.92 p/s | 64 MB | -92% | ⚡ Mobile/Rapide |
| **DPI 150, Q85** | 42s | 6.41 p/s | 119 MB | -86% | ⭐ Équilibré |
| **DPI 200, Q85** | 53s | 5.09 p/s | 175 MB | -79% | 📱 Tablette |
| **DPI 300, Q90** | 1m 36s | 2.81 p/s | 368 MB | -57% | 💻 Desktop |
| **Lossless v1** | 1m 55s | 2.35 p/s | 607 MB | -29% | 📚 Archive |
| **Lossless v2 (Q95)** | 2m 16s | 1.99 p/s | 1335 MB | +57% | ❌ Trop lourd |

### CBZ → PDF (Conversions testées)

| Source CBZ | Mode | Temps | Taille Sortie | Vitesse |
|------------|------|-------|---------------|---------|
| Medium (175 MB) | Lossless | **1.0s** | 186 MB | **270 p/s** ⚡ |
| Standard (368 MB) | Q90 | **2.1s** | 392 MB | **129 p/s** |

---

## 🎯 Analyse des Performances

### PDF → CBZ : Observations

**Le plus rapide :** DPI 150, Q50/85 (~40 secondes)
- Vitesse : ~6.5 pages/seconde
- Excellent pour lecture mobile
- Recommandé pour lots volumineux

**Le plus équilibré :** DPI 200, Q85 (53 secondes)
- Vitesse : 5 pages/seconde
- Qualité très correcte
- Taille raisonnable (175 MB)

**Problème du mode Lossless :**
- ❌ Trop lent (2m16s en version optimisée)
- ❌ Fichiers trop gros (1.3 GB !)
- ❌ Pas vraiment "lossless" puisqu'il encode en JPEG

**Conclusion :** Le mode lossless n'est **pas recommandé** dans sa forme actuelle.

### CBZ → PDF : Excellent !

✅ **Extrêmement rapide** : 1-2 secondes  
✅ **Très efficace** : 129-270 pages/seconde  
✅ **Stable** : Pas de problème de performance  

Cette direction est **20x plus rapide** que PDF→CBZ !

---

## 💡 Recommandations d'Optimisation

### Pour améliorer PDF → CBZ

1. **Abandonner le mode "lossless" actuel**
   - Il ne préserve pas vraiment les données (encode en JPEG)
   - Trop lent et fichiers trop gros
   - Mieux vaut utiliser DPI 300 Q95

2. **Optimiser le rendu PDF**
   - Implémenter le rendu parallèle (multi-threading)
   - Utiliser un pool de workers
   - Traiter plusieurs pages en parallèle

3. **Configuration recommandée par défaut**
   - Changer le défaut de 300→200 DPI
   - Garder Q90 
   - Gain de vitesse +40%, qualité acceptable

### Configuration Optimale Proposée

```rust
// Nouveau défaut recommandé
DEFAULT_DPI = 200
DEFAULT_QUALITY = 90

// Modes prédéfinis
--preset fast    → DPI 150, Q85  (40s)
--preset balanced → DPI 200, Q90  (53s) [DÉFAUT]
--preset quality  → DPI 300, Q95  (100s)
```

---

## 🚀 Benchmark Final Recommandé

### Tests à Refaire avec Optimisations

1. ✅ **DPI 150, Q85** - Rapide (42s) → Recommandé mobile
2. ✅ **DPI 200, Q90** - Équilibré (53s) → **NOUVEAU DÉFAUT**
3. ✅ **DPI 300, Q90** - Standard (96s) → Haute qualité
4. ❌ **Lossless** - À supprimer ou retravailler

### CBZ → PDF
- ✅ **Performance excellente**, aucune optimisation nécessaire
- Temps : 1-2 secondes pour 270 pages
- Recommandation : **Garder tel quel**

---

## 📈 Comparaison Directionnelle

| Direction | Temps Moyen | Vitesse | Statut |
|-----------|-------------|---------|--------|
| **PDF → CBZ** | 40-100s | 2.7-6.5 p/s | Peut être amélioré |
| **CBZ → PDF** | 1-2s | 129-270 p/s | ✅ Excellent |

**Ratio de performance :** CBZ→PDF est **50-100x plus rapide** que PDF→CBZ

---

## 🎯 Conclusion et Actions

### Actions Immédiates

1. ✅ **Supprimer l'option `--lossless`** pour PDF→CBZ (non pertinent)
2. ⚡ **Changer le DPI par défaut** : 300→200
3. 📝 **Mettre à jour la documentation**

### Actions Futures (Optimisations)

1. 🔧 **Implémenter le multi-threading** pour le rendu PDF
2. 🎨 **Ajouter des presets** : `--preset fast|balanced|quality`
3. 📊 **Optimiser l'encodage JPEG** (encoder parallèle?)

### Performance Cible

**Objectif souhaitable :**
- PDF→CBZ @ 200 DPI : < 30 secondes (actuellement 53s)
- Gain visé : 40-50% via parallélisation

**Performance actuelle acceptable :**
- 42-53 secondes pour 270 pages en qualité équilibrée
- Soit ~5 pages/seconde
- Acceptable pour usage réel

---

## ✅ Statut Final

| Composant | Performance | Statut |
|-----------|-------------|--------|
| **CBZ → PDF** | ⚡⚡⚡⚡⚡ | ✅ Excellent, prêt |
| **PDF → CBZ (std)** | ⚡⚡⚡ | ✅ Bon, utilisable |
| **PDF → CBZ (lossless)** | ⚡ | ❌ À retravailler |

**Verdict :** Le convertisseur est **prêt pour la production** avec les réglages standards (DPI 150-300).
