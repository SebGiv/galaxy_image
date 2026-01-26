use crate::{Image, ImageError, ImageResult, PixelFormat, ComponentType};
use std::io::Cursor;

pub fn load_jpeg(data: &[u8]) -> ImageResult<Image> {
    let mut decoder = jpeg_decoder::Decoder::new(Cursor::new(data));
    let pixels = decoder.decode()?;
    let metadata = decoder.info().ok_or_else(|| {
        ImageError::Other("Failed to get JPEG metadata".to_string())
    })?;

    let width = metadata.width as u32;
    let height = metadata.height as u32;

    // Determine pixel format from JPEG color space
    let pixel_format = match metadata.pixel_format {
        jpeg_decoder::PixelFormat::L8 => PixelFormat::R,
        jpeg_decoder::PixelFormat::L16 => PixelFormat::R,  // 16-bit grayscale
        jpeg_decoder::PixelFormat::RGB24 => PixelFormat::RGB,
        jpeg_decoder::PixelFormat::CMYK32 => {
            return Err(ImageError::UnsupportedFormat(
                "JPEG CMYK format not supported".to_string()
            ));
        }
    };

    Ok(Image::from_raw(
        pixels,
        width,
        height,
        pixel_format,
        ComponentType::U8,
    ))
}

pub fn save_jpeg(image: &Image, quality: u8) -> ImageResult<Vec<u8>> {
    // JPEG only supports U8 component type
    if image.component_type() != ComponentType::U8 {
        return Err(ImageError::UnsupportedFormat(
            "JPEG only supports U8 component type".to_string()
        ));
    }

    // JPEG supports RGB and grayscale
    let (data, color_type) = match image.pixel_format() {
        PixelFormat::R => (image.data().to_vec(), jpeg_encoder::ColorType::Luma),
        PixelFormat::RGB => (image.data().to_vec(), jpeg_encoder::ColorType::Rgb),
        PixelFormat::BGR => {
            // Convert BGR to RGB
            let mut img = image.clone();
            img.bgr_to_rgb();
            (img.into_data(), jpeg_encoder::ColorType::Rgb)
        }
        PixelFormat::RGBA | PixelFormat::BGRA => {
            // Strip alpha channel
            let mut rgb_data = Vec::with_capacity((image.width() * image.height() * 3) as usize);
            let channels = image.pixel_format().channel_count();
            for pixel in image.data().chunks_exact(channels) {
                if image.pixel_format() == PixelFormat::RGBA {
                    rgb_data.push(pixel[0]); // R
                    rgb_data.push(pixel[1]); // G
                    rgb_data.push(pixel[2]); // B
                } else {
                    // BGRA -> RGB
                    rgb_data.push(pixel[2]); // R
                    rgb_data.push(pixel[1]); // G
                    rgb_data.push(pixel[0]); // B
                }
            }
            (rgb_data, jpeg_encoder::ColorType::Rgb)
        }
        PixelFormat::RG => {
            // Use only the first channel (grayscale)
            let mut gray_data = Vec::with_capacity((image.width() * image.height()) as usize);
            for pixel in image.data().chunks_exact(2) {
                gray_data.push(pixel[0]);
            }
            (gray_data, jpeg_encoder::ColorType::Luma)
        }
    };

    // Encode JPEG
    let mut buffer = Vec::new();
    let encoder = jpeg_encoder::Encoder::new(&mut buffer, quality);

    encoder
        .encode(&data, image.width() as u16, image.height() as u16, color_type)
        .map_err(|e| ImageError::JpegEncodeError(format!("{:?}", e)))?;

    Ok(buffer)
}
