use secp256k1::rand::rngs::OsRng;
use secp256k1::{PublicKey, Secp256k1, SecretKey};

use openssl::ec::{EcGroup, EcKey};
use openssl::nid::Nid;

fn create(path: String) -> (SecretKey, PublicKey) {
    let secp = Secp256k1::new();
    let mut rng = OsRng::new().expect("OsRng");
    let (secret_key, public_key) = secp.generate_keypair(&mut rng);

    // todo: Implement file write
    if !path.is_empty() {
        let group = EcGroup::from_curve_name(Nid::SECP256K1).unwrap();
        let key = EcKey::generate(&group).unwrap();
        let pem = key.private_key_to_pem().unwrap();
    }

    (secret_key, public_key)
}
