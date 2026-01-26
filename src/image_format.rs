/// Supported image file formats
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ImageFormat {
    /// PNG format
    Png,

    /// BMP format
    Bmp,

    /// JPEG format
    Jpeg,

    /// Unknown or unsupported format
    Unknown,
}

impl ImageFormat {
    /// Detect format from file magic bytes
    pub fn detect_from_bytes(data: &[u8]) -> Self {
        if data.len() < 8 {
            return ImageFormat::Unknown;
        }

        // PNG: 0x89 'P' 'N' 'G' 0x0D 0x0A 0x1A 0x0A
        if data.len() >= 8 && data[0..8] == [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A] {
            return ImageFormat::Png;
        }

        // BMP: 'B' 'M' (0x42 0x4D)
        if data.len() >= 2 && data[0..2] == [0x42, 0x4D] {
            return ImageFormat::Bmp;
        }

        // JPEG: 0xFF 0xD8 (SOI marker)
        if data.len() >= 2 && data[0..2] == [0xFF, 0xD8] {
            return ImageFormat::Jpeg;
        }

        ImageFormat::Unknown
    }

    /// Detect format from file extension
    pub fn from_extension(path: &str) -> Self {
        let path_lower = path.to_lowercase();

        if path_lower.ends_with(".png") {
            ImageFormat::Png
        } else if path_lower.ends_with(".bmp") {
            ImageFormat::Bmp
        } else if path_lower.ends_with(".jpg") || path_lower.ends_with(".jpeg") {
            ImageFormat::Jpeg
        } else {
            ImageFormat::Unknown
        }
    }

    /// Get file extension for this format
    pub fn extension(&self) -> &'static str {
        match self {
            ImageFormat::Png => "png",
            ImageFormat::Bmp => "bmp",
            ImageFormat::Jpeg => "jpg",
            ImageFormat::Unknown => "",
        }
    }
}
