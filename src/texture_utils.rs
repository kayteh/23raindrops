use super::texture_gen::{InterpolatorBlock, Pixel};
use image::io::Reader as ImageReader;
use png;
use std::io::{Error, Write};

pub fn write_texture_as_png<W: Write>(
    pixels: &[u8],
    texture_size: u32,
    destination: &mut W,
) -> Result<(), ()> {
    let mut encoder = png::Encoder::new(destination, texture_size, texture_size);
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);

    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(pixels).unwrap();

    Ok(())
}

pub fn read_texture_from_png(path: &str) -> Result<image::RgbaImage, ()> {
    let img = ImageReader::open(path).unwrap().decode().unwrap();
    Ok(img.to_rgba8())
}

pub fn interleave_texture(original: &mut Vec<InterpolatorBlock>, input: image::RgbaImage) {
    let interleave = input.pixels().map(|p| p).collect::<Vec<_>>();
    for block_index in 0..original.len() {
        let pixel = &interleave[block_index];
        original[block_index].pixels[3] = Pixel {
            r: pixel[0],
            g: pixel[1],
            b: pixel[2],
            a: pixel[3],
        };
    }
}
