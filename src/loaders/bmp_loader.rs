use crate::{Image, ImageError, ImageResult, PixelFormat, ComponentType};
use std::io::Cursor;

pub fn load_bmp(data: &[u8]) -> ImageResult<Image> {
    let img = bmp::from_reader(&mut Cursor::new(data))?;

    let width = img.get_width();
    let height = img.get_height();

    // BMP pixels are stored as BGR(A)
    let has_alpha = false; // bmp crate doesn't expose alpha info easily
    let pixel_format = if has_alpha {
        PixelFormat::BGRA
    } else {
        PixelFormat::BGR
    };

    // Convert bmp pixels to raw bytes
    let mut data = Vec::with_capacity((width * height * 3) as usize);

    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y);
            data.push(pixel.b);
            data.push(pixel.g);
            data.push(pixel.r);
        }
    }

    let mut image = Image::from_raw(
        data,
        width,
        height,
        pixel_format,
        ComponentType::U8,
    );

    // Convert BGR to RGB for consistency
    image.bgr_to_rgb();

    Ok(image)
}

pub fn save_bmp(image: &Image) -> ImageResult<Vec<u8>> {
    // BMP only supports U8 component type
    if image.component_type() != ComponentType::U8 {
        return Err(ImageError::UnsupportedFormat(
            "BMP only supports U8 component type".to_string()
        ));
    }

    // BMP only supports RGB format (no alpha)
    let image_rgb = match image.pixel_format() {
        PixelFormat::RGB => image.clone(),
        PixelFormat::BGR => {
            let mut img = image.clone();
            img.bgr_to_rgb();
            img
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
            Image::from_raw(
                rgb_data,
                image.width(),
                image.height(),
                PixelFormat::RGB,
                ComponentType::U8,
            )
        }
        PixelFormat::R => {
            // Convert grayscale to RGB
            let mut rgb_data = Vec::with_capacity((image.width() * image.height() * 3) as usize);
            for gray in image.data() {
                rgb_data.push(*gray);
                rgb_data.push(*gray);
                rgb_data.push(*gray);
            }
            Image::from_raw(
                rgb_data,
                image.width(),
                image.height(),
                PixelFormat::RGB,
                ComponentType::U8,
            )
        }
        PixelFormat::RG => {
            // Convert grayscale+alpha to RGB (ignore alpha)
            let mut rgb_data = Vec::with_capacity((image.width() * image.height() * 3) as usize);
            for pixel in image.data().chunks_exact(2) {
                let gray = pixel[0];
                rgb_data.push(gray);
                rgb_data.push(gray);
                rgb_data.push(gray);
            }
            Image::from_raw(
                rgb_data,
                image.width(),
                image.height(),
                PixelFormat::RGB,
                ComponentType::U8,
            )
        }
    };

    // Create BMP image
    let mut bmp_img = bmp::Image::new(image_rgb.width(), image_rgb.height());

    let rgb_data = image_rgb.data();
    for y in 0..image_rgb.height() {
        for x in 0..image_rgb.width() {
            let idx = ((y * image_rgb.width() + x) * 3) as usize;
            let r = rgb_data[idx];
            let g = rgb_data[idx + 1];
            let b = rgb_data[idx + 2];
            bmp_img.set_pixel(x, y, bmp::Pixel::new(r, g, b));
        }
    }

    // Write to buffer
    let mut buffer = Vec::new();
    bmp_img.to_writer(&mut buffer)?;

    Ok(buffer)
}
