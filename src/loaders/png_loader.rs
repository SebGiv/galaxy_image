use crate::{Image, ImageError, ImageResult, PixelFormat, ComponentType};
use std::io::Cursor;

pub fn load_png(data: &[u8]) -> ImageResult<Image> {
    let decoder = png::Decoder::new(Cursor::new(data));
    let mut reader = decoder.read_info()?;

    let info = reader.info();
    let width = info.width;
    let height = info.height;
    let color_type = info.color_type;
    let bit_depth = info.bit_depth;

    // Determine pixel format
    let pixel_format = match color_type {
        png::ColorType::Grayscale => PixelFormat::R,
        png::ColorType::GrayscaleAlpha => PixelFormat::RG,
        png::ColorType::Rgb => PixelFormat::RGB,
        png::ColorType::Rgba => PixelFormat::RGBA,
        png::ColorType::Indexed => {
            return Err(ImageError::UnsupportedFormat(
                "PNG indexed color not supported".to_string()
            ));
        }
    };

    // Determine component type
    let component_type = match bit_depth {
        png::BitDepth::Eight => ComponentType::U8,
        png::BitDepth::Sixteen => ComponentType::U16,
        _ => {
            return Err(ImageError::UnsupportedFormat(
                format!("PNG bit depth {:?} not supported", bit_depth)
            ));
        }
    };

    // Allocate buffer
    let mut buffer = vec![0u8; reader.output_buffer_size()];
    let info = reader.next_frame(&mut buffer)?;
    let bytes = &buffer[..info.buffer_size()];

    Ok(Image::from_raw(
        bytes.to_vec(),
        width,
        height,
        pixel_format,
        component_type,
    ))
}

pub fn save_png(image: &Image) -> ImageResult<Vec<u8>> {
    // PNG only supports U8 and U16 component types
    if image.component_type() != ComponentType::U8 && image.component_type() != ComponentType::U16 {
        return Err(ImageError::UnsupportedFormat(
            "PNG only supports U8 and U16 component types".to_string()
        ));
    }

    let mut buffer = Vec::new();
    {
        let mut encoder = png::Encoder::new(
            &mut buffer,
            image.width(),
            image.height(),
        );

        // Set color type
        let color_type = match image.pixel_format() {
            PixelFormat::R => png::ColorType::Grayscale,
            PixelFormat::RG => png::ColorType::GrayscaleAlpha,
            PixelFormat::RGB => png::ColorType::Rgb,
            PixelFormat::RGBA => png::ColorType::Rgba,
            PixelFormat::BGR => {
                // Convert BGR to RGB for PNG
                let mut img_copy = image.clone();
                img_copy.bgr_to_rgb();
                return save_png(&img_copy);
            }
            PixelFormat::BGRA => {
                // Convert BGRA to RGBA for PNG
                let mut img_copy = image.clone();
                img_copy.bgr_to_rgb();
                return save_png(&img_copy);
            }
        };

        encoder.set_color(color_type);

        // Set bit depth
        let bit_depth = match image.component_type() {
            ComponentType::U8 => png::BitDepth::Eight,
            ComponentType::U16 => png::BitDepth::Sixteen,
            _ => unreachable!(),
        };
        encoder.set_depth(bit_depth);

        let mut writer = encoder.write_header()?;
        writer.write_image_data(image.data())?;
    }

    Ok(buffer)
}
