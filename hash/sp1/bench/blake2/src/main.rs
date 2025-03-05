
#![no_main]
sp1_zkvm::entrypoint!(main);

// blake2 of RustCrypto
use blake2::Digest;
use blake2::Blake2b;
pub fn main() {

    let input = sp1_zkvm::io::read::<Vec<u8>>();

    // create a Blake2b object
    let mut hasher = Blake2b::new();

    // write input message
    hasher.update(input);

    // read hash digest
    let result: [u8;32] = hasher.finalize().into();

    sp1_zkvm::io::write(&result);

}
