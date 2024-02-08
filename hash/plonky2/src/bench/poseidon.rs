use anyhow::Result;
use plonky2::field::types::Field;
// use plonky2::hash::hash_types::{HashOutTarget, RichField};
use plonky2::field::goldilocks_field::GoldilocksField;
// use plonky2::hash::keccak;
// use plonky2::hash::keccak::KeccakHash;
use plonky2::hash::poseidon::PoseidonHash;
use plonky2::iop::witness::{PartialWitness, WitnessWrite};
use plonky2::plonk::circuit_builder::CircuitBuilder;
use plonky2::plonk::circuit_data::CircuitConfig;
use plonky2::plonk::config::{/*AlgebraicHasher,*/ GenericConfig, PoseidonGoldilocksConfig};
use rand::Rng;

fn generate_data(size: usize) -> Vec<GoldilocksField> {
    // let mut rng = rand::thread_rng();
    // (0..size).map(|_| rng.gen()).collect()

    let mut data: Vec<GoldilocksField> = Vec::new();
    for _ in 0..(1<<size) {
        let mut rng = rand::thread_rng();
        let random_u64: u64 = rng.gen();
        data.push(GoldilocksField::from_canonical_u64(random_u64));
    }
    // eprint!("data: {:?}", data);
    data

}


pub fn poseidon_bench(depth: usize) -> Result<()> {

    let data = generate_data(depth);

    const D: usize = 2;
    type C = PoseidonGoldilocksConfig;
    type F = <C as GenericConfig<D>>::F;

    let config = CircuitConfig::standard_recursion_config();
    let mut builder = CircuitBuilder::<F, D>::new(config);

    // The arithmetic circuit.
    let initial = builder.add_virtual_targets(data.len());

    let hash = builder.hash_or_noop::<PoseidonHash>(initial.clone());

    // Public inputs are the initial value (provided below) and the result (which is generated).
    builder.register_public_inputs(initial.clone().as_slice());
    builder.register_public_input(hash.elements[0]);
    builder.register_public_input(hash.elements[1]);
    builder.register_public_input(hash.elements[2]);
    builder.register_public_input(hash.elements[3]);

    // Provide initial values.
    let mut pw = PartialWitness::new();
    pw.set_target_arr(initial.as_slice(), data.as_slice());


    let data = builder.build::<C>();
    let proof = data.prove(pw)?;


    data.verify(proof)

}
