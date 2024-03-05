pub struct u256 {
    arr128:[u128; 2],
    binary_string: String,
    hex_string: String,
}

impl u256 {
    pub fn new(arr: &mut [u8;32]) -> Self {
        let mut left_sum:u128 = 0;
        let mut right_sum:u128 = 0;
        for i in 0..16 {
            left_sum += arr[i] as u128;
        }
        for i in 16..32 {
            right_sum += arr[i] as u128;
        }
        let new_arr:[u128;2] = [left_sum, right_sum];
        let binary_string = [format!("{:b}", left_sum), format!("{:b}", right_sum)].join("");
        let hex_string = [format!("{:X}", left_sum), format!("{:X}", right_sum)].join("");
        Self {arr128: new_arr, binary_string, hex_string}
    }

    pub fn get_value(&self) -> &[u128; 2] {
        &self.arr128
    }

    pub fn get_binary(&self) -> &String {
        &self.binary_string
    }

    pub fn get_hex(&self) -> &String {
        &self.hex_string
    }
}