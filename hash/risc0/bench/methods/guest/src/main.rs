#![no_main]
// If you want to try std support, also update the guest Cargo.toml file


use risc0_zkvm::{guest::env, sha, sha::Sha256};

risc0_zkvm::guest::entry!(main);


pub fn main() {
    // // TODO: Implement your guest code here

    // // read the input
    // let input: u32 = env::read();

    // // TODO: do something with the input

    // // write public output to the journal
    // env::commit(&input);

    let data: Vec<u8> = env::read();
    let hash = sha::Impl::hash_bytes(&data);
    env::commit(&hash)
}
