#![no_main]

use risc0_zkvm::{guest::env, sha::Digest};
use blake3::hash;

risc0_zkvm::guest::entry!(main);

pub fn main() {

    let data: Vec<u8> = env::read();
    let result = hash(&data);
    let digest = Digest::try_from(*result.as_bytes()).unwrap();
    env::commit(&digest)

}
