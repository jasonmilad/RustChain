pub mod keys {
    use openssl::bn::{BigNumContext, BigNumRef};
    use openssl::ec::{EcGroup, EcKey, EcPointRef};
    use openssl::error::ErrorStack;
    use openssl::nid::Nid;
    const SECP256K1: Nid = Nid::SECP256K1;
    pub struct User {
        public_key: EcPointRef,
        private_key: BigNumRef,
    }
    impl User {
        pub fn new() -> Result<User, ErrorStack> {
            #[allow(non_snake_case)]
            let SECP256K1_Curve: EcGroup = EcGroup::from_curve_name(SECP256K1)?;
            let key = EcKey::generate(&SECP256K1_Curve)?;
            let new_user = User {
                public_key: *key.public_key(),
                private_key: *key.private_key(),
            };
            Ok(new_user)
        }
    }
}
mod tests {
    #[test]
    fn public_key_valid() {
        let private_key = keys::PrivateKey::new().unwrap();
        let key = private_key.key;
        println!("{:?}", secp256k1::ec_point_mul(U512::from(key)));
    }

    #[test]
    fn private_key_valid() {
        let private_key = keys::PrivateKey::new().unwrap();
        let key = private_key.key;
        let zero_array: [u64; 4] = [0u64; 4];
        assert_ne!(key.0, zero_array);
    }
}
