use std::io::{Write, BufWriter};
use png;

pub struct Pixel {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl Pixel {
    fn to_bytes(&self) -> [u8; 4] {
        [self.r, self.g, self.b, self.a]
    }

    fn from_bytes(bytes: [u8; 4]) -> Pixel {
        Pixel {
            r: bytes[0],
            g: bytes[1],
            b: bytes[2],
            a: bytes[3],
        }
    }
}

pub struct InterpolatorBlock {
    pixels: Vec<Pixel>,
}

impl InterpolatorBlock {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        for pixel in &self.pixels {
            bytes.extend_from_slice(&pixel.to_bytes());
        }
        bytes
    }

    pub fn from_interpolators(interpolators: Vec<u8>) -> InterpolatorBlock {
        let mut pixels = Vec::new();
        
        for i in 0..interpolators.len() / 4 {
            let pixel = Pixel::from_bytes([
                interpolators[i * 4],
                interpolators[i * 4 + 1],
                interpolators[i * 4 + 2],
                255,
            ]);
            pixels.push(pixel);
        }

        InterpolatorBlock { pixels }
    }
}

pub fn output_texture<W: Write>(pixels: Vec<InterpolatorBlock>, destination: &mut W) -> Result<(), ()> {
    let ref mut buffer = BufWriter::new(destination);
    let mut encoder = png::Encoder::new(buffer, 128, 128);
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);
    encoder.set_compression(png::Compression::Fast);

    let mut writer = encoder.write_header().unwrap();
    // flatten each pixel into a u8 array
    let mut flattened_pixels: Vec<u8> = Vec::new();
    for pixel in pixels {
        flattened_pixels.extend_from_slice(&pixel.to_bytes());
    }
    writer.write_image_data(&flattened_pixels).unwrap();

    Ok(())
}