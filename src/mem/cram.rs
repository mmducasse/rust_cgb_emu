use crate::util::bits::Bits;

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
            mem: [0; CRAM_SIZE],
        }
    }

    pub fn index(&self) -> u8 {
        self.index.bits(5, 0)
    }

    pub fn read(&self) -> u8 {
        self.mem[self.index() as usize]
    }

    pub fn write(&mut self, data: u8) {
        self.mem[self.index() as usize] = data;

        let auto_inc = data.bit(7) == 1;
        if auto_inc {
            self.index = u8::saturating_add(self.index, 1);
        }
    }
}
