#![no_main]
#![allow(non_snake_case)]
use risc0_zkvm::guest::env;
use zkhash::poseidon2::poseidon2;
use zkhash::poseidon2::poseidon2::Poseidon2;
use zkhash::poseidon2::poseidon2_instance_babybear::{/*POSEIDON2_BABYBEAR_16_PARAMS , */POSEIDON2_BABYBEAR_24_PARAMS};
use zkhash::fields::babybear::FpBabyBear;
use ark_serialize::{CanonicalSerialize, CanonicalDeserialize};
use std::marker::PhantomData;

use ark_ff::PrimeField;

pub trait MerkleTreeHash<F: PrimeField> {
    fn compress(&self, input: &[&F]) -> Vec<F>;
}

#[derive(Clone, Debug)]
pub struct MerkleTree<F: PrimeField, P: MerkleTreeHash<F>> {
    perm: P,
    field: PhantomData<F>,
}

impl<F: PrimeField, P: MerkleTreeHash<F>> MerkleTree<F, P> {
    pub fn new(perm: P) -> Self {
        MerkleTree {
            perm,
            field: PhantomData,
        }
    }

    fn round_up_pow_n(input: usize, n: usize) -> usize {
        debug_assert!(n >= 1);
        let mut res = 1;
        // try powers, starting from n
        loop {
            res *= n;
            if res >= input {
                break;
            }
        }
        res
    }

    pub fn accumulate(&mut self, set: &[F]) -> Vec<F> {
        assert!(set.len()%8 == 0);
        let set_size = set.len() / 8; 
        let mut bound = Self::round_up_pow_n(set_size, 2);
        loop {
            if bound >= 2 {
                break;
            }
            bound *= 2;
        }
        let mut nodes: Vec<F> = Vec::with_capacity(bound * 8);
        for s in set {
            nodes.push(s.to_owned());
        }
        // pad
        for _ in nodes.len()..bound {
            nodes.push(nodes[set_size * 8 - 1].to_owned());
        }

        while nodes.len() > 8 {
            let new_len = nodes.len() / 2;
            let mut new_nodes: Vec<F> = Vec::with_capacity(new_len);
            for i in (0..nodes.len()).step_by(16) {
                let inp = [
                    &nodes[i], &nodes[i + 1], &nodes[i + 2], &nodes[i + 3], &nodes[i + 4], &nodes[i + 5], &nodes[i + 6], &nodes[i + 7], 
                    &nodes[i + 8], &nodes[i + 9], &nodes[i + 10], &nodes[i + 11], &nodes[i + 12], &nodes[i + 13], &nodes[i + 14], &nodes[i + 15]
                ];
                let dig = self.perm.compress(&inp);

                for j in 0..8 {
                    new_nodes.push(dig[j]);
                }
                
            }
            nodes = new_nodes;
        }
        vec![nodes[0].to_owned(), nodes[1].to_owned(), nodes[2].to_owned(), nodes[3].to_owned(), nodes[4].to_owned(), nodes[5].to_owned(), nodes[6].to_owned(), nodes[7].to_owned()]
    }
}

impl<F: PrimeField> MerkleTreeHash<F> for Poseidon2<F> {
    fn compress(&self, input: &[&F]) -> Vec<F> {
        let p = self.permutation(&[
            input[0].to_owned(), input[1].to_owned(),input[2].to_owned(), input[3].to_owned(),input[4].to_owned(), input[5].to_owned(),input[6].to_owned(), input[7].to_owned(),
            input[8].to_owned(), input[9].to_owned(),input[10].to_owned(), input[11].to_owned(),input[12].to_owned(), input[13].to_owned(),input[14].to_owned(), input[15].to_owned(),
            F::zero(), F::zero(), F::zero(), F::zero(), F::zero(), F::zero(), F::zero(), F::zero()
        ]);

        vec![p[0], p[1], p[2], p[3], p[4], p[5], p[6], p[7]]
    }
}

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
    

    let permutation = poseidon2::Poseidon2::new(&POSEIDON2_BABYBEAR_24_PARAMS);
    let mut merkle_tree = MerkleTree::new(permutation.clone());
    let cycles3 = env::get_cycle_count();
    let hash_final = merkle_tree.accumulate(&hash_data);

    let cycles4 = env::get_cycle_count();
    
    let mut perm_seralised: Vec<Vec<u8>> = Vec::new();
    for i in 0..8 {
        let mut temp: Vec<u8> = Vec::new();
        hash_final.get(i).unwrap().serialize_uncompressed(&mut temp).unwrap();
        perm_seralised.push(temp);
    }
    let cycles6 = env::get_cycle_count();

    env::commit(&perm_seralised);

    eprintln!("number of cycles for input builder: {:?}", cycles2 - cycles1);
    eprintln!("number of cycles for hash permutation builder: {:?}", cycles3 - cycles2);
    eprintln!("number of cycles for hash  accumulation: {:?}", cycles4 - cycles3);

    eprintln!("number of cycles for permutation seralisation: {:?}", cycles6 - cycles4);

}
