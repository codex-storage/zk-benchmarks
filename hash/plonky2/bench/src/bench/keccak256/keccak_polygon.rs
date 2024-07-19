use evm_arithmetization::keccak::keccak_stark::KeccakStark;
use anyhow::Result;
use env_logger::{try_init_from_env, Env, DEFAULT_FILTER_ENV};
use plonky2::field::types::PrimeField64;
use plonky2::fri::oracle::PolynomialBatch;
use plonky2::iop::challenger::Challenger;
use plonky2::plonk::config::{GenericConfig, PoseidonGoldilocksConfig};
use starky::config::StarkConfig;
use starky::cross_table_lookup::{CtlData, CtlZData};
use starky::lookup::{GrandProductChallenge, GrandProductChallengeSet};
use starky::stark_testing::{test_stark_circuit_constraints, test_stark_low_degree};
use tiny_keccak::keccakf;
use plonky2::field::polynomial::PolynomialValues;
use plonky2::field::types::Field;
use plonky2::timed;
use evm_arithmetization::testing_utils::init_logger;
use plonky2::util::timing::TimingTree;
use evm_arithmetization::prover::prove_single_table;
use starky::lookup::Filter;
use starky::lookup::Column;
use rand::random;
const NUM_INPUTS: usize = 84;

pub fn keccak_polygon_bench() -> Result<()> {

    const NUM_PERMS: usize = 85;
        const D: usize = 2;
        type C = PoseidonGoldilocksConfig;
        type F = <C as GenericConfig<D>>::F;
        type S = KeccakStark<F, D>;
        let stark = S::default();
        let config = StarkConfig::standard_fast_config();

        init_logger();

        let input: Vec<([u64; NUM_INPUTS], usize)> =
            (0..NUM_PERMS).map(|_| (rand::random(), 0)).collect();

        let mut timing = TimingTree::new("prove", log::Level::Debug);
        let trace_poly_values = timed!(
            timing,
            "generate trace",
            stark.generate_trace(input, 8, &mut timing)
        );

        let cloned_trace_poly_values = timed!(timing, "clone", trace_poly_values.clone());

        let trace_commitments = timed!(
            timing,
            "compute trace commitment",
            PolynomialBatch::<F, C, D>::from_values(
                cloned_trace_poly_values,
                config.fri_config.rate_bits,
                false,
                config.fri_config.cap_height,
                &mut timing,
                None,
            )
        );
        let degree = 1 << trace_commitments.degree_log;

        // Fake CTL data.
        let ctl_z_data = CtlZData::new(
            vec![PolynomialValues::zero(degree)],
            PolynomialValues::zero(degree),
            GrandProductChallenge {
                beta: F::ZERO,
                gamma: F::ZERO,
            },
            vec![],
            vec![Filter::new_simple(Column::constant(F::ZERO))],
        );
        let ctl_data = CtlData {
            zs_columns: vec![ctl_z_data.clone(); config.num_challenges],
        };

        prove_single_table(
            &stark,
            &config,
            &trace_poly_values,
            &trace_commitments,
            &ctl_data,
            &GrandProductChallengeSet {
                challenges: vec![ctl_z_data.challenge; config.num_challenges],
            },
            &mut Challenger::new(),
            &mut timing,
            None,
        )?;

        timing.print();
        Ok(())
}