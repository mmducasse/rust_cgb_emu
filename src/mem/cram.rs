use crate::util::{bits::Bits, math::join_16};

const COLOR_SIZE: usize = 2;
const PALETTE_LEN: usize = 4;
const PALETTES_PER_CRAM: usize = 8;
const CRAM_SIZE: usize = PALETTES_PER_CRAM * PALETTE_LEN * COLOR_SIZE;

/// Color RAM (aka Palette RAM). An instance of CRAM
/// stores 8 color palettes. Each palette consists of
/// 4 colors, each stored in little-endian RGB555 format.
pub struct Cram {
    pub index: u8,
    mem: [u8; CRAM_SIZE],
}

impl Cram {
    pub fn new() -> Self {
        Self {
            index: 0,
            mem: [0xFF; CRAM_SIZE],
        }
    }

    pub fn index(&self) -> u8 {
        self.index.bits(5, 0)
    }

    pub fn auto_inc(&self) -> u8 {
        self.index.bit(7)
    }

    pub fn get(&self, palette_id: u8, color_id: u8) -> u16 {
        let palette_id = palette_id as usize;
        let color_id = color_id as usize;
        let idx = ((palette_id * PALETTE_LEN) + color_id) * COLOR_SIZE;

        // Color data stored in RGB555 little-endian.
        let lo = self.mem[idx];
        let hi = self.mem[idx + 1];
        let data = join_16(hi, lo);

        return data;
    }

    pub fn read(&self) -> u8 {
        self.mem[self.index() as usize]
    }

    pub fn write(&mut self, data: u8) {
        self.mem[self.index() as usize] = data;

        if self.auto_inc() == 1 {
            let next = (self.index() + 1) % (CRAM_SIZE as u8);
            self.index.set_bits(5, 0, next);
        }
    }
}
