use secp256k1::{Secp256k1, SecretKey, Message};
use sha2::{Sha256, Digest};
use rug::Integer;
use rand::Rng;

/// Message signature with MSB leakage from k
pub fn sign_message(secp: &Secp256k1, sk: &SecretKey, message: &[u8], msb: usize) -> (u64, Integer, Integer, Integer) {
    let mut rng = rand::thread_rng();
    let k: u64 = rng.gen_range(1..(1 << 256)); // Génère k aléatoire

    let k_bin = format!("{:b}", k);
    let leaked_k = &k_bin[..msb]; // MSB recovery

    let hash = Sha256::digest(message);
    let msg = Message::from_slice(&hash).expect("Message invalide");
    let sig = secp.sign_ecdsa(&msg, sk);

    let (r, s) = match sig.serialize_der().split_last() {
        Some((s, r)) => (Integer::from_digits(r, rug::integer::Order::MsfBe), Integer::from(*s)),
        None => panic!("Erreur de signature"),
    };

    (leaked_k.parse::<u64>().unwrap(), Integer::from(hash.as_slice()), r, s)
}
