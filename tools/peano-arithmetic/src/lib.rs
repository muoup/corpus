pub mod axioms;
pub mod parsing;
pub mod prover;
pub mod syntax;

use corpus_core::NodeStorage;
pub use prover::{PeanoLogicalProver, create_logical_prover};

use crate::syntax::PeanoArithmeticExpression;

pub struct PeanoStores {
    pub arithmetic_store: NodeStorage<PeanoArithmeticExpression>,
}

impl PeanoStores {
    /// Create new PeanoStores with fresh node storages.
    pub fn new() -> Self {
        Self {
            arithmetic_store: NodeStorage::new(),
        }
    }
}
