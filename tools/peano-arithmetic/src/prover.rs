//! Peano Arithmetic prover using the generic proving framework.
//!
//! This module provides a thin wrapper around the core `Prover` type,
//! specializing it for Peano Arithmetic with default implementations.

use corpus_core::proving::{Prover, SizeHashCostEstimator, HashEqualityGoalChecker};
use crate::syntax::ArithmeticExpression;
use crate::opcodes::PeanoOpcodeMapper;

/// Type alias for the PA prover with default implementations.
///
/// This combines:
/// - `ArithmeticExpression` as the term type
/// - `PeanoOpcodeMapper` for expression construction
/// - `SizeHashCostEstimator` for cost estimation (size + hash distance)
/// - `HashEqualityGoalChecker` for goal detection (same hash = equal)
pub type PeanoProver = Prover<
    ArithmeticExpression,
    PeanoOpcodeMapper,
    SizeHashCostEstimator,
    HashEqualityGoalChecker,
>;

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
pub fn create_prover(max_nodes: usize) -> PeanoProver {
    Prover::new(
        max_nodes,
        SizeHashCostEstimator,
        HashEqualityGoalChecker,
    )
}

// Re-export commonly used types from core for convenience
pub use corpus_core::proving::{ProofStep, ProofState, ProofResult};

/// Extension trait for printing PA-specific proofs.
pub trait ProofResultExt {
    /// Print the proof result in a human-readable format.
    fn print(&self);
}

impl ProofResultExt for ProofResult<ArithmeticExpression> {
    fn print(&self) {
        println!("✓ Theorem proved!");
        println!("Nodes explored: {}", self.nodes_explored);
        println!();

        if !self.lhs_steps.is_empty() {
            println!("LHS transformations:");
            for (i, step) in self.lhs_steps.iter().enumerate() {
                println!("  {}. Apply \"{}\":", i + 1, step.rule_name);
                println!("     {} → {}", step.old_expr, step.new_expr);
            }
            println!();
        }

        if !self.rhs_steps.is_empty() {
            println!("RHS transformations:");
            for (i, step) in self.rhs_steps.iter().enumerate() {
                println!("  {}. Apply \"{}\":", i + 1, step.rule_name);
                println!("     {} → {}", step.old_expr, step.new_expr);
            }
            println!();
        }

        println!("Final: {} = {} ✓",
            self.lhs_steps.last().map_or(&self.final_expr, |s| &s.new_expr),
            self.rhs_steps.last().map_or(&self.final_expr, |s| &s.new_expr)
        );
    }
}
