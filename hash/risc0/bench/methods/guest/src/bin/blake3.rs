#![no_main]

use risc0_zkvm::{guest::env, sha::Digest};
use blake3::hash;
use std::io::Read;

risc0_zkvm::guest::entry!(main);

pub fn main() {

    let mut data = Vec::<u8>::new();
    env::stdin().read_to_end(&mut data).unwrap();

    let result = hash(&data);
    let digest = Digest::try_from(*result.as_bytes()).unwrap();
    env::commit(&digest)

}
