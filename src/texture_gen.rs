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
                interpolators[i * 4 + 3],
            ]);
            pixels.push(pixel);
        }

        InterpolatorBlock { pixels }
    }
}

pub fn output_texture<W: Write>(image_size: u32, pixels: Vec<Pixel>, destination: &mut W) -> Result<(), ()> {
    let ref mut buffer = BufWriter::new(destination);
    let mut encoder = png::Encoder::new(buffer, image_size, image_size);
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

// Make a grid of interpolator blocks, and output them as pixels.
pub fn pixels_from_interpolator_blocks(image_size: u32, interpolator_blocks: Vec<InterpolatorBlock>) -> Vec<Pixel> {
    let mut grid: Vec<Vec<Pixel>> = Vec::new();

    // prefill grid with empty pixels
    for y in 0..image_size {
        let mut row: Vec<Pixel> = Vec::new();
        for x in 0..image_size {
            row.push(Pixel {
                r: (x % 255) as u8,
                g: (y % 255) as u8,
                b: 0,
                a: 255,
            });
        }
        grid.push(row);
    }

    // TODO: fill in the grid with interpolator blocks
    for block_index in 0..interpolator_blocks.len()-1 {
        // every block is 2x2
        let block_x = block_index % (image_size / 2) as usize;
        let block_y = block_index / (image_size / 2) as usize;
        let block_x_offset = block_x * 2;
        let block_y_offset = block_y * 2;
        for x in 0..2 {
            for y in 0..2 {
                let block_x = block_x_offset + x;
                let block_y = block_y_offset + y;
                let pixel = &interpolator_blocks[block_index].pixels[y * 2 + x];
                grid[block_y as usize][block_x as usize] = Pixel {
                    r: pixel.r,
                    g: pixel.g,
                    b: pixel.b,
                    a: pixel.a,
                };
            }
        }
    }

    // return the grid as a vector of pixels
    let mut pixels: Vec<Pixel> = Vec::new();
    for row in grid {
        for pixel in row {
            pixels.push(pixel);
        }
    }
    pixels
}