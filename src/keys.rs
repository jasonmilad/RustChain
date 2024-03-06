pub mod keys {
    use primitive_types::U256;
    use rand_core::{Error, OsRng, RngCore};
    use std::{thread, time};

    //TODO: Use polymorphism/traits on the keys for greater readability. Low priority.
    pub struct PrivateKey {
        pub key: U256,
    }

    //hexadecimal and raw binary to be used for private key.
    impl PrivateKey {
        pub fn new() -> Result<PrivateKey, Error> {
            // wiggling mouse increases entropy which increases randomness of private key
            println!("Please wiggle your mouse around for a few seconds to increase the entropy, and thus security of your private key!");
            println!("You have 5 seconds to wiggle your mouse around");
            thread::sleep(time::Duration::from_secs(5));
            let mut array = [0u8; 32]; // 256 bit zero-initialized number.
            let zero_array = [0u8; 32];

            //make sure that the private key is greater than 0
            while array == zero_array {
                match OsRng.try_fill_bytes(&mut array) {
                    // safe fill array with random
                    Err(e) => return Err(e),
                    _ => (),
                }
            }
            let key = U256::from(array); //create private key value, which is a 256 bit number.
            // println!("New private key(binary): {:?}", key.get_binary()); //print private key
            // println!("New private key(hexadecimal): {:?}", key.get_hex()); //print private key
            Ok(PrivateKey { key }) // transfer ownership out.
        }
    }

    //compressed or uncompressed public key are the two formats.
    //uncompressed starts with 04, compressed starts with 02 or 03
    //02 for even, 03 for odd (two possiblities due to square root of y coordinate)
    //TODO: implement secp256k1 from scratch.
    // fn generate_public_key() -> String {
    //     //return public key
    //     // const G_X:[u8;32] = ;//x coordinate of the generator point
    //     // const G_Y:[u8;32] = ;//y coordinate of the generator point
    //     //uncompressed form of G
    //     //TODO: move this to seperate module and file.
    //     let G = "79BE667E F9DCBBAC 55A06295 CE870B07 029BFCDB 2DCE28D9 59F2815B 16F81798 483ADA77"
    //         .split(" "); //returns an iterator.
    // }
}

#[cfg(test)]
mod tests {
    use crate::keys::keys;
    #[test]
    fn private_key_valid() {
        let private_key = keys::PrivateKey::new().unwrap();
        let key = private_key.key;
        let zero_array: [u64;4] = [0u64; 4];
        assert_ne!(key.0, zero_array);
    }
}
