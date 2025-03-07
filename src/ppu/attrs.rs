use crate::{mem::Addr, sys::Sys, util::bits::Bits};

/// Interpretation of background tile attributes byte (CGB mode only).
pub struct BgAttrs {
    pub priority: u8,
    pub y_flip: bool,
    pub x_flip: bool,
    pub bank: usize,
    pub color_palette: u8,
}

impl BgAttrs {
    pub fn new(sys: &Sys, addr: Addr) -> Self {
        let data = sys.mem.vram.get(1, addr);

        Self {
            priority: data.bit(7),
            y_flip: data.bit(6) == 1,
            x_flip: data.bit(5) == 1,
            bank: data.bit(3) as usize,
            color_palette: data.bits(2, 0),
        }
    }
}
