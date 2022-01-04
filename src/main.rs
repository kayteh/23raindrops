#[macro_use]
extern crate glium;

use std::fs::OpenOptions;
use std::path::Path;
use std::io::BufWriter;

mod generate_interpolators;
mod graphics;
mod texture_gen;
mod texture_utils;

fn main() {
    println!("Building beatmap texture...");
    let path = Path::new("example.png");
    let mut texture = Vec::<u8>::new();
    
    let image_size = 512;
    let interpolator_count = 16;
    
    let blocks = generate_interpolators::sine_interpolation(image_size, interpolator_count, 1.0);
    let pixels = texture_gen::pixels_from_interpolator_blocks(image_size, blocks);
    texture_gen::output_texture(pixels, &mut texture).unwrap();
    println!("Created texture with {} bytes (input size={}, output size={})", texture.len(), image_size, (image_size as f32/4.0).sqrt());
    
    let mut file = OpenOptions::new().write(true).create(true).truncate(true).open(path).unwrap();
    let mut file_write_buf = BufWriter::new(&mut file);
    texture_utils::write_texture_as_png(&texture, image_size, &mut file_write_buf).unwrap();
    println!("Wrote texture to {}", path.display());

    println!("Starting renderer...");
    graphics::start_graphics(image_size, texture);
}