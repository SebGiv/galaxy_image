/// Simple test example for galaxy_image
///
/// This example creates a simple gradient image and saves it in multiple formats.

use galaxy_image::{GalaxyImage, Image, PixelFormat, ComponentType, ImageFormat};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("galaxy_image - Simple Test Example");
    println!("===================================\n");

    // Create a simple 256x256 RGB gradient image
    println!("Creating 256x256 RGB gradient image...");
    let mut image = Image::new(256, 256, PixelFormat::RGB, ComponentType::U8);

    // Fill with gradient
    {
        let data = image.data_mut();
        for y in 0..256 {
            for x in 0..256 {
                let idx = (y * 256 + x) * 3;
                data[idx] = x as u8;       // Red increases left to right
                data[idx + 1] = y as u8;   // Green increases top to bottom
                data[idx + 2] = 128;       // Blue constant
            }
        }
    }

    println!("Image created: {}x{} pixels, {:?} format\n",
        image.width(), image.height(), image.pixel_format());

    // Save as PNG
    println!("Saving as PNG...");
    GalaxyImage::save_to_file(&image, "test_gradient.png", ImageFormat::Png)?;
    println!("✓ Saved: test_gradient.png");

    // Save as BMP
    println!("Saving as BMP...");
    GalaxyImage::save_to_file(&image, "test_gradient.bmp", ImageFormat::Bmp)?;
    println!("✓ Saved: test_gradient.bmp");

    // Save as JPEG (different qualities)
    println!("Saving as JPEG (quality: 90)...");
    GalaxyImage::save_to_file_with_quality(&image, "test_gradient_q90.jpg", ImageFormat::Jpeg, 90)?;
    println!("✓ Saved: test_gradient_q90.jpg");

    println!("Saving as JPEG (quality: 50)...");
    GalaxyImage::save_to_file_with_quality(&image, "test_gradient_q50.jpg", ImageFormat::Jpeg, 50)?;
    println!("✓ Saved: test_gradient_q50.jpg");

    println!("\nLoading back PNG to verify...");
    let loaded = GalaxyImage::load_from_file("test_gradient.png")?;
    println!("✓ Loaded: {}x{} pixels, {:?} format",
        loaded.width(), loaded.height(), loaded.pixel_format());

    // Verify dimensions match
    assert_eq!(loaded.width(), image.width());
    assert_eq!(loaded.height(), image.height());
    println!("✓ Dimensions match!");

    // Test format detection
    println!("\nTesting format detection from bytes...");
    let png_bytes = std::fs::read("test_gradient.png")?;
    let bmp_bytes = std::fs::read("test_gradient.bmp")?;
    let jpg_bytes = std::fs::read("test_gradient_q90.jpg")?;

    let png_format = ImageFormat::detect_from_bytes(&png_bytes);
    let bmp_format = ImageFormat::detect_from_bytes(&bmp_bytes);
    let jpg_format = ImageFormat::detect_from_bytes(&jpg_bytes);

    println!("✓ PNG detected: {:?}", png_format);
    println!("✓ BMP detected: {:?}", bmp_format);
    println!("✓ JPEG detected: {:?}", jpg_format);

    assert_eq!(png_format, ImageFormat::Png);
    assert_eq!(bmp_format, ImageFormat::Bmp);
    assert_eq!(jpg_format, ImageFormat::Jpeg);

    println!("\n✅ All tests passed!");
    Ok(())
}
