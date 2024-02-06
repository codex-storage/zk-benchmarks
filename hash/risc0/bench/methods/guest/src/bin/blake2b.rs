#![no_main]

use risc0_zkvm::{guest::env, sha::Digest};
use risc0_zkp::core::hash::blake2b::{Blake2b, Blake2bCpuImpl};

risc0_zkvm::guest::entry!(main);

pub fn main() {

    let data: Vec<u8> = env::read();
    let hash = Blake2bCpuImpl::blake2b(&data);
    let digest: Digest = hash.into();
    env::commit(&digest)

}
