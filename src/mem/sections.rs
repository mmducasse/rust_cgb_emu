use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use super::{array::Array, Addr};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, EnumIter, FromPrimitive, Debug)]
pub enum MemSection {
    CartRom,
    Vram,
    ExtRam,
    Wram,
    EchoRam,
    Oam,
    UnusableMemory,
    IoRegs,
    Hram,
    IeReg,
}

impl MemSection {
    /// Returns the starting address of the memory section.
    pub fn start_addr(self) -> Addr {
        use MemSection::*;

        match self {
            CartRom => 0x0000,
            Vram => 0x8000,
            ExtRam => 0xA000,
            Wram => 0xC000,
            EchoRam => 0xE000,
            Oam => 0xFE00,
            UnusableMemory => 0xFEA0,
            IoRegs => 0xFF00,
            Hram => 0xFF80,
            IeReg => 0xFFFF,
        }
    }

    /// Creates an array that represents this section of memory.
    pub fn into_array(section: Self) -> Array {
        return Array::new(section.start_addr(), section.size());
    }

    /// The number of bytes in this section of memory.
    pub fn size(self) -> u16 {
        if self == Self::IeReg {
            return 1;
        }

        let next = (self as usize) + 1;
        let next: Self = num::FromPrimitive::from_usize(next).unwrap();

        let section_size = next.start_addr() - self.start_addr();
        return section_size;
    }

    /// Returns the memory section that the address belongs to, as
    /// well as it's relative address within that section.
    pub fn from_abs_addr(addr: Addr) -> Self {
        for section in MemSection::iter().rev() {
            let start_addr = section.start_addr();
            if addr >= start_addr {
                return section;
            }
        }

        panic!("Unable to determine memory section of addr: {}", addr);
    }
}
