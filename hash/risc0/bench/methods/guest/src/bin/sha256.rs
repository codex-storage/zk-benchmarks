#![no_main]

use risc0_zkvm::{guest::env, sha, sha::Sha256};
risc0_zkvm::guest::entry!(main);

pub fn main() {

    let data: Vec<u8> = env::read();
    let hash = sha::Impl::hash_bytes(&data);
    eprintln!("total cycle count for hashing: {:?}",env::get_cycle_count());
    env::commit(&hash)
}

