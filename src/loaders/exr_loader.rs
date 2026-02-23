use crate::{Image, ImageError, ImageResult, PixelFormat, ComponentType};
use std::io::Cursor;

// Selective imports to avoid name conflict with our Image type
use exr::image::read::read;
use exr::image::{
    Image as ExrImage, Layer, AnyChannels, AnyChannel, FlatSamples,
};
use exr::math::Vec2;
use exr::meta::header::{ImageAttributes, LayerAttributes};
use exr::meta::attribute::{IntegerBounds, Text};
use exr::image::Encoding;
use exr::prelude::{f16, ReadChannels, ReadLayers, WritableImage};

/// Load an EXR image from raw bytes
///
/// Reads the first layer and detects channels (R/G/B/A/Y).
/// Supports F16, F32, and U32 (converted to F32) sample types.
pub fn load_exr(data: &[u8]) -> ImageResult<Image> {
    let reader = Cursor::new(data);

    // Read all channels from the first valid layer
    let exr_image = read()
        .no_deep_data()
        .largest_resolution_level()
        .all_channels()
        .first_valid_layer()
        .all_attributes()
        .from_buffered(reader)?;

    let layer = &exr_image.layer_data;
    let width = layer.size.0 as u32;
    let height = layer.size.1 as u32;
    let channels = &layer.channel_data.list;

    if channels.is_empty() {
        return Err(ImageError::Other(
            "EXR file contains no channels".to_string(),
        ));
    }

    // Find channels by name
    let find_channel = |name: &str| -> Option<usize> {
        channels.iter().position(|c| c.name.to_string() == name)
    };

    let r_idx = find_channel("R");
    let g_idx = find_channel("G");
    let b_idx = find_channel("B");
    let a_idx = find_channel("A");
    let y_idx = find_channel("Y");

    // Determine pixel format and channel mapping
    let (pixel_format, channel_indices) = match (r_idx, g_idx, b_idx, a_idx, y_idx) {
        (Some(r), Some(g), Some(b), Some(a), _) => (PixelFormat::RGBA, vec![r, g, b, a]),
        (Some(r), Some(g), Some(b), None, _) => (PixelFormat::RGB, vec![r, g, b]),
        (_, _, _, Some(a), Some(y)) => (PixelFormat::RG, vec![y, a]),
        (_, _, _, None, Some(y)) => (PixelFormat::R, vec![y]),
        (Some(r), _, _, Some(a), _) => (PixelFormat::RG, vec![r, a]),
        (Some(r), _, _, None, _) => (PixelFormat::R, vec![r]),
        _ => {
            return Err(ImageError::UnsupportedFormat(
                "No recognized channels (R/G/B/A/Y) in EXR file".to_string(),
            ));
        }
    };

    // Determine component type from channel sample types
    let first_sample = &channels[channel_indices[0]].sample_data;
    let all_same_type = channel_indices.iter().all(|&idx| {
        std::mem::discriminant(&channels[idx].sample_data)
            == std::mem::discriminant(first_sample)
    });

    let (component_type, force_f32) = if all_same_type {
        match first_sample {
            FlatSamples::F16(_) => (ComponentType::F16, false),
            FlatSamples::F32(_) => (ComponentType::F32, false),
            FlatSamples::U32(_) => (ComponentType::F32, true),
        }
    } else {
        // Mixed sample types: convert everything to F32
        (ComponentType::F32, true)
    };

    // Build interleaved pixel data
    let pixel_count = (width as usize) * (height as usize);
    let channel_count = pixel_format.channel_count();
    let bytes_per_component = component_type.size_bytes();
    let total_bytes = pixel_count * channel_count * bytes_per_component;
    let mut output = vec![0u8; total_bytes];

    for pixel_idx in 0..pixel_count {
        for (ch_out, &ch_in) in channel_indices.iter().enumerate() {
            let byte_offset = (pixel_idx * channel_count + ch_out) * bytes_per_component;
            let sample = &channels[ch_in].sample_data;

            if force_f32 {
                // Convert all sample types to F32
                let value: f32 = match sample {
                    FlatSamples::F16(s) => s[pixel_idx].to_f32(),
                    FlatSamples::F32(s) => s[pixel_idx],
                    FlatSamples::U32(s) => s[pixel_idx] as f32,
                };
                output[byte_offset..byte_offset + 4]
                    .copy_from_slice(&value.to_le_bytes());
            } else {
                // Keep native format
                match sample {
                    FlatSamples::F16(s) => {
                        output[byte_offset..byte_offset + 2]
                            .copy_from_slice(&s[pixel_idx].to_le_bytes());
                    }
                    FlatSamples::F32(s) => {
                        output[byte_offset..byte_offset + 4]
                            .copy_from_slice(&s[pixel_idx].to_le_bytes());
                    }
                    FlatSamples::U32(_) => unreachable!(),
                }
            }
        }
    }

    Ok(Image::from_raw(output, width, height, pixel_format, component_type))
}

/// Save an image as EXR format bytes
///
/// Supports F16 and F32 component types. Uses ZIP compression (lossless).
/// Automatically handles BGR/BGRA to RGB/RGBA conversion.
pub fn save_exr(image: &Image) -> ImageResult<Vec<u8>> {
    // EXR only supports F16 and F32
    match image.component_type() {
        ComponentType::F16 | ComponentType::F32 => {}
        other => {
            return Err(ImageError::UnsupportedFormat(
                format!("EXR does not support {:?} component type, use F16 or F32", other),
            ));
        }
    }

    let width = image.width() as usize;
    let height = image.height() as usize;
    let pixel_count = width * height;
    let data = image.data();
    let bytes_per_component = image.component_type().size_bytes();
    let source_channel_count = image.pixel_format().channel_count();

    // Map pixel format to EXR channel names and source channel indices
    // BGR/BGRA are handled by remapping source indices (no clone needed)
    let (channel_names, source_indices): (&[&str], &[usize]) = match image.pixel_format() {
        PixelFormat::R => (&["Y"], &[0]),
        PixelFormat::RG => (&["Y", "A"], &[0, 1]),
        PixelFormat::RGB => (&["R", "G", "B"], &[0, 1, 2]),
        PixelFormat::RGBA => (&["R", "G", "B", "A"], &[0, 1, 2, 3]),
        PixelFormat::BGR => (&["R", "G", "B"], &[2, 1, 0]),
        PixelFormat::BGRA => (&["R", "G", "B", "A"], &[2, 1, 0, 3]),
    };

    // De-interleave pixel data into separate EXR channels
    let mut channel_list: Vec<AnyChannel<FlatSamples>> = channel_names
        .iter()
        .zip(source_indices.iter())
        .map(|(name, &src_ch)| {
            let sample_data = match image.component_type() {
                ComponentType::F16 => {
                    let samples: Vec<f16> = (0..pixel_count)
                        .map(|pixel_idx| {
                            let offset =
                                (pixel_idx * source_channel_count + src_ch) * bytes_per_component;
                            f16::from_le_bytes([data[offset], data[offset + 1]])
                        })
                        .collect();
                    FlatSamples::F16(samples)
                }
                ComponentType::F32 => {
                    let samples: Vec<f32> = (0..pixel_count)
                        .map(|pixel_idx| {
                            let offset =
                                (pixel_idx * source_channel_count + src_ch) * bytes_per_component;
                            f32::from_le_bytes([
                                data[offset],
                                data[offset + 1],
                                data[offset + 2],
                                data[offset + 3],
                            ])
                        })
                        .collect();
                    FlatSamples::F32(samples)
                }
                _ => unreachable!(),
            };

            AnyChannel {
                name: Text::new_or_panic(name),
                sample_data,
                quantize_linearly: false,
                sampling: Vec2(1, 1),
            }
        })
        .collect();

    // EXR spec requires channels sorted alphabetically by name
    channel_list.sort_by(|a, b| a.name.to_string().cmp(&b.name.to_string()));

    let exr_channels = AnyChannels {
        list: channel_list.into_iter().collect(),
    };

    // Build EXR image structure
    let exr_image = ExrImage {
        attributes: ImageAttributes::new(IntegerBounds::from_dimensions(Vec2(width, height))),
        layer_data: Layer {
            channel_data: exr_channels,
            attributes: LayerAttributes::named(Text::new_or_panic("main")),
            size: Vec2(width, height),
            encoding: Encoding::FAST_LOSSLESS,
        },
    };

    // Write to buffer
    let mut cursor = Cursor::new(Vec::new());
    exr_image.write().to_buffered(&mut cursor)?;
    Ok(cursor.into_inner())
}
