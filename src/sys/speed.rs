// In Double Speed Mode the following will operate 2x:
//   The CPU (2.10 MHz, so 1 M-cycle = approx. 0.5 µs)
//   Timer and Divider Registers
//   Serial Port (Link Cable)
//   DMA Transfer to OAM

// And the following will keep operating as usual:
//   LCD Video Controller
//   HDMA Transfer to VRAM
//   All Sound Timings and Frequencies

// The CPU stops for 2050 M-cycles (= 8200 T-cycles)
// after the stop instruction is executed.
// During this time, the CPU is in a strange state.
// DIV does not tick, so some audio events are not processed.
// Additionally, VRAM/OAM/… locking is “frozen”, yielding
// different results depending on the PPU mode it’s started in.

use crate::{mem::io_regs::IoReg, util::bits::Bits};

use super::Sys;

/// Keeps track of Double-Speed mode timing.
pub struct SpeedControl {
    stop_mcycles_left: u32,
    mcycle: u32,
}

impl SpeedControl {
    pub fn new() -> Self {
        Self {
            stop_mcycles_left: 0,
            mcycle: 0,
        }
    }

    pub fn is_stop_active(&self) -> bool {
        return self.stop_mcycles_left > 0;
    }

    pub fn stop(&mut self) {
        self.stop_mcycles_left = 2050;
    }
}

pub fn is_double_speed_mode_active(sys: &mut Sys) -> bool {
    let key1 = sys.mem.io_regs.get(IoReg::Key1);
    return key1.bit(7) == 1;
}

/// Does everything in the system update during the current M-Cycle?
pub fn is_full_mcycle(sys: &Sys) -> bool {
    return sys.speed_ctrl.mcycle == 0;
}

pub fn update_speed_ctrl(sys: &mut Sys) {
    if sys.speed_ctrl.stop_mcycles_left > 0 {
        sys.speed_ctrl.stop_mcycles_left -= 1;
        return;
    }

    let cycle_len = if is_double_speed_mode_active(sys) {
        2
    } else {
        1
    };

    sys.speed_ctrl.mcycle = (sys.speed_ctrl.mcycle + 1) % cycle_len;
}
