// code is taken from https://github.com/polymerdao/plonky2-sha256

use plonky2::iop::target::BoolTarget;
use plonky2::hash::hash_types::RichField;
use plonky2::plonk::circuit_builder::CircuitBuilder;
use plonky2::field::extension::Extendable;
use plonky2_u32::gadgets::arithmetic_u32::{CircuitBuilderU32, U32Target};
use anyhow::Result;
use plonky2::plonk::config::PoseidonGoldilocksConfig;
use plonky2::plonk::config::GenericConfig;
use plonky2::plonk::circuit_data::CircuitConfig;
use plonky2::iop::witness::{PartialWitness, WitnessWrite};
use sha2::{Digest, Sha256};
use super::sigma::big_sigma0;
use super::sigma::big_sigma1;
use super::sigma::sigma0;
use super::sigma::sigma1;
use super::maj::maj;
use super::constants::*;
use super::ch::ch;
use crate::arithmetic::u32_arithmetic::add_u32;
use rand::Rng;

pub struct Sha256Targets {
    pub message: Vec<BoolTarget>,
    pub digest: Vec<BoolTarget>,
}

pub fn array_to_bits(bytes: &[u8]) -> Vec<bool> {
    let len = bytes.len();
    let mut ret = Vec::new();
    for i in 0..len {
        for j in 0..8 {
            let b = (bytes[i] >> (7 - j)) & 1;
            ret.push(b == 1);
        }
    }
    ret
}


pub fn make_circuits<F: RichField + Extendable<D>, const D: usize>(
    builder: &mut CircuitBuilder<F, D>,
    msg_len_in_bits: u64,
    
) -> Sha256Targets {


    let mut message = Vec::new();
    let mut digest = Vec::new();

    let block_count = (msg_len_in_bits + 65 + 511) / 512;
    let padded_msg_len = 512 * block_count;
    let p = padded_msg_len - 64 - msg_len_in_bits;
    assert!(p > 1);

    //msg
    for _ in 0..msg_len_in_bits {
        message.push(builder.add_virtual_bool_target_unsafe());
    }

    //append a single bit '1'
    message.push(builder.constant_bool(true));

    //append '0' bit so that total length become multiple of 512
    for _ in 0..p - 1 {
        message.push(builder.constant_bool(false));
    }

    //append the msg length as 64bit big-endian integer
    for i in 0..64 {
        let b = ((msg_len_in_bits as u64) >> (63 - i)) & 1;
        message.push(builder.constant_bool(b == 1));
    }

    // init states
    let mut state = Vec::new();
    for i in 0..8 {
        state.push(builder.constant_u32(H[i]));
    }

    let mut k256 = Vec::new();
    for i in 0..64 {
        k256.push(builder.constant_u32(K[i]));
    }

    for blk in 0..block_count {
        let mut x = Vec::new();
        let mut a = state[0].clone();
        let mut b = state[1].clone();
        let mut c = state[2].clone();
        let mut d = state[3].clone();
        let mut e = state[4].clone();
        let mut f = state[5].clone();
        let mut g = state[6].clone();
        let mut h = state[7].clone();

        for i in 0..16 {
            let index = blk as usize * 512 + i * 32;
            let u32_target = builder.le_sum(message[index..index + 32].iter().rev());

            x.push(U32Target(u32_target));
            let mut t1 = h.clone();
            let big_sigma1_e = big_sigma1(builder, &e);
            t1 = add_u32(builder, &t1, &big_sigma1_e);
            let ch_e_f_g = ch(builder, &e, &f, &g);
            t1 = add_u32(builder, &t1, &ch_e_f_g);
            t1 = add_u32(builder, &t1, &k256[i]);
            t1 = add_u32(builder, &t1, &x[i]);

            let mut t2 = big_sigma0(builder, &a);
            let maj_a_b_c = maj(builder, &a, &b, &c);
            t2 = add_u32(builder, &t2, &maj_a_b_c);

            h = g;
            g = f;
            f = e;
            e = add_u32(builder, &d, &t1);
            d = c;
            c = b;
            b = a;
            a = add_u32(builder, &t1, &t2);
        }

        for i in 16..64 {
            let s0 = sigma0(builder, &x[(i + 1) & 0x0f]);
            let s1 = sigma1(builder, &x[(i + 14) & 0x0f]);

            let s0_add_s1 = add_u32(builder, &s0, &s1);
            let s0_add_s1_add_x = add_u32(builder, &s0_add_s1, &x[(i + 9) & 0xf]);
            x[i & 0xf] = add_u32(builder, &x[i & 0xf], &s0_add_s1_add_x);

            let big_sigma0_a = big_sigma0(builder, &a);
            let big_sigma1_e = big_sigma1(builder, &e);
            let ch_e_f_g = ch(builder, &e, &f, &g);
            let maj_a_b_c = maj(builder, &a, &b, &c);

            let h_add_sigma1 = add_u32(builder, &h, &big_sigma1_e);
            let h_add_sigma1_add_ch_e_f_g = add_u32(builder, &h_add_sigma1, &ch_e_f_g);
            let h_add_sigma1_add_ch_e_f_g_add_k256 =
                add_u32(builder, &h_add_sigma1_add_ch_e_f_g, &k256[i]);

            let t1 = add_u32(builder, &x[i & 0xf], &h_add_sigma1_add_ch_e_f_g_add_k256);
            let t2 = add_u32(builder, &big_sigma0_a, &maj_a_b_c);

            h = g;
            g = f;
            f = e;
            e = add_u32(builder, &d, &t1);
            d = c;
            c = b;
            b = a;
            a = add_u32(builder, &t1, &t2);
        }

        state[0] = add_u32(builder, &state[0], &a);
        state[1] = add_u32(builder, &state[1], &b);
        state[2] = add_u32(builder, &state[2], &c);
        state[3] = add_u32(builder, &state[3], &d);
        state[4] = add_u32(builder, &state[4], &e);
        state[5] = add_u32(builder, &state[5], &f);
        state[6] = add_u32(builder, &state[6], &g);
        state[7] = add_u32(builder, &state[7], &h);
    }

    for i in 0..8 {
        let bit_targets = builder.split_le_base::<2>(state[i].0, 32);
        for j in (0..32).rev() {
            digest.push(BoolTarget::new_unsafe(bit_targets[j]));
        }
    }

    Sha256Targets { message, digest }
}


fn generate_random_bytes(size: usize) -> Vec<u8> {
    
    let mut rng = rand::thread_rng();
    let mut bytes = vec![0u8; size];
    rng.fill(&mut bytes[..]);

    bytes
    
}

pub fn sha256_bench(size: usize) -> Result<()> {
    let msg = generate_random_bytes(size);

    let mut hasher = Sha256::new();
    hasher.update(msg.clone());
    let hash = hasher.finalize();

    let msg_bits = array_to_bits(&msg.clone());
    let len = msg.len() * 8;
    println!("block count: {}", (len + 65 + 511) / 512);
    const D: usize = 2;
    type C = PoseidonGoldilocksConfig;
    type F = <C as GenericConfig<D>>::F;
    let mut builder = CircuitBuilder::<F, D>::new(CircuitConfig::standard_recursion_config());
    let targets = make_circuits(&mut builder, len as u64);
    let mut pw = PartialWitness::new();

    for i in 0..len {
        pw.set_bool_target(targets.message[i], msg_bits[i]);
    }

    let expected_res = array_to_bits(hash.as_slice());
    for i in 0..expected_res.len() {
        if expected_res[i] {
            builder.assert_one(targets.digest[i].target);
        } else {
            builder.assert_zero(targets.digest[i].target);
        }
    }

    println!(
        "number of gates: {}",
        builder.num_gates()
    );
    let data = builder.build::<C>();

    let (proof_time, proof ) = {

        let start = std::time::Instant::now();
        let proof = data.prove(pw).unwrap();
        let end = start.elapsed();
        (end, proof)
    };
    let proof_size = proof.to_bytes().len();
    
    let (verification_time, res) = {
        let start = std::time::Instant::now();
        let res = data.verify(proof);
        let end = start.elapsed();
        (end, res)
    };

    eprintln!("Proof Generation Time: {:?}", proof_time);
    eprintln!("Verification Time: {:?}", verification_time);
    eprintln!("Proof size: {:?}", proof_size);
    
    res

}