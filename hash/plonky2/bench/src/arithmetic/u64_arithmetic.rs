use plonky2::hash::hash_types::RichField;
use plonky2::field::extension::Extendable;
use plonky2::plonk::circuit_builder::CircuitBuilder;
use std::marker::PhantomData;
use plonky2::iop::target::BoolTarget;
use plonky2::iop::witness::PartialWitness;
use plonky2::iop::witness::WitnessWrite;

use crate::arithmetic::binary_arithmetic::{rotate_u64,xor_const_circuit,u64_to_bits,xor_circuit};

#[derive(Clone, Debug)]
pub struct U64Target<F, const D: usize> {
    pub bits: Vec<BoolTarget>,
    _phantom: PhantomData<F>,
}

impl<F, const D: usize> U64Target<F, D>
where
    F: RichField + Extendable<D>,
{
    pub fn new(builder: &mut CircuitBuilder<F, D>) -> Self {
        let mut result = vec![];
        for _ in 0..64 {
            result.push(builder.add_virtual_bool_target_safe());
        }
        Self {
            bits: result,
            _phantom: PhantomData,
        }
    }

    pub fn from(bits: Vec<BoolTarget>) -> Self {
        assert_eq!(bits.len(), 64);
        Self {
            bits,
            _phantom: PhantomData,
        }
    }

    pub fn set_witness(&self, bits: Vec<bool>, pw: &mut PartialWitness<F>) {
        for i in 0..64 {
            pw.set_bool_target(self.bits[i], bits[i]);
        }
    }

    pub fn constant(x: u64, builder: &mut CircuitBuilder<F, D>) -> Self {
        let mut result = vec![];
        let x_bits = u64_to_bits(x);
        for i in 0..64 {
            result.push(builder.constant_bool(x_bits[i]));
        }
        Self {
            bits: result,
            _phantom: PhantomData,
        }
    }

    pub fn connect(&self, other: &Self, builder: &mut CircuitBuilder<F, D>) {
        for i in 0..64 {
            builder.connect(self.bits[i].target, other.bits[i].target);
        }
    }

    pub fn to_bits(&self, builder: &mut CircuitBuilder<F, D>) -> Vec<BoolTarget> {
        let output = Self::new(builder);
        self.connect(&output, builder);
        output.bits
    }

    pub fn xor(&self, other: &Self, builder: &mut CircuitBuilder<F, D>) -> Self {
        let mut result = vec![];
        for i in 0..64 {
            let xor_target = xor_circuit(self.bits[i], other.bits[i], builder);
            result.push(xor_target);
        }
        Self {
            bits: result,
            _phantom: PhantomData,
        }
    }

    pub fn xor_const(&self, other: u64, builder: &mut CircuitBuilder<F, D>) -> Self {
        let other_bits = u64_to_bits(other);
        let mut result = vec![];
        for i in 0..64 {
            let xor_target = xor_const_circuit(self.bits[i], other_bits[i], builder);
            result.push(xor_target);
        }
        Self {
            bits: result,
            _phantom: PhantomData,
        }
    }

    /* Rotate left by n
     * Note that the input parameter n is constant. It is not necessary to make n a constant target or public input,
     * because different n generates a different circuit.
     */
    pub fn rotl(&self, n: usize) -> Self {
        let rotate = rotate_u64(n);
        let mut output = vec![];
        for i in 0..64 {
            output.push(self.bits[rotate[i]]);
        }

        Self {
            bits: output,
            _phantom: PhantomData,
        }
    }

    pub fn and(&self, other: &Self, builder: &mut CircuitBuilder<F, D>) -> Self {
        let mut result = vec![];
        for i in 0..64 {
            result.push(builder.and(self.bits[i], other.bits[i]));
        }
        Self {
            bits: result,
            _phantom: PhantomData,
        }
    }

    pub fn not(&self, builder: &mut CircuitBuilder<F, D>) -> Self {
        let mut result = vec![];
        for i in 0..64 {
            result.push(builder.not(self.bits[i]));
        }
        Self {
            bits: result,
            _phantom: PhantomData,
        }
    }

    /// Calculate `self & !other`.
    pub fn and_not(&self, other: &Self, builder: &mut CircuitBuilder<F, D>) -> Self {
        let mut result = vec![];
        for i in 0..64 {
            // x(1 - y) = x - xy
            result.push(BoolTarget::new_unsafe(builder.arithmetic(
                F::NEG_ONE,
                F::ONE,
                self.bits[i].target,
                other.bits[i].target,
                self.bits[i].target,
            )));
        }
        Self {
            bits: result,
            _phantom: PhantomData,
        }
    }
}