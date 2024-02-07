use anyhow::Result;
use plonky2::field::types::Field;
use plonky2::hash::hash_types::RichField;
use plonky2::field::goldilocks_field::GoldilocksField;
use plonky2::hash::keccak;
use plonky2::hash::keccak::KeccakHash;
use plonky2::hash::poseidon::PoseidonHash;
use plonky2::iop::witness::{PartialWitness, WitnessWrite};
use plonky2::plonk::circuit_builder::CircuitBuilder;
use plonky2::plonk::circuit_data::CircuitConfig;
use plonky2::plonk::config::{AlgebraicHasher, GenericConfig, PoseidonGoldilocksConfig};

fn main() -> Result<()> {
    const D: usize = 2;
    type C = PoseidonGoldilocksConfig;
    type F = <C as GenericConfig<D>>::F;

    let config = CircuitConfig::standard_recursion_config();
    let mut builder = CircuitBuilder::<F, D>::new(config);

    // The arithmetic circuit.
    let initial = builder.add_virtual_target();
    let hash = builder.hash_or_noop::<PoseidonHash>(vec![initial]);

    // Public inputs are the initial value (provided below) and the result (which is generated).
    builder.register_public_input(initial);
    builder.register_public_input(hash.elements[0]);
    builder.register_public_input(hash.elements[1]);
    builder.register_public_input(hash.elements[2]);
    builder.register_public_input(hash.elements[3]);

    // Provide initial values.
    let mut pw = PartialWitness::new();
    pw.set_target(initial, F::ONE);


    let data = builder.build::<C>();
    let proof = data.prove(pw)?;

    println!(
        "hash of {} is: {}",
        proof.public_inputs[0], proof.public_inputs[1]
    );

    data.verify(proof)

}
