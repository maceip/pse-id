use curve25519_dalek::ristretto::RistrettoPoint;
use curve25519_dalek::scalar::Scalar;
use sha2::{Digest, Sha256};
use rand::rngs::OsRng;
use rand::RngCore;

fn main() {
    // Step 1: Server's secret key setup
    let mut rng = OsRng; // Secure random number generator
    let mut random_bytes = [0u8; 32];
    rng.fill_bytes(&mut random_bytes);
    let server_key = Scalar::from_bytes_mod_order(random_bytes);

    println!("Server key generated: {:?}", server_key);

    // Step 2: User's account name is hashed to a group element
    let account_name_bytes = b"@exampleuser";

    // Hash the account name into a RistrettoPoint
    let mut hasher = Sha256::new();
    hasher.update(account_name_bytes);
    let hash_output = hasher.finalize();

    // RistrettoPoint requires 64 bytes for `from_uniform_bytes`, so we extend the hash
    let mut extended_hash = [0u8; 64];
    extended_hash[..32].copy_from_slice(&hash_output);
    let account_name = RistrettoPoint::from_uniform_bytes(&extended_hash);

    println!("Account name as Ristretto Point: {:?}", account_name);

    // Step 3: User blinds the input
    rng.fill_bytes(&mut random_bytes);
    let blinding_factor = Scalar::from_bytes_mod_order(random_bytes);
    let blinded_input = account_name * blinding_factor; // B = r * A

    println!("Blinded input: {:?}", blinded_input);

    // Step 4: Server computes the OPRF
    let blinded_output = blinded_input * server_key; // F_k(B) = k * B

    println!("Blinded output from server: {:?}", blinded_output);

    // Step 5: User unblinds the output
    let unblinded_output = blinded_output * blinding_factor.invert(); // F_k(A) = F_k(B) / r

    println!("Unblinded OPRF output: {:?}", unblinded_output);

    // Step 6: User hashes the OPRF output for Merkle tree inclusion
    let mut oprf_hasher = Sha256::new();
    oprf_hasher.update(unblinded_output.compress().as_bytes());
    let oprf_hashed_output = oprf_hasher.finalize();

    println!("Hashed OPRF output for Merkle tree: {:?}", oprf_hashed_output);
}
