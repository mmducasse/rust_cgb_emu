#[inline]
pub fn split_16(data: u16) -> (u8, u8) {
    let hi = ((data & 0xFF00) >> 8) as u8;
    let lo = (data & 0x00FF) as u8;

    return (hi, lo);
}

#[inline]
pub fn join_16(hi: u8, lo: u8) -> u16 {
    let hi = (hi as u16) << 8;
    let lo = lo as u16;

    return hi | lo;
}

#[inline]
pub fn add16_ui(a: u16, b: i16) -> u16 {
    if b >= 0 {
        let b = b as u16;
        u16::wrapping_add(a, b)
    } else {
        let b = (-b) as u16;
        u16::wrapping_sub(a, b)
    }
}

#[inline]
pub fn add16_uu(a: u16, b: u16) -> u16 {
    u16::wrapping_add(a, b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_join16_split16() {
        let x = join_16(0xFF, 0x77);
        assert_eq!(x, 0xFF77);

        let (hi, lo) = split_16(x);
        assert_eq!(hi, 0xFF);
        assert_eq!(lo, 0x77);

        let x = join_16(0x12, 0xAD);
        assert_eq!(x, 0x12AD);

        let (hi, lo) = split_16(x);
        assert_eq!(hi, 0x12);
        assert_eq!(lo, 0xAD);
    }

    #[test]
    fn test_add16_ui() {
        let y = add16_ui(0xFFFF, 0);
        assert_eq!(y, 0xFFFF);

        let y = add16_ui(0xFFFF, 1);
        assert_eq!(y, 0);

        let y = add16_ui(0, -1);
        assert_eq!(y, 0xFFFF);
    }
}
