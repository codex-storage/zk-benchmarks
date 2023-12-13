#![no_main]

use risc0_zkvm::{guest::env, sha, sha::Sha256};
risc0_zkvm::guest::entry!(main);

pub fn main() {

    let data: Vec<u8> = env::read();
    let hash = sha::Impl::hash_bytes(&data);
    env::commit(&hash)
}

// accelerated sha2 crate
// #![no_main]

// use risc0_zkvm::{guest::env};
// use sha2::{Sha256, Digest};
// // use base16ct::lower::encode_str;
// risc0_zkvm::guest::entry!(main);

// pub fn main() {

//     let data: Vec<u8> = env::read();
//     let result: [u8;32] = Sha256::digest(data).into();
//     env::commit(&result)
// }
