# galaxy_image

Bibliothèque de chargement et sauvegarde d'images pour le moteur Galaxy3D avec support des formats PNG, BMP et JPEG.

## Fonctionnalités

- **Formats multiples** : PNG, BMP, JPEG avec détection automatique du format
- **Détection par magic bytes** : Détection robuste du format depuis le contenu du fichier, pas seulement l'extension
- **Type-safe** : Formats de pixels et types de composants fortement typés
- **API simple** : Pattern manager/factory propre avec `GalaxyImage`
- **Conversion de formats** : Conversion automatique entre formats de pixels selon les besoins
- **Commercial** : Toutes les dépendances utilisent les licences MIT/Apache-2.0

## Démarrage Rapide

Ajouter à votre `Cargo.toml` :

```toml
[dependencies]
galaxy_image = { path = "../galaxy_image" }
```

### Utilisation de Base

```rust
use galaxy_image::{GalaxyImage, ImageFormat};

// Charger une image (format auto-détecté depuis le contenu du fichier)
let image = GalaxyImage::load_from_file("texture.png")?;

println!("Image chargée : {}x{}", image.width(), image.height());
println!("Format : {:?}", image.pixel_format());

// Sauvegarder dans un format différent
GalaxyImage::save_to_file(&image, "output.jpg", ImageFormat::Jpeg)?;
```

### Chargement depuis la Mémoire

```rust
use galaxy_image::{GalaxyImage, ImageFormat};

// Depuis un buffer avec format explicite
let bytes = std::fs::read("texture.png")?;
let image = GalaxyImage::load_from_bytes(&bytes, ImageFormat::Png)?;

// Depuis un buffer avec auto-détection
let image = GalaxyImage::load_from_bytes_auto(&bytes)?;
```

### Sauvegarde avec Options

```rust
use galaxy_image::{GalaxyImage, ImageFormat};

// Sauvegarder JPEG avec qualité personnalisée (1-100)
GalaxyImage::save_to_file_with_quality(
    &image,
    "output.jpg",
    ImageFormat::Jpeg,
    95  // Haute qualité
)?;

// Sauvegarder dans un buffer mémoire
let png_bytes = GalaxyImage::save_to_bytes(&image, ImageFormat::Png, 90)?;
```

### Créer des Images par Code

```rust
use galaxy_image::{Image, PixelFormat, ComponentType};

// Créer une image RGB vierge
let mut image = Image::new(
    800,  // largeur
    600,  // hauteur
    PixelFormat::RGB,
    ComponentType::U8
);

// Accéder aux données de pixels brutes
let pixels = image.data_mut();
// ... modifier les pixels ...

// Sauvegarder
GalaxyImage::save_to_file(&image, "generated.png", ImageFormat::Png)?;
```

## Formats Supportés

| Format | Lecture | Écriture | Profondeurs | Canal Alpha | Notes |
|--------|---------|----------|-------------|-------------|-------|
| PNG    | ✅      | ✅       | U8, U16     | ✅          | Sans perte, support complet |
| BMP    | ✅      | ✅       | U8          | ❌          | RGB uniquement, alpha supprimé |
| JPEG   | ✅      | ✅       | U8          | ❌          | Avec perte, contrôle qualité |

## Formats de Pixels

- `PixelFormat::R` - Niveaux de gris (1 canal)
- `PixelFormat::RG` - Niveaux de gris + Alpha (2 canaux)
- `PixelFormat::RGB` - Rouge, Vert, Bleu (3 canaux)
- `PixelFormat::RGBA` - Rouge, Vert, Bleu, Alpha (4 canaux)
- `PixelFormat::BGR` - Bleu, Vert, Rouge (3 canaux, natif BMP)
- `PixelFormat::BGRA` - Bleu, Vert, Rouge, Alpha (4 canaux)

## Types de Composants

- `ComponentType::U8` - Entier non signé 8-bit (0-255)
- `ComponentType::U16` - Entier non signé 16-bit (0-65535)
- `ComponentType::F32` - Virgule flottante 32-bit (0.0-1.0)

## Détection de Format

La bibliothèque utilise la détection par magic bytes pour une identification robuste :

```rust
use galaxy_image::ImageFormat;

let bytes = std::fs::read("unknown.img")?;
let format = ImageFormat::detect_from_bytes(&bytes);

match format {
    ImageFormat::Png => println!("C'est un PNG !"),
    ImageFormat::Bmp => println!("C'est un BMP !"),
    ImageFormat::Jpeg => println!("C'est un JPEG !"),
    ImageFormat::Unknown => println!("Format inconnu"),
}
```

Magic bytes reconnus :
- **PNG** : `89 50 4E 47 0D 0A 1A 0A` (`\x89PNG\r\n\x1A\n`)
- **BMP** : `42 4D` (`BM`)
- **JPEG** : `FF D8` (marqueur SOI)

## Gestion des Erreurs

```rust
use galaxy_image::{GalaxyImage, ImageError};

match GalaxyImage::load_from_file("texture.png") {
    Ok(image) => {
        println!("Chargé avec succès");
    }
    Err(ImageError::IoError(e)) => {
        eprintln!("Erreur I/O fichier : {}", e);
    }
    Err(ImageError::UnsupportedFormat(fmt)) => {
        eprintln!("Format non supporté : {}", fmt);
    }
    Err(e) => {
        eprintln!("Autre erreur : {}", e);
    }
}
```

## Conversions Automatiques

La bibliothèque gère automatiquement les conversions de format :

```rust
// Charger BMP (format BGR en interne)
let image = GalaxyImage::load_from_file("texture.bmp")?;
// Automatiquement converti en RGB

// Sauvegarder une image RGBA en JPEG
let rgba_image = /* ... */;
GalaxyImage::save_to_file(&rgba_image, "output.jpg", ImageFormat::Jpeg)?;
// Canal alpha automatiquement supprimé
```

## Intégration avec Galaxy3D

```rust
use galaxy_image::GalaxyImage;
use galaxy_3d_engine::{RendererTexture, TextureDesc, Format};

// Charger l'image de texture
let image = GalaxyImage::load_from_file("texture.png")?;

// Créer une texture GPU
let texture = renderer.create_texture(TextureDesc {
    width: image.width(),
    height: image.height(),
    format: Format::R8G8B8A8_SRGB,
    usage: TextureUsage::Sampled,
    data: Some(image.data()),
})?;
```

## Astuces de Performance

1. **Utiliser le bon format** :
   - PNG pour images sans perte avec transparence
   - JPEG pour photos (ajuster qualité selon besoins)
   - BMP pour images simples, non compressées

2. **Opérations par lot** :
   ```rust
   let images: Vec<Image> = file_paths
       .iter()
       .map(|path| GalaxyImage::load_from_file(path))
       .collect::<Result<_, _>>()?;
   ```

3. **Réutiliser les buffers** : La méthode `Image::into_data()` transfère la propriété sans copier.

## Licence

Cette bibliothèque est sous licence MIT. Voir [LICENSE-MIT](LICENSE-MIT) pour les détails.

### Licences des Dépendances

Toutes les dépendances utilisent des licences compatibles commerciales (MIT et/ou Apache-2.0) :

- `png` - MIT/Apache-2.0
- `bmp` - MIT
- `jpeg-decoder` - MIT/Apache-2.0
- `jpeg-encoder` - MIT/Apache-2.0

Les textes complets des licences sont disponibles dans le répertoire `LICENSES/`.

## Contribution

Cette bibliothèque fait partie du projet du moteur Galaxy3D. Pour signaler des bugs ou demander des fonctionnalités, veuillez contacter le mainteneur.

## Journal des Modifications

### 0.1.0 (2026-01-26)

- Version initiale
- Support PNG (lecture/écriture, profondeurs U8/U16)
- Support BMP (lecture/écriture, profondeur U8)
- Support JPEG (lecture/écriture, profondeur U8, contrôle qualité)
- Détection de format par magic bytes
- Conversion automatique de format
- API pattern Manager/Factory
