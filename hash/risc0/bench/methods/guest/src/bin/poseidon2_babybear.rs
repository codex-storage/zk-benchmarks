#![no_main]
#![allow(non_snake_case)]
use ark_ff::Field;
use risc0_zkvm::{guest::env/* , sha::Digest*/};
// use sha3::{Digest as _, Keccak256};
use zkhash::poseidon2::poseidon2;
use zkhash::poseidon2::poseidon2::Poseidon2;
use zkhash::poseidon2::poseidon2_instance_babybear::{/*POSEIDON2_BABYBEAR_16_PARAMS , */POSEIDON2_BABYBEAR_24_PARAMS};
use zkhash::fields::babybear::FpBabyBear;
use ark_serialize::{CanonicalSerialize, CanonicalDeserialize};
// use zkhash::merkle_tree::merkle_tree_fp::MerkleTreeHash;
use risc0_core::field::Elem;

use std::sync::Arc;
use std::marker::PhantomData;
use risc0_core::field::baby_bear::BabyBear;
use lazy_static::lazy_static;

// #[derive(Clone, Debug)]
// pub struct Poseidon2Params<F: Elem> {
//     pub(crate) t: usize, // statesize
//     pub(crate) d: usize, // sbox degree
//     pub(crate) rounds_f_beginning: usize,
//     pub(crate) rounds_p: usize,
//     #[allow(dead_code)]
//     pub(crate) rounds_f_end: usize,
//     pub(crate) rounds: usize,
//     pub(crate) mat_internal_diag_m_1: Vec<F>,
//     pub(crate) _mat_internal: Vec<Vec<F>>,
//     pub(crate) round_constants: Vec<Vec<F>>,
// }

// pub fn mat_inverse<F: Elem>(mat: &[Vec<F>]) -> Vec<Vec<F>> {
//     let n = mat.len();
//     assert!(mat[0].len() == n);

//     let mut m = mat.to_owned();
//     let mut inv = vec![vec![F::ZERO; n]; n];
//     for (i, invi) in inv.iter_mut().enumerate() {
//         invi[i] = F::ONE;
//     }

//     // upper triangle
//     for row in 0..n {
//         for j in 0..row {
//             // subtract from these rows
//             let el = m[row][j];
//             for col in 0..n {
//                 // do subtraction for each col
//                 if col < j {
//                     m[row][col] = F::ZERO;
//                 } else {
//                     let mut tmp = m[j][col];
//                     tmp.mul_assign(el);
//                     m[row][col].sub_assign(tmp);
//                 }
//                 if col > row {
//                     inv[row][col] = F::ZERO;
//                 } else {
//                     let mut tmp = inv[j][col];
//                     tmp.mul_assign(el);
//                     inv[row][col].sub_assign(tmp);
//                 }
//             }
//         }
//         // make 1 in diag
//         let el_inv = m[row][row].inv();
//         for col in 0..n {
//             match col.cmp(&row) {
//                 std::cmp::Ordering::Less => inv[row][col].mul_assign(el_inv),
//                 std::cmp::Ordering::Equal => {
//                     m[row][col] = F::ONE;
//                     inv[row][col].mul_assign(el_inv)
//                 }
//                 std::cmp::Ordering::Greater => m[row][col].mul_assign(el_inv),
//             }
//         }
//     }

//     // upper triangle
//     for row in (0..n).rev() {
//         for j in (row + 1..n).rev() {
//             // subtract from these rows
//             let el = m[row][j];
//             for col in 0..n {
//                 // do subtraction for each col

//                 #[cfg(debug_assertions)]
//                 {
//                     if col >= j {
//                         m[row][col] = F::ZERO;
//                     }
//                 }
//                 let mut tmp = inv[j][col];
//                 tmp.mul_assign(el);
//                 inv[row][col].sub_assign(tmp);
//             }
//         }
//     }

//     #[cfg(debug_assertions)]
//     {
//         for (row, mrow) in m.iter().enumerate() {
//             for (col, v) in mrow.iter().enumerate() {
//                 if row == col {
//                     debug_assert!(*v == F::ONE);
//                 } else {
//                     debug_assert!(*v == F::ZERO);
//                 }
//             }
//         }
//     }

//     inv
// }

// impl<F: Elem> Poseidon2Params<F> {
//     #[allow(clippy::too_many_arguments)]

//     pub const INIT_SHAKE: &'static str = "Poseidon2";

//     pub fn new(
//         t: usize,
//         d: usize,
//         rounds_f: usize,
//         rounds_p: usize,
//         mat_internal_diag_m_1: &[F],
//         mat_internal: &[Vec<F>],
//         round_constants: &[Vec<F>],
//     ) -> Self {
//         assert!(d == 3 || d == 5 || d == 7 || d == 11);
//         assert_eq!(rounds_f % 2, 0);
//         let r = rounds_f / 2;
//         let rounds = rounds_f + rounds_p;

//         Poseidon2Params {
//             t,
//             d,
//             rounds_f_beginning: r,
//             rounds_p,
//             rounds_f_end: r,
//             rounds,
//             mat_internal_diag_m_1: mat_internal_diag_m_1.to_owned(),
//             _mat_internal: mat_internal.to_owned(),
//             round_constants: round_constants.to_owned(),
//         }
//     }

    
//     // Unused
//     pub fn equivalent_round_constants(
//         round_constants: &[Vec<F>],
//         mat_internal: &[Vec<F>],
//         rounds_f_beginning: usize,
//         rounds_p: usize,
//     ) -> Vec<Vec<F>> {
//         let mut opt = vec![Vec::new(); rounds_p + 1];
//         let mat_internal_inv = mat_inverse(mat_internal);

//         let p_end = rounds_f_beginning + rounds_p - 1;
//         let mut tmp = round_constants[p_end].clone();
//         for i in (0..rounds_p - 1).rev() {
//             let inv_cip = Self::mat_vec_mul(&mat_internal_inv, &tmp);
//             opt[i + 1] = vec![inv_cip[0]];
//             tmp = round_constants[rounds_f_beginning + i].clone();
//             for i in 1..inv_cip.len() {
//                 tmp[i].add_assign(inv_cip[i]);
//             }
//         }
//         opt[0] = tmp;
//         opt[rounds_p] = vec![F::ZERO; opt[0].len()]; // opt[0].len() = t

//         opt
//     }

//     pub fn mat_vec_mul(mat: &[Vec<F>], input: &[F]) -> Vec<F> {
//         let t = mat.len();
//         debug_assert!(t == input.len());
//         let mut out = vec![F::ZERO; t];
//         for row in 0..t {
//             for (col, inp) in input.iter().enumerate() {
//                 let mut tmp = mat[row][col];
//                 tmp.mul_assign(*inp);
//                 out[row].add_assign(tmp);
//             }
//         }
//         out
//     }

// }

// #[derive(Clone, Debug)]
// pub struct Poseidon2<F: Elem> {
//     pub(crate) params: Arc<Poseidon2Params<F>>,
// }

// impl<F: Elem> Poseidon2<F> {
//     pub fn new(params: &Arc<Poseidon2Params<F>>) -> Self {
//         Poseidon2 {
//             params: Arc::clone(params),
//         }
//     }

//     pub fn get_t(&self) -> usize {
//         self.params.t
//     }

//     pub fn permutation(&self, input: &[F]) -> Vec<F> {
//         let t = self.params.t;
//         assert_eq!(input.len(), t);

//         let mut current_state = input.to_owned();

//         // Linear layer at beginning
//         self.matmul_external(&mut current_state);

//         for r in 0..self.params.rounds_f_beginning {
//             current_state = self.add_rc(&current_state, &self.params.round_constants[r]);
//             current_state = self.sbox(&current_state);
//             self.matmul_external(&mut current_state);
//         }

//         let p_end = self.params.rounds_f_beginning + self.params.rounds_p;
//         for r in self.params.rounds_f_beginning..p_end {
//             current_state[0].add_assign(self.params.round_constants[r][0]);
//             current_state[0] = self.sbox_p(&current_state[0]);
//             self.matmul_internal(&mut current_state, &self.params.mat_internal_diag_m_1);
//         }
        
//         for r in p_end..self.params.rounds {
//             current_state = self.add_rc(&current_state, &self.params.round_constants[r]);
//             current_state = self.sbox(&current_state);
//             self.matmul_external(&mut current_state);
//         }
//         current_state
//     }

//     fn sbox(&self, input: &[F]) -> Vec<F> {
//         input.iter().map(|el| self.sbox_p(el)).collect()
//     }

//     fn sbox_p(&self, input: &F) -> F {
//         let mut input2 = *input;
//         input2.mul_assign(input2);

//         match self.params.d {
//             3 => {
//                 let mut out = input2;
//                 out.mul_assign(*input);
//                 out
//             }
//             5 => {
//                 let mut out = input2;
//                 out.mul_assign(out);
//                 out.mul_assign(*input);
//                 out
//             }
//             7 => {
//                 let mut out = input2;
//                 out.mul_assign(out);
//                 out.mul_assign(input2);
//                 out.mul_assign(*input);
//                 out
//             }
//             _ => {
//                 panic!()
//             }
//         }
//     }

//     fn matmul_m4(&self, input: &mut[F]) {
//         let t = self.params.t;
//         let t4 = t / 4;
//         for i in 0..t4 {
//             let start_index = i * 4;
//             let mut t_0 = input[start_index];
//             t_0.add_assign(input[start_index + 1]);
//             let mut t_1 = input[start_index + 2];
//             t_1.add_assign(input[start_index + 3]);
//             let mut t_2 = input[start_index + 1];
//             t_2.add_assign(t_2);
//             t_2.add_assign(t_1);
//             let mut t_3 = input[start_index + 3];
//             t_3.add_assign(t_3);
//             t_3.add_assign(t_0);
//             let mut t_4 = t_1;
//             t_4.add_assign(t_4);
//             t_4.add_assign(t_4);
//             t_4.add_assign(t_3);
//             let mut t_5 = t_0;
//             t_5.add_assign(t_5);
//             t_5.add_assign(t_5);
//             t_5.add_assign(t_2);
//             let mut t_6 = t_3;
//             t_6.add_assign(t_5);
//             let mut t_7 = t_2;
//             t_7.add_assign(t_4);
//             input[start_index] = t_6;
//             input[start_index + 1] = t_5;
//             input[start_index + 2] = t_7;
//             input[start_index + 3] = t_4;
//         }
//     }

//     fn matmul_external(&self, input: &mut[F]) {
//         let t = self.params.t;
//         match t {
//             2 => {
//                 // Matrix circ(2, 1)
//                 let mut sum = input[0];
//                 sum.add_assign(input[1]);
//                 input[0].add_assign(sum);
//                 input[1].add_assign(sum);
//             }
//             3 => {
//                 // Matrix circ(2, 1, 1)
//                 let mut sum = input[0];
//                 sum.add_assign(input[1]);
//                 sum.add_assign(input[2]);
//                 input[0].add_assign(sum);
//                 input[1].add_assign(sum);
//                 input[2].add_assign(sum);
//             }
//             4 => {
//                 // Applying cheap 4x4 MDS matrix to each 4-element part of the state
//                 self.matmul_m4(input);
//             }
//             8 | 12 | 16 | 20 | 24 => {
//                 // Applying cheap 4x4 MDS matrix to each 4-element part of the state
//                 self.matmul_m4(input);

//                 // Applying second cheap matrix for t > 4
//                 let t4 = t / 4;
//                 let mut stored = [F::ZERO; 4];
//                 for l in 0..4 {
//                     stored[l] = input[l];
//                     for j in 1..t4 {
//                         stored[l].add_assign(input[4 * j + l]);
//                     }
//                 }
//                 for i in 0..input.len() {
//                     input[i].add_assign(stored[i % 4]);
//                 }
//             }
//             _ => {
//                 panic!()
//             }
//         }
//     }

//     fn matmul_internal(&self, input: &mut[F], mat_internal_diag_m_1: &[F]) {
//         let t = self.params.t;

//         match t {
//             2 => {
//                 // [2, 1]
//                 // [1, 3]
//                 let mut sum = input[0];
//                 sum.add_assign(input[1]);
//                 input[0].add_assign(sum);
//                 input[1].add_assign(input[1]);
//                 input[1].add_assign(sum);
//             }
//             3 => {
//                 // [2, 1, 1]
//                 // [1, 2, 1]
//                 // [1, 1, 3]
//                 let mut sum = input[0];
//                 sum.add_assign(input[1]);
//                 sum.add_assign(input[2]);
//                 input[0].add_assign(sum);
//                 input[1].add_assign(sum);
//                 input[2].add_assign(input[2]);
//                 input[2].add_assign(sum);
//             }
//             4 | 8 | 12 | 16 | 20 | 24 => {
//                 // Compute input sum
//                 let mut sum = input[0];
//                 input
//                     .iter()
//                     .skip(1)
//                     .take(t-1)
//                     .for_each(|el| sum.add_assign(*el));
//                 // Add sum + diag entry * element to each element
//                 for i in 0..input.len() {
//                     input[i].mul_assign(mat_internal_diag_m_1[i]);
//                     input[i].add_assign(sum);
//                 }
//             }
//             _ => {
//                 panic!()
//             }
//         }
//     }

//     fn add_rc(&self, input: &[F], rc: &[F]) -> Vec<F> {
//         input
//             .iter()
//             .zip(rc.iter())
//             .map(|(a, b)| {
//                 let mut r = *a;
//                 r.add_assign(*b);
//                 r
//             })
//             .collect()
//     }
// }

// pub trait MerkleTreeHash<F: Elem> {
//     fn compress(&self, input: &[&F]) -> Vec<F>;
// }

// #[derive(Clone, Debug)]
// pub struct MerkleTree<F: Elem, P: MerkleTreeHash<F>> {
//     perm: P,
//     field: PhantomData<F>,
// }

// impl<F: Elem, P: MerkleTreeHash<F>> MerkleTree<F, P> {
//     pub fn new(perm: P) -> Self {
//         MerkleTree {
//             perm,
//             field: PhantomData,
//         }
//     }

//     fn round_up_pow_n(input: usize, n: usize) -> usize {
//         debug_assert!(n >= 1);
//         let mut res = 1;
//         // try powers, starting from n
//         loop {
//             res *= n;
//             if res >= input {
//                 break;
//             }
//         }
//         res
//     }

//     // pub fn accumulate(&mut self, set: &[F]) -> F {
//     //     let set_size = set.len();
//     //     let mut bound = Self::round_up_pow_n(set_size, 2);
//     //     loop {
//     //         if bound >= 2 {
//     //             break;
//     //         }
//     //         bound *= 2;
//     //     }
//     //     let mut nodes: Vec<F> = Vec::with_capacity(bound);
//     //     for s in set {
//     //         nodes.push(s.to_owned());
//     //     }
//     //     // pad
//     //     for _ in nodes.len()..bound {
//     //         nodes.push(nodes[set_size - 1].to_owned());
//     //     }

//     //     while nodes.len() > 1 {
//     //         let new_len = nodes.len() / 2;
//     //         let mut new_nodes: Vec<F> = Vec::with_capacity(new_len);
//     //         for i in (0..nodes.len()).step_by(2) {
//     //             let inp = [&nodes[i], &nodes[i + 1]];
//     //             let dig = self.perm.compress(&inp);
//     //             new_nodes.push(dig);
//     //         }
//     //         nodes = new_nodes;
//     //     }
//     //     nodes[0].to_owned()
//     // }

//     pub fn accumulate(&mut self, set: &[F]) -> Vec<F> {
//         let set_size = set.len();
//         let mut bound = Self::round_up_pow_n(set_size, 2);
//         loop {
//             if bound >= 16 {
//                 break;
//             }
//             bound *= 2;
//         }
    
//         let mut nodes: Vec<F> = Vec::with_capacity(bound);
    
//         // Populate nodes with set elements
//         for s in set {
//             nodes.push(s.to_owned());
//         }
    
//         // Pad nodes to reach the required size
//         while nodes.len() < bound {
//             nodes.push(nodes[set_size - 1].to_owned());
//         }
    
//         // Compress pairs of 8 elements until a single set of 8 elements is left
//         while nodes.len() > 8 {
//             let new_len = nodes.len() / 2;
//             let mut new_nodes: Vec<F> = Vec::with_capacity(new_len);
    
//             // Compress pairs of 8 elements at a time
//             for i in (0..nodes.len()).step_by(16) {
//                 let inp: Vec<_> = nodes[i..i + 16].iter().collect();
//                 let dig = self.perm.compress(&inp);
//                 new_nodes.push(dig[0]);
//                 new_nodes.push(dig[1]);
//                 new_nodes.push(dig[2]);
//                 new_nodes.push(dig[3]);
//                 new_nodes.push(dig[4]);
//                 new_nodes.push(dig[5]);
//                 new_nodes.push(dig[6]);
//                 new_nodes.push(dig[7]);
//             }
    
//             nodes = new_nodes;
//         }
    
//         nodes
//     }
    
// }

// impl<F: Elem> MerkleTreeHash<F> for Poseidon2<F> {
//     fn compress(&self, input: &[&F]) -> Vec<F> {
//         let res = self.permutation(&[
//             input[0].to_owned(), input[0].to_owned(), input[0].to_owned(), input[0].to_owned(), input[0].to_owned(), input[0].to_owned(), input[0].to_owned(), input[0].to_owned(),
//             input[0].to_owned(), input[0].to_owned(), input[0].to_owned(), input[0].to_owned(), input[0].to_owned(), input[0].to_owned(), input[0].to_owned(), input[0].to_owned(),
//             F::ZERO, F::ZERO, F::ZERO, F::ZERO, F::ZERO, F::ZERO, F::ZERO, F::ZERO]);

//             vec![res[0], res[1], res[2], res[3], res[4], res[5], res[6], res[7] ]
//     }
// }










































// for POSEIDON2_BABYBEAR_24_PARAMS only
pub fn compress(perm: Poseidon2<FpBabyBear>, input: Vec<FpBabyBear>) -> Vec<FpBabyBear>{
    let p = perm.permutation(
        &[
        *input.get(0).unwrap(), *input.get(1).unwrap(), *input.get(2).unwrap(), *input.get(3).unwrap(), *input.get(4).unwrap(), *input.get(5).unwrap(), *input.get(6).unwrap(), *input.get(7).unwrap(),
        *input.get(8).unwrap(), *input.get(9).unwrap(), *input.get(10).unwrap(), *input.get(11).unwrap(), *input.get(12).unwrap(), *input.get(13).unwrap(), *input.get(14).unwrap(), *input.get(15).unwrap(),
        FpBabyBear::from(0), FpBabyBear::from(0), FpBabyBear::from(0), FpBabyBear::from(0), FpBabyBear::from(0), FpBabyBear::from(0), FpBabyBear::from(0), FpBabyBear::from(0)
        ]
    );
    vec![p[0], p[1], p[2], p[3], p[4], p[5], p[6], p[7] ]
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
    // let perm: Vec<FpBabyBear> = permutation.permutation(&hash_data);
    let input1: [FpBabyBear;24] = [
        *hash_data.get(0).unwrap(), *hash_data.get(1).unwrap(), *hash_data.get(2).unwrap(), *hash_data.get(3).unwrap(), *hash_data.get(4).unwrap(), *hash_data.get(5).unwrap(), *hash_data.get(6).unwrap(), *hash_data.get(7).unwrap(),
        *hash_data.get(8).unwrap(), *hash_data.get(9).unwrap(), *hash_data.get(10).unwrap(), *hash_data.get(11).unwrap(), *hash_data.get(12).unwrap(), *hash_data.get(13).unwrap(), *hash_data.get(14).unwrap(), *hash_data.get(15).unwrap(),
        FpBabyBear::from(0), FpBabyBear::from(0), FpBabyBear::from(0), FpBabyBear::from(0), FpBabyBear::from(0), FpBabyBear::from(0), FpBabyBear::from(0), FpBabyBear::from(0)
        ]; 

    let input2: [FpBabyBear;24] = [
        *hash_data.get(16).unwrap(), *hash_data.get(17).unwrap(), *hash_data.get(18).unwrap(), *hash_data.get(19).unwrap(), *hash_data.get(20).unwrap(), *hash_data.get(21).unwrap(), *hash_data.get(22).unwrap(), *hash_data.get(23).unwrap(),
        *hash_data.get(24).unwrap(), *hash_data.get(25).unwrap(), *hash_data.get(26).unwrap(), *hash_data.get(27).unwrap(), *hash_data.get(28).unwrap(), *hash_data.get(29).unwrap(), *hash_data.get(30).unwrap(), *hash_data.get(31).unwrap(),
        FpBabyBear::from(0), FpBabyBear::from(0), FpBabyBear::from(0), FpBabyBear::from(0), FpBabyBear::from(0), FpBabyBear::from(0), FpBabyBear::from(0), FpBabyBear::from(0)
        ]; 

    let output1 = compress(permutation.clone(), input1.into());
    let output2 = compress(permutation.clone(), input2.into());

    let input3 = vec![
        output1[0], output1[1], output1[2], output1[3], output1[4], output1[5], output1[6], output1[7], 
        output2[0], output2[1], output2[2], output2[3], output2[4], output2[5], output2[6], output2[7], 
        FpBabyBear::from(0), FpBabyBear::from(0), FpBabyBear::from(0), FpBabyBear::from(0), FpBabyBear::from(0), FpBabyBear::from(0), FpBabyBear::from(0), FpBabyBear::from(0)
    ];
    let output_final = compress(permutation.clone(), input3);
    

    // // let mut merkle_tree = MerkleTree::new(permutation.clone());
    // let cycles4 = env::get_cycle_count();
    // // let hash_final = merkle_tree.accumulate(&hash_data);
    // let cycles5 = env::get_cycle_count();

    let cycles4 = env::get_cycle_count();
    
    let mut perm_seralised: Vec<Vec<u8>> = Vec::new();
    for i in 0..8 {
        let mut temp: Vec<u8> = Vec::new();
        output_final.get(i).unwrap().serialize_uncompressed(&mut temp).unwrap();
        perm_seralised.push(temp);
    }
    let cycles6 = env::get_cycle_count();

    env::commit(&perm_seralised);

    eprintln!("number of cycles for input builder: {:?}", cycles2 - cycles1);
    eprintln!("number of cycles for hash permutation builder: {:?}", cycles4 - cycles2);
    eprintln!("number of cycles for permutation seralisation: {:?}", cycles6 - cycles4);

}
