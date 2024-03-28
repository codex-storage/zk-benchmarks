#![no_main]
#![allow(non_snake_case)]
use risc0_core::field::baby_bear::BabyBearElem;
use risc0_zkp::core::hash::poseidon2::Poseidon2HashSuite;
use risc0_zkvm::guest::env;

risc0_zkvm::guest::entry!(main);

pub fn main() {

    let data: Vec<u32> = env::read();
    
    let mut hash_data: Vec<BabyBearElem> = Vec::new();
    for i in 0..data.len() {
        let a_uncompressed = BabyBearElem::from(*data.get(i).unwrap());
        hash_data.push(a_uncompressed);
    }

    let result = Poseidon2HashSuite::new_suite().hashfn.hash_elem_slice(&hash_data);

    env::commit(&result);


}
