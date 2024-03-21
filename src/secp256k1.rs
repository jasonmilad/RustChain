pub mod secp256k1 {
    use bigint::uint::{U256, U512};
    use num::{BigInt, Num};
    // this is the equivalent of OpenSSL's EC_POINT_ADD and EC_POINT_MUL

    fn hex_to_dec(hex_str: &str) -> String {
        BigInt::from_str_radix(hex_str, 16).unwrap().to_string()
    }
    // CREDIT: https://crypto.stanford.edu/pbc/notes/elliptic/explicit.html
    fn ec_point_double((x, y): (U512, U512)) -> (U512, U512) {
        let dec_gx = hex_to_dec("79BE667EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B16F81798");
        let gx: U512 = U512::from_dec_str(dec_gx.as_str()).unwrap();
        let dec_gy = hex_to_dec("483ADA7726A3C4655DA4FBFC0E1108A8FD17B448A68554199C47D08FFB10D4B8");
        let gy: U512 = U512::from_dec_str(dec_gy.as_str()).unwrap();
        let two: U512 = U512::from(2);
        let three: U512 = U512::from(3);
        let dec_n = hex_to_dec("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F");
        let n: U512 = U512::from_dec_str(dec_n.as_str()).unwrap();

        // only the U256 type defines modular inverse, so we need to do some type casting.
        let lambda: U512 = (((((three % n) * (x % n)) % n * (x % n)) % n
            * (U512::from(U256::from(((two) * (y))).mod_inverse(U256::from(n)))))
            % n);
        println!("{:?}", (x, y));
        println!("{:?}", lambda);
        let temp = ((lambda % n) * (lambda % n)) % n;
        let temp2 = ((x % n) * (two % n)) % n;
        println!("{:?}", temp);
        println!("{:?}", temp2);
        let new_x = (temp - temp2) % n;
        let new_y = (((lambda % n * (x % n)) % n + (gy % n - (lambda % n * gx % n) % n) % n) % n);
        (new_x, new_y)
    }

    fn ec_point_add(p1: (U512, U512), p2: (U512, U512)) -> (U512, U512) {
        let dec_n = hex_to_dec("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F");
        let n: U512 = U512::from_dec_str(dec_n.as_str()).unwrap();
        let lambda = ((p2.1 - p1.1) % (n)
            * U512::from((U256::from((p2.0 - p1.0)).mod_inverse(U256::from(n)))))
            % n;
        let x = ((lambda % n) * (lambda % n) % n + (p1.0 % n + p2.0 % n)) % n;
        let y = ((lambda) % n * ((p1.0 - x) % n - p1.1 % n)) % n;
        println!("{:?}", (x, y));
        (x, y)
    }

    // https://en.wikipedia.org/wiki/Elliptic_curve_point_multiplication
    // there was a chance to complete this with dynamic programming, but I felt that this solution was just as
    // elegant and efficient without having to keep a memo of all the values.
    pub fn ec_point_mul(factor: U512) -> (U512, U512) {
        let dec_gx = hex_to_dec("79BE667EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B16F81798");
        let gx: U512 = U512::from_dec_str(dec_gx.as_str()).unwrap();
        let dec_gy = hex_to_dec("483ADA7726A3C4655DA4FBFC0E1108A8FD17B448A68554199C47D08FFB10D4B8");
        let gy: U512 = U512::from_dec_str(dec_gy.as_str()).unwrap();
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
    use bigint::uint::U512;
    #[test]
    fn private_key_valid() {
        let private_key = keys::PrivateKey::new().unwrap();
        let key = private_key.key;
        println!("{:?}", secp256k1::ec_point_mul(U512::from(key)));
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
