
#![no_main]
sp1_zkvm::entrypoint!(main);
use tiny_keccak::{Hasher, Keccak};
pub fn main() {

    let input = sp1_zkvm::io::read::<Vec<u8>>();

    // create a keccak object
    let mut hasher = Keccak::v256();

    // write input message
    hasher.update(&input);

    // read hash digest
    let mut result: [u8;32] = [0;32];

    hasher.finalize(&mut result);

    sp1_zkvm::io::write(&result);
}
