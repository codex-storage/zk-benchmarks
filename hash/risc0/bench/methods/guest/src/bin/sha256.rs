#![no_main]

use std::io::Read;
use risc0_zkvm::{guest::env, sha, sha::Sha256};
risc0_zkvm::guest::entry!(main);

pub fn main() {

    let mut data = Vec::<u8>::new();
    env::stdin().read_to_end(&mut data).unwrap();

    let hash = sha::Impl::hash_bytes(&data);

    env::commit(&hash)
}

