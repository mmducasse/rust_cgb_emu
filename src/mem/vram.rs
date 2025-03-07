use std::ops::Range;

use crate::util::bits::Bits;

use super::{
    array::Array,
    io_regs::{IoReg, IoRegs},
    sections::MemSection,
    Addr,
};

pub struct Vram {
    banks: Vec<Array>,
}

impl Vram {
    pub fn new(is_cgb_mode: bool) -> Self {
        let start_addr = MemSection::Vram.start_addr();
        let len = MemSection::Vram.size();

        let mut banks = vec![];
        banks.push(Array::new(start_addr, len));
        if is_cgb_mode {
            banks.push(Array::new(start_addr, len));
        }

        Self { banks }
    }

    pub fn num_banks(&self) -> usize {
        self.banks.len()
    }

    #[inline]
    pub fn get(&self, bank: usize, addr: Addr) -> u8 {
        return self.banks[bank].read(addr);
    }

    pub fn get_range(&self, bank: usize, range: Range<usize>) -> &[u8] {
        return &self.banks[bank].as_slice()[range];
    }

    pub fn read(&self, io_regs: &IoRegs, addr: Addr) -> u8 {
        let bank = self.get_bank(io_regs);
        return self.get(bank, addr);
    }

    pub fn write(&mut self, io_regs: &IoRegs, addr: Addr, data: u8) {
        let bank = self.get_bank(io_regs);
        self.banks[bank].write(addr, data);
    }

    fn get_bank(&self, io_regs: &IoRegs) -> usize {
        if self.banks.len() > 1 {
            let vbk = io_regs.get(IoReg::Vbk);
            return vbk.bit(0) as usize;
        }

        return 0;
    }
}
