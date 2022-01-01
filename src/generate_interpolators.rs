use super::texture_gen;

pub fn sine_interpolation(image_size: u32, interpolator_count: u8, scale: f32) -> Vec<texture_gen::InterpolatorBlock> {
    let mut blocks = Vec::<texture_gen::InterpolatorBlock>::new();
    let phase_separation = (2.0 * std::f32::consts::PI) / (interpolator_count as f32);
    let block_count = (image_size*image_size) / interpolator_count as u32;

    for i in 0..block_count {
        let mut interpolators = Vec::<u8>::new();

        // For each interpolator, generate a sine wave with equally spaced phases between interpolators.
        for j in 0..interpolator_count {
            let phase = (j as f32) * phase_separation;
            let amplitude = 255.0;
            let value = (amplitude * (f32::sin(phase + (scale * i as f32)) + phase_separation)) as u8;
            interpolators.push(value);
        }

        let block = texture_gen::InterpolatorBlock::from_interpolators(interpolators);
        blocks.push(block);
    }

    blocks
}