pub trait Bits<T> {
    fn bit(&self, idx: u8) -> u8;
    fn set_bit(&mut self, idx: u8, value: u8);

    fn toggle_bit(&mut self, idx: u8) -> u8 {
        let next = !self.bit(idx);
        self.set_bit(idx, next);
        return next;
    }
}

impl Bits<u8> for u8 {
    #[inline]
    fn bit(&self, idx: u8) -> u8 {
        (*self >> idx) & 0b1
    }

    #[inline]
    fn set_bit(&mut self, idx: u8, value: u8) {
        let mask = 0x1 << idx;

        if (value & 0x1) > 0 {
            *self |= mask;
        } else {
            *self &= !mask;
        }
    }
}

// impl Bits<u16> for u16 {
//     fn bit(&self, idx: u8) -> u8 {
//         return bit16(self, idx);
//     }

//     fn set_bit(&mut self, idx: u8, value: u8) {
//         set_bit16(self, idx, value);
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bit8() {
        let a0 = 0b0000_1110;

        assert_eq!(a0.bit(0), 0b0);
        assert_eq!(a0.bit(1), 0b1);
        assert_eq!(a0.bit(3), 0b1);
        assert_eq!(a0.bit(4), 0b0);
        assert_eq!(a0.bit(7), 0b0);
    }

    #[test]
    fn test_set_bit8() {
        let mut x = 0b0000_0000;
        x.set_bit(5, 1);
        assert_eq!(x, 0b0010_0000);

        let mut x = 0b1111_1111;
        x.set_bit(5, 0);
        assert_eq!(x, 0b1101_1111);
    }
}
