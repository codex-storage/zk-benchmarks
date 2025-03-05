#![no_main]

use risc0_zkvm::{guest::env, sha, sha::Sha256};
use std::io::Read;

risc0_zkvm::guest::entry!(main);

fn main() {
    let mut data = Vec::<u8>::new();
    env::stdin().read_to_end(&mut data).unwrap();

    let hash = sha::Impl::hash_bytes(&data);
    env::commit(&hash)
}


