#![no_main]

use risc0_zkvm::{guest::env, sha::Digest};
use sha3::{Digest as _, Keccak256};

risc0_zkvm::guest::entry!(main);

pub fn main() {

    let data: Vec<u8> = env::read();
    let hash: [u8;32] = Keccak256::digest(data).into();
    let digest  = Digest::try_from(hash).unwrap();
    env::commit(&digest)

}
