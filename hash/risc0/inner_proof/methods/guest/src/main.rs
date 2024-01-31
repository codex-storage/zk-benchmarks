#![no_main]

use risc0_zkvm::{guest::env, sha, sha::Sha256};

risc0_zkvm::guest::entry!(main);

fn main() {
    let data: String = env::read();
    let hash = sha::Impl::hash_bytes(&data.as_bytes());
    env::commit(&hash)
}


