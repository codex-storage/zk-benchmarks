#![no_main]
#![allow(non_snake_case)]
use risc0_core::field::baby_bear::BabyBearElem;
use risc0_zkvm::guest::env;
use std::marker::PhantomData;
use std::sync::Arc;
use risc0_core::field::Elem;
use lazy_static::lazy_static;

// This code is adapted from https://github.com/HorizenLabs/poseidon2/tree/main
#[derive(Clone, Debug)]
pub struct Poseidon2Params<F: Elem> {
    pub(crate) t: usize, // statesize
    pub(crate) d: usize, // sbox degree
    pub(crate) rounds_f_beginning: usize,
    pub(crate) rounds_p: usize,
    #[allow(dead_code)]
    pub(crate) rounds_f_end: usize,
    pub(crate) rounds: usize,
    pub(crate) mat_internal_diag_m_1: Vec<F>,
    pub(crate) _mat_internal: Vec<Vec<F>>,
    pub(crate) round_constants: Vec<Vec<F>>,
}

impl<F: Elem> Poseidon2Params<F> {
    #[allow(clippy::too_many_arguments)]

    pub const INIT_SHAKE: &'static str = "Poseidon2";

    pub fn new(
        t: usize,
        d: usize,
        rounds_f: usize,
        rounds_p: usize,
        mat_internal_diag_m_1: &[F],
        mat_internal: &[Vec<F>],
        round_constants: &[Vec<F>],
    ) -> Self {
        assert!(d == 3 || d == 5 || d == 7 || d == 11);
        assert_eq!(rounds_f % 2, 0);
        let r = rounds_f / 2;
        let rounds = rounds_f + rounds_p;

        Poseidon2Params {
            t,
            d,
            rounds_f_beginning: r,
            rounds_p,
            rounds_f_end: r,
            rounds,
            mat_internal_diag_m_1: mat_internal_diag_m_1.to_owned(),
            _mat_internal: mat_internal.to_owned(),
            round_constants: round_constants.to_owned(),
        }
    }
}
#[derive(Clone, Debug)]
pub struct Poseidon2<F: Elem> {
    pub(crate) params: Arc<Poseidon2Params<F>>,
}

impl<F: Elem> Poseidon2<F> {
    pub fn new(params: &Arc<Poseidon2Params<F>>) -> Self {
        Poseidon2 {
            params: Arc::clone(params),
        }
    }

    pub fn get_t(&self) -> usize {
        self.params.t
    }

    pub fn permutation(&self, input: &[F]) -> Vec<F> {
        let t = self.params.t;
        assert_eq!(input.len(), t);

        let mut current_state = input.to_owned();

        // Linear layer at beginning
        self.matmul_external(&mut current_state);

        for r in 0..self.params.rounds_f_beginning {
            current_state = self.add_rc(&current_state, &self.params.round_constants[r]);
            current_state = self.sbox(&current_state);
            self.matmul_external(&mut current_state);
        }

        let p_end = self.params.rounds_f_beginning + self.params.rounds_p;
        for r in self.params.rounds_f_beginning..p_end {
            current_state[0].add_assign(self.params.round_constants[r][0]);
            current_state[0] = self.sbox_p(&current_state[0]);
            self.matmul_internal(&mut current_state, &self.params.mat_internal_diag_m_1);
        }
        
        for r in p_end..self.params.rounds {
            current_state = self.add_rc(&current_state, &self.params.round_constants[r]);
            current_state = self.sbox(&current_state);
            self.matmul_external(&mut current_state);
        }
        current_state
    }

    fn sbox(&self, input: &[F]) -> Vec<F> {
        input.iter().map(|el| self.sbox_p(el)).collect()
    }

    fn sbox_p(&self, input: &F) -> F {
        let mut input2 = *input;
        input2.mul_assign(input2);

        match self.params.d {
            3 => {
                let mut out = input2;
                out.mul_assign(*input);
                out
            }
            5 => {
                let mut out = input2;
                out.mul_assign(out);
                out.mul_assign(*input);
                out
            }
            7 => {
                let mut out = input2;
                out.mul_assign(out);
                out.mul_assign(input2);
                out.mul_assign(*input);
                out
            }
            _ => {
                panic!()
            }
        }
    }

    fn matmul_m4(&self, input: &mut[F]) {
        let t = self.params.t;
        let t4 = t / 4;
        for i in 0..t4 {
            let start_index = i * 4;
            let mut t_0 = input[start_index];
            t_0.add_assign(input[start_index + 1]);
            let mut t_1 = input[start_index + 2];
            t_1.add_assign(input[start_index + 3]);
            let mut t_2 = input[start_index + 1];
            t_2.add_assign(t_2);
            t_2.add_assign(t_1);
            let mut t_3 = input[start_index + 3];
            t_3.add_assign(t_3);
            t_3.add_assign(t_0);
            let mut t_4 = t_1;
            t_4.add_assign(t_4);
            t_4.add_assign(t_4);
            t_4.add_assign(t_3);
            let mut t_5 = t_0;
            t_5.add_assign(t_5);
            t_5.add_assign(t_5);
            t_5.add_assign(t_2);
            let mut t_6 = t_3;
            t_6.add_assign(t_5);
            let mut t_7 = t_2;
            t_7.add_assign(t_4);
            input[start_index] = t_6;
            input[start_index + 1] = t_5;
            input[start_index + 2] = t_7;
            input[start_index + 3] = t_4;
        }
    }

    fn matmul_external(&self, input: &mut[F]) {
        let t = self.params.t;
        match t {
            2 => {
                // Matrix circ(2, 1)
                let mut sum = input[0];
                sum.add_assign(input[1]);
                input[0].add_assign(sum);
                input[1].add_assign(sum);
            }
            3 => {
                // Matrix circ(2, 1, 1)
                let mut sum = input[0];
                sum.add_assign(input[1]);
                sum.add_assign(input[2]);
                input[0].add_assign(sum);
                input[1].add_assign(sum);
                input[2].add_assign(sum);
            }
            4 => {
                // Applying cheap 4x4 MDS matrix to each 4-element part of the state
                self.matmul_m4(input);
            }
            8 | 12 | 16 | 20 | 24 => {
                // Applying cheap 4x4 MDS matrix to each 4-element part of the state
                self.matmul_m4(input);

                // Applying second cheap matrix for t > 4
                let t4 = t / 4;
                let mut stored = [F::ZERO; 4];
                for l in 0..4 {
                    stored[l] = input[l];
                    for j in 1..t4 {
                        stored[l].add_assign(input[4 * j + l]);
                    }
                }
                for i in 0..input.len() {
                    input[i].add_assign(stored[i % 4]);
                }
            }
            _ => {
                panic!()
            }
        }
    }

    fn matmul_internal(&self, input: &mut[F], mat_internal_diag_m_1: &[F]) {
        let t = self.params.t;

        match t {
            2 => {
                // [2, 1]
                // [1, 3]
                let mut sum = input[0];
                sum.add_assign(input[1]);
                input[0].add_assign(sum);
                input[1].add_assign(input[1]);
                input[1].add_assign(sum);
            }
            3 => {
                // [2, 1, 1]
                // [1, 2, 1]
                // [1, 1, 3]
                let mut sum = input[0];
                sum.add_assign(input[1]);
                sum.add_assign(input[2]);
                input[0].add_assign(sum);
                input[1].add_assign(sum);
                input[2].add_assign(input[2]);
                input[2].add_assign(sum);
            }
            4 | 8 | 12 | 16 | 20 | 24 => {
                // Compute input sum
                let mut sum = input[0];
                input
                    .iter()
                    .skip(1)
                    .take(t-1)
                    .for_each(|el| sum.add_assign(*el));
                // Add sum + diag entry * element to each element
                for i in 0..input.len() {
                    input[i].mul_assign(mat_internal_diag_m_1[i]);
                    input[i].add_assign(sum);
                }
            }
            _ => {
                panic!()
            }
        }
    }

    fn add_rc(&self, input: &[F], rc: &[F]) -> Vec<F> {
        input
            .iter()
            .zip(rc.iter())
            .map(|(a, b)| {
                let mut r = *a;
                r.add_assign(*b);
                r
            })
            .collect()
    }
}


//merkle tree
pub trait MerkleTreeHash<F: Elem> {
    fn compress(&self, input: &[&F]) -> Vec<F>;
}

#[derive(Clone, Debug)]
pub struct MerkleTree<F: Elem, P: MerkleTreeHash<F>> {
    perm: P,
    field: PhantomData<F>,
}

impl<F: Elem, P: MerkleTreeHash<F>> MerkleTree<F, P> {
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

impl<F: Elem> MerkleTreeHash<F> for Poseidon2<F> {
    fn compress(&self, input: &[&F]) -> Vec<F> {
        let p = self.permutation(&[
            input[0].to_owned(), input[1].to_owned(),input[2].to_owned(), input[3].to_owned(),input[4].to_owned(), input[5].to_owned(),input[6].to_owned(), input[7].to_owned(),
            input[8].to_owned(), input[9].to_owned(),input[10].to_owned(), input[11].to_owned(),input[12].to_owned(), input[13].to_owned(),input[14].to_owned(), input[15].to_owned(),
            F::ZERO, F::ZERO, F::ZERO, F::ZERO, F::ZERO, F::ZERO, F::ZERO, F::ZERO
        ]);

        vec![p[0], p[1], p[2], p[3], p[4], p[5], p[6], p[7]]
    }
}

//---------------------
lazy_static!{
pub static ref MAT_DIAG24_M_1: Vec<BabyBearElem> = vec![
    BabyBearElem::from(0x409133f0 as u32),
    BabyBearElem::from(0x1667a8a1 as u32),
    BabyBearElem::from(0x06a6c7b6 as u32),
    BabyBearElem::from(0x6f53160e as u32),
    BabyBearElem::from(0x273b11d1 as u32),
    BabyBearElem::from(0x03176c5d as u32),
    BabyBearElem::from(0x72f9bbf9 as u32),
    BabyBearElem::from(0x73ceba91 as u32),
    BabyBearElem::from(0x5cdef81d as u32),
    BabyBearElem::from(0x01393285 as u32),
    BabyBearElem::from(0x46daee06 as u32),
    BabyBearElem::from(0x065d7ba6 as u32),
    BabyBearElem::from(0x52d72d6f as u32),
    BabyBearElem::from(0x05dd05e0 as u32),
    BabyBearElem::from(0x3bab4b63 as u32),
    BabyBearElem::from(0x6ada3842 as u32),
    BabyBearElem::from(0x2fc5fbec as u32),
    BabyBearElem::from(0x770d61b0 as u32),
    BabyBearElem::from(0x5715aae9 as u32),
    BabyBearElem::from(0x03ef0e90 as u32),
    BabyBearElem::from(0x75b6c770 as u32),
    BabyBearElem::from(0x242adf5f as u32),
    BabyBearElem::from(0x00d0ca4c as u32),
    BabyBearElem::from(0x36c0e388 as u32),
    ];

    pub static ref MAT_INTERNAL24: Vec<Vec<BabyBearElem>> = vec![
    vec![BabyBearElem::from(0x409133f1 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    ],
    vec![BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x1667a8a2 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    ],
    vec![BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x06a6c7b7 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    ],
    vec![BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x6f53160f as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    ],
    vec![BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x273b11d2 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    ],
    vec![BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x03176c5e as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    ],
    vec![BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x72f9bbfa as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    ],
    vec![BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x73ceba92 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    ],
    vec![BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x5cdef81e as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    ],
    vec![BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x01393286 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    ],
    vec![BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x46daee07 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    ],
    vec![BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x065d7ba7 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    ],
    vec![BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x52d72d70 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    ],
    vec![BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x05dd05e1 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    ],
    vec![BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x3bab4b64 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    ],
    vec![BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x6ada3843 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    ],
    vec![BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x2fc5fbed as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    ],
    vec![BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x770d61b1 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    ],
    vec![BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x5715aaea as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    ],
    vec![BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x03ef0e91 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    ],
    vec![BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x75b6c771 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    ],
    vec![BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x242adf60 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    ],
    vec![BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00d0ca4d as u32),
    BabyBearElem::from(0x00000001 as u32),
    ],
    vec![BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x00000001 as u32),
    BabyBearElem::from(0x36c0e389 as u32),
    ],
    ];
    
    pub static ref RC24: Vec<Vec<BabyBearElem>> = vec![
    vec![BabyBearElem::from(0x0fa20c37 as u32),
    BabyBearElem::from(0x0795bb97 as u32),
    BabyBearElem::from(0x12c60b9c as u32),
    BabyBearElem::from(0x0eabd88e as u32),
    BabyBearElem::from(0x096485ca as u32),
    BabyBearElem::from(0x07093527 as u32),
    BabyBearElem::from(0x1b1d4e50 as u32),
    BabyBearElem::from(0x30a01ace as u32),
    BabyBearElem::from(0x3bd86f5a as u32),
    BabyBearElem::from(0x69af7c28 as u32),
    BabyBearElem::from(0x3f94775f as u32),
    BabyBearElem::from(0x731560e8 as u32),
    BabyBearElem::from(0x465a0ecd as u32),
    BabyBearElem::from(0x574ef807 as u32),
    BabyBearElem::from(0x62fd4870 as u32),
    BabyBearElem::from(0x52ccfe44 as u32),
    BabyBearElem::from(0x14772b14 as u32),
    BabyBearElem::from(0x4dedf371 as u32),
    BabyBearElem::from(0x260acd7c as u32),
    BabyBearElem::from(0x1f51dc58 as u32),
    BabyBearElem::from(0x75125532 as u32),
    BabyBearElem::from(0x686a4d7b as u32),
    BabyBearElem::from(0x54bac179 as u32),
    BabyBearElem::from(0x31947706 as u32),
    ],
    vec![BabyBearElem::from(0x29799d3b as u32),
    BabyBearElem::from(0x6e01ae90 as u32),
    BabyBearElem::from(0x203a7a64 as u32),
    BabyBearElem::from(0x4f7e25be as u32),
    BabyBearElem::from(0x72503f77 as u32),
    BabyBearElem::from(0x45bd3b69 as u32),
    BabyBearElem::from(0x769bd6b4 as u32),
    BabyBearElem::from(0x5a867f08 as u32),
    BabyBearElem::from(0x4fdba082 as u32),
    BabyBearElem::from(0x251c4318 as u32),
    BabyBearElem::from(0x28f06201 as u32),
    BabyBearElem::from(0x6788c43a as u32),
    BabyBearElem::from(0x4c6d6a99 as u32),
    BabyBearElem::from(0x357784a8 as u32),
    BabyBearElem::from(0x2abaf051 as u32),
    BabyBearElem::from(0x770f7de6 as u32),
    BabyBearElem::from(0x1794b784 as u32),
    BabyBearElem::from(0x4796c57a as u32),
    BabyBearElem::from(0x724b7a10 as u32),
    BabyBearElem::from(0x449989a7 as u32),
    BabyBearElem::from(0x64935cf1 as u32),
    BabyBearElem::from(0x59e14aac as u32),
    BabyBearElem::from(0x0e620bb8 as u32),
    BabyBearElem::from(0x3af5a33b as u32),
    ],
    vec![BabyBearElem::from(0x4465cc0e as u32),
    BabyBearElem::from(0x019df68f as u32),
    BabyBearElem::from(0x4af8d068 as u32),
    BabyBearElem::from(0x08784f82 as u32),
    BabyBearElem::from(0x0cefdeae as u32),
    BabyBearElem::from(0x6337a467 as u32),
    BabyBearElem::from(0x32fa7a16 as u32),
    BabyBearElem::from(0x486f62d6 as u32),
    BabyBearElem::from(0x386a7480 as u32),
    BabyBearElem::from(0x20f17c4a as u32),
    BabyBearElem::from(0x54e50da8 as u32),
    BabyBearElem::from(0x2012cf03 as u32),
    BabyBearElem::from(0x5fe52950 as u32),
    BabyBearElem::from(0x09afb6cd as u32),
    BabyBearElem::from(0x2523044e as u32),
    BabyBearElem::from(0x5c54d0ef as u32),
    BabyBearElem::from(0x71c01f3c as u32),
    BabyBearElem::from(0x60b2c4fb as u32),
    BabyBearElem::from(0x4050b379 as u32),
    BabyBearElem::from(0x5e6a70a5 as u32),
    BabyBearElem::from(0x418543f5 as u32),
    BabyBearElem::from(0x71debe56 as u32),
    BabyBearElem::from(0x1aad2994 as u32),
    BabyBearElem::from(0x3368a483 as u32),
    ],
    vec![BabyBearElem::from(0x07a86f3a as u32),
    BabyBearElem::from(0x5ea43ff1 as u32),
    BabyBearElem::from(0x2443780e as u32),
    BabyBearElem::from(0x4ce444f7 as u32),
    BabyBearElem::from(0x146f9882 as u32),
    BabyBearElem::from(0x3132b089 as u32),
    BabyBearElem::from(0x197ea856 as u32),
    BabyBearElem::from(0x667030c3 as u32),
    BabyBearElem::from(0x2317d5dc as u32),
    BabyBearElem::from(0x0c2c48a7 as u32),
    BabyBearElem::from(0x56b2df66 as u32),
    BabyBearElem::from(0x67bd81e9 as u32),
    BabyBearElem::from(0x4fcdfb19 as u32),
    BabyBearElem::from(0x4baaef32 as u32),
    BabyBearElem::from(0x0328d30a as u32),
    BabyBearElem::from(0x6235760d as u32),
    BabyBearElem::from(0x12432912 as u32),
    BabyBearElem::from(0x0a49e258 as u32),
    BabyBearElem::from(0x030e1b70 as u32),
    BabyBearElem::from(0x48caeb03 as u32),
    BabyBearElem::from(0x49e4d9e9 as u32),
    BabyBearElem::from(0x1051b5c6 as u32),
    BabyBearElem::from(0x6a36dbbe as u32),
    BabyBearElem::from(0x4cff27a5 as u32),
    ],
    vec![BabyBearElem::from(0x1da78ec2 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    ],
    vec![BabyBearElem::from(0x730b0924 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    ],
    vec![BabyBearElem::from(0x3eb56cf3 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    ],
    vec![BabyBearElem::from(0x5bd93073 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    ],
    vec![BabyBearElem::from(0x37204c97 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    ],
    vec![BabyBearElem::from(0x51642d89 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    ],
    vec![BabyBearElem::from(0x66e943e8 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    ],
    vec![BabyBearElem::from(0x1a3e72de as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    ],
    vec![BabyBearElem::from(0x70beb1e9 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    ],
    vec![BabyBearElem::from(0x30ff3b3f as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    ],
    vec![BabyBearElem::from(0x4240d1c4 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    ],
    vec![BabyBearElem::from(0x12647b8d as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    ],
    vec![BabyBearElem::from(0x65d86965 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    ],
    vec![BabyBearElem::from(0x49ef4d7c as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    ],
    vec![BabyBearElem::from(0x47785697 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    ],
    vec![BabyBearElem::from(0x46b3969f as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    ],
    vec![BabyBearElem::from(0x5c7b7a0e as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    ],
    vec![BabyBearElem::from(0x7078fc60 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    ],
    vec![BabyBearElem::from(0x4f22d482 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    ],
    vec![BabyBearElem::from(0x482a9aee as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    ],
    vec![BabyBearElem::from(0x6beb839d as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    BabyBearElem::from(0x00000000 as u32),
    ],
    vec![BabyBearElem::from(0x032959ad as u32),
    BabyBearElem::from(0x2b18af6a as u32),
    BabyBearElem::from(0x55d3dc8c as u32),
    BabyBearElem::from(0x43bd26c8 as u32),
    BabyBearElem::from(0x0c41595f as u32),
    BabyBearElem::from(0x7048d2e2 as u32),
    BabyBearElem::from(0x00db8983 as u32),
    BabyBearElem::from(0x2af563d7 as u32),
    BabyBearElem::from(0x6e84758f as u32),
    BabyBearElem::from(0x611d64e1 as u32),
    BabyBearElem::from(0x1f9977e2 as u32),
    BabyBearElem::from(0x64163a0a as u32),
    BabyBearElem::from(0x5c5fc27b as u32),
    BabyBearElem::from(0x02e22561 as u32),
    BabyBearElem::from(0x3a2d75db as u32),
    BabyBearElem::from(0x1ba7b71a as u32),
    BabyBearElem::from(0x34343f64 as u32),
    BabyBearElem::from(0x7406b35d as u32),
    BabyBearElem::from(0x19df8299 as u32),
    BabyBearElem::from(0x6ff4480a as u32),
    BabyBearElem::from(0x514a81c8 as u32),
    BabyBearElem::from(0x57ab52ce as u32),
    BabyBearElem::from(0x6ad69f52 as u32),
    BabyBearElem::from(0x3e0c0e0d as u32),
    ],
    vec![BabyBearElem::from(0x48126114 as u32),
    BabyBearElem::from(0x2a9d62cc as u32),
    BabyBearElem::from(0x17441f23 as u32),
    BabyBearElem::from(0x485762bb as u32),
    BabyBearElem::from(0x2f218674 as u32),
    BabyBearElem::from(0x06fdc64a as u32),
    BabyBearElem::from(0x0861b7f2 as u32),
    BabyBearElem::from(0x3b36eee6 as u32),
    BabyBearElem::from(0x70a11040 as u32),
    BabyBearElem::from(0x04b31737 as u32),
    BabyBearElem::from(0x3722a872 as u32),
    BabyBearElem::from(0x2a351c63 as u32),
    BabyBearElem::from(0x623560dc as u32),
    BabyBearElem::from(0x62584ab2 as u32),
    BabyBearElem::from(0x382c7c04 as u32),
    BabyBearElem::from(0x3bf9edc7 as u32),
    BabyBearElem::from(0x0e38fe51 as u32),
    BabyBearElem::from(0x376f3b10 as u32),
    BabyBearElem::from(0x5381e178 as u32),
    BabyBearElem::from(0x3afc61c7 as u32),
    BabyBearElem::from(0x5c1bcb4d as u32),
    BabyBearElem::from(0x6643ce1f as u32),
    BabyBearElem::from(0x2d0af1c1 as u32),
    BabyBearElem::from(0x08f583cc as u32),
    ],
    vec![BabyBearElem::from(0x5d6ff60f as u32),
    BabyBearElem::from(0x6324c1e5 as u32),
    BabyBearElem::from(0x74412fb7 as u32),
    BabyBearElem::from(0x70c0192e as u32),
    BabyBearElem::from(0x0b72f141 as u32),
    BabyBearElem::from(0x4067a111 as u32),
    BabyBearElem::from(0x57388c4f as u32),
    BabyBearElem::from(0x351009ec as u32),
    BabyBearElem::from(0x0974c159 as u32),
    BabyBearElem::from(0x539a58b3 as u32),
    BabyBearElem::from(0x038c0cff as u32),
    BabyBearElem::from(0x476c0392 as u32),
    BabyBearElem::from(0x3f7bc15f as u32),
    BabyBearElem::from(0x4491dd2c as u32),
    BabyBearElem::from(0x4d1fef55 as u32),
    BabyBearElem::from(0x04936ae3 as u32),
    BabyBearElem::from(0x58214dd4 as u32),
    BabyBearElem::from(0x683c6aad as u32),
    BabyBearElem::from(0x1b42f16b as u32),
    BabyBearElem::from(0x6dc79135 as u32),
    BabyBearElem::from(0x2d4e71ec as u32),
    BabyBearElem::from(0x3e2946ea as u32),
    BabyBearElem::from(0x59dce8db as u32),
    BabyBearElem::from(0x6cee892a as u32),
    ],
    vec![BabyBearElem::from(0x47f07350 as u32),
    BabyBearElem::from(0x7106ce93 as u32),
    BabyBearElem::from(0x3bd4a7a9 as u32),
    BabyBearElem::from(0x2bfe636a as u32),
    BabyBearElem::from(0x430011e9 as u32),
    BabyBearElem::from(0x001cd66a as u32),
    BabyBearElem::from(0x307faf5b as u32),
    BabyBearElem::from(0x0d9ef3fe as u32),
    BabyBearElem::from(0x6d40043a as u32),
    BabyBearElem::from(0x2e8f470c as u32),
    BabyBearElem::from(0x1b6865e8 as u32),
    BabyBearElem::from(0x0c0e6c01 as u32),
    BabyBearElem::from(0x4d41981f as u32),
    BabyBearElem::from(0x423b9d3d as u32),
    BabyBearElem::from(0x410408cc as u32),
    BabyBearElem::from(0x263f0884 as u32),
    BabyBearElem::from(0x5311bbd0 as u32),
    BabyBearElem::from(0x4dae58d8 as u32),
    BabyBearElem::from(0x30401cea as u32),
    BabyBearElem::from(0x09afa575 as u32),
    BabyBearElem::from(0x4b3d5b42 as u32),
    BabyBearElem::from(0x63ac0b37 as u32),
    BabyBearElem::from(0x5fe5bb14 as u32),
    BabyBearElem::from(0x5244e9d4 as u32),
    ],
    ];

    pub static ref POSEIDON2_BABYBEAR_24_PARAMS: Arc<Poseidon2Params<BabyBearElem>> = Arc::new(Poseidon2Params::new(24, 7, 8, 21, &MAT_DIAG24_M_1, &MAT_INTERNAL24, &RC24));

}

risc0_zkvm::guest::entry!(main);

pub fn main() {

    let data: Vec<u32> = env::read();
    
    let cycles1 = env::get_cycle_count();
    let mut hash_data: Vec<BabyBearElem> = Vec::new();
    for i in 0..data.len() {
        let a_uncompressed = BabyBearElem::from(*data.get(i).unwrap());
        hash_data.push(a_uncompressed);
    }
    let cycles2 = env::get_cycle_count();
    

    let permutation = Poseidon2::new(&POSEIDON2_BABYBEAR_24_PARAMS);
    let mut merkle_tree = MerkleTree::new(permutation.clone());
    let cycles3 = env::get_cycle_count();
    let hash_final = merkle_tree.accumulate(&hash_data);

    let cycles4 = env::get_cycle_count();
    
    let mut perm_seralised: Vec<u32> = Vec::new();
    for i in 0..8 {
        let temp: u32 = hash_final.get(i).unwrap().into();
        perm_seralised.push(temp);

    }
    let cycles6 = env::get_cycle_count();

    env::commit(&perm_seralised);

    eprintln!("number of cycles for input builder: {:?}", cycles2 - cycles1);
    eprintln!("number of cycles for hash permutation builder: {:?}", cycles3 - cycles2);
    eprintln!("number of cycles for hash  accumulation: {:?}", cycles4 - cycles3);

    eprintln!("number of cycles for permutation seralisation: {:?}", cycles6 - cycles4);

}
