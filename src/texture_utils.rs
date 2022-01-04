use png;
use std::io::Write;

pub fn write_texture_as_png<W: Write>(pixels: &[u8], texture_size: u32, destination: &mut W) -> Result<(), ()> {
    let mut encoder = png::Encoder::new(destination, texture_size, texture_size);
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);

    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(pixels).unwrap();

    Ok(())
}