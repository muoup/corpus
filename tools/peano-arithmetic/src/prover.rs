//! Peano Arithmetic prover using the generic proving framework.
//!
//! This module provides a thin wrapper around the core `Prover` type,
//! specializing it for Peano Arithmetic with default implementations.
//!
//! It uses `ClassicalLogicalExpression` to support quantifiers and full first-order logic.

use crate::syntax::PeanoLogicalExpression;
use corpus_classical_logic::{BinaryTruth, ClassicalTruthChecker};
use corpus_core::proving::{Prover, SizeCostEstimator};

/// Type alias for the PA prover with LogicalExpression support.
///
/// This combines:
/// - `PeanoLogicalExpression` as the expression type (supports quantifiers and FOL)
/// - `SizeCostEstimator` for cost estimation (expression size)
/// - `PeanoGoalChecker` for goal detection (uses axiom-based goal checking)
pub type PeanoLogicalProver =
    Prover<PeanoLogicalExpression, SizeCostEstimator, BinaryTruth, ClassicalTruthChecker>;

/// Create a new PA prover with quantifier support.
///
/// # Arguments
///
/// * `max_nodes` - Maximum number of states to explore before giving up
///
/// # Examples
///
/// ```ignore
/// let mut prover = create_logical_prover(10000);
/// // The prover will be initialized with axiom-based goal checking
/// // and logical rewrite rules that preserve quantifier structure
/// ```
pub fn create_logical_prover(max_nodes: usize) -> PeanoLogicalProver {
    Prover::new(max_nodes, SizeCostEstimator, ClassicalTruthChecker::new())
}
