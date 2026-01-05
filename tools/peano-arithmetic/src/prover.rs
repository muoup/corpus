//! Peano Arithmetic prover using the generic proving framework.
//!
//! This module provides a thin wrapper around the core `Prover` type,
//! specializing it for Peano Arithmetic with default implementations.

use crate::syntax::{ArithmeticExpression, PeanoContent};
use corpus_classical_logic::BinaryTruth;
use corpus_core::{
    RewriteRule,
    proving::{AxiomMatchChecker, Prover, SizeCostEstimator},
};

/// Type alias for the PA prover with default implementations.
///
/// This combines:
/// - `ArithmeticExpression` as the term type
/// - `SizeHashCostEstimator` for cost estimation (size + hash distance)
/// - `HashEqualityGoalChecker` for goal detection (same hash = equal)
pub type PeanoProver<'a> = Prover<PeanoContent, SizeCostEstimator, BinaryTruth, AxiomMatchChecker<'a, PeanoContent>>;

/// Create a new PA prover with the given node limit.
///
/// # Arguments
///
/// * `max_nodes` - Maximum number of states to explore before giving up
///
/// # Examples
///
/// ```ignore
/// let mut prover = create_prover(10000);
/// for rule in peano_arithmetic_rules() {
///     prover.add_rule(rule);
/// }
/// ```
pub fn create_prover(max_nodes: usize, axioms: &[RewriteRule<PeanoContent>]) -> PeanoProver {
    Prover::new(max_nodes, SizeCostEstimator, AxiomMatchChecker::new(axioms))
}

// Re-export commonly used types from core for convenience
pub use corpus_core::proving::{ProofResult, ProofState, ProofStep};

/// Extension trait for printing PA-specific proofs.
pub trait ProofResultExt {
    /// Print the proof result in a human-readable format.
    fn print(&self);
}

impl ProofResultExt for ProofResult<ArithmeticExpression, BinaryTruth> {
    fn print(&self) {
        println!("✓ Theorem proved!");
        println!("Nodes explored: {}", self.nodes_explored);
        println!();

        if !self.steps.is_empty() {
            println!("Proof steps:");
            for (i, step) in self.steps.iter().enumerate() {
                println!("  {}. Apply \"{}\":", i + 1, step.rule_name);
                println!("     {} → {}", step.old_expr, step.new_expr);
            }
            println!();
        }

        println!("Final: {} ✓", self.final_expr);
    }
}
