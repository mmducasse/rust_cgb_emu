use crate::{
    cpu::regs::{CpuReg16, CpuReg8},
    mem::io_regs::IoReg,
    other::mode::CompatibilityMode,
};

use super::Sys;

pub fn init(sys: &mut Sys) {
    use CompatibilityMode::*;

    match sys.mem.cart.header().compatibility_mode() {
        DmgOnly => {
            init_cpu_dmg_only(sys);
            init_io_regs_dmg(sys);
        }
        CgbBackward => {
            init_cpu_cgb_backward(sys);
            init_io_regs_cgb(sys);
        }
        CgbOnly => {
            init_cpu_cgb_only(sys);
            init_io_regs_cgb(sys);
        }
    }
}

/// Set CPU registers to defaults for DMG only mode.
fn init_cpu_dmg_only(sys: &mut Sys) {
    sys.regs.set_8(CpuReg8::A, 0x01);
    sys.regs.set_8(CpuReg8::F, 0b1000_0000);
    sys.regs.set_8(CpuReg8::B, 0x00);
    sys.regs.set_8(CpuReg8::C, 0x13);

    sys.regs.set_8(CpuReg8::D, 0x00);
    sys.regs.set_8(CpuReg8::E, 0xD8);
    sys.regs.set_8(CpuReg8::H, 0x01);
    sys.regs.set_8(CpuReg8::L, 0x48);

    sys.regs.set_16(CpuReg16::PC, 0x0100);
    sys.regs.set_16(CpuReg16::SP, 0xFFFE);
}

/// Set CPU registers to defaults for CGB backward-compatibile mode.
fn init_cpu_cgb_backward(sys: &mut Sys) {
    sys.regs.set_8(CpuReg8::A, 0x11);
    sys.regs.set_8(CpuReg8::F, 0b1000_0000);
    sys.regs.set_8(CpuReg8::B, 0x00);
    sys.regs.set_8(CpuReg8::C, 0x00);

    sys.regs.set_8(CpuReg8::D, 0x00);
    sys.regs.set_8(CpuReg8::E, 0x08);
    sys.regs.set_8(CpuReg8::H, 0x00);
    sys.regs.set_8(CpuReg8::L, 0x00);

    sys.regs.set_16(CpuReg16::PC, 0x0100);
    sys.regs.set_16(CpuReg16::SP, 0xFFFE);
}

/// Set CPU registers to defaults for CGB only mode.
fn init_cpu_cgb_only(sys: &mut Sys) {
    sys.regs.set_8(CpuReg8::A, 0x11);
    sys.regs.set_8(CpuReg8::F, 0b1000_0000);
    sys.regs.set_8(CpuReg8::B, 0x00);
    sys.regs.set_8(CpuReg8::C, 0x00);

    sys.regs.set_8(CpuReg8::D, 0xFF);
    sys.regs.set_8(CpuReg8::E, 0x56);
    sys.regs.set_8(CpuReg8::H, 0x00);
    sys.regs.set_8(CpuReg8::L, 0x0D);

    sys.regs.set_16(CpuReg16::PC, 0x0100);
    sys.regs.set_16(CpuReg16::SP, 0xFFFE);
}

/// Set IO registers to defaults for DMG.
fn init_io_regs_dmg(sys: &mut Sys) {
    use IoReg::*;
    sys.mem.io_regs.set(P1, 0xCF);
    sys.mem.io_regs.set(Sb, 0x00);
    sys.mem.io_regs.set(Sc, 0x7E);
    sys.mem.io_regs.set(Div, 0xAB);
    sys.mem.io_regs.set(Tima, 0x00);
    sys.mem.io_regs.set(Tma, 0x00);
    sys.mem.io_regs.set(Tac, 0xF8);
    sys.mem.io_regs.set(If, 0xE1);
    sys.mem.io_regs.set(Lcdc, 0x91);
    sys.mem.io_regs.set(Stat, 0x85);
    sys.mem.io_regs.set(Scy, 0x00);
    sys.mem.io_regs.set(Scx, 0x00);
    sys.mem.io_regs.set(Ly, 0x00);
    sys.mem.io_regs.set(Lyc, 0x00);
    sys.mem.io_regs.set(Dma, 0xFF);
    sys.mem.io_regs.set(Bgp, 0xFC);
    sys.mem.io_regs.set(Obp0, 0);
    sys.mem.io_regs.set(Obp1, 0);
    sys.mem.io_regs.set(Wy, 0x00);
    sys.mem.io_regs.set(Wx, 0x00);

    // Key1..Svbk are not initialized.

    sys.mem.io_regs.set(Ie, 0x00);
}

/// Set IO registers to defaults for CGB.
fn init_io_regs_cgb(sys: &mut Sys) {
    use IoReg::*;
    sys.mem.io_regs.set(P1, 0xCF);
    sys.mem.io_regs.set(Sb, 0x00);
    sys.mem.io_regs.set(Sc, 0x7E);
    sys.mem.io_regs.set(Div, 0xAB);
    sys.mem.io_regs.set(Tima, 0x00);
    sys.mem.io_regs.set(Tma, 0x00);
    sys.mem.io_regs.set(Tac, 0xF8);
    sys.mem.io_regs.set(If, 0xE1);
    sys.mem.io_regs.set(Lcdc, 0x91);
    sys.mem.io_regs.set(Stat, 0x85);
    sys.mem.io_regs.set(Scy, 0x00);
    sys.mem.io_regs.set(Scx, 0x00);
    sys.mem.io_regs.set(Ly, 0x00);
    sys.mem.io_regs.set(Lyc, 0x00);
    sys.mem.io_regs.set(Dma, 0xFF);
    sys.mem.io_regs.set(Bgp, 0xFC);
    sys.mem.io_regs.set(Obp0, 0);
    sys.mem.io_regs.set(Obp1, 0);
    sys.mem.io_regs.set(Wy, 0x00);
    sys.mem.io_regs.set(Wx, 0x00);

    // todo: Key1..Svbk.

    sys.mem.io_regs.set(Ie, 0x00);
}
