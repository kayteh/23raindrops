use std::io::Write;

pub struct InterpolationFrame {
    pub interpolators: Vec<f32>,
}

impl InterpolationFrame {
    pub fn linear(interpolator_count: u32) -> InterpolationFrame {
        let mut interpolators = Vec::new();

        for i in 0..interpolator_count {
            let interpolator = i as f32 / interpolator_count as f32;
            interpolators.push(interpolator);
        }
    
        InterpolationFrame { interpolators }
    }

    pub fn sine(interpolator_count: u32) -> InterpolationFrame {
        let mut interpolators = Vec::new();

        for i in 0..interpolator_count {
            let interpolator = (i as f32 / interpolator_count as f32).sin();
            interpolators.push(interpolator);
        }
    
        InterpolationFrame { interpolators }
    }

    pub fn repeat(&self, length: u32) -> Vec<&InterpolationFrame> {
        let mut frames = Vec::<&InterpolationFrame>::new();

        for _i in 0..length {
            frames.push(self.clone());
        }

        frames
    }
}

pub fn frames_to_vec(frames: Vec<&InterpolationFrame>) -> Vec<f32> {
    let mut vec = Vec::<f32>::new();

    for interpolator in frames {
        for i in 0..interpolator.interpolators.len() {
            vec.push(interpolator.interpolators[i]);
        }
    }

    vec
}