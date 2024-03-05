use rand_core::OsRng;
use std::{thread, time};
use k256;
use u256;
pub mod key {

    struct PrivateKey {
        key: u256,
    }


    //hexadecimal and raw binary to be used for private key.
    impl PrivateKey {
        pub fn new() -> PrivateKey {
            // wiggling mouse increases entropy which increases randomness of private key
            println!("Please wiggle your mouse around for a few seconds to increase the entropy, and thus security of your private key!");
            println!("You have 5 seconds to wiggle your mouse around");
            thread::sleep(time::Duration::from_secs(5));
            let mut array = [0u8; 32]; // 256 bit zero-initialized number.
            OsRng.fill_bytes(&mut array); // fill the number with random numbers.
            let key = u256 {array}; //create private key, which is a 256 bit number.
            println!("New private key(binary): {:?}", key.get_binary()); //print private key
            println!("New private key(hexadecimal): {:?}", key.get_hex()); //print private key
            key // transfer ownership out.
        }
    }

    //compressed or uncompressed public key are the two formats. 
    //uncompressed starts with 04, compressed starts with 02 or 03
    //02 for even, 03 for odd (two possiblities due to square root)
    //TODO: implement secp256k1 from scratch.
    fn generate_public_key() -> String { //return public key

        //uncompressed form of G
        let G = "79BE667E F9DCBBAC 55A06295 CE870B07 029BFCDB 2DCE28D9 59F2815B 16F81798 483ADA77".split(" "); //returns an iterator.

        let curve = k256::Secp256k1 {};
    }
}