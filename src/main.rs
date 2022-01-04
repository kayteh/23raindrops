#[macro_use]
extern crate glium;

use std::fs::OpenOptions;
use std::path::Path;
use std::io::Write;

mod graphics;
mod texture_gen;
mod generate_interpolators;

fn main() {
    println!("Building beatmap texture...");
    let path = Path::new("example.png");
    let mut texture = Vec::<u8>::new();
    
    let image_size = 512;
    let interpolator_count = 16;
    
    let blocks = generate_interpolators::sine_interpolation(image_size, interpolator_count, 1.0);
    let pixels = texture_gen::pixels_from_interpolator_blocks(image_size, blocks);
    
    texture_gen::output_texture(pixels, &mut texture).unwrap();
    
    // let mut file = OpenOptions::new().write(true).create(true).truncate(true).open(path).unwrap();
    // file.write_all(&texture.clone().as_slice()).unwrap();
    // file.sync_all().unwrap();
    // println!("Wrote texture to {}", path.display());

    println!("Texture info: size={}", texture.len());

    println!("Starting renderer...");
    graphics::start_graphics(image_size, texture);
}