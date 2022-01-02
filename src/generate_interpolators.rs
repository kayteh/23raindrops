use super::texture_gen;

pub fn sine_interpolation(image_size: u32, interpolator_count: u8, scale: f32) -> Vec<texture_gen::InterpolatorBlock> {
    let mut blocks = Vec::<texture_gen::InterpolatorBlock>::new();
    let phase_separation = (2.0 * std::f32::consts::PI) / (interpolator_count as f32);
    let block_count = 4 * (image_size * image_size) / interpolator_count as u32;

    for i in 0..block_count {
        let mut interpolators = Vec::<u8>::new();

        // For each interpolator, generate a sine wave with equally spaced phases between interpolators.
        for j in 0..interpolator_count {
            let wave_value = f32::sin(i as f32 * scale + j as f32 * phase_separation);

            let value = 0.5 * (1.0 + wave_value);
            interpolators.push((255.0 * value) as u8);
        }

        let block = texture_gen::InterpolatorBlock::from_interpolators(interpolators);
        blocks.push(block);
    }

    blocks
}

pub fn linear_interpolation(image_size: u32, interpolator_count: u8) -> Vec<texture_gen::InterpolatorBlock> {
    let mut blocks = Vec::<texture_gen::InterpolatorBlock>::new();
    let block_count = 4 * (image_size * image_size) / interpolator_count as u32;

    for i in 0..block_count {
        let mut interpolators = Vec::<u8>::new();

        // For each interpolator, generate a linear wave with equally spaced phases between interpolators.
        for j in 0..interpolator_count {
            let value = (i as f32 + j as f32) / (block_count as f32);
            interpolators.push((255.0 * value) as u8);
        }

        let block = texture_gen::InterpolatorBlock::from_interpolators(interpolators);
        blocks.push(block);
    }

    blocks
}

pub fn debug_interpolation(image_size: u32, interpolator_count: u8) -> Vec<texture_gen::InterpolatorBlock> {
    let mut blocks = Vec::<texture_gen::InterpolatorBlock>::new();
    let block_count = 4 * (image_size * image_size) / interpolator_count as u32;

    for i in 0..block_count {
        let mut interpolators = Vec::<u8>::new();

        // red block
        interpolators.push(255);
        interpolators.push(0);
        interpolators.push(0);
        interpolators.push(255);

        // greeb block
        interpolators.push(0);
        interpolators.push(255);
        interpolators.push(0);
        interpolators.push(255);

        // blue block
        interpolators.push(0);
        interpolators.push(0);
        interpolators.push(255);
        interpolators.push(255);

        // white block
        interpolators.push(255);
        interpolators.push(255);        
        interpolators.push(255);
        interpolators.push(255);

        let block = texture_gen::InterpolatorBlock::from_interpolators(interpolators);
        blocks.push(block);
    }

    blocks
}