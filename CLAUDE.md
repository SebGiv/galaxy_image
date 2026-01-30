# GalaxyImage - Règles de Développement

> **Project**: Image loading and saving library for the Galaxy3D engine
> **Date**: 2026-01-30

---

## 📋 Règles de Communication

### Langue de Communication

**TOUJOURS parler en français** avec l'utilisateur dans toutes les conversations.

**Exception** : Le code source, les commentaires dans le code, et les logs doivent être **en anglais**.

---

## 📁 Organisation des Fichiers

### Fichiers de Documentation

- **`CLAUDE.md`** (ce fichier) : Contient UNIQUEMENT les règles de développement du projet
- **`README.md`** : Documentation utilisateur de la bibliothèque
  - **Mise à jour automatique** : Claude doit mettre à jour ce fichier après chaque modification de l'API publique
  - **Langue** : Anglais
- **`Cargo.toml`** : Manifeste du projet avec les dépendances et métadonnées

---

## 🔧 Règles de Développement

### 1. Avant Tout Développement (Codage, Résolution de Bug, etc.)

**RÈGLE IMPÉRATIVE** :

1. ✋ **Exposer clairement** ce qui va être fait (changements prévus, fichiers impactés, approche technique)
2. ⏸️ **Attendre le feu vert** de l'utilisateur avant de commencer
3. ✅ Si l'utilisateur répond **"dev"** ou **"vas-y"** → Commencer le développement
4. ❌ Si l'utilisateur demande des modifications → Ajuster l'approche et re-exposer

**Exemple** :
```
Claude: "Je vais ajouter le support du format WebP en créant :
- src/loaders/webp_loader.rs : Nouveau loader pour WebP
- src/loaders/mod.rs : Exporter le nouveau loader
- src/image_format.rs : Ajouter ImageFormat::Webp
- src/galaxy_image.rs : Détecter les magic bytes WebP
Approche : Utiliser la crate 'image' pour le décodage WebP
Est-ce que je peux commencer le développement ?"

User: "dev"  ← Feu vert

Claude: [commence le développement]
```

---

### 2. Avant Tout Commit/Push

**RÈGLE IMPÉRATIVE** :

1. ✋ **Exposer le message de commit** complet (titre + description)
2. ⏸️ **Attendre le feu vert** de l'utilisateur
3. ✅ Si l'utilisateur répond **"commit"** → Faire `git commit` SEULEMENT
4. ✅ Si l'utilisateur répond **"commit/push"** ou **"push"** → Faire `git commit` ET `git push`
5. ❌ Si l'utilisateur demande des modifications → Ajuster le message et re-exposer

**Langue des Messages de Commit** : **Anglais** uniquement

- Les titres de commit doivent être en anglais
- Les descriptions de commit doivent être en anglais
- Suivre les conventions Git standard (feat:, fix:, docs:, refactor:, etc.)

**Exemple** :
```
Claude: "Je propose le message de commit suivant :

Titre: feat: Add WebP format support

Description:
- Add WebP loader using 'image' crate
- Detect WebP magic bytes (RIFF + WEBP)
- Update README.md with WebP format documentation
- Add example for loading WebP images

Est-ce que je peux commit/push ?"

User: "commit"  ← Commit seulement (pas de push)

Claude: [fait git commit seulement]
```

---

### 3. Code Source et Logs

**Langue** : **Anglais** uniquement

**Commentaires dans le code** :
```rust
// ✅ CORRECT (English)
/// Loads an image from a file path with automatic format detection
pub fn load_from_file<P: AsRef<Path>>(path: P) -> ImageResult<Image> {
    // Detect format from magic bytes
    let format = detect_format(&data)?;
    // ...
}

// ❌ INCORRECT (Français)
/// Charge une image depuis un fichier avec détection automatique du format
pub fn load_from_file<P: AsRef<Path>>(path: P) -> ImageResult<Image> {
    // Détecter le format depuis les magic bytes
    let format = detect_format(&data)?;
    // ...
}
```

**Logs** :
```rust
// ✅ CORRECT (English)
log::info!("Loaded {}x{} image from {:?}", width, height, path);
log::error!("Failed to decode PNG image: {}", err);

// ❌ INCORRECT (Français)
log::info!("Image chargée {}x{} depuis {:?}", width, height, path);
log::error!("Échec du décodage de l'image PNG: {}", err);
```

---

## 🎯 Workflow de Développement

### Workflow Type pour une Nouvelle Feature

1. **Analyse et Planification**
   - Discuter de la feature avec l'utilisateur
   - Identifier les fichiers à modifier

2. **Proposition de Développement**
   - Exposer les changements prévus
   - Attendre le feu vert ("dev")

3. **Développement**
   - Coder la feature (code + commentaires en anglais)
   - Ajouter des tests si nécessaire

4. **Documentation**
   - Mettre à jour `README.md` si l'API publique change
   - Mettre à jour les exemples si nécessaire

5. **Commit**
   - Exposer le message de commit
   - Attendre le feu vert ("commit" ou "commit/push")
   - Commit/push selon l'instruction

---

## 📖 Référence Rapide

| Situation | Action Claude | Attente User |
|-----------|---------------|--------------|
| Avant dev | Exposer les changements prévus | "dev" / "vas-y" |
| Avant commit | Exposer le message de commit | "commit" / "commit/push" |
| Code source | Écrire en anglais (commentaires + logs) | - |
| Conversation | Parler en français | - |
| Mise à jour doc | Automatique après modification API | - |

---

## ✅ Checklist Avant Chaque Action

### Avant de Coder
- [ ] J'ai exposé clairement ce que je vais faire
- [ ] J'ai attendu le feu vert de l'utilisateur
- [ ] Je vais écrire le code et les commentaires en anglais

### Avant de Commit
- [ ] J'ai exposé le message de commit complet
- [ ] J'ai attendu l'instruction ("commit" ou "commit/push")
- [ ] Je vais suivre l'instruction exactement

### Après Développement
- [ ] J'ai mis à jour `README.md` si l'API publique a changé
- [ ] Les logs sont en anglais
- [ ] Les commentaires sont en anglais
- [ ] Les tests passent (si applicable)

---

**Note** : Ces règles sont **impératives** et doivent être suivies à chaque fois, sans exception.
