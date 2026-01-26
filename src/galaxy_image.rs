use crate::{Image, ImageError, ImageFormat, ImageResult};
use crate::loaders::{load_png, save_png, load_bmp, save_bmp, load_jpeg, save_jpeg};
use std::fs;
use std::path::Path;

/// Main manager/factory for image operations
///
/// This is the primary interface for loading and saving images.
/// All methods are static, making this a singleton-like pattern.
pub struct GalaxyImage;

impl GalaxyImage {
    /// Load an image from a file path
    ///
    /// Format is automatically detected from magic bytes in the file content.
    /// Falls back to extension-based detection if magic bytes are not recognized.
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the image file
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use galaxy_image::GalaxyImage;
    ///
    /// let image = GalaxyImage::load_from_file("texture.png").unwrap();
    /// println!("Loaded {}x{} image", image.width(), image.height());
    /// ```
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> ImageResult<Image> {
        let bytes = fs::read(&path)?;

        // Detect format from magic bytes
        let mut format = ImageFormat::detect_from_bytes(&bytes);

        // Fallback to extension if magic bytes didn't work
        if format == ImageFormat::Unknown {
            format = ImageFormat::from_extension(
                path.as_ref().to_str().unwrap_or("")
            );
        }

        Self::load_from_bytes(&bytes, format)
    }

    /// Load an image from a byte buffer
    ///
    /// Format must be explicitly specified.
    ///
    /// # Arguments
    ///
    /// * `bytes` - Raw image file data
    /// * `format` - Image format (PNG, BMP, JPEG)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use galaxy_image::{GalaxyImage, ImageFormat};
    ///
    /// let bytes = vec![/* ... */];
    /// let image = GalaxyImage::load_from_bytes(&bytes, ImageFormat::Png).unwrap();
    /// ```
    pub fn load_from_bytes(bytes: &[u8], format: ImageFormat) -> ImageResult<Image> {
        match format {
            ImageFormat::Png => load_png(bytes),
            ImageFormat::Bmp => load_bmp(bytes),
            ImageFormat::Jpeg => load_jpeg(bytes),
            ImageFormat::Unknown => {
                Err(ImageError::UnsupportedFormat("Unknown format".to_string()))
            }
        }
    }

    /// Load an image from a byte buffer with automatic format detection
    ///
    /// Format is detected from magic bytes.
    ///
    /// # Arguments
    ///
    /// * `bytes` - Raw image file data
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use galaxy_image::GalaxyImage;
    ///
    /// let bytes = vec![/* ... */];
    /// let image = GalaxyImage::load_from_bytes_auto(&bytes).unwrap();
    /// ```
    pub fn load_from_bytes_auto(bytes: &[u8]) -> ImageResult<Image> {
        let format = ImageFormat::detect_from_bytes(bytes);
        Self::load_from_bytes(bytes, format)
    }

    /// Save an image to a file
    ///
    /// # Arguments
    ///
    /// * `image` - Image to save
    /// * `path` - Output file path
    /// * `format` - Output format (PNG, BMP, JPEG)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use galaxy_image::{GalaxyImage, ImageFormat};
    /// # use galaxy_image::Image;
    /// # let image = Image::new(100, 100, galaxy_image::PixelFormat::RGB, galaxy_image::ComponentType::U8);
    ///
    /// GalaxyImage::save_to_file(&image, "output.png", ImageFormat::Png).unwrap();
    /// ```
    pub fn save_to_file<P: AsRef<Path>>(
        image: &Image,
        path: P,
        format: ImageFormat,
    ) -> ImageResult<()> {
        let bytes = Self::save_to_bytes(image, format, 90)?;
        fs::write(path, bytes)?;
        Ok(())
    }

    /// Save an image to a file with JPEG quality parameter
    ///
    /// # Arguments
    ///
    /// * `image` - Image to save
    /// * `path` - Output file path
    /// * `format` - Output format (PNG, BMP, JPEG)
    /// * `jpeg_quality` - JPEG quality (1-100, only used for JPEG format)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use galaxy_image::{GalaxyImage, ImageFormat};
    /// # use galaxy_image::Image;
    /// # let image = Image::new(100, 100, galaxy_image::PixelFormat::RGB, galaxy_image::ComponentType::U8);
    ///
    /// GalaxyImage::save_to_file_with_quality(&image, "output.jpg", ImageFormat::Jpeg, 95).unwrap();
    /// ```
    pub fn save_to_file_with_quality<P: AsRef<Path>>(
        image: &Image,
        path: P,
        format: ImageFormat,
        jpeg_quality: u8,
    ) -> ImageResult<()> {
        let bytes = Self::save_to_bytes(image, format, jpeg_quality)?;
        fs::write(path, bytes)?;
        Ok(())
    }

    /// Save an image to a byte buffer
    ///
    /// # Arguments
    ///
    /// * `image` - Image to save
    /// * `format` - Output format (PNG, BMP, JPEG)
    /// * `jpeg_quality` - JPEG quality (1-100, only used for JPEG format)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use galaxy_image::{GalaxyImage, ImageFormat};
    /// # use galaxy_image::Image;
    /// # let image = Image::new(100, 100, galaxy_image::PixelFormat::RGB, galaxy_image::ComponentType::U8);
    ///
    /// let bytes = GalaxyImage::save_to_bytes(&image, ImageFormat::Png, 90).unwrap();
    /// ```
    pub fn save_to_bytes(
        image: &Image,
        format: ImageFormat,
        jpeg_quality: u8,
    ) -> ImageResult<Vec<u8>> {
        match format {
            ImageFormat::Png => save_png(image),
            ImageFormat::Bmp => save_bmp(image),
            ImageFormat::Jpeg => save_jpeg(image, jpeg_quality.clamp(1, 100)),
            ImageFormat::Unknown => {
                Err(ImageError::UnsupportedFormat("Unknown format".to_string()))
            }
        }
    }
}
