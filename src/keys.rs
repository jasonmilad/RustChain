use openssl::bn::BigNumContext;
use openssl::ec::{EcGroup, EcKey, PointConversionForm};
use openssl::error::ErrorStack;
use openssl::nid::Nid;
use openssl::pkey::Private;
use openssl::string::OpensslString;
const SECP256K1: Nid = Nid::SECP256K1;
pub struct User {
    full_key: EcKey<Private>,
}
impl User {
    pub fn new() -> Result<User, ErrorStack> {
        #[allow(non_snake_case)]
        let SECP256K1_Curve: EcGroup = EcGroup::from_curve_name(SECP256K1)?;
        let key = EcKey::generate(&SECP256K1_Curve)?;
        let new_user = User { full_key: key };
        Ok(new_user)
    }
    pub fn get_private_key(&self) -> Result<OpensslString, ErrorStack> {
        self.full_key.private_key().to_hex_str()
    }
    //TODO: convert the public key to hex
    pub fn get_compressed_public_key(&self) -> Result<OpensslString, ErrorStack> {
        #[allow(non_snake_case)]
        let SECP256K1_Curve: EcGroup = EcGroup::from_curve_name(SECP256K1)?;
        let mut ctxt = BigNumContext::new()?;
        self.full_key.public_key().to_hex_str(
            &SECP256K1_Curve,
            PointConversionForm::COMPRESSED,
            &mut ctxt,
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::keys;
    #[test]
    fn compressed_public_key_len_valid() {
        let user = keys::User::new().unwrap();
        //TODO: add support for uncompressed form.
        assert_eq!(user.get_compressed_public_key().unwrap().len(), 66)
    }
    #[test]
    fn private_key_len_valid() {
        let user = keys::User::new().unwrap();
        assert_eq!(user.get_private_key().unwrap().len(), 64);
    }
}
