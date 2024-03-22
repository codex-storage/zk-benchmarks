
// accelerated sha2 crate
#![no_main]
use std::io::Read;
use risc0_zkvm::guest::env;
use sha2::{Sha256, Digest};

risc0_zkvm::guest::entry!(main);

pub fn main() {

    let mut data = Vec::<u8>::new();
    env::stdin().read_to_end(&mut data).unwrap();

    let result = Sha256::digest(data);

    let result_bytes: [u8;32] = result.into();

    env::commit(&result_bytes)
}
