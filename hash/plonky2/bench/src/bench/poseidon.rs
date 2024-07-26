use anyhow::Result;
use plonky2::{
    field::types::Field,
    field::goldilocks_field::GoldilocksField,
    hash::poseidon::PoseidonHash,
    iop::witness::{PartialWitness, WitnessWrite},
    plonk::circuit_builder::CircuitBuilder,
    plonk::circuit_data::CircuitConfig,
    plonk::config::{GenericConfig, PoseidonGoldilocksConfig},
};
use rand::Rng;
use std::time;

fn generate_data(size: usize) -> Vec<GoldilocksField> {

    let mut data: Vec<GoldilocksField> = Vec::new();
    for _ in 0..(1<<size) {
        let mut rng = rand::thread_rng();
        let random_u64: u64 = rng.gen();
        data.push(GoldilocksField::from_canonical_u64(random_u64));
    }
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
    // builder.register_public_inputs(initial.clone().as_slice());
    builder.register_public_input(hash.elements[0]);
    builder.register_public_input(hash.elements[1]);
    builder.register_public_input(hash.elements[2]);
    builder.register_public_input(hash.elements[3]);

    // Provide initial values.
    let mut pw = PartialWitness::new();
    pw.set_target_arr(initial.as_slice(), data.as_slice());


    let data = builder.build::<C>();

    let (proof_generation_time, proof) = {

        let start = time::Instant::now();
        let proof = data.prove(pw)?;
        let end_time = start.elapsed();
        (end_time, proof)
    };
    
    let (verification_time, result) = {
        let start = time::Instant::now();
        let result = data.verify(proof);
        let end_time = start.elapsed();
        (end_time, result)
    };

    eprintln!("proof generation time: {:?}", proof_generation_time);
    eprintln!("verification time: {:?}", verification_time);
    result
}
