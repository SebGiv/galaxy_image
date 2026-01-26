/// Error types for galaxy_image operations
use std::fmt;
use std::io;

#[derive(Debug)]
pub enum ImageError {
    /// I/O error (file not found, permission denied, etc.)
    IoError(io::Error),

    /// PNG decoding error
    PngDecodingError(png::DecodingError),

    /// PNG encoding error
    PngEncodingError(png::EncodingError),

    /// BMP decoding error
    BmpError(bmp::BmpError),

    /// JPEG decoding error
    JpegDecodeError(jpeg_decoder::Error),

    /// JPEG encoding error (string message)
    JpegEncodeError(String),

    /// Unsupported image format
    UnsupportedFormat(String),

    /// Invalid pixel format
    InvalidPixelFormat(String),

    /// Invalid dimensions
    InvalidDimensions { width: u32, height: u32 },

    /// Empty image data
    EmptyData,

    /// Generic error with message
    Other(String),
}

impl fmt::Display for ImageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ImageError::IoError(e) => write!(f, "I/O error: {}", e),
            ImageError::PngDecodingError(e) => write!(f, "PNG decoding error: {:?}", e),
            ImageError::PngEncodingError(e) => write!(f, "PNG encoding error: {:?}", e),
            ImageError::BmpError(e) => write!(f, "BMP error: {:?}", e),
            ImageError::JpegDecodeError(e) => write!(f, "JPEG decode error: {:?}", e),
            ImageError::JpegEncodeError(msg) => write!(f, "JPEG encode error: {}", msg),
            ImageError::UnsupportedFormat(fmt) => write!(f, "Unsupported format: {}", fmt),
            ImageError::InvalidPixelFormat(fmt) => write!(f, "Invalid pixel format: {}", fmt),
            ImageError::InvalidDimensions { width, height } => {
                write!(f, "Invalid dimensions: {}x{}", width, height)
            }
            ImageError::EmptyData => write!(f, "Empty image data"),
            ImageError::Other(msg) => write!(f, "{}", msg),
        }
    }
}

impl std::error::Error for ImageError {}

impl From<io::Error> for ImageError {
    fn from(error: io::Error) -> Self {
        ImageError::IoError(error)
    }
}

impl From<png::DecodingError> for ImageError {
    fn from(error: png::DecodingError) -> Self {
        ImageError::PngDecodingError(error)
    }
}

impl From<png::EncodingError> for ImageError {
    fn from(error: png::EncodingError) -> Self {
        ImageError::PngEncodingError(error)
    }
}

impl From<bmp::BmpError> for ImageError {
    fn from(error: bmp::BmpError) -> Self {
        ImageError::BmpError(error)
    }
}

impl From<jpeg_decoder::Error> for ImageError {
    fn from(error: jpeg_decoder::Error) -> Self {
        ImageError::JpegDecodeError(error)
    }
}

pub type ImageResult<T> = Result<T, ImageError>;
