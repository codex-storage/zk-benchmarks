use plonky2::iop::target::BoolTarget;
use plonky2::hash::hash_types::RichField;
use plonky2::field::extension::Extendable;
use plonky2::plonk::circuit_builder::CircuitBuilder;
use plonky2_u32::gadgets::arithmetic_u32::U32Target;
use super::binary_arithmetic::CircuitBuilderBoolTarget;
use plonky2_u32::gadgets::arithmetic_u32::CircuitBuilderU32;

//TODO: remove the dead codes later
#[allow(dead_code)]
pub trait CircuitBuilderU32M<F: RichField + Extendable<D>, const D: usize> {
    fn or_u32(&mut self, a: U32Target, b: U32Target) -> U32Target;
    fn and_u32(&mut self, a: U32Target, b: U32Target) -> U32Target;
    fn xor_u32(&mut self, a: U32Target, b: U32Target) -> U32Target;
    fn rotate_left_u32(&mut self, a: U32Target, n: u8) -> U32Target;

    fn from_u32(&mut self, a: U32Target) -> Vec<BoolTarget>;
    fn to_u32(&mut self, a: Vec<BoolTarget>) -> U32Target;

    // not := 0xFFFFFFFF - x
    fn not_u32(&mut self, a: U32Target) -> U32Target;
        
}

impl<F: RichField + Extendable<D>, const D: usize> CircuitBuilderU32M<F, D>
    for CircuitBuilder<F, D>{

        fn from_u32(&mut self, a: U32Target) -> Vec<BoolTarget> {

            let mut res = Vec::new();
            let bit_targets = self.split_le_base::<2>(a.0, 32);

            for i in (0..32).rev() {
                res.push(BoolTarget::new_unsafe(bit_targets[i]));
            }
            res
        }

        fn to_u32(&mut self, a: Vec<BoolTarget>) -> U32Target {
            let bit_len = a.len();
            assert_eq!(bit_len, 32);
            U32Target(self.le_sum(a[0..32].iter().rev()))
        }


        fn or_u32(&mut self, a: U32Target, b: U32Target) -> U32Target {
            let binary_target_a = self.from_u32(a);
            let binary_target_b = self.from_u32(b);

            let mut res = Vec::<BoolTarget>::new();
            for i in 0..32 {

                let r = self.or(binary_target_a[i], binary_target_b[i]);
                res.push(r);
            }
            self.to_u32(res)
        }

        fn and_u32(&mut self, a: U32Target, b: U32Target) -> U32Target {
            let binary_target_a = self.from_u32(a);
            let binary_target_b = self.from_u32(b);

            let mut res = Vec::<BoolTarget>::new();
            for i in 0..32 {

                let r = self.and(binary_target_a[i], binary_target_b[i]);
                res.push(r);
            }
            self.to_u32(res)

        }

        fn xor_u32(&mut self, a: U32Target, b: U32Target) -> U32Target {
            let binary_target_a = self.from_u32(a);
            let binary_target_b = self.from_u32(b);

            let mut res = Vec::<BoolTarget>::new();
            for i in 0..32 {

                let r = self.xor(binary_target_a[i], binary_target_b[i]);
                res.push(r);
            }
            self.to_u32(res)
        }

        fn rotate_left_u32(&mut self, a: U32Target, n: u8) -> U32Target {
            let two_power_n = self.constant_u32(0x1 << n);
            let (lo, hi) = self.mul_u32(a, two_power_n);
            self.add_u32(lo, hi).0
        }

        // not := 0xFFFFFFFF - x
        fn not_u32(&mut self, a: U32Target) -> U32Target {
            let zero = self.zero_u32();
            let ff = self.constant_u32(0xFFFFFFFF);
            self.sub_u32(ff, a, zero).0
        }

    }