use plonky2::hash::hash_types::RichField;
use plonky2::field::extension::Extendable;
use plonky2::plonk::circuit_builder::CircuitBuilder;
use plonky2_u32::gadgets::arithmetic_u32::U32Target;
use super::u32_arithmetic::CircuitBuilderU32M;
use plonky2_u32::gadgets::arithmetic_u32::CircuitBuilderU32;

#[derive(Clone, Copy, Debug)]
pub struct U64Target(pub [U32Target;2]);

//TODO: remove the dead codes later
#[allow(dead_code)]
pub trait CircuitBuilderU64<F: RichField + Extendable<D>, const D: usize> {
    fn and_u64(&mut self, a: U64Target, b: U64Target) -> U64Target;
    fn xor_u64(&mut self, a: U64Target, b: U64Target) -> U64Target;
    fn rotate_left_u64(&mut self, a: U64Target, n: u8) -> U64Target;

    fn zero_u64(&mut self) -> U64Target;

    fn not_u64(&mut self, a: U64Target) -> U64Target;
}

impl<F: RichField + Extendable<D>, const D: usize> CircuitBuilderU64<F, D>
    for CircuitBuilder<F, D>{
    fn xor_u64(&mut self, a: U64Target, b: U64Target) -> U64Target {
        let mut result = Vec::new();
        for i in 0..2 {
            result.push(self.xor_u32(a.0[i], b.0[i]));
        }
        U64Target([result[0], result[1]])
    }

    fn and_u64(&mut self, a: U64Target, b: U64Target) -> U64Target {
        let mut result = Vec::new();

        for i in 0..2 {
            result.push(self.and_u32(a.0[i], b.0[i]));
        }
        U64Target([result[0], result[1]])
    }

    fn rotate_left_u64(&mut self, a: U64Target, n: u8) -> U64Target {
        let (lo, hi) = if n < 32 { (a.0[0], a.0[1]) } else { (a.0[1], a.0[0]) };

        let two_power_x = self.constant_u32(0x1 << (n % 32));
        let (lo0, hi0) = self.mul_u32(lo, two_power_x);
        let (lo1, hi1) = self.mul_add_u32(hi, two_power_x, hi0);

        U64Target([self.add_u32(lo0, hi1).0, lo1])
    }

    fn zero_u64(&mut self) -> U64Target {
        let zero_u32 = self.zero_u32();
        U64Target([zero_u32,zero_u32])
    }

    fn not_u64(&mut self, a: U64Target) -> U64Target {
        U64Target([self.not_u32(a.0[0]), self.not_u32(a.0[1])])
    }
}