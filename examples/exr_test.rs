/// EXR format test example for galaxy_image
///
/// Tests EXR creation, save, reload, and format detection.

use galaxy_image::{GalaxyImage, Image, ImageFormat, PixelFormat, ComponentType};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("galaxy_image - EXR Format Test");
    println!("==============================\n");

    // 1. Create an F32 RGB gradient image
    println!("Creating 256x256 F32 RGB gradient...");
    let width = 256u32;
    let height = 256u32;
    let mut image = Image::new(width, height, PixelFormat::RGB, ComponentType::F32);
    {
        let data = image.data_mut();
        for y in 0..height as usize {
            for x in 0..width as usize {
                let offset = (y * width as usize + x) * 3 * 4; // 3 channels * 4 bytes
                let r = (x as f32) / (width as f32 - 1.0);
                let g = (y as f32) / (height as f32 - 1.0);
                let b: f32 = 2.5; // HDR value > 1.0
                data[offset..offset + 4].copy_from_slice(&r.to_le_bytes());
                data[offset + 4..offset + 8].copy_from_slice(&g.to_le_bytes());
                data[offset + 8..offset + 12].copy_from_slice(&b.to_le_bytes());
            }
        }
    }
    println!("  Created: {}x{} | {:?} | {:?}", image.width(), image.height(), image.pixel_format(), image.component_type());

    // 2. Save as EXR
    println!("\nSaving as EXR...");
    GalaxyImage::save_to_file(&image, "test_gradient.exr", ImageFormat::Exr)?;
    println!("  Saved: test_gradient.exr");

    // 3. Reload and verify
    println!("\nReloading EXR...");
    let reloaded = GalaxyImage::load_from_file("test_gradient.exr")?;
    println!(
        "  Loaded: {}x{} | {:?} | {:?} | {} bytes",
        reloaded.width(), reloaded.height(),
        reloaded.pixel_format(), reloaded.component_type(),
        reloaded.size_bytes()
    );
    assert_eq!(reloaded.width(), width);
    assert_eq!(reloaded.height(), height);
    assert_eq!(reloaded.pixel_format(), PixelFormat::RGB);
    assert_eq!(reloaded.component_type(), ComponentType::F32);

    // Verify HDR value preserved
    let data = reloaded.data();
    let b_value = f32::from_le_bytes([data[8], data[9], data[10], data[11]]);
    assert!((b_value - 2.5).abs() < 0.001, "HDR value not preserved: {}", b_value);
    println!("  HDR value preserved: B = {}", b_value);

    // 4. Test format detection
    println!("\nTesting EXR magic bytes detection...");
    let exr_bytes = std::fs::read("test_gradient.exr")?;
    let detected = ImageFormat::detect_from_bytes(&exr_bytes);
    assert_eq!(detected, ImageFormat::Exr);
    println!("  Detected: {:?}", detected);

    // 5. Test RGBA with F16
    println!("\nCreating 64x64 F16 RGBA image...");
    let mut rgba_img = Image::new(64, 64, PixelFormat::RGBA, ComponentType::F16);
    {
        let data = rgba_img.data_mut();
        // Fill with constant color using f16 bytes
        // f16 for 1.0 = 0x3C00 (little-endian: 0x00, 0x3C)
        for pixel in data.chunks_exact_mut(8) { // 4 channels * 2 bytes
            pixel[0] = 0x00; pixel[1] = 0x3C; // R = 1.0
            pixel[2] = 0x00; pixel[3] = 0x00; // G = 0.0
            pixel[4] = 0x00; pixel[5] = 0x3C; // B = 1.0
            pixel[6] = 0x00; pixel[7] = 0x3C; // A = 1.0
        }
    }
    GalaxyImage::save_to_file(&rgba_img, "test_f16_rgba.exr", ImageFormat::Exr)?;
    let reloaded_f16 = GalaxyImage::load_from_file("test_f16_rgba.exr")?;
    assert_eq!(reloaded_f16.pixel_format(), PixelFormat::RGBA);
    assert_eq!(reloaded_f16.component_type(), ComponentType::F16);
    println!("  F16 RGBA round-trip OK: {}x{}", reloaded_f16.width(), reloaded_f16.height());

    // 6. Try loading real EXR textures (may fail if DWAA compressed)
    println!("\nTrying real EXR textures (DWAA compression)...");
    let exr_files = [
        "F:/dev/rust/Galaxy/Games/galaxy3d_scene_demo/assets/mesh/lion_head/textures/lion_head_metal_4k.exr",
        "F:/dev/rust/Galaxy/Games/galaxy3d_scene_demo/assets/mesh/lion_head/textures/lion_head_nor_gl_4k.exr",
        "F:/dev/rust/Galaxy/Games/galaxy3d_scene_demo/assets/mesh/lion_head/textures/lion_head_rough_4k.exr",
    ];
    for path in &exr_files {
        let filename = path.rsplit('/').next().unwrap_or(path);
        match GalaxyImage::load_from_file(path) {
            Ok(img) => println!("  {} -> {}x{} {:?} {:?}", filename, img.width(), img.height(), img.pixel_format(), img.component_type()),
            Err(e) => println!("  {} -> {}", filename, e),
        }
    }

    // Cleanup
    let _ = std::fs::remove_file("test_gradient.exr");
    let _ = std::fs::remove_file("test_f16_rgba.exr");

    println!("\nAll EXR tests passed!");
    Ok(())
}
