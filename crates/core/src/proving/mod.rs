//! Generic proving framework with trait hooks for domain-specific behavior.
//!
//! This module provides a generic prover that can work with any logical system
//! by implementing the `CostEstimator` and `GoalChecker` traits.

use crate::base::nodes::{HashNode, NodeStorage, HashNodeInner};
use crate::base::opcodes::OpcodeMapper;
use crate::rewriting::RewriteRule;
use std::collections::{BinaryHeap, HashSet};
use std::cmp::Ordering;

/// Trait for domain-specific cost estimation in proof search.
///
/// Implementations define how to estimate the "cost" or distance between
/// two expressions. Lower costs indicate states that should be explored first.
pub trait CostEstimator<T: HashNodeInner> {
    /// Estimate the cost between LHS and RHS expressions.
    ///
    /// Lower values indicate the expressions are "closer" and should be
    /// prioritized in the A* search.
    fn estimate_cost(&self, lhs: &HashNode<T>, rhs: &HashNode<T>) -> u64;
}

/// Trait for domain-specific goal checking.
///
/// Implementations define when a proof state is considered a "goal" or
/// success condition. For equational proofs, this is typically when both
/// sides have the same hash.
pub trait GoalChecker<T: HashNodeInner> {
    /// Check if the current state represents a goal (proof complete).
    fn is_goal(&self, lhs: &HashNode<T>, rhs: &HashNode<T>) -> bool;
}

/// Trait for recursive subterm rewriting.
///
/// Domains implement this for `HashNode<T>` to allow the prover to apply rewrite rules
/// to nested expressions, not just top-level ones. This is essential
/// for proving theorems that require rewriting subterms (e.g.,
/// `(S(0) + S(0)) = S(S(0))` requires rewriting `(S(0) + 0)` inside `S(...)`).
pub trait SubtermRewritable: Clone {
    /// The expression type that can be rewritten.
    type Expr: HashNodeInner;

    /// Try to rewrite any subterm (including self) using the given function.
    ///
    /// Returns `Some(new_expression)` if any subterm was successfully rewritten,
    /// or `None` if no rewrite applied.
    ///
    /// The implementation should:
    /// 1. Try rewriting `self` first using `try_rewrite`
    /// 2. If that fails, recursively try subterms
    /// 3. Rebuild the expression with the rewritten subterm
    fn rewrite_any_subterm<F>(
        &self,
        store: &NodeStorage<Self::Expr>,
        try_rewrite: &F,
    ) -> Option<HashNode<Self::Expr>>
    where
        F: Fn(&HashNode<Self::Expr>) -> Option<HashNode<Self::Expr>>;
}

/// Blanket implementation of `SubtermRewritable` for all `HashNode<T>`.
///
/// This delegates to the `HashNodeInner::rewrite_any_subterm` method,
/// which domains can override for their specific expression types.
impl<T: HashNodeInner> SubtermRewritable for HashNode<T> {
    type Expr = T;

    fn rewrite_any_subterm<F>(
        &self,
        store: &NodeStorage<T>,
        try_rewrite: &F,
    ) -> Option<HashNode<T>>
    where
        F: Fn(&HashNode<T>) -> Option<HashNode<T>>,
    {
        self.value.rewrite_any_subterm(self, store, try_rewrite)
    }
}

/// A single transformation step in a proof.
#[derive(Clone)]
pub struct ProofStep<T: HashNodeInner> {
    /// Name of the rewrite rule that was applied.
    pub rule_name: String,
    /// The expression before the transformation.
    pub old_expr: HashNode<T>,
    /// The expression after the transformation.
    pub new_expr: HashNode<T>,
}

/// A state in the proof search with LHS/RHS expressions and associated metadata.
#[derive(Clone)]
pub struct ProofState<T: HashNodeInner> {
    /// Left-hand side expression.
    pub lhs: HashNode<T>,
    /// Right-hand side expression.
    pub rhs: HashNode<T>,
    /// Transformations applied to reach LHS.
    pub lhs_steps: Vec<ProofStep<T>>,
    /// Transformations applied to reach RHS.
    pub rhs_steps: Vec<ProofStep<T>>,
    /// Estimated cost to goal (for A* priority queue ordering).
    pub estimated_cost: u64,
}

/// Result of a successful proof.
pub struct ProofResult<T: HashNodeInner> {
    /// Transformations applied to LHS.
    pub lhs_steps: Vec<ProofStep<T>>,
    /// Transformations applied to RHS.
    pub rhs_steps: Vec<ProofStep<T>>,
    /// Number of states explored during proof search.
    pub nodes_explored: usize,
    /// The final expression where both sides met.
    pub final_expr: HashNode<T>,
}

/// Generic prover using trait hooks for domain-specific behavior.
///
/// # Type Parameters
///
/// * `T` - The expression type (must implement `HashNodeInner` and `Clone`)
/// * `M` - The opcode mapper type for rewrite rules (must implement `OpcodeMapper<T>` and `Clone`)
/// * `C` - The cost estimator for ordering search states
/// * `G` - The goal checker for determining proof completion
pub struct Prover<T: HashNodeInner + Clone, M: OpcodeMapper<T> + Clone, C: CostEstimator<T>, G: GoalChecker<T>> {
    rules: Vec<RewriteRule<T, M>>,
    store: NodeStorage<T>,
    max_nodes: usize,
    cost_estimator: C,
    goal_checker: G,
}

impl<T: HashNodeInner + Clone, M: OpcodeMapper<T> + Clone, C: CostEstimator<T>, G: GoalChecker<T>> Prover<T, M, C, G> {
    /// Create a new prover with the given cost estimator and goal checker.
    pub fn new(max_nodes: usize, cost_estimator: C, goal_checker: G) -> Self {
        Self {
            rules: Vec::new(),
            store: NodeStorage::new(),
            max_nodes,
            cost_estimator,
            goal_checker,
        }
    }

    /// Add a rewrite rule to this prover.
    pub fn add_rule(&mut self, rule: RewriteRule<T, M>) {
        self.rules.push(rule);
    }

    /// Attempt to prove that lhs and rhs are equivalent.
    ///
    /// Uses A* search with bidirectional rewriting. Returns `Some(ProofResult)`
    /// if a proof is found within `max_nodes` states, otherwise `None`.
    pub fn prove(&self, initial_lhs: &HashNode<T>, initial_rhs: &HashNode<T>) -> Option<ProofResult<T>>
    where
        HashNode<T>: SubtermRewritable<Expr = T>,
    {
        let mut heap = BinaryHeap::new();
        let mut visited: HashSet<(u64, u64)> = HashSet::new();
        let mut nodes_explored = 0usize;

        let initial_cost = self.cost_estimator.estimate_cost(initial_lhs, initial_rhs);
        let initial_state = ProofState {
            lhs: initial_lhs.clone(),
            rhs: initial_rhs.clone(),
            lhs_steps: Vec::new(),
            rhs_steps: Vec::new(),
            estimated_cost: initial_cost,
        };

        heap.push(initial_state);

        while let Some(state) = heap.pop() {
            nodes_explored += 1;

            if nodes_explored > self.max_nodes {
                return None;
            }

            if self.goal_checker.is_goal(&state.lhs, &state.rhs) {
                return Some(ProofResult {
                    lhs_steps: state.lhs_steps,
                    rhs_steps: state.rhs_steps,
                    nodes_explored,
                    final_expr: state.lhs,
                });
            }

            let key = (state.lhs.hash(), state.rhs.hash());
            if visited.contains(&key) {
                continue;
            }
            visited.insert(key);

            for successor in self.expand_state(&state) {
                heap.push(successor);
            }
        }

        None
    }

    /// Expand a state by applying all rewrite rules to LHS and RHS (including subterms).
    fn expand_state(&self, state: &ProofState<T>) -> Vec<ProofState<T>>
    where
        HashNode<T>: SubtermRewritable<Expr = T>,
    {
        let mut successors = Vec::new();

        for rule in &self.rules {
            if rule.is_bidirectional() {
                // Try rewriting any subterm (including top-level) on LHS using forward direction
                if let Some(new_lhs) = state.lhs.rewrite_any_subterm(&self.store, &|term| rule.apply(term, &self.store)) {
                    let new_cost = self.cost_estimator.estimate_cost(&new_lhs, &state.rhs);
                    let mut lhs_steps = state.lhs_steps.clone();
                    lhs_steps.push(ProofStep {
                        rule_name: rule.name.clone(),
                        old_expr: state.lhs.clone(),
                        new_expr: new_lhs.clone(),
                    });
                    successors.push(ProofState {
                        lhs: new_lhs,
                        rhs: state.rhs.clone(),
                        lhs_steps,
                        rhs_steps: state.rhs_steps.clone(),
                        estimated_cost: new_cost,
                    });
                }

                // Try rewriting any subterm on RHS using reverse direction
                if let Some(new_rhs) = state.rhs.rewrite_any_subterm(&self.store, &|term| rule.apply_reverse(term, &self.store)) {
                    let new_cost = self.cost_estimator.estimate_cost(&state.lhs, &new_rhs);
                    let mut rhs_steps = state.rhs_steps.clone();
                    rhs_steps.push(ProofStep {
                        rule_name: rule.name.clone(),
                        old_expr: state.rhs.clone(),
                        new_expr: new_rhs.clone(),
                    });
                    successors.push(ProofState {
                        lhs: state.lhs.clone(),
                        rhs: new_rhs,
                        lhs_steps: state.lhs_steps.clone(),
                        rhs_steps,
                        estimated_cost: new_cost,
                    });
                }
            }
        }

        successors
    }
}

// Implement Ord for BinaryHeap (min-heap by cost)
impl<T: HashNodeInner> PartialEq for ProofState<T> {
    fn eq(&self, other: &Self) -> bool {
        self.estimated_cost == other.estimated_cost
    }
}

impl<T: HashNodeInner> Eq for ProofState<T> {}

impl<T: HashNodeInner> PartialOrd for ProofState<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: HashNodeInner> Ord for ProofState<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.estimated_cost.cmp(&self.estimated_cost) // Reverse for min-heap
    }
}

// ============================================================================
// Default Implementations
// ============================================================================

/// Default cost estimator: combines expression sizes and hash distance.
///
/// Cost = size(LHS) + size(RHS) + |hash(LHS) - hash(RHS)|
///
/// This encourages exploring smaller expressions with closer hash values.
pub struct SizeHashCostEstimator;

impl<T: HashNodeInner> CostEstimator<T> for SizeHashCostEstimator {
    fn estimate_cost(&self, lhs: &HashNode<T>, rhs: &HashNode<T>) -> u64 {
        let lhs_size = lhs.size();
        let rhs_size = rhs.size();
        let hash_distance = lhs.hash().abs_diff(rhs.hash());
        lhs_size + rhs_size + hash_distance
    }
}

/// Default goal checker: hash equality.
///
/// Considers the proof complete when both sides have the same hash,
/// which is a fast approximation of structural equality for hash-consed terms.
pub struct HashEqualityGoalChecker;

impl<T: HashNodeInner> GoalChecker<T> for HashEqualityGoalChecker {
    fn is_goal(&self, lhs: &HashNode<T>, rhs: &HashNode<T>) -> bool {
        lhs.hash() == rhs.hash()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Simple test expression type (just u64 numbers)
    #[derive(Clone, Copy)]
    struct TestMapper;
    impl crate::base::opcodes::OpcodeMapper<u64> for TestMapper {
        fn construct(&self, _opcode: u8, _children: Vec<HashNode<u64>>, _store: &NodeStorage<u64>) -> HashNode<u64> {
            panic!("u64 has no compound expressions")
        }
        fn get_opcode(&self, _expr: &HashNode<u64>) -> Option<u8> {
            None
        }
        fn is_valid_opcode(&self, _opcode: u8) -> bool {
            false
        }
        fn arity_for_opcode(&self, _opcode: u8) -> Option<usize> {
            None
        }
    }

    #[test]
    fn test_cost_estimator() {
        let store = NodeStorage::new();
        let lhs = HashNode::from_store(42u64, &store);
        let rhs = HashNode::from_store(42u64, &store);
        let estimator = SizeHashCostEstimator;

        let cost = estimator.estimate_cost(&lhs, &rhs);
        // Same value should have low cost (size + size + 0 hash distance)
        assert_eq!(cost, 2); // 1 + 1 + 0
    }

    #[test]
    fn test_goal_checker() {
        let store = NodeStorage::new();
        let expr = HashNode::from_store(42u64, &store);
        let checker = HashEqualityGoalChecker;

        assert!(checker.is_goal(&expr, &expr));
    }
}
