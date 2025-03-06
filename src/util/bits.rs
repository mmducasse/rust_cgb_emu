/// Helper functions for getting and setting bit fields in an integer.
pub trait Bits<T> {
    /// Gets the value of the bit at `idx`.
    fn bit(&self, idx: u8) -> u8;

    /// Gets the value of the bit field from `hi` to `lo`.
    fn bits(&self, hi: u8, lo: u8) -> T;

    /// Sets the value of the bit at `idx`.
    fn set_bit(&mut self, idx: u8, value: u8);

    /// Sets the value of the bit field from `hi` to `lo`.
    fn set_bits(&mut self, hi: u8, lo: u8, value: u8);

    /// Copies the bits from `value` to `self` wherever `mask` is 1.
    fn set_bits_masked(&mut self, mask: u8, value: u8);

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_u8_bit() {
        let a0 = 0b0000_1110;

        assert_eq!(a0.bit(0), 0b0);
        assert_eq!(a0.bit(1), 0b1);
        assert_eq!(a0.bit(3), 0b1);
        assert_eq!(a0.bit(4), 0b0);
        assert_eq!(a0.bit(7), 0b0);
    }

    #[test]
    fn test_u8_bits() {
        let x = 0b0000_1110;
        assert_eq!(x.bits(3, 1), 0b111);
        assert_eq!(x.bits(4, 2), 0b011);

        let x = 0b0100_0001;
        assert_eq!(x.bits(7, 0), 0b0100_0001);
        assert_eq!(x.bits(3, 0), 0b0001);
        assert_eq!(x.bits(7, 4), 0b0100);
    }

    #[test]
    fn test_u8_set_bit() {
        let mut x = 0b0000_0000;
        x.set_bit(5, 1);
        assert_eq!(x, 0b0010_0000);

        let mut x = 0b1111_1111;
        x.set_bit(5, 0);
        assert_eq!(x, 0b1101_1111);
    }

    #[test]
    fn test_u8_set_bits() {
        let mut x = 0b0000_0000;
        x.set_bits(5, 2, 0b1111);
        assert_eq!(x, 0b0011_1100);

        let mut x = 0b1011_0110;
        x.set_bits(6, 3, 0b1001);
        assert_eq!(x, 0b1100_1110);
    }

    #[test]
    fn test_u8_set_bits_masked() {
        let mut x = 0b0000_0000;
        x.set_bits_masked(0b1010_1010, 0b1111_1111);
        assert_eq!(x, 0b1010_1010);

        let mut x = 0b1010_1010;
        x.set_bits_masked(0b0000_1111, 0b0000_0000);
        assert_eq!(x, 0b1010_0000);
    }
}
