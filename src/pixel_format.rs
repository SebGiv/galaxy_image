/// Pixel format describing channel layout
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PixelFormat {
    /// Single channel (grayscale)
    R,

    /// Two channels (grayscale + alpha)
    RG,

    /// Three channels (red, green, blue)
    RGB,

    /// Four channels (red, green, blue, alpha)
    RGBA,

    /// Three channels (blue, green, red) - used by BMP
    BGR,

    /// Four channels (blue, green, red, alpha)
    BGRA,
}

impl PixelFormat {
    /// Number of channels in this pixel format
    pub fn channel_count(&self) -> usize {
        match self {
            PixelFormat::R => 1,
            PixelFormat::RG => 2,
            PixelFormat::RGB | PixelFormat::BGR => 3,
            PixelFormat::RGBA | PixelFormat::BGRA => 4,
        }
    }

    /// Check if format has alpha channel
    pub fn has_alpha(&self) -> bool {
        matches!(self, PixelFormat::RG | PixelFormat::RGBA | PixelFormat::BGRA)
    }
}
