#[macro_export]
macro_rules! bits {
    (@msb; $num:expr, $n:expr) => {{
        let shift = 8 - $n;
        let result: u8 = $num >> shift;
        result
    }};
    (@lsb; $num:expr, $n:expr) => {{
        let shift = 8 - $n;
        let left_shift: u8 = $num << shift;
        let right_shift: u8 = left_shift >> shift;
        right_shift
    }};
}
#[macro_export]
macro_rules! bits16 {
    (@msb; $num:expr, $n:expr) => {{
        let shift = 16 - $n;
        let result: u16 = $num >> shift;
        result
    }};
    (@lsb; $num:expr, $n:expr) => {{
        let shift = 16 - $n;
        let left_shift: u16 = $num << shift;
        let right_shift: u16 = left_shift >> shift;
        right_shift
    }};
}

pub fn push_bits(n: &mut u16, bit_to_push: u8) {
    if bit_to_push > 1 {
        panic!("???? bit to push should to either 0 or 1");
    }
    *n = *n << 1;
    if bit_to_push != 0 {
        *n = *n + 0b1;
    }
}

#[cfg(test)]
mod tests {
    use crate::common::binary_macros::push_bits;

    #[test]
    fn test_push_bits() {
        let mut n = 0;
        push_bits(&mut n, 1);
        assert_eq!(n, 0b0000_0001);

        push_bits(&mut n, 0);
        assert_eq!(n, 0b0000_0010);

        push_bits(&mut n, 1);
        assert_eq!(n, 0b0000_101);
        push_bits(&mut n, 1);
        assert_eq!(n, 0b0000_1011);
    }

    #[test]
    fn test_msb() {
        assert_eq!(bits!(@msb; 0b1100_0000, 2), 0b11);
        assert_eq!(bits!(@msb; 0b1100_0000, 4), 0b1100);

        assert_eq!(bits16!(@msb; 0b1011_0000_1100_0000, 4), 0b1011);
        assert_eq!(bits16!(@msb; 0b1011_0000_1100_0000, 8), 0b1011_0000);
    }
    #[test]
    fn test_lsb() {
        assert_eq!(bits!(@lsb; 0b1100_1101, 2), 0b01);
        assert_eq!(bits!(@lsb; 0b1100_1101, 4), 0b1101);

        assert_eq!(bits16!(@lsb; 0b1011_0000_1100_1101, 4), 0b1101);
        assert_eq!(bits16!(@lsb; 0b1011_0000_1100_1101, 8), 0b1100_1101);
    }
}
