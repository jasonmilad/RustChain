use byteorder::{ByteOrder, LittleEndian};
    pub struct U256 {
        pub buf:[u128; 2],
    }

    impl U256 {
        pub fn new(arr: [u8;32]) -> Self {
            let mut side = &arr[0..16];
            let mut u128_buf = [0u128; 2];
            u128_buf[0] = LittleEndian::read_u128(side);
            side = &arr[16..32];
            u128_buf[1] = LittleEndian::read_u128(side);
            Self {buf: u128_buf}
        }

        pub fn get_value(&self) -> &[u128; 2] {
            &self.buf
        }

        pub fn get_binary(&self) -> String {
            [format!("{:128b}", self.buf[0]), format!("{:128b}", self.buf[1])].join("")
        }

        pub fn get_hex(&self) -> String {
            [format!("{:32X}", self.buf[0]), format!("{:32X}", self.buf[1])].join("")
        }
    }