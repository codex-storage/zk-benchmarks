#![no_main]

use std::io::Read;
use risc0_zkvm::{guest::env, sha, sha::Sha256};
risc0_zkvm::guest::entry!(main);
use risc0_zkvm::guest::env::cycle_count;

pub fn main() {
    let start = cycle_count();
    let mut data = Vec::<u8>::new();
    env::stdin().read_to_end(&mut data).unwrap();
    let end = cycle_count();
    let hash = sha::Impl::hash_bytes(&data);
    eprintln!("total cycle count for input: {:?}",end - start);
    eprintln!("total cycle count for hashing: {:?}",cycle_count());
    env::commit(&hash)
}

