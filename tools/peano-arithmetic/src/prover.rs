//! Peano Arithmetic prover using the generic proving framework.
//!
//! This module provides a thin wrapper around the core `Prover` type,
//! specializing it for Peano Arithmetic with default implementations.

use crate::syntax::PeanoContent;
use crate::goal::AxiomPatternChecker;
use crate::axioms::peano_arithmetic_rules;
use corpus_classical_logic::BinaryTruth;
use corpus_core::{
    base::nodes::{HashNode, NodeStorage},
    proving::{Prover, SizeCostEstimator, GoalChecker, CostEstimator},
    rewriting::RewriteRule,
};

/// Type alias for the PA prover with default implementations.
///
/// This combines:
/// - `PeanoContent` as the expression type (equality expressions)
/// - `SizeCostEstimator` for cost estimation (expression size)
/// - `AxiomPatternChecker` for goal detection (matches axiom patterns)
pub type PeanoProver = Prover<PeanoContent, SizeCostEstimator, BinaryTruth, AxiomPatternChecker>;

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
/// // The prover will be initialized with axiom pattern checking
/// // and arithmetic rewrite rules
/// ```
pub fn create_prover(max_nodes: usize) -> PeanoProver {
    Prover::new(max_nodes, SizeCostEstimator, AxiomPatternChecker::new())
}

/// Custom proof function for PA that handles the type mismatch between
/// PeanoContent (equalities) and ArithmeticExpression (arithmetic terms).
///
/// This function uses the arithmetic rewrite rules to transform the subterms
/// of the equality, checking if the result matches an axiom pattern.
pub fn prove_pa(
    initial_expr: &HashNode<PeanoContent>,
    store: &NodeStorage<PeanoContent>,
    max_nodes: usize,
) -> Option<crate::prover::ProofResult<PeanoContent, BinaryTruth>> {
    use std::collections::{BinaryHeap, HashSet};
    use crate::prover::{ProofState, ProofStep, ProofResult};

    let arithmetic_rules = peano_arithmetic_rules();
    let goal_checker = AxiomPatternChecker::new();
    let cost_estimator = SizeCostEstimator;

    let mut heap = BinaryHeap::new();
    let mut visited = HashSet::new();
    let mut nodes_explored = 0usize;

    let initial_cost = cost_estimator.estimate_cost(initial_expr);
    let initial_state = ProofState {
        expr: initial_expr.clone(),
        steps: Vec::new(),
        estimated_cost: initial_cost,
    };

    heap.push(initial_state);

    while let Some(state) = heap.pop() {
        nodes_explored += 1;

        if nodes_explored > max_nodes {
            return None;
        }

        // Check if we've reached the goal (matches an axiom pattern)
        if let Some(truth) = goal_checker.check(&state.expr) {
            return Some(ProofResult {
                steps: state.steps,
                nodes_explored,
                final_expr: state.expr,
                truth_result: truth,
            });
        }

        let key = state.expr.hash();
        if visited.contains(&key) {
            continue;
        }
        visited.insert(key);

        // Get all rewrites by applying arithmetic rules to subterms
        for (rewritten_expr, rule_name) in get_all_rewrites_with_names(&state.expr, store, &arithmetic_rules) {
            let cost = cost_estimator.estimate_cost(&rewritten_expr);
            heap.push(ProofState {
                expr: rewritten_expr.clone(),
                steps: {
                    let mut new_steps = state.steps.clone();
                    new_steps.push(ProofStep {
                        rule_name,
                        old_expr: state.expr.clone(),
                        new_expr: rewritten_expr,
                    });
                    new_steps
                },
                estimated_cost: cost,
            });
        }
    }

    None
}

/// Helper function to get rewrites with rule names.
fn get_all_rewrites_with_names(
    equality: &HashNode<PeanoContent>,
    store: &NodeStorage<PeanoContent>,
    arithmetic_rules: &[RewriteRule<crate::syntax::ArithmeticExpression>],
) -> Vec<(HashNode<PeanoContent>, String)> {
    let mut results = Vec::new();

    let PeanoContent::Equals(left, right) = equality.value.as_ref();
    let arith_store = NodeStorage::<crate::syntax::ArithmeticExpression>::new();

    // Try each arithmetic rule on both sides
    for rule in arithmetic_rules {
        // Forward direction on left
        if let Some(new_left) = rule.apply(left, &arith_store) {
            let new_content = PeanoContent::Equals(new_left, right.clone());
            let new_expr = HashNode::from_store(new_content, store);
            results.push((new_expr, rule.name.clone()));
        }

        // Reverse direction on left
        if let Some(new_left) = rule.apply_reverse(left, &arith_store) {
            let new_content = PeanoContent::Equals(new_left, right.clone());
            let new_expr = HashNode::from_store(new_content, store);
            results.push((new_expr, format!("{}_reverse", rule.name)));
        }

        // Forward direction on right
        if let Some(new_right) = rule.apply(right, &arith_store) {
            let new_content = PeanoContent::Equals(left.clone(), new_right);
            let new_expr = HashNode::from_store(new_content, store);
            results.push((new_expr, rule.name.clone()));
        }

        // Reverse direction on right
        if let Some(new_right) = rule.apply_reverse(right, &arith_store) {
            let new_content = PeanoContent::Equals(left.clone(), new_right);
            let new_expr = HashNode::from_store(new_content, store);
            results.push((new_expr, format!("{}_reverse", rule.name)));
        }
    }

    // Try successor injectivity at the top level: S(x) = S(y) -> x = y
    if let Some(rewritten) = crate::syntax::apply_successor_injectivity(equality, store) {
        results.push((rewritten, "successor_injectivity".to_string()));
    }

    results
}

// Re-export commonly used types from core for convenience
pub use corpus_core::proving::{ProofResult, ProofState, ProofStep};

/// Extension trait for printing PA-specific proofs.
pub trait ProofResultExt {
    /// Print the proof result in a human-readable format.
    fn print(&self);
}

impl ProofResultExt for ProofResult<PeanoContent, BinaryTruth> {
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
