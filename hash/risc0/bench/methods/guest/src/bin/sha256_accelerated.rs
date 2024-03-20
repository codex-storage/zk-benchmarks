
// accelerated sha2 crate
#![no_main]
use std::io::Read;
use risc0_zkvm::guest::env;
use sha2::{Sha256, Digest};
use risc0_zkvm::guest::env::cycle_count;
// use base16ct::lower::encode_str;
risc0_zkvm::guest::entry!(main);

pub fn main() {

    let mut data = Vec::<u8>::new();
    env::stdin().read_to_end(&mut data).unwrap();

    let result = Sha256::digest(data);
    let c1 = cycle_count();
    eprintln!("total cycle count for hashing: {:?}",c1);
    let result_bytes: [u8;32] = result.into();
    let c2 = cycle_count();
    eprintln!("cycle count to convert into bytes array: {:?}",c2-c1);
    env::commit(&result_bytes)
}
