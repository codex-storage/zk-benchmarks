use plonky2::hash::hash_types::RichField;
use plonky2::field::extension::Extendable;
use plonky2::plonk::circuit_builder::CircuitBuilder;
use plonky2::iop::target::BoolTarget;

pub trait CircuitBuilderBoolTarget<F: RichField + Extendable<D>, const D: usize> {
    fn and(&mut self, a: BoolTarget, b: BoolTarget) -> BoolTarget;
    fn or(&mut self, a: BoolTarget, b: BoolTarget) -> BoolTarget;
    fn xor(&mut self, a: BoolTarget, b: BoolTarget) -> BoolTarget;

}

impl<F: RichField + Extendable<D>, const D: usize> CircuitBuilderBoolTarget<F, D>
    for CircuitBuilder<F, D>{
        fn xor(&mut self, a: BoolTarget, b: BoolTarget) -> BoolTarget {

            // a ^ b := (a - b)^2
            let s = self.sub(a.target, b.target);
            BoolTarget::new_unsafe(self.mul(s, s))

        }

        fn and(&mut self, a: BoolTarget, b: BoolTarget) -> BoolTarget {
            self.and(a, b)
        }

        fn or(&mut self, a: BoolTarget, b: BoolTarget) -> BoolTarget {
            self.or(a, b)
        }
    }