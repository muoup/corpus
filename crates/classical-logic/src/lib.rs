pub mod axioms;
pub mod operators;
pub mod truth;

use std::ops::{Deref, DerefMut};

use corpus_core::logic::LogicalOperatorSet;
use corpus_core::truth::TruthValue;

pub use axioms::ClassicalAxiomConverter;
pub use corpus_core::base::axioms::{InferenceDirection, InferenceDirectional, NamedAxiom};
pub use operators::ClassicalOperator;
pub use truth::BinaryTruth;

#[repr(transparent)]
pub struct ClassicalLogicalSystem<T>(LogicalOperatorSet<T, ClassicalOperator>)
where
    T: TruthValue;

impl<T: TruthValue> From<LogicalOperatorSet<T, ClassicalOperator>> for ClassicalLogicalSystem<T> {
    fn from(set: LogicalOperatorSet<T, ClassicalOperator>) -> Self {
        ClassicalLogicalSystem(set)
    }
}

impl<T: TruthValue> Deref for ClassicalLogicalSystem<T> {
    type Target = LogicalOperatorSet<T, ClassicalOperator>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: TruthValue> DerefMut for ClassicalLogicalSystem<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T: TruthValue> ClassicalLogicalSystem<T> {
    pub fn with_classical_operators() -> Self {
        let mut system = LogicalOperatorSet::new();

        system.add_operator(ClassicalOperator::And);
        system.add_operator(ClassicalOperator::Or);
        system.add_operator(ClassicalOperator::Implies);
        system.add_operator(ClassicalOperator::Iff);
        system.add_operator(ClassicalOperator::Not);
        system.add_operator(ClassicalOperator::Forall);
        system.add_operator(ClassicalOperator::Exists);

        system.into()
    }
}
