use secp256k1::{Secp256k1, SecretKey, PublicKey};

/// Generates an ECDSA key pair
pub fn key_gen() -> (SecretKey, PublicKey) {
    let secp = Secp256k1::new();
    let sk = SecretKey::new(&mut rand::thread_rng());
    let pk = PublicKey::from_secret_key(&secp, &sk);
    (sk, pk)
}
