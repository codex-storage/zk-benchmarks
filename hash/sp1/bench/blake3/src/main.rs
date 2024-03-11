
#![no_main]
sp1_zkvm::entrypoint!(main);

// blake3 of https://github.com/BLAKE3-team/BLAKE3 (official implementation)
use blake3::Hasher;
pub fn main() {

    let input = sp1_zkvm::io::read::<Vec<u8>>();

    // create a Blake3 object
    let mut hasher = Hasher::new();

    // write input message
    hasher.update(&input);

    // read hash digest
    let result: [u8;32] = hasher.finalize().into();

    sp1_zkvm::io::write(&result);

}
