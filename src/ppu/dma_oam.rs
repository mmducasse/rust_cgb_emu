use crate::{mem::io_regs::IoReg, sys::Sys};

const DMA_DURATION_M_CYCLES: u16 = 160;

/// Represents the OAM DMA Transfer state.
pub struct DmaOam {
    is_active: bool,
    next_idx: u16,
}

impl DmaOam {
    pub fn new() -> Self {
        Self {
            is_active: false,
            next_idx: 0,
        }
    }
}

/// Advances the OAM DMA state by one M-Cycle.
pub fn update_oam_dma(sys: &mut Sys) {
    let dma = sys.ppu.oam_dma_mut();
    if !dma.is_active {
        if sys.mem.io_regs.dma_requested {
            sys.mem.io_regs.dma_requested = false;
            start_dma(sys);
        } else {
            return;
        }
    }

    transfer_one_byte(sys);
}

fn start_dma(sys: &mut Sys) {
    let dma = sys.ppu.oam_dma_mut();
    dma.is_active = true;
    dma.next_idx = 0;
}

fn transfer_one_byte(sys: &mut Sys) {
    let dma = sys.ppu.oam_dma_mut();

    let idx = dma.next_idx;
    let dma_val = sys.mem.io_regs.get(IoReg::Dma) as u16;
    let src_addr = (dma_val * 0x100) + idx;
    let dst_addr = 0xFE00 + idx;

    let data = sys.mem.read(src_addr);
    sys.mem.write(dst_addr, data);

    dma.next_idx += 1;
    if dma.next_idx >= DMA_DURATION_M_CYCLES {
        dma.is_active = false;
    }
}
