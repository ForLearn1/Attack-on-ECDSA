use rug::Integer;
use nalgebra::DMatrix;

///ECDSA attack by exploiting the MSB of k
pub fn break_ecdsa(samples: &Vec<(u64, Integer, Integer, Integer)>, msb: usize) {
    let q = Integer::from(1) << 256; // Order of secp256k1 curve
    let mut a_i = Vec::new();
    let mut t_i = Vec::new();

    for (k_msb, h, r, s) in samples.iter() {
        let s_inv = s.clone().invert(&q).unwrap();
        let a = ((Integer::from(*k_msb) - (s_inv.clone() * h)) % &q).abs();
        let t = (s_inv * r) % &q;
        a_i.push(a);
        t_i.push(t);
    }

    let m = samples.len();
    let mut mat = DMatrix::<Integer>::identity(m + 1, m + 1);

    for i in 0..m {
        mat[(i, m)] = t_i[i].clone();
        mat[(m, i)] = a_i[i].clone();
    }

    /// println!("Matrice construite, attaquez via LLL externe !");
}
