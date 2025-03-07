Elliptic Curve Cryptography (ECDSA) and MSB Attack
ECDSA is widely used to secure blockchain transactions, encrypted communications, and many other applications. But did you know that poor nonce 
𝑘 management can compromise the private key?

I developed a Rust project that implements an attack exploiting the MSB of 𝑘, allowing private key recovery via lattice reduction (LLL).
How does it work?
ECDSA relies on a random nonce 𝑘. If part of 𝑘's bits leak (e.g., via a side-channel attack), a linear system can be formulated. Lattice reduction is then used to recover the private key.

🛠️ Project Structure:
✅ Keygen module: ECDSA key generation
✅ Signature module: Signing with MSB leakage
✅ Break_ecdsa module: Attack and private key recovery
