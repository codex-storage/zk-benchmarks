use plonky2::hash::hash_types::RichField;
use plonky2::plonk::circuit_builder::CircuitBuilder;
use plonky2::field::extension::Extendable;
use crate::arithmetic::u64_arithmetic::CircuitBuilderU64;


use crate::arithmetic::u64_arithmetic::U64Target;

// Theta
pub fn theta<F: RichField + Extendable<D>, const D: usize>(
    builder: &mut CircuitBuilder<F, D>,
    state: &mut [[U64Target; 5]; 5]
){
    let mut c = [builder.zero_u64(); 5];
    let mut d = [builder.zero_u64(); 5];

    // Compute column parities
    for x in 0..5 {

        let xor_x0_x1 = builder.xor_u64(state[x][0], state[x][1]);
        let xor_x0_x1_x2 = builder.xor_u64(xor_x0_x1, state[x][2]);
        let xor_x0_x1_x2_x3 = builder.xor_u64(xor_x0_x1_x2, state[x][3]);
        c[x] = builder.xor_u64(xor_x0_x1_x2_x3, state[x][4]);
        
    }

    // Compute rotated parities
    for x in 0..5 {
        let c_left = c[(x + 4) % 5];
        let c_right_rot = builder.rotate_left_u64(c[(x + 1) % 5], 1);
        d[x] = builder.xor_u64(c_left, c_right_rot);
    }

    // Modify the state
    for x in 0..5 {
        for y in 0..5 {
            state[x][y] = builder.xor_u64(state[x][y], d[x]);
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use plonky2::field::goldilocks_field::GoldilocksField;
    use plonky2::hash::hash_types::RichField;
    use plonky2::iop::witness::PartialWitness;
    use plonky2::plonk::circuit_builder::CircuitBuilder;
    use plonky2::plonk::circuit_data::CircuitConfig;
    use plonky2::plonk::config::PoseidonGoldilocksConfig;
    use crate::arithmetic::u64_arithmetic::U64Target;
    use plonky2_u32::gadgets::arithmetic_u32::CircuitBuilderU32;
    use crate::bench::keccak256::keccak::WitnessU64;

    fn create_u64_target<F: RichField + Extendable<D>, const D: usize>(
        builder: &mut CircuitBuilder<F, D>,
        value: u64
    ) -> U64Target {
        let lo = value as u32;
        let hi = (value >> 32) as u32;
        U64Target([builder.constant_u32(lo), builder.constant_u32(hi)])
    }

    #[test]
    fn test_theta_function() {
        type F = GoldilocksField;
        const D: usize = 2;

        // Create circuit builder
        let config = CircuitConfig::standard_recursion_config();
        let mut builder = CircuitBuilder::<F, D>::new(config);

        // Input state
        let input_state: [[U64Target; 5]; 5] = [
            [
                create_u64_target(&mut builder, 0x0000000000000001),
                create_u64_target(&mut builder, 0x0000000000000002),
                create_u64_target(&mut builder, 0x0000000000000003),
                create_u64_target(&mut builder, 0x0000000000000004),
                create_u64_target(&mut builder, 0x0000000000000005)
            ],
            [
                create_u64_target(&mut builder, 0x0000000000000006),
                create_u64_target(&mut builder, 0x0000000000000007),
                create_u64_target(&mut builder, 0x0000000000000008),
                create_u64_target(&mut builder, 0x0000000000000009),
                create_u64_target(&mut builder, 0x000000000000000A)
            ],
            [
                create_u64_target(&mut builder, 0x000000000000000B),
                create_u64_target(&mut builder, 0x000000000000000C),
                create_u64_target(&mut builder, 0x000000000000000D),
                create_u64_target(&mut builder, 0x000000000000000E),
                create_u64_target(&mut builder, 0x000000000000000F)
            ],
            [
                create_u64_target(&mut builder, 0x0000000000000010),
                create_u64_target(&mut builder, 0x0000000000000011),
                create_u64_target(&mut builder, 0x0000000000000012),
                create_u64_target(&mut builder, 0x0000000000000013),
                create_u64_target(&mut builder, 0x0000000000000014)
            ],
            [
                create_u64_target(&mut builder, 0x0000000000000015),
                create_u64_target(&mut builder, 0x0000000000000016),
                create_u64_target(&mut builder, 0x0000000000000017),
                create_u64_target(&mut builder, 0x0000000000000018),
                create_u64_target(&mut builder, 0x0000000000000019)
            ]
        ];

        // Expected output state (after theta)
        let expected_state = [
            [
                create_u64_target(&mut builder, 0x7B7B7B7B7B7B7B7B),
                create_u64_target(&mut builder, 0x8B8B8B8B8B8B8B8B),
                create_u64_target(&mut builder, 0x9B9B9B9B9B9B9B9B),
                create_u64_target(&mut builder, 0xABABABABABABABAB),
                create_u64_target(&mut builder, 0xBBBBBBBBBBBBBBBB)
            ],
            [
                create_u64_target(&mut builder, 0xCBCBCBCBCBCBCBCB),
                create_u64_target(&mut builder, 0xDBDBDBDBDBDBDBDB),
                create_u64_target(&mut builder, 0xEBEBEBEBEBEBEBEB),
                create_u64_target(&mut builder, 0xFBFBFBFBFBFBFBFB),
                create_u64_target(&mut builder, 0x0B0B0B0B0B0B0B0B)
            ],
            [
                create_u64_target(&mut builder, 0x1B1B1B1B1B1B1B1B),
                create_u64_target(&mut builder, 0x2B2B2B2B2B2B2B2B),
                create_u64_target(&mut builder, 0x3B3B3B3B3B3B3B3B),
                create_u64_target(&mut builder, 0x4B4B4B4B4B4B4B4B),
                create_u64_target(&mut builder, 0x5B5B5B5B5B5B5B5B)
            ],
            [
                create_u64_target(&mut builder, 0x6B6B6B6B6B6B6B6B),
                create_u64_target(&mut builder, 0x7B7B7B7B7B7B7B7B),
                create_u64_target(&mut builder, 0x8B8B8B8B8B8B8B8B),
                create_u64_target(&mut builder, 0x9B9B9B9B9B9B9B9B),
                create_u64_target(&mut builder, 0xABABABABABABABAB)
            ],
            [
                create_u64_target(&mut builder, 0xBBBBBBBBBBBBBBBB),
                create_u64_target(&mut builder, 0xCBCBCBCBCBCBCBCB),
                create_u64_target(&mut builder, 0xDBDBDBDBDBDBDBDB),
                create_u64_target(&mut builder, 0xEBEBEBEBEBEBEBEB),
                create_u64_target(&mut builder, 0xFBFBFBFBFBFBFBFB)
            ]
        ];

        // Run the theta function
        let mut state = input_state;
        let _ = theta(&mut builder, &mut state);

        
        // Check if the output state matches the expected state
        for x in 0..5 {
            for y in 0..5 {
                for i in 0..2 {
                    println!("Comparing: {:?} and {:?}", state[x][y].0[i].0, expected_state[x][y].0[i].0);
                    let res = builder.is_equal(state[x][y].0[i].0, expected_state[x][y].0[i].0);
                    builder.assert_bool(res);
                }
            }
        }

        println!("{:?}", builder.num_gates());
        // Build the circuit
        let data = builder.build::<PoseidonGoldilocksConfig>();

        // Create witness and prove
        let mut pw = PartialWitness::new();
        pw.set_u64_target(input_state[0][0], 0x0000000000000001);
        pw.set_u64_target(input_state[0][1], 0x0000000000000002);
        pw.set_u64_target(input_state[0][2], 0x0000000000000003);
        pw.set_u64_target(input_state[0][3], 0x0000000000000004);
        pw.set_u64_target(input_state[0][4], 0x0000000000000005);
        pw.set_u64_target(input_state[1][0], 0x0000000000000006);
        pw.set_u64_target(input_state[1][1], 0x0000000000000007);
        pw.set_u64_target(input_state[1][2], 0x0000000000000008);
        pw.set_u64_target(input_state[1][3], 0x0000000000000009);
        pw.set_u64_target(input_state[1][4], 0x000000000000000A);
        pw.set_u64_target(input_state[2][0], 0x000000000000000B);
        pw.set_u64_target(input_state[2][1], 0x000000000000000C);
        pw.set_u64_target(input_state[2][2], 0x000000000000000D);
        pw.set_u64_target(input_state[2][3], 0x000000000000000E);
        pw.set_u64_target(input_state[2][4], 0x000000000000000F);
        pw.set_u64_target(input_state[3][0], 0x0000000000000010);
        pw.set_u64_target(input_state[3][1], 0x0000000000000011);
        pw.set_u64_target(input_state[3][2], 0x0000000000000012);
        pw.set_u64_target(input_state[3][3], 0x0000000000000013);
        pw.set_u64_target(input_state[3][4], 0x0000000000000014);
        pw.set_u64_target(input_state[4][0], 0x0000000000000015);
        pw.set_u64_target(input_state[4][1], 0x0000000000000016);
        pw.set_u64_target(input_state[4][2], 0x0000000000000017);
        pw.set_u64_target(input_state[4][3], 0x0000000000000018);
        pw.set_u64_target(input_state[4][4], 0x0000000000000019);

        let proof = data.prove(pw).unwrap();
        assert!(data.verify(proof).is_ok());
    }
}