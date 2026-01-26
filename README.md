# galaxy_image

Image loading and saving library for the Galaxy3D engine with support for PNG, BMP, and JPEG formats.

## Features

- **Multiple formats**: PNG, BMP, JPEG with automatic format detection
- **Magic byte detection**: Robust format detection from file content, not just extensions
- **Type-safe**: Strongly typed pixel formats and component types
- **Simple API**: Clean manager/factory pattern with `GalaxyImage`
- **Format conversion**: Automatic conversion between pixel formats when needed
- **Commercial-friendly**: All dependencies use MIT/Apache-2.0 licenses

## Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
galaxy_image = { path = "../galaxy_image" }
```

### Basic Usage

```rust
use galaxy_image::{GalaxyImage, ImageFormat};

// Load an image (format auto-detected from file content)
let image = GalaxyImage::load_from_file("texture.png")?;

println!("Loaded {}x{} image", image.width(), image.height());
println!("Format: {:?}", image.pixel_format());

// Save to a different format
GalaxyImage::save_to_file(&image, "output.jpg", ImageFormat::Jpeg)?;
```

### Loading from Memory

```rust
use galaxy_image::{GalaxyImage, ImageFormat};

// From byte buffer with explicit format
let bytes = std::fs::read("texture.png")?;
let image = GalaxyImage::load_from_bytes(&bytes, ImageFormat::Png)?;

// From byte buffer with auto-detection
let image = GalaxyImage::load_from_bytes_auto(&bytes)?;
```

### Saving with Options

```rust
use galaxy_image::{GalaxyImage, ImageFormat};

// Save JPEG with custom quality (1-100)
GalaxyImage::save_to_file_with_quality(
    &image,
    "output.jpg",
    ImageFormat::Jpeg,
    95  // High quality
)?;

// Save to memory buffer
let png_bytes = GalaxyImage::save_to_bytes(&image, ImageFormat::Png, 90)?;
```

### Creating Images Programmatically

```rust
use galaxy_image::{Image, PixelFormat, ComponentType};

// Create a blank RGB image
let mut image = Image::new(
    800,  // width
    600,  // height
    PixelFormat::RGB,
    ComponentType::U8
);

// Access raw pixel data
let pixels = image.data_mut();
// ... modify pixels ...

// Save it
GalaxyImage::save_to_file(&image, "generated.png", ImageFormat::Png)?;
```

## Supported Formats

| Format | Read | Write | Bit Depths | Alpha Channel | Notes |
|--------|------|-------|------------|---------------|-------|
| PNG    | ✅   | ✅    | U8, U16    | ✅           | Lossless, full support |
| BMP    | ✅   | ✅    | U8         | ❌           | RGB only, alpha stripped |
| JPEG   | ✅   | ✅    | U8         | ❌           | Lossy, quality control |

## Pixel Formats

- `PixelFormat::R` - Grayscale (1 channel)
- `PixelFormat::RG` - Grayscale + Alpha (2 channels)
- `PixelFormat::RGB` - Red, Green, Blue (3 channels)
- `PixelFormat::RGBA` - Red, Green, Blue, Alpha (4 channels)
- `PixelFormat::BGR` - Blue, Green, Red (3 channels, BMP native)
- `PixelFormat::BGRA` - Blue, Green, Red, Alpha (4 channels)

## Component Types

- `ComponentType::U8` - 8-bit unsigned integer (0-255)
- `ComponentType::U16` - 16-bit unsigned integer (0-65535)
- `ComponentType::F32` - 32-bit floating point (0.0-1.0)

## Format Detection

The library uses magic byte detection for robust format identification:

```rust
use galaxy_image::ImageFormat;

let bytes = std::fs::read("unknown.img")?;
let format = ImageFormat::detect_from_bytes(&bytes);

match format {
    ImageFormat::Png => println!("It's a PNG!"),
    ImageFormat::Bmp => println!("It's a BMP!"),
    ImageFormat::Jpeg => println!("It's a JPEG!"),
    ImageFormat::Unknown => println!("Unknown format"),
}
```

Magic bytes recognized:
- **PNG**: `89 50 4E 47 0D 0A 1A 0A` (`\x89PNG\r\n\x1A\n`)
- **BMP**: `42 4D` (`BM`)
- **JPEG**: `FF D8` (SOI marker)

## Error Handling

```rust
use galaxy_image::{GalaxyImage, ImageError};

match GalaxyImage::load_from_file("texture.png") {
    Ok(image) => {
        println!("Loaded successfully");
    }
    Err(ImageError::IoError(e)) => {
        eprintln!("File I/O error: {}", e);
    }
    Err(ImageError::UnsupportedFormat(fmt)) => {
        eprintln!("Unsupported format: {}", fmt);
    }
    Err(e) => {
        eprintln!("Other error: {}", e);
    }
}
```

## Automatic Conversions

The library automatically handles format conversions:

```rust
// Load BMP (BGR format internally)
let image = GalaxyImage::load_from_file("texture.bmp")?;
// Automatically converted to RGB

// Save RGBA image as JPEG
let rgba_image = /* ... */;
GalaxyImage::save_to_file(&rgba_image, "output.jpg", ImageFormat::Jpeg)?;
// Alpha channel automatically stripped
```

## Integration with Galaxy3D

```rust
use galaxy_image::GalaxyImage;
use galaxy_3d_engine::{RendererTexture, TextureDesc, Format};

// Load texture image
let image = GalaxyImage::load_from_file("texture.png")?;

// Create GPU texture
let texture = renderer.create_texture(TextureDesc {
    width: image.width(),
    height: image.height(),
    format: Format::R8G8B8A8_SRGB,
    usage: TextureUsage::Sampled,
    data: Some(image.data()),
})?;
```

## Performance Tips

1. **Use the right format**:
   - PNG for lossless images with transparency
   - JPEG for photos (set quality based on needs)
   - BMP for simple, uncompressed images

2. **Batch operations**:
   ```rust
   let images: Vec<Image> = file_paths
       .iter()
       .map(|path| GalaxyImage::load_from_file(path))
       .collect::<Result<_, _>>()?;
   ```

3. **Reuse buffers**: The `Image::into_data()` method transfers ownership without copying.

## License

This library is licensed under the MIT License. See [LICENSE-MIT](LICENSE-MIT) for details.

### Dependency Licenses

All dependencies use commercial-friendly licenses (MIT and/or Apache-2.0):

- `png` - MIT/Apache-2.0
- `bmp` - MIT
- `jpeg-decoder` - MIT/Apache-2.0
- `jpeg-encoder` - MIT/Apache-2.0

Full license texts are available in the `LICENSES/` directory.

## Contributing

This library is part of the Galaxy3D engine project. For bug reports or feature requests, please contact the maintainer.

## Changelog

### 0.1.0 (2026-01-26)

- Initial release
- PNG support (read/write, U8/U16 bit depths)
- BMP support (read/write, U8 bit depth)
- JPEG support (read/write, U8 bit depth, quality control)
- Magic byte format detection
- Automatic format conversion
- Manager/Factory pattern API
