use crate::{
    mem::Addr,
    util::{bits::Bits, math::bits8},
};

use super::{
    cart_hw::CartHw,
    consts::{RAM_BANK_SIZE, ROM_BANK_SIZE},
};

/// MBC5 cartridge hardware. Features 8MB ROM and/or 128KB RAM.
pub struct HwMbc5 {
    rom: Vec<u8>,

    ram: Vec<u8>,
    ram_enable: bool,

    rom_bank_sel_lower_8: u8,
    rom_bank_sel_upper_1: u8,
    ram_bank_sel: u8,
}

impl HwMbc5 {
    pub fn new(rom_banks: usize, ram_banks: usize) -> Self {
        Self {
            rom: vec![0; rom_banks * ROM_BANK_SIZE],

            ram: vec![0; ram_banks * RAM_BANK_SIZE],
            ram_enable: false,

            rom_bank_sel_lower_8: 0,
            rom_bank_sel_upper_1: 0,
            ram_bank_sel: 0,
        }
    }

    pub fn rom_bank_sel(&self) -> u16 {
        let lower = self.rom_bank_sel_lower_8 as u16;
        let upper = self.rom_bank_sel_upper_1 as u16;

        let bank = (upper << 8) | lower;
        return bank;
    }

    pub fn ram_bank_sel(&self) -> u8 {
        return self.ram_bank_sel;
    }
}

impl CartHw for HwMbc5 {
    fn rom_mut(&mut self) -> &mut [u8] {
        &mut self.rom
    }

    fn ram(&self) -> &[u8] {
        &self.ram
    }

    fn ram_mut(&mut self) -> &mut [u8] {
        &mut self.ram
    }

    // todo cleanup
    fn read(&self, addr: Addr) -> u8 {
        match addr {
            0x0000..=0x3FFF => {
                // ROM Bank 00
                self.rom[addr as usize]
            }
            0x4000..=0x7FFF => {
                // ROM Bank 01-1FF
                let rel_addr = addr - 0x4000;
                let bank_sel = self.rom_bank_sel() as usize;
                let bank_offs = bank_sel * ROM_BANK_SIZE;
                let addr = bank_offs + (rel_addr as usize);
                if addr >= self.rom.len() {
                    return 0;
                }
                self.rom[addr]
            }
            0xA000..=0xBFFF => {
                // RAM Bank 00-0F
                let rel_addr = addr - 0xA000;
                let bank_offs = (self.ram_bank_sel() as usize) * RAM_BANK_SIZE;
                let addr = bank_offs + (rel_addr as usize);

                if addr >= self.ram.len() {
                    return 0;
                }

                self.ram[addr]
            }
            _ => {
                panic!("Invalid MBC5 read address");
            }
        }
    }

    // todo cleanup
    fn write(&mut self, addr: Addr, data: u8) {
        match addr {
            0x0000..=0x1FFF => {
                if data == 0x0A {
                    self.ram_enable = true;
                }
                if data == 0x00 {
                    self.ram_enable = false;
                }
            }
            0x2000..=0x2FFF => {
                self.rom_bank_sel_lower_8 = data;
            }
            0x3000..=0x3FFF => {
                self.rom_bank_sel_upper_1 = data.bit(0);
            }
            0x4000..=0x5FFF => {
                self.ram_bank_sel = bits8(&data, 3, 0);
            }
            0xA000..=0xBFFF => {
                if self.ram_enable {
                    // RAM Bank 00-03
                    let rel_addr = addr - 0xA000;
                    let bank_offs = (self.ram_bank_sel() as usize) * RAM_BANK_SIZE;
                    let addr = bank_offs + (rel_addr as usize);

                    if let Some(val) = self.ram.get_mut(addr) {
                        *val = data;
                    }
                }
            }
            _ => {
                panic!("Invalid MBC5 write address");
            }
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_mbc1_rw_rom() {
//         let mut hw = HwMbc1::new(0xFF, 0x04);

//         hw.write(0x6000, Mode::RomBanking as u8);

//         for upper in 0..=3 {
//             for mut lower in 0..=0x1F {
//                 hw.write(0x2000, lower);
//                 hw.write(0x4000, upper);

//                 if (lower & 0x1F) == 0 {
//                     lower += 1;
//                 }
//                 let bank = (upper << 5) | lower;
//                 let addr = bank as usize * ROM_BANK_SIZE;
//                 let write_value = bank;
//                 hw.rom_mut()[addr] = write_value;
//                 let read_value = hw.read(0x4000);

//                 //assert_eq!(write_value, read_value);
//                 println!("Read bank {:0>4X}: {:0>4X}", bank, read_value);
//             }
//         }
//     }

//     #[test]
//     fn test_mbc1_rw_ram() {
//         let mut hw = HwMbc1::new(0xFF, 0x04);

//         hw.write(0x6000, Mode::RamBanking as u8);

//         for upper in 0..=3 {
//             for lower in 0..=0x1F {
//                 hw.write(0x2000, lower);
//                 hw.write(0x4000, upper);

//                 let bank = upper;
//                 let addr = bank as usize * RAM_BANK_SIZE;
//                 let write_value = bank;
//                 hw.ram_mut()[addr] = write_value;
//                 let read_value = hw.read(0xA000);

//                 //assert_eq!(write_value, read_value);
//                 println!("Read bank {:0>4X}: {:0>4X}", bank, read_value);
//             }
//         }
//     }
// }
