use rand_core::{OsRng};
use std::{thread, time};
pub mod key {

    fn generate_key() -> [u8:32] {
        println!("Please wiggle your mouse around for a few seconds to increase the entropy, and thus security of your private key!");
        println!("You have 5 seconds to wiggle your mouse around");
        thread::sleep(time::Duration::from_secs(5));
        let mut key = [0u8: 32]; // 256 bit random number
        OsRng.fill_bytes(&mut key);
        key
    }

}