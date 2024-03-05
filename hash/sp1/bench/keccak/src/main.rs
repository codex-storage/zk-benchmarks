
#![no_main]
sp1_zkvm::entrypoint!(main);
use sha3::{Digest, Keccak256};

pub fn main() {

    let input = sp1_zkvm::io::read::<Vec<u8>>();

    // create a keccak object
    let mut hasher = Keccak256::new();

    // write input message
    hasher.update(input);

    // read hash digest
    let result: [u8;32] = hasher.finalize().into();

    sp1_zkvm::io::write(&result);
}
