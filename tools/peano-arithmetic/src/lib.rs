pub mod axioms;
pub mod parsing;
pub mod prover;
pub mod syntax;

use corpus_classical_logic::LogicalStorage;
use corpus_core::{NodeStorage, RewriteRule};
pub use prover::{PeanoLogicalProver, create_logical_prover};

use crate::syntax::{PeanoArithmeticExpression, PeanoDomainExpression};

/// Storage for Peano Arithmetic domain expressions.
/// Contains separate storage for arithmetic expressions and domain-level expressions (like Equality).
pub struct PeanoStorage {
    /// Storage for arithmetic expressions (Add, Successor, Number, DeBruijn)
    pub arithmetic_storage: NodeStorage<PeanoArithmeticExpression>,
    /// Storage for domain-level expressions (Equality)
    pub domain_content_storage: NodeStorage<PeanoDomainExpression>,
    /// Storage for arithmetic rewrite rules (applied to expressions within domain content)
    pub arithmetic_rules: Vec<RewriteRule<PeanoArithmeticExpression>>,
}

impl PeanoStorage {
    /// Add an arithmetic rewrite rule to storage.
    pub fn add_arithmetic_rule(&mut self, rule: RewriteRule<PeanoArithmeticExpression>) {
        self.arithmetic_rules.push(rule);
    }

    /// Get all arithmetic rewrite rules.
    pub fn get_arithmetic_rules(&self) -> &[RewriteRule<PeanoArithmeticExpression>] {
        &self.arithmetic_rules
    }
}

impl Default for PeanoStorage {
    fn default() -> Self {
        Self {
            arithmetic_storage: NodeStorage::new(),
            domain_content_storage: NodeStorage::new(),
            arithmetic_rules: Vec::new(),
        }
    }
}

pub struct PeanoStores {
    pub storage: LogicalStorage<PeanoDomainExpression>,
}

impl PeanoStores {
    /// Create new PeanoStores with fresh node storages.
    pub fn new() -> Self {
        Self {
            storage: LogicalStorage {
                logical_storage: NodeStorage::new(),
                domain_storage: PeanoStorage::default(),
            },
        }
    }

    /// Convenience method to access the domain storage
    pub fn pa_storage(&self) -> &PeanoStorage {
        &self.storage.domain_storage
    }

    /// Convenience method to access the domain storage mutably
    pub fn pa_storage_mut(&mut self) -> &mut PeanoStorage {
        &mut self.storage.domain_storage
    }
}
