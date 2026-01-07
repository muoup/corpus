pub mod axioms;
pub mod parsing;
pub mod prover;
pub mod syntax;

use corpus_classical_logic::LogicalStorage;
use corpus_core::NodeStorage;
pub use prover::{PeanoLogicalProver, create_logical_prover};

use crate::syntax::PeanoArithmeticExpression;

pub struct PeanoStores {
    pub storage: LogicalStorage<PeanoArithmeticExpression>,
}

impl PeanoStores {
    /// Create new PeanoStores with fresh node storages.
    pub fn new() -> Self {
        Self {
            storage: LogicalStorage {
                logical_storage: NodeStorage::new(),
                domain_storage: NodeStorage::new(),
            },
        }
    }
}
