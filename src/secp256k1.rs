pub mod secp256k1 {
    use bigint::uint::U256;
    use num::{BigInt, Num};
    // this is the equivalent of OpenSSL's EC_POINT_ADD and EC_POINT_MUL

    fn hex_to_dec(hex_str: &str) -> String {
        BigInt::from_str_radix(hex_str, 16).unwrap().to_string()
    }
    // CREDIT: https://crypto.stanford.edu/pbc/notes/elliptic/explicit.html
    fn ec_point_double((x, y): (U256, U256)) -> (U256, U256) {
        let dec_gx = hex_to_dec("79BE667EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B16F81798");
        let gx: U256 = U256::from_dec_str(dec_gx.as_str()).unwrap();
        let dec_gy = hex_to_dec("483ADA7726A3C4655DA4FBFC0E1108A8FD17B448A68554199C47D08FFB10D4B8");
        let gy: U256 = U256::from_dec_str(dec_gy.as_str()).unwrap();
        let two: U256 = U256::from(2);
        let three: U256 = U256::from(3);
        let dec_n = hex_to_dec("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F");
        let n: U256 = U256::from_dec_str(dec_n.as_str()).unwrap();
        let lambda: U256 = (((((three % n) * (x % n)) % n * (x % n)) * two.mod_inverse(n)) % n
            * y.mod_inverse(n))
            % n;
        let new_x: U256 = ((lambda % n) * (lambda % n)) % n - ((x % n) * (two % n)) % n;
        let new_y: U256 =
            ((lambda % n * (x % n)) % n + (gy % n - (lambda % n * gx % n) % n) % n) % n;
        (new_x, new_y)
    }

    fn ec_point_add(p1: (U256, U256), p2: (U256, U256)) -> (U256, U256) {
        let dec_n = hex_to_dec("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F");
        let n: U256 = U256::from_dec_str(dec_n.as_str()).unwrap();
        let lambda = ((p2.1 - p1.1) % (n) * (p2.0 - p1.0).mod_inverse(n)) % n;
        let x = ((lambda % n) * (lambda % n) % n + (p1.0 % n + p2.0 % n)) % n;
        let y = ((lambda) % n * ((p1.0 - x) % n - p1.1 % n)) % n;
        println!("{:?}", (x, y));
        (x, y)
    }

    // https://en.wikipedia.org/wiki/Elliptic_curve_point_multiplication
    // there was a chance to complete this with dynamic programming, but I felt that this solution was just as
    // elegant and efficient without having to keep a memo of all the values.
    pub fn ec_point_mul(factor: U256) -> (U256, U256) {
        let dec_gx = hex_to_dec("79BE667EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B16F81798");
        let gx: U256 = U256::from_dec_str(dec_gx.as_str()).unwrap();
        let dec_gy = hex_to_dec("483ADA7726A3C4655DA4FBFC0E1108A8FD17B448A68554199C47D08FFB10D4B8");
        let gy: U256 = U256::from_dec_str(dec_gy.as_str()).unwrap();
        let mut res = (gx, gy);
        let mut bin: String = "".to_string();
        for element in factor.0.iter() {
            bin = [bin, format!("{:064b}", element)].join("");
        }
        println!("{}", bin);
        //let mut bits = bin.chars();
        //bits.next_back();
        for i in bin.chars().rev() {
            println!("{}", i);
            res = ec_point_double(res);
            if i == '1' {
                res = ec_point_add(res, (gx, gy));
            }
        }
        res
    }
}
#[cfg(test)]
mod tests {
    use crate::keys::keys;
    use crate::secp256k1::secp256k1;
    #[test]
    fn private_key_valid() {
        let private_key = keys::PrivateKey::new().unwrap();
        let key = private_key.key;
        println!("{:?}", secp256k1::ec_point_mul(key));
    }
}
// #[cfg(test)]
// mod tests {
//     use crate::secp256k1::secp256k1;
//     const gx:U256 = 79BE667EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B16F81798;
//     const gy:U256 = 483ADA7726A3C4655DA4FBFC0E1108A8FD17B448A68554199C47D08FFB10D4B8;
//     #[test]
//     fn test_ec_output() {
//         let num = ec_point_double((gx, gy));
//         assert_eq!()
//     }
// }
