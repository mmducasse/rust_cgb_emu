/// Helper functions for getting and setting bit fields in an integer.
pub trait Bits<T> {
    /// Gets the value of the bit at `idx`.
    fn bit(&self, idx: u8) -> u8;

    /// Gets the value of the bit field from `hi` to `lo`.
    fn bits(&self, hi: u8, lo: u8) -> T;

    /// Sets the value of the bit at `idx`.
    fn set_bit(&mut self, idx: u8, value: u8);

    /// Sets the value of the bit field from `hi` to `lo`.
    fn set_bits(&mut self, hi: u8, lo: u8, value: T);

    /// Copies the bits from `value` to `self` wherever `mask` is 1.
    fn set_bits_masked(&mut self, mask: T, value: T);

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
    fn bits(&self, hi: u8, lo: u8) -> u8 {
        let mask = 0xFF;
        let mask = mask << (8 - (hi + 1));
        let mask = mask >> (8 - (hi + 1 - lo));

        return (*self >> lo) & mask;
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

    #[inline]
    fn set_bits(&mut self, hi: u8, lo: u8, value: u8) {
        let shift_r = 7 - (hi - lo);
        let shift_l = lo;
        let mask = (0xFF >> shift_r) << shift_l;
        let value = value << lo;

        self.set_bits_masked(mask, value);
    }

    #[inline]
    fn set_bits_masked(&mut self, mask: u8, value: u8) {
        *self = (*self & !mask) | (value & mask);
    }
}

impl Bits<u16> for u16 {
    #[inline]
    fn bit(&self, idx: u8) -> u8 {
        ((*self >> idx) & 0b1) as u8
    }

    #[inline]
    fn bits(&self, hi: u8, lo: u8) -> u16 {
        let mask = 0xFFFF;
        let mask = mask << (16 - (hi + 1));
        let mask = mask >> (16 - (hi + 1 - lo));

        return (*self >> lo) & mask;
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

    #[inline]
    fn set_bits(&mut self, hi: u8, lo: u8, value: u16) {
        let shift_r = 15 - (hi - lo);
        let shift_l = lo;
        let mask = (0xFFFF >> shift_r) << shift_l;
        let value = value << lo;

        self.set_bits_masked(mask, value);
    }

    #[inline]
    fn set_bits_masked(&mut self, mask: u16, value: u16) {
        *self = (*self & !mask) | (value & mask);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_u8_bit() {
        let a0: u8 = 0b0000_1110;

        assert_eq!(a0.bit(0), 0b0);
        assert_eq!(a0.bit(1), 0b1);
        assert_eq!(a0.bit(3), 0b1);
        assert_eq!(a0.bit(4), 0b0);
        assert_eq!(a0.bit(7), 0b0);
    }

    #[test]
    fn test_u8_bits() {
        let x: u8 = 0b0000_1110;
        assert_eq!(x.bits(3, 1), 0b111);
        assert_eq!(x.bits(4, 2), 0b011);

        let x: u8 = 0b0100_0001;
        assert_eq!(x.bits(7, 0), 0b0100_0001);
        assert_eq!(x.bits(3, 0), 0b0001);
        assert_eq!(x.bits(7, 4), 0b0100);
    }

    #[test]
    fn test_u8_set_bit() {
        let mut x: u8 = 0b0000_0000;
        x.set_bit(5, 1);
        assert_eq!(x, 0b0010_0000);

        let mut x: u8 = 0b1111_1111;
        x.set_bit(5, 0);
        assert_eq!(x, 0b1101_1111);
    }

    #[test]
    fn test_u8_set_bits() {
        let mut x: u8 = 0b0000_0000;
        x.set_bits(5, 2, 0b1111);
        assert_eq!(x, 0b0011_1100);

        let mut x: u8 = 0b1011_0110;
        x.set_bits(6, 3, 0b1001);
        assert_eq!(x, 0b1100_1110);
    }

    #[test]
    fn test_u8_set_bits_masked() {
        let mut x: u8 = 0b0000_0000;
        x.set_bits_masked(0b1010_1010, 0b1111_1111);
        assert_eq!(x, 0b1010_1010);

        let mut x: u8 = 0b1010_1010;
        x.set_bits_masked(0b0000_1111, 0b0000_0000);
        assert_eq!(x, 0b1010_0000);
    }

    #[test]
    fn test_u16_bit() {
        let a0: u16 = 0b1111_1101_0000_1110;

        assert_eq!(a0.bit(0), 0b0);
        assert_eq!(a0.bit(1), 0b1);
        assert_eq!(a0.bit(3), 0b1);
        assert_eq!(a0.bit(4), 0b0);
        assert_eq!(a0.bit(7), 0b0);
        assert_eq!(a0.bit(9), 0b0);
        assert_eq!(a0.bit(11), 0b1);
        assert_eq!(a0.bit(15), 0b1);
    }

    #[test]
    fn test_u16_bits() {
        let x: u16 = 0b0000_1110_0000_1110;
        assert_eq!(x.bits(3, 1), 0b111);
        assert_eq!(x.bits(4, 2), 0b011);

        let x: u16 = 0b0100_0001_0100_0001;
        assert_eq!(x.bits(7, 0), 0b0100_0001);
        assert_eq!(x.bits(3, 0), 0b0001);
        assert_eq!(x.bits(12, 4), 0b0_0001_0100);
        assert_eq!(x.bits(15, 8), 0b0100_0001);
    }
}
