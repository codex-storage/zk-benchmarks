use plonky2::hash::hash_types::RichField;
use plonky2::field::extension::Extendable;
use plonky2::plonk::circuit_builder::CircuitBuilder;
use super::u32_arithmetic::CircuitBuilderU32;
use super::u32_arithmetic::U32Target;
#[derive(Clone, Copy, Debug)]
pub struct U64Target(pub [U32Target;2]);

pub trait CircuitBuilderU64<F: RichField + Extendable<D>, const D: usize> {
    fn and(&mut self, a: U64Target, b: U64Target) -> U64Target;
    fn xor(&mut self, a: U64Target, b: U64Target) -> U64Target;
}

impl<F: RichField + Extendable<D>, const D: usize> CircuitBuilderU64<F, D>
    for CircuitBuilder<F, D>{
    fn xor(&mut self, a: U64Target, b: U64Target) -> U64Target {
        let mut result = Vec::new();
        for i in 0..2 {
            result.push(self.xor_u32(a.0[i], b.0[i]));
        }
        U64Target([result[0], result[1]])
    }

    fn and(&mut self, a: U64Target, b: U64Target) -> U64Target {
        let mut result = Vec::new();

        for i in 0..2 {
            result.push(self.and_u32(a.0[i], b.0[i]));
        }
        U64Target([result[0], result[1]])
    }
}