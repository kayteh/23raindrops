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

    let image_size = 256;
    let interpolator_count = 16;

    let blocks = generate_interpolators::sine_interpolation(image_size, interpolator_count, 10.0);
    texture_gen::output_texture(blocks, &mut file).unwrap();
}