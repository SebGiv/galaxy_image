use crate::{ComponentType, PixelFormat};

/// Image data container
#[derive(Debug, Clone)]
pub struct Image {
    /// Raw pixel data
    data: Vec<u8>,

    /// Image width in pixels
    width: u32,

    /// Image height in pixels
    height: u32,

    /// Pixel format (channel layout)
    pixel_format: PixelFormat,

    /// Component type (U8, U16, F32)
    component_type: ComponentType,
}

impl Image {
    /// Create a new image with specified dimensions and format
    pub fn new(
        width: u32,
        height: u32,
        pixel_format: PixelFormat,
        component_type: ComponentType,
    ) -> Self {
        let bytes_per_pixel = pixel_format.channel_count() * component_type.size_bytes();
        let total_bytes = (width as usize) * (height as usize) * bytes_per_pixel;

        Self {
            data: vec![0; total_bytes],
            width,
            height,
            pixel_format,
            component_type,
        }
    }

    /// Create an image from raw pixel data
    pub fn from_raw(
        data: Vec<u8>,
        width: u32,
        height: u32,
        pixel_format: PixelFormat,
        component_type: ComponentType,
    ) -> Self {
        Self {
            data,
            width,
            height,
            pixel_format,
            component_type,
        }
    }

    /// Get image width
    pub fn width(&self) -> u32 {
        self.width
    }

    /// Get image height
    pub fn height(&self) -> u32 {
        self.height
    }

    /// Get pixel format
    pub fn pixel_format(&self) -> PixelFormat {
        self.pixel_format
    }

    /// Get component type
    pub fn component_type(&self) -> ComponentType {
        self.component_type
    }

    /// Get raw pixel data as slice
    pub fn data(&self) -> &[u8] {
        &self.data
    }

    /// Get mutable raw pixel data
    pub fn data_mut(&mut self) -> &mut [u8] {
        &mut self.data
    }

    /// Get pixel data as owned Vec
    pub fn into_data(self) -> Vec<u8> {
        self.data
    }

    /// Calculate bytes per pixel
    pub fn bytes_per_pixel(&self) -> usize {
        self.pixel_format.channel_count() * self.component_type.size_bytes()
    }

    /// Calculate total size in bytes
    pub fn size_bytes(&self) -> usize {
        self.data.len()
    }

    /// Convert BGR to RGB in-place (for BMP files)
    pub fn bgr_to_rgb(&mut self) {
        if self.pixel_format != PixelFormat::BGR && self.pixel_format != PixelFormat::BGRA {
            return;
        }

        let channels = self.pixel_format.channel_count();
        for pixel in self.data.chunks_exact_mut(channels) {
            pixel.swap(0, 2); // Swap B and R channels
        }

        // Update pixel format
        self.pixel_format = if self.pixel_format == PixelFormat::BGR {
            PixelFormat::RGB
        } else {
            PixelFormat::RGBA
        };
    }

    /// Convert RGB to BGR in-place (for saving to BMP)
    pub fn rgb_to_bgr(&mut self) {
        if self.pixel_format != PixelFormat::RGB && self.pixel_format != PixelFormat::RGBA {
            return;
        }

        let channels = self.pixel_format.channel_count();
        for pixel in self.data.chunks_exact_mut(channels) {
            pixel.swap(0, 2); // Swap R and B channels
        }

        // Update pixel format
        self.pixel_format = if self.pixel_format == PixelFormat::RGB {
            PixelFormat::BGR
        } else {
            PixelFormat::BGRA
        };
    }
}
