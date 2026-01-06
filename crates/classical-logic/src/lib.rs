pub mod expression;
pub mod pattern;
pub mod proving;
pub mod truth;

use corpus_core::LogicSystem;

// Re-exports from expression module
pub use expression::{ClassicalLogicalExpression, DomainContent};

pub use proving::ClassicalTruthChecker;
pub use truth::BinaryTruth;

pub struct ClassicalLogicSystem;

impl LogicSystem for ClassicalLogicSystem {
    type TruthType = BinaryTruth;
}
