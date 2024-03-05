
#![no_main]
sp1_zkvm::entrypoint!(main);

//The patched sha2 rust crate https://github.com/sp1-patches/RustCrypto-hashes
use sha2_v0_10_8::{Digest, Sha256};

pub fn main() {

    let input = sp1_zkvm::io::read::<Vec<u8>>();

    // create a Sha256 object
    let mut hasher = Sha256::new();

    // write input message
    hasher.update(input);

    // read hash digest
    let result: [u8;32] = hasher.finalize().into();

    sp1_zkvm::io::write(&result);
}