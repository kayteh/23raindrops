#[macro_use]
extern crate glium;

use std::fs::OpenOptions;
use std::path::Path;

mod graphics;
mod texture_gen;
mod generate_interpolators;

fn main() {
    println!("Hello, world!");
    let path = Path::new("example.png");
    let mut file = OpenOptions::new().write(true).create(true).truncate(true).open(path).unwrap();

    let image_size = 512;
    let interpolator_count = 16;

    let blocks = generate_interpolators::debug_interpolation(image_size, interpolator_count);
    let pixels = texture_gen::pixels_from_interpolator_blocks(image_size, blocks);
    texture_gen::output_texture(image_size, pixels, &mut file).unwrap();
}