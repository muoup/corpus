//! Peano Arithmetic prover using the generic proving framework.
//!
//! This module provides a thin wrapper around the core `Prover` type,
//! specializing it for Peano Arithmetic with default implementations.
//!
//! It provides support for both the original PeanoContent-based prover
//! and the new LogicalExpression-based prover that supports quantifiers
//! and full first-order logic.

use crate::syntax::{PeanoContent, PeanoLogicalExpression, PeanoLogicalNode};
use crate::goal::{AxiomPatternChecker, PeanoGoalChecker};
use crate::axioms::peano_arithmetic_rules;
use corpus_classical_logic::BinaryTruth;
use corpus_core::{
    base::nodes::{HashNode, NodeStorage},
    proving::{Prover, SizeCostEstimator, GoalChecker, CostEstimator},
    rewriting::RewriteRule,
    expression::LogicalExpression,
};

/// Type alias for the PA prover with default implementations.
///
/// This combines:
/// - `PeanoContent` as the expression type (equality expressions)
/// - `SizeCostEstimator` for cost estimation (expression size)
/// - `AxiomPatternChecker` for goal detection (matches axiom patterns)
pub type PeanoProver = Prover<PeanoContent, SizeCostEstimator, BinaryTruth, AxiomPatternChecker>;

/// Type alias for the PA prover with quantifier support.
///
/// This combines:
/// - `PeanoLogicalExpression` as the expression type (supports quantifiers and FOL)
/// - `SizeCostEstimator` for cost estimation (expression size)
/// - `PeanoGoalChecker` for goal detection (uses axiom-based goal checking)
pub type PeanoLogicalProver = Prover<
    PeanoLogicalExpression,
    SizeCostEstimator,
    BinaryTruth,
    PeanoGoalChecker,
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
/// // The prover will be initialized with axiom pattern checking
/// // and arithmetic rewrite rules
/// ```
pub fn create_prover(max_nodes: usize) -> PeanoProver {
    Prover::new(max_nodes, SizeCostEstimator, AxiomPatternChecker::new())
}

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
    Prover::new(max_nodes, SizeCostEstimator, PeanoGoalChecker::new())
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

    // This function only handles Equals, not Arithmetic
    let PeanoContent::Equals(left, right) = equality.value.as_ref() else {
        return results;
    };
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

/// Custom proof function for PA with LogicalExpression support.
///
/// This function handles quantified formulas by preserving quantifier structure
/// during proof search. It applies arithmetic rewrites under quantifiers while
/// maintaining the original binding structure.
///
/// # Arguments
///
/// * `initial_expr` - The initial logical expression to prove
/// * `store` - The node storage for creating new nodes
/// * `max_nodes` - Maximum number of states to explore
pub fn prove_pa_logical(
    initial_expr: &PeanoLogicalNode,
    store: &NodeStorage<PeanoLogicalExpression>,
    max_nodes: usize,
) -> Option<ProofResult<PeanoLogicalExpression, BinaryTruth>> {
    use std::collections::{BinaryHeap, HashSet};

    let arithmetic_rules = peano_arithmetic_rules();
    let goal_checker = PeanoGoalChecker::new();
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

        // Check if we've reached the goal
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

        // Get all rewrites while preserving quantifier structure
        for (rewritten_expr, rule_name) in get_all_rewrites_logical(&state.expr, store, &arithmetic_rules) {
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

/// Helper function to get rewrites with rule names for LogicalExpression.
///
/// This function applies arithmetic rewrites while preserving quantifier structure.
/// It uses the `apply_under_quantifiers` utility to recursively apply rewrites to
/// the body of quantified formulas.
fn get_all_rewrites_logical(
    expr: &PeanoLogicalNode,
    store: &NodeStorage<PeanoLogicalExpression>,
    arithmetic_rules: &[RewriteRule<crate::syntax::ArithmeticExpression>],
) -> Vec<(PeanoLogicalNode, String)> {
    let mut results = Vec::new();

    match expr.value.as_ref() {
        // For quantified expressions, apply to body while preserving structure
        LogicalExpression::Compound { operator, operands, .. }
            if operator.symbol() == "∀" || operator.symbol() == "∃" =>
        {
            if let Some(body) = operands.first() {
                // Recursively get rewrites from the body
                for (new_body, rule_name) in get_all_rewrites_logical(body, store, arithmetic_rules) {
                    // Re-wrap with the quantifier
                    let new_expr = LogicalExpression::Compound {
                        operator: operator.clone(),
                        operands: vec![new_body],
                        _phantom: std::marker::PhantomData,
                    };
                    let wrapped = HashNode::from_store(new_expr, store);
                    results.push((wrapped, rule_name));
                }
            }
        }
        // For atomic domain expressions, apply arithmetic rules
        LogicalExpression::Atomic(domain) => {
            match domain.value.as_ref() {
                PeanoContent::Equals(left, right) => {
                    let arith_store = NodeStorage::<crate::syntax::ArithmeticExpression>::new();

                    // Try each arithmetic rule on both sides
                    for rule in arithmetic_rules {
                        // Forward direction on left
                        if let Some(new_left) = rule.apply(left, &arith_store) {
                            let new_content = PeanoContent::Equals(new_left, right.clone());
                            let new_domain = HashNode::from_store(new_content, &NodeStorage::new());
                            let new_expr = LogicalExpression::Atomic(new_domain);
                            let wrapped = HashNode::from_store(new_expr, store);
                            results.push((wrapped, rule.name.clone()));
                        }

                        // Reverse direction on left
                        if let Some(new_left) = rule.apply_reverse(left, &arith_store) {
                            let new_content = PeanoContent::Equals(new_left, right.clone());
                            let new_domain = HashNode::from_store(new_content, &NodeStorage::new());
                            let new_expr = LogicalExpression::Atomic(new_domain);
                            let wrapped = HashNode::from_store(new_expr, store);
                            results.push((wrapped, format!("{}_reverse", rule.name)));
                        }

                        // Forward direction on right
                        if let Some(new_right) = rule.apply(right, &arith_store) {
                            let new_content = PeanoContent::Equals(left.clone(), new_right);
                            let new_domain = HashNode::from_store(new_content, &NodeStorage::new());
                            let new_expr = LogicalExpression::Atomic(new_domain);
                            let wrapped = HashNode::from_store(new_expr, store);
                            results.push((wrapped, rule.name.clone()));
                        }

                        // Reverse direction on right
                        if let Some(new_right) = rule.apply_reverse(right, &arith_store) {
                            let new_content = PeanoContent::Equals(left.clone(), new_right);
                            let new_domain = HashNode::from_store(new_content, &NodeStorage::new());
                            let new_expr = LogicalExpression::Atomic(new_domain);
                            let wrapped = HashNode::from_store(new_expr, store);
                            results.push((wrapped, format!("{}_reverse", rule.name)));
                        }
                    }

                    // Try successor injectivity at the top level
                    if let Some(rewritten_content) = apply_successor_injectivity_to_logical(domain) {
                        let new_domain = HashNode::from_store(rewritten_content, &NodeStorage::new());
                        let new_expr = LogicalExpression::Atomic(new_domain);
                        let wrapped = HashNode::from_store(new_expr, store);
                        results.push((wrapped, "successor_injectivity".to_string()));
                    }
                }
                PeanoContent::Arithmetic(_) => {
                    // No rewrites for pure arithmetic expressions
                }
            }
        }
        // For other compound expressions (AND, OR, etc.), apply to operands
        LogicalExpression::Compound { operator, operands, .. } => {
            // Try applying rewrites to each operand
            for (i, operand) in operands.iter().enumerate() {
                for (new_operand, rule_name) in get_all_rewrites_logical(operand, store, arithmetic_rules) {
                    let mut new_operands = operands.clone();
                    new_operands[i] = new_operand;
                    let new_expr = LogicalExpression::Compound {
                        operator: operator.clone(),
                        operands: new_operands,
                        _phantom: std::marker::PhantomData,
                    };
                    let wrapped = HashNode::from_store(new_expr, store);
                    results.push((wrapped, rule_name.clone()));
                }
            }
        }
    }

    results
}

/// Apply successor injectivity to an atomic domain expression.
fn apply_successor_injectivity_to_logical(
    domain: &HashNode<PeanoContent>,
) -> Option<PeanoContent> {
    let PeanoContent::Equals(left, right) = domain.value.as_ref() else {
        return None;
    };

    // Check if both sides are Successor expressions
    let crate::syntax::ArithmeticExpression::Successor(left_inner) = left.value.as_ref() else {
        return None;
    };

    let crate::syntax::ArithmeticExpression::Successor(right_inner) = right.value.as_ref() else {
        return None;
    };

    // Create new equality: left_inner = right_inner
    Some(PeanoContent::Equals(left_inner.clone(), right_inner.clone()))
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
        if self.truth_result == BinaryTruth::False {
            println!("✗ Statement disproved (contradiction)!");
        } else {
            println!("✓ Theorem proved!");
        }
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

        println!("Final: {} {}", self.final_expr, if self.truth_result == BinaryTruth::False { "✗" } else { "✓" });
    }
}
