#![no_main]
#![allow(non_snake_case)]
use risc0_zkvm::{guest::env/* , sha::Digest*/};
// use sha3::{Digest as _, Keccak256};
use zkhash::poseidon2::poseidon2;
use zkhash::poseidon2::poseidon2_instance_babybear::{POSEIDON2_BABYBEAR_16_PARAMS/* , POSEIDON2_BABYBEAR_24_PARAMS*/};
use zkhash::fields::babybear::FpBabyBear;
use ark_serialize::{CanonicalSerialize, CanonicalDeserialize};

risc0_zkvm::guest::entry!(main);

pub fn main() {

    let data: Vec<Vec<u8>> = env::read();
    let cycles1 = env::get_cycle_count();
    let mut hash_data: Vec<FpBabyBear> = Vec::new();
    for i in 0..data.len() {
        let a_uncompressed = FpBabyBear::deserialize_uncompressed(&**data.get(i).unwrap()).unwrap();
        hash_data.push(a_uncompressed);
    }
    let cycles2 = env::get_cycle_count();
    
    let permutation = poseidon2::Poseidon2::new(&POSEIDON2_BABYBEAR_16_PARAMS);
    let perm: Vec<FpBabyBear> = permutation.permutation(&hash_data);

    let cycles4 = env::get_cycle_count();
    
    let mut perm_seralised: Vec<Vec<u8>> = Vec::new();
    for i in 0..data.len() {
        let mut temp: Vec<u8> = Vec::new();
        perm.get(i).unwrap().serialize_uncompressed(&mut temp).unwrap();
        perm_seralised.push(temp);
    }
    let cycles6 = env::get_cycle_count();

    env::commit(&perm_seralised);

    eprintln!("number of cycles for input builder: {:?}", cycles2 - cycles1);
    eprintln!("number of cycles for hash permutation builder: {:?}", cycles4 - cycles2);
    eprintln!("number of cycles for permutation seralisation: {:?}", cycles6 - cycles4);

}
