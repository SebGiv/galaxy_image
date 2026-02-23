//! # galaxy_image
//!
//! Image loading and saving library for the Galaxy3D engine.
//!
//! Supports PNG, BMP, JPEG, and EXR (HDR) formats with automatic format detection.
//!
//! ## Features
//!
//! - **Multiple formats**: PNG, BMP, JPEG, EXR (HDR)
//! - **Automatic detection**: Magic byte recognition for format detection
//! - **Simple API**: Manager/Factory pattern with `GalaxyImage`
//! - **Flexible pixel formats**: RGB, RGBA, BGR, BGRA, Grayscale
//! - **Type safety**: Strongly typed pixel formats and component types
//!
//! ## Example
//!
//! ```no_run
//! use galaxy_image::{GalaxyImage, ImageFormat};
//!
//! // Load an image (format auto-detected)
//! let image = GalaxyImage::load_from_file("texture.png").unwrap();
//! println!("Loaded {}x{} image", image.width(), image.height());
//!
//! // Save to different format
//! GalaxyImage::save_to_file(&image, "output.jpg", ImageFormat::Jpeg).unwrap();
//! ```

mod error;
mod component_type;
mod pixel_format;
mod image_format;
mod image;
mod galaxy_image;
mod loaders;

pub use error::{ImageError, ImageResult};
pub use component_type::ComponentType;
pub use pixel_format::PixelFormat;
pub use image_format::ImageFormat;
pub use image::Image;
pub use galaxy_image::GalaxyImage;
