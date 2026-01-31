# 📋 PDF-to-CBZ Converter - TODO List

> **56 améliorations identifiées** • Dernière mise à jour : 31 janvier 2026

---

## 🎯 Quick Wins (Faciles + Haute Priorité)

- [ ] Supprimer les `console.log` de debug en production
- [ ] Ajouter les raccourcis clavier (Cmd+O, Cmd+Enter, Esc)
- [ ] Ajouter validation de taille de fichier max
- [ ] Nettoyer les fichiers `.backup` du repo
- [ ] Ajouter des états skeleton pendant le chargement
- [ ] Créer CONTRIBUTING.md

---

## 1. 🚀 PERFORMANCE

### Haute Priorité
- [ ] **Memory-mapped files** - Utiliser `memmap2` pour les gros PDFs (>100MB)
- [ ] **Streaming CBZ** - Écrire directement sur disque au lieu de tout garder en RAM
- [ ] **Cache des previews** - Mettre en cache les pages rendues pour éviter le re-rendu

### Moyenne Priorité
- [ ] Optimiser le niveau de compression PNG (`CompressionType::Fast`)
- [ ] Ajouter l'encodage JPEG progressif pour les grandes images
- [ ] Implémenter la conversion par plage de pages (ex: pages 1-50)

### Basse Priorité
- [ ] Pré-charger PDFium au démarrage de l'app
- [ ] Ajouter le support du format WebP comme option de sortie

---

## 2. ✨ FONCTIONNALITÉS

### Haute Priorité
- [ ] **Batch CBZ→PDF** - Compléter le mode batch pour CBZ vers PDF
- [ ] **PDFs protégés** - Support des PDFs avec mot de passe
- [ ] **Toggle dark mode** - Ajouter un bouton de bascule clair/sombre dans les réglages

### Moyenne Priorité
- [ ] Prévisualisation avant conversion (première/dernière page)
- [ ] Préservation des métadonnées (titre, auteur → ComicInfo.xml)
- [ ] Réorganisation des pages par drag-and-drop
- [ ] Fonctionnalité split/merge (diviser par chapitre, fusionner)

### Basse Priorité
- [ ] Historique des conversions récentes
- [ ] Format de sortie EPUB
- [ ] Flag de lecture droite-à-gauche pour les mangas

---

## 3. 🎨 INTERFACE (UI/UX)

### Haute Priorité
- [ ] **Raccourcis clavier** - Cmd/Ctrl+O, Cmd+Enter, Escape
- [ ] **Accessibilité** - Ajouter `aria-*`, `role`, `tabIndex`
- [ ] **États skeleton** - Afficher des placeholders pendant le chargement

### Moyenne Priorité
- [ ] Estimation du temps restant pendant la conversion
- [ ] Améliorer l'affichage des erreurs (bouton dismiss, suggestion retry)
- [ ] Confirmation avant "Clear All" en mode batch
- [ ] Persister les préférences utilisateur (DPI, qualité) entre sessions

### Basse Priorité
- [ ] Indicateur de progression animé
- [ ] Son/notification de succès
- [ ] Améliorer la responsivité mobile/tablette

---

## 4. 🔧 QUALITÉ DU CODE

### Haute Priorité
- [ ] **Remplacer les .unwrap()** - 19 appels à risque de panic
- [ ] **Supprimer console.log** - 50+ appels debug à nettoyer
- [ ] **Nettoyer fichiers backup** - Supprimer les `.backup` du repo

### Moyenne Priorité
- [ ] Extraire les valeurs hardcodées (DPI, qualité) en constantes
- [ ] Activer TypeScript strict mode dans tsconfig.json
- [ ] Découper page.tsx (773 lignes) en composants plus petits
- [ ] Supprimer le code mort dans utils (ghostscript_renderer, imagemagick_converter)

### Basse Priorité
- [ ] Ajouter configuration ESLint/Prettier
- [ ] Ajouter configuration Clippy pour Rust

---

## 5. 🔒 SÉCURITÉ

### Haute Priorité
- [ ] **Validation taille fichier** - Limiter la taille max des fichiers
- [ ] **Validation dimensions** - Vérifier les dimensions d'images extrêmes

### Moyenne Priorité
- [ ] Validation signature fichier (magic bytes vs extension)
- [ ] Sanitiser les noms de fichiers de sortie
- [ ] Rate limiting sur les requêtes de conversion

### Basse Priorité
- [ ] Revoir la politique CSP (retirer 'unsafe-inline')
- [ ] Vérification d'intégrité des fichiers convertis

---

## 6. 🧪 TESTS

### Haute Priorité
- [ ] **Tests unitaires Rust** - Tester les fonctions de conversion
- [ ] **Tests d'intégration** - Tester les commandes Tauri IPC
- [ ] **Tests composants React** - Jest/Vitest + React Testing Library

### Moyenne Priorité
- [ ] Tests E2E avec Playwright/WebdriverIO
- [ ] Ajouter des fichiers PDF de test dans le repo
- [ ] Tests de régression visuelle

### Basse Priorité
- [ ] Tests de fuzzing pour le parsing PDF
- [ ] Job CI pour les benchmarks de performance

---

## 7. 📚 DOCUMENTATION

### Haute Priorité
- [ ] **Exemples CLI** - Mettre à jour les exemples dans README
- [ ] **CONTRIBUTING.md** - Guide de contribution

### Moyenne Priorité
- [ ] Documentation API (rustdoc) pour les fonctions publiques
- [ ] Commentaires JSDoc dans le code TypeScript
- [ ] Traduction anglaise de la documentation

### Basse Priorité
- [ ] FAQ de dépannage
- [ ] CHANGELOG.md

---

## 8. ⚙️ DEVOPS/CI

### Haute Priorité
- [ ] **Job de tests CI** - Ajouter tests dans le pipeline
- [ ] **Job de lint CI** - ESLint + Clippy

### Moyenne Priorité
- [ ] Reporting de couverture de code
- [ ] Script de bump de version automatique
- [ ] Pre-commit hooks (Husky)

### Basse Priorité
- [ ] Builds nightly
- [ ] Monitoring de taille des artefacts
- [ ] Configuration Dependabot

---

## 📊 Résumé par Priorité

| Priorité | Nombre | Catégories principales |
|----------|--------|------------------------|
| **HAUTE** | 22 | Perf (3), Features (3), UI (3), Code (3), Sécurité (2), Tests (3), Docs (2), DevOps (2) |
| **MOYENNE** | 24 | Perf (3), Features (4), UI (4), Code (3), Sécurité (3), Tests (3), Docs (3), DevOps (3) |
| **BASSE** | 18 | Perf (2), Features (3), UI (3), Code (2), Sécurité (2), Tests (2), Docs (2), DevOps (3) |

---

## 🏗️ Projets Majeurs Recommandés

1. **Infrastructure de tests** - Fondation pour la qualité
2. **Refonte accessibilité** - Conformité WCAG
3. **Optimisation mémoire** - Memory-mapped + streaming
4. **Compléter batch CBZ→PDF** - Parité des fonctionnalités
