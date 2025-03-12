use macroquad::color::Color;

use crate::util::bits::Bits;

const NUM_COLORS: usize = 1 << 15;

/// Provides a quick lookup for CGB LCD pixel colors.
pub struct Colors {
    colors: Vec<Color>,
}

impl Colors {
    pub fn new() -> Self {
        let mut colors = vec![Color::default(); NUM_COLORS];

        for idx in 0..(NUM_COLORS as u16) {
            let c = idx;

            let r = convert_to_float(c.bits(4, 0));
            let g = convert_to_float(c.bits(9, 5));
            let b = convert_to_float(c.bits(14, 10));

            let color = Color::new(r, g, b, 1.0);
            colors[idx as usize] = color;
        }

        return Self { colors };
    }

    pub fn get(&self, color_data: u16) -> Color {
        let idx = color_data.bits(14, 0);
        return self.colors[idx as usize];
    }
}

fn convert_to_float(x: u16) -> f32 {
    const MAX: f32 = 31 as f32;
    return (x as f32) / MAX;
}
