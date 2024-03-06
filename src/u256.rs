use byteorder::{ByteOrder, LittleEndian};
pub struct U256 {
    //TODO: implement buffer as u64 due to native support of 64-bit data and thus greater efficiency.
    pub buf: [u128; 2],
}

impl U256 {
    //TODO: add support for 64-bit buffer elements.
    pub fn new(arr: [u8; 32]) -> Self {
        let mut side = &arr[0..16];
        let mut u128_buf = [0u128; 2];
        u128_buf[0] = LittleEndian::read_u128(side);
        side = &arr[16..32];
        u128_buf[1] = LittleEndian::read_u128(side);
        Self { buf: u128_buf }
    }

    pub fn get_value(&self) -> &[u128; 2] {
        &self.buf
    }

    pub fn get_binary(&self) -> String {
        //TODO: don't hardcode the number of elements in the array
        [
            format!("{:128b}", self.buf[0]),
            format!("{:128b}", self.buf[1]),
        ]
        .join("")
    }

    pub fn get_hex(&self) -> String {
        //TODO: don't hardcode the number of elements in the array.
        [
            format!("{:32X}", self.buf[0]),
            format!("{:32X}", self.buf[1]),
        ]
        .join("")
    }
}

#[cfg(test)]
mod tests {
    use crate::u256::U256;
    use std::mem;
    #[test]
    fn u256_preserves_value() {
        let num:U256 = U256::new([15u8;32]);
        let mut digit = 0;
        for _ in 0..=(mem::size_of_val(&num.buf[0])) {
            digit = (digit << 8) + 15;
        }
        println!("{}", num.buf[1]);
        for i in num.buf.iter() {
            assert_eq!(&digit, i);
        }
    }

    #[test]
    fn correct_string_len() {
        let num:U256 = U256::new([10u8;32]);
        assert_eq!(num.buf.len(), 2);
        assert_eq!(num.get_hex().chars().count(), 64);
        assert_eq!(num.get_binary().chars().count(), 256);
    }
}