pub mod secp256k1 {
    use primitive_types::U256;
    // this is the equivalent of implementing OpenSSL's EC_POINT_ADD and EC_POINT_MUL, although they seem to implement some magic so that it runs in constant time.
    // so that theirs runs in constant time.

    // CREDIT: https://crypto.stanford.edu/pbc/notes/elliptic/explicit.html
    fn ec_point_double((x, y): (U256, U256)) -> (U256, U256) {
        let Gx: U256 = U256::from_str_radix(
            "79BE667EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B16F81798",
            16,
        )
        .unwrap();
        let Gy: U256 = U256::from_str_radix(
            "79BE667EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B16F81798",
            16,
        )
        .unwrap();
        let gradient: U256 = (x * 3) * modular_inverse(y * 2);
        let new_x: U256 = gradient * 2 - x * 2;
        let new_y: U256 = gradient * x - gradient * Gx + Gy;
        (new_x, new_y)
    }

    fn ec_point_add(p1: (U256, U256), p2: (U256, U256)) -> (U256, U256) {
        let lambda = (p2.1 - p1.1) * modular_inverse(p2.0 - p1.0);
        let x = lambda * 2 - p1.0 - p2.0;
        let y = lambda * (x - p1.0) + p1.1;
        println!("{:?}", (x, y));
        (x, y)
    }
    fn modular_inverse(val: U256) -> U256 {
        let n: U256 = U256::from_str_radix(
            "fffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd0364141",
            16,
        )
        .unwrap();
        // We are using the extended euclidean algorithm to calculate the modular inverse of val
    }

    // https://en.wikipedia.org/wiki/Elliptic_curve_point_multiplication
    // there was a chance to complete this with dynamic programming, but I felt that this solution was just as
    // elegant and efficient without having to keep a memo of all the values.
    pub fn ec_point_mul(factor: U256) -> (U256, U256) {
        let Gx: U256 = U256::from_str_radix(
            "79BE667EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B16F81798",
            16,
        )
        .unwrap();
        let Gy: U256 = U256::from_str_radix(
            "79BE667EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B16F81798",
            16,
        )
        .unwrap();
        let mut res = (Gx, Gy);
        let mut bin: String = "".to_string();
        for element in factor.0.iter() {
            bin = [bin, format!("{:64b}", element)].join("");
        }
        let mut bits = bin.chars();
        bits.next_back();
        for i in bits.rev() {
            res = ec_point_double(res);
            if i == '1' {
                res = ec_point_add(res, (Gx, Gy));
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
//     const Gx:U256 = 79BE667EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B16F81798;
//     const Gy:U256 = 483ADA7726A3C4655DA4FBFC0E1108A8FD17B448A68554199C47D08FFB10D4B8;
//     #[test]
//     fn test_ec_output() {
//         let num = ec_point_double((Gx, Gy));
//         assert_eq!()
//     }
// }
