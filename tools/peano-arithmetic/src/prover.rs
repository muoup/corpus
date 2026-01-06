//! Peano Arithmetic prover using the generic proving framework.
//!
//! This module provides a thin wrapper around the core `Prover` type,
//! specializing it for Peano Arithmetic with default implementations.
//!
//! It uses `LogicalExpression` to support quantifiers and full first-order logic.

use crate::syntax::{PeanoContent, PeanoLogicalExpression, PeanoLogicalNode};
use crate::goal::PeanoGoalChecker;
use crate::axioms::peano_arithmetic_rules;
use corpus_classical_logic::BinaryTruth;
use corpus_core::{
    base::nodes::{HashNode, NodeStorage},
    proving::{Prover, SizeCostEstimator, CostEstimator, GoalChecker, ProofResult, ProofState, ProofStep},
    rewriting::RewriteRule,
    expression::LogicalExpression,
};

/// Type alias for the PA prover with LogicalExpression support.
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
