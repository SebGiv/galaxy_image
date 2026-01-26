# galaxy_image - Context Documentation

## Project Overview

**galaxy_image** is an image loading and saving library developed for the Galaxy3D game engine. It provides a simple, type-safe API for working with image files in multiple formats.

## Development Context

### Initial Requirements (from user)

1. **Architecture**: Manager/Factory/Singleton pattern with a struct named `GalaxyImage`
2. **Functionality**:
   - Load images from files or byte buffers
   - Save images to files or byte buffers
   - Support PNG, BMP, and JPEG formats initially
3. **Design Principles**:
   - Separate .rs files for each object type
   - Image struct encapsulating pixel buffer with metadata
   - Advanced format detection via magic bytes (not just file extensions)
   - Optimized serialization/deserialization
4. **Dependencies**: Use low-level crates as backends (wrapper approach)
5. **Licensing**: Include all dependency licenses in LICENSES/ folder

### Technology Stack

#### Core Dependencies

- **png** (0.17) - MIT/Apache-2.0
  - Low-level PNG encoding/decoding
  - Supports U8 and U16 bit depths
  - Handles RGB, RGBA, Grayscale, and GrayscaleAlpha

- **bmp** (0.5) - MIT
  - Low-level BMP encoding/decoding
  - Simple format, supports RGB only
  - U8 component type

- **jpeg-decoder** (0.3) - MIT/Apache-2.0
  - Low-level JPEG decoding
  - Supports RGB and Grayscale
  - U8 component type

- **jpeg-encoder** (0.6) - MIT/Apache-2.0
  - Low-level JPEG encoding
  - Configurable quality (1-100)
  - Supports RGB and Grayscale

### Architecture Decisions

#### 1. Manager/Factory Pattern

The `GalaxyImage` struct acts as a singleton-like factory with static methods:
- `load_from_file()` - Load from file path with auto-detection
- `load_from_bytes()` - Load from byte buffer with explicit format
- `load_from_bytes_auto()` - Load from byte buffer with auto-detection
- `save_to_file()` - Save to file
- `save_to_bytes()` - Save to byte buffer

**Rationale**: Provides a clean, centralized API without requiring instantiation.

#### 2. Type-Safe Pixel Formats

**PixelFormat enum**:
- `R` (Grayscale)
- `RG` (Grayscale + Alpha)
- `RGB` (Red, Green, Blue)
- `RGBA` (Red, Green, Blue, Alpha)
- `BGR` (Blue, Green, Red - BMP native)
- `BGRA` (Blue, Green, Red, Alpha)

**ComponentType enum**:
- `U8` - 8-bit unsigned (0-255)
- `U16` - 16-bit unsigned (0-65535)
- `F32` - 32-bit float (0.0-1.0)

**Rationale**: Explicit typing prevents format confusion and enables compile-time validation.

#### 3. Magic Byte Detection

**ImageFormat enum** with `detect_from_bytes()`:
- PNG: `0x89 0x50 0x4E 0x47 0x0D 0x0A 0x1A 0x0A`
- BMP: `0x42 0x4D` (ASCII "BM")
- JPEG: `0xFF 0xD8` (SOI marker)

**Rationale**: More reliable than file extension checking, enables correct handling of renamed or extension-less files.

#### 4. Automatic Format Conversion

The library automatically converts between pixel formats when needed:
- BGR ↔ RGB conversion for BMP files
- Alpha channel stripping for formats that don't support it
- Grayscale to RGB conversion when needed

**Rationale**: Simplifies user code by handling format compatibility internally.

#### 5. Separate Loader Modules

Each format has its own loader module:
- `loaders/png_loader.rs` - PNG implementation
- `loaders/bmp_loader.rs` - BMP implementation
- `loaders/jpeg_loader.rs` - JPEG implementation

**Rationale**: Clean separation of concerns, easy to add new formats, easier testing.

### File Structure

```
galaxy_image/
├── Cargo.toml              # Package configuration
├── LICENSE-MIT             # MIT license for this library
├── CLAUDE.md               # This file (context documentation)
├── README.md               # English usage documentation
├── README_FR.md            # French usage documentation
├── LICENSES/               # Dependency licenses
│   ├── png-LICENSE-MIT.txt
│   ├── png-LICENSE-APACHE.txt
│   ├── bmp-LICENSE-MIT.txt
│   ├── jpeg-decoder-LICENSE-MIT.txt
│   ├── jpeg-decoder-LICENSE-APACHE.txt
│   ├── jpeg-encoder-LICENSE-MIT.txt
│   └── jpeg-encoder-LICENSE-APACHE.txt
└── src/
    ├── lib.rs              # Library root and exports
    ├── error.rs            # Error types
    ├── component_type.rs   # ComponentType enum
    ├── pixel_format.rs     # PixelFormat enum
    ├── image_format.rs     # ImageFormat enum with magic byte detection
    ├── image.rs            # Image struct (pixel data container)
    ├── galaxy_image.rs     # GalaxyImage manager/factory
    └── loaders/
        ├── mod.rs          # Loader module exports
        ├── png_loader.rs   # PNG implementation
        ├── bmp_loader.rs   # BMP implementation
        └── jpeg_loader.rs  # JPEG implementation
```

### Design Patterns

#### 1. Factory Pattern (GalaxyImage)
Central point for creating Image instances from various sources.

#### 2. Builder Pattern (Image)
Image struct provides multiple construction methods for different scenarios.

#### 3. Strategy Pattern (Loaders)
Each format has its own loading/saving strategy, selected based on ImageFormat enum.

#### 4. Error Handling
Custom `ImageError` enum with conversions from underlying crate errors.

### Format Support Matrix

| Format | Read | Write | Bit Depths | Alpha | Notes |
|--------|------|-------|------------|-------|-------|
| PNG    | ✅   | ✅    | U8, U16    | ✅    | Full support |
| BMP    | ✅   | ✅    | U8         | ❌    | RGB only |
| JPEG   | ✅   | ✅    | U8         | ❌    | Lossy compression, quality control |

### Known Limitations

1. **BMP**: No alpha channel support (RGBA stripped to RGB on save)
2. **JPEG**: No alpha channel support (RGBA stripped to RGB on save)
3. **JPEG**: Lossy compression (not suitable for lossless workflows)
4. **Indexed color**: Not supported (PNG indexed color rejected)
5. **CMYK**: Not supported (JPEG CMYK rejected)

### Future Enhancements (Potential)

1. **Additional formats**: WebP, TIFF, TGA, DDS
2. **GPU texture formats**: BC1-BC7 compression
3. **HDR formats**: EXR, Radiance HDR
4. **Mipmap generation**: Automatic mipmap chain generation
5. **Resize/transform**: Basic image manipulation operations
6. **Pixel data access**: Iterator-based pixel access for processing
7. **SIMD optimization**: Vectorized format conversions

### Integration with Galaxy3D

This library is designed to integrate with the Galaxy3D engine's texture system:

```rust
// Load texture from file
let image = GalaxyImage::load_from_file("texture.png")?;

// Create GPU texture from image
let texture = renderer.create_texture(TextureDesc {
    width: image.width(),
    height: image.height(),
    format: Format::R8G8B8A8_SRGB,
    data: Some(image.data()),
    ..Default::default()
})?;
```

### Commercial Use

All dependencies use MIT and/or Apache-2.0 licenses, which are:
- ✅ Commercial-friendly
- ✅ No royalties or fees
- ✅ No copyleft requirements
- ✅ Modification allowed
- ✅ Distribution allowed

Only requirement: Include license notices (already done in LICENSES/ folder).

### Testing Strategy

1. **Unit tests**: Test each loader independently
2. **Round-trip tests**: Load → Save → Load, verify data integrity
3. **Format detection tests**: Verify magic byte detection
4. **Conversion tests**: Test BGR↔RGB, alpha stripping, etc.
5. **Error handling tests**: Invalid data, unsupported formats, etc.

### Performance Considerations

1. **Zero-copy where possible**: Data passed as Vec<u8> ownership
2. **Lazy conversion**: Format conversions only when needed
3. **Streaming I/O**: Decoders use streaming interfaces
4. **Memory allocation**: Pre-sized allocations to avoid reallocations

### Maintenance Notes

- Keep dependency versions up to date (check quarterly)
- Monitor for security advisories on image parsing crates
- Test with real-world image files from various sources
- Update licenses when dependencies are updated

## Version History

- **0.1.0** (2026-01-26): Initial implementation
  - PNG support (read/write, U8/U16)
  - BMP support (read/write, U8 only)
  - JPEG support (read/write, U8 only, quality control)
  - Magic byte format detection
  - Automatic format conversion
  - Manager/Factory pattern API
