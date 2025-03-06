use crate::util::math::bit8;

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

    pub fn read(&self, io_regs: &IoRegs, addr: Addr) -> u8 {
        let b = self.get_bank(io_regs);
        return self.banks[b].read(addr);
    }

    pub fn write(&mut self, io_regs: &IoRegs, addr: Addr, data: u8) {
        let b = self.get_bank(io_regs);
        self.banks[b].write(addr, data);
    }

    fn get_bank(&self, io_regs: &IoRegs) -> usize {
        if self.banks.len() > 1 {
            let vbk = io_regs.get(IoReg::Vbk);
            return bit8(&vbk, 0) as usize;
        }

        return 0;
    }

    pub fn as_slice(&self) -> &[u8] {
        return self.banks[0].as_slice();
    }
}
