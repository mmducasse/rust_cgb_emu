use crate::{
    mem::{io_regs::IoReg, Addr},
    sys::Sys,
    util::math::{bit8, bits8, set_bit8},
};

use super::ppu::PpuMode;

/// Represents the VRAM DMA Transfer state (CGB only).
pub struct DmaVram {
    is_active: bool,
    src_addr: Addr,
    dst_addr: Addr,
    data_len: u8,
    transfer_mode: TransferMode,
    pending_hblank_transfer: bool,
    next_idx: u16,
}

//In both Normal Speed and Double Speed Mode it
//takes about 8 μs to transfer a block of $10 bytes.
//That is, 8 M-cycles in Normal Speed Mode [1],
//and 16 “fast” M-cycles in Double Speed Mode [2].
//Older MBC controllers (like MBC1-3) and slower ROMs
//are not guaranteed to support General Purpose or HBlank DMA,
//that’s because there are always 2 bytes transferred
//per microsecond (even if the itself program runs
//it Normal Speed Mode).

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum TransferMode {
    General,
    HBlank,
}

/// Represents the VRAM DMA Transfer state.
impl DmaVram {
    pub fn new() -> Self {
        Self {
            is_active: false,
            src_addr: 0,
            dst_addr: 0,
            data_len: 0,
            transfer_mode: TransferMode::General,
            pending_hblank_transfer: false,
            next_idx: 0,
        }
    }
}

/// Advances the VRAM DMA state by one M-Cycle.
pub fn update_vram_dma(sys: &mut Sys) {
    // Is VRAM DMA supported?
    if !sys.mem.cart.header().compatibility_mode().is_cgb() {
        return;
    }

    let ppu_mode = sys.ppu.mode();

    // Check if VRAM DMA was requested.
    //let hdma = sys.ppu.hdma_mut();
    if !sys.ppu.vram_dma_mut().is_active {
        if sys.mem.io_regs.hdma_requested {
            sys.mem.io_regs.hdma_requested = false;
            start_hdma(sys);
            //return;
        } else {
            return;
        }
    }

    //transfer_0x10_bytes(sys);

    // Transfer data.
    if sys.ppu.vram_dma_mut().transfer_mode == TransferMode::HBlank {
        if (ppu_mode == PpuMode::HBlank) && sys.ppu.vram_dma_mut().pending_hblank_transfer {
            transfer_0x10_bytes(sys);
            sys.ppu.vram_dma_mut().pending_hblank_transfer = false;
        } else {
            sys.ppu.vram_dma_mut().pending_hblank_transfer = true;
        }
    } else {
        transfer_0x10_bytes(sys);
    }
}

fn start_hdma(sys: &mut Sys) {
    let hdma1 = sys.mem.io_regs.get(IoReg::Hdma1) as u16;
    let hdma2 = sys.mem.io_regs.get(IoReg::Hdma2) as u16;
    let src_addr = ((hdma1 << 8) | hdma2) & 0xFFF0;

    let hdma3 = sys.mem.io_regs.get(IoReg::Hdma3) as u16;
    let hdma4 = sys.mem.io_regs.get(IoReg::Hdma4) as u16;
    let dst_addr = ((hdma3 << 8) | hdma4) & 0x1FF0;

    let hdma5 = sys.mem.io_regs.get(IoReg::Hdma5);
    let transfer_mode = if bit8(&hdma5, 7) == 0 {
        TransferMode::General
    } else {
        TransferMode::HBlank
    };
    let data_len = (bits8(&hdma5, 6, 0) + 1) * 0x10;

    let hdma = sys.ppu.vram_dma_mut();
    hdma.is_active = true;
    hdma.src_addr = src_addr;
    hdma.dst_addr = dst_addr;
    hdma.data_len = data_len;
    hdma.transfer_mode = transfer_mode;
    hdma.next_idx = 0;

    // println!("Start HDMA:");
    // println!("  src_addr   = {:0>4X}", src_addr);
    // println!("  dst_addr   = {:0>4X}", dst_addr);
    // println!("  len        = {:0>4X}", data_len);
    // println!("  txfer mode = {:?}", transfer_mode);

    // Set Hdma5.7 hi to indicate transfer is active.
    sys.mem.io_regs.mut_(IoReg::Hdma5, |hdma5| {
        set_bit8(hdma5, 7, 1);
    });
}

fn transfer_0x10_bytes(sys: &mut Sys) {
    for _ in 0..0x10 {
        transfer_one_byte(sys);

        let hdma = sys.ppu.vram_dma_mut();
        if hdma.next_idx >= (hdma.data_len as u16) {
            hdma.is_active = false;

            // Set Hdma5.7 lo to indicate transfer is inactive.
            sys.mem.io_regs.mut_(IoReg::Hdma5, |hdma5| {
                set_bit8(hdma5, 7, 0);
            });
        }
    }
}

fn transfer_one_byte(sys: &mut Sys) {
    let hdma = sys.ppu.vram_dma_mut();

    let idx = hdma.next_idx;
    let src_addr = hdma.src_addr + idx;
    let dst_addr = hdma.dst_addr + idx;

    let data = sys.mem.read(src_addr);
    sys.mem.write(dst_addr, data);

    // println!(
    //     "  txfer {:0>2X} from {:0>4X} to {:0>4X}",
    //     data, src_addr, dst_addr
    // );

    hdma.next_idx += 1;
}
