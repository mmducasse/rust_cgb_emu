use crate::{mem::sections::MemSection, util::math::bits8};

use super::{
    array::Array,
    io_regs::{IoReg, IoRegs},
    Addr,
};

const WRAM_BANK_LEN: u16 = 0x1000;
const DMG_WRAM_BANK_COUNT: usize = 2;
const CBG_WRAM_BANK_COUNT: usize = 8;

pub struct Wram {
    banks: Vec<Array>,
}

impl Wram {
    pub fn new(is_cgb_mode: bool) -> Self {
        const BANK_0_ADDR: Addr = MemSection::Wram.start_addr();
        const BANK_1_7_ADDR: Addr = BANK_0_ADDR + WRAM_BANK_LEN;

        let mut banks = vec![];
        banks.push(Array::new(BANK_0_ADDR, WRAM_BANK_LEN));

        let bank_count = if is_cgb_mode {
            CBG_WRAM_BANK_COUNT
        } else {
            DMG_WRAM_BANK_COUNT
        };

        for _ in 1..bank_count {
            banks.push(Array::new(BANK_1_7_ADDR, WRAM_BANK_LEN));
        }

        Self { banks }
    }

    pub fn read(&self, io_regs: &IoRegs, addr: Addr) -> u8 {
        if self.banks[0].contains_addr(addr) {
            return self.banks[0].read(addr);
        } else {
            let alt_bank = if self.banks.len() > 2 {
                let svbk = io_regs.get(IoReg::Svbk);
                u8::max(1, bits8(&svbk, 2, 0)) as usize
            } else {
                1
            };

            return self.banks[alt_bank].read(addr);
        }
    }

    pub fn write(&mut self, io_regs: &IoRegs, addr: Addr, data: u8) {
        if self.banks[0].contains_addr(addr) {
            self.banks[0].write(addr, data);
        } else {
            let alt_bank = if self.banks.len() > 2 {
                let svbk = io_regs.get(IoReg::Svbk);
                u8::max(1, bits8(&svbk, 2, 0)) as usize
            } else {
                1
            };

            self.banks[alt_bank].write(addr, data);
        }
    }
}
