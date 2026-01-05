//! Generic proving framework with trait hooks for domain-specific behavior.
//!
//! This module provides a generic prover that can work with any logical system
//! by implementing the `CostEstimator` and `GoalChecker` traits.

use crate::base::nodes::{HashNode, HashNodeInner, NodeStorage};
use crate::rewriting::RewriteRule;
use crate::{BinaryTruth, Pattern, TruthValue};
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};

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
pub trait GoalChecker<Node: HashNodeInner, T: TruthValue> {
    /// Check if the current state represents a goal (proof complete).
    fn check(&self, expr: &HashNode<Node>) -> Option<T>;
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

    fn rewrite_any_subterm<F>(&self, store: &NodeStorage<T>, try_rewrite: &F) -> Option<HashNode<T>>
    where
        F: Fn(&HashNode<T>) -> Option<HashNode<T>>,
    {
        self.value.rewrite_any_subterm(self, store, try_rewrite)
    }
}

impl<T: HashNodeInner> HashNode<T> {
    pub fn get_all_rewrites<F>(&self, store: &NodeStorage<T>, try_rewrite: &F) -> Vec<HashNode<T>>
    where
        F: Fn(&HashNode<T>) -> Option<HashNode<T>>,
    {
        let mut rewrites = Vec::new();

        if let Some(rewritten) = try_rewrite(self) {
            rewrites.push(rewritten);
        }

        let Some((opcode, parts)) = self.value.decompose() else {
            return rewrites;
        };

        for (i, part) in parts.iter().enumerate() {
            for rewrite in part.get_all_rewrites(store, try_rewrite).into_iter() {
                let mut new_parts = parts.clone();
                new_parts[i] = rewrite;

                rewrites.push(T::construct_from_parts(opcode, new_parts, store).unwrap());
            }
        }

        rewrites
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
    /// Expression
    pub expr: HashNode<T>,
    /// Transformations applied to reach this state.
    pub steps: Vec<ProofStep<T>>,
    /// Estimated cost to goal (for A* priority queue ordering).
    pub estimated_cost: u64,
}

/// Result of a successful proof.
pub struct ProofResult<Node: HashNodeInner, T: TruthValue> {
    /// Transformations applied
    pub steps: Vec<ProofStep<Node>>,
    /// Number of states explored during proof search.
    pub nodes_explored: usize,
    /// The final expression where both sides met.
    pub final_expr: HashNode<Node>,
    /// Result
    pub truth_result: T,
}

/// Generic prover using trait hooks for domain-specific behavior.
///
/// # Type Parameters
///
/// * `T` - The expression type (must implement `HashNodeInner` and `Clone`)
/// * `C` - The cost estimator for ordering search states
/// * `G` - The goal checker for determining proof completion
pub struct Prover<
    Node: HashNodeInner + Clone,
    C: CostEstimator<Node>,
    T: TruthValue,
    G: GoalChecker<Node, T>,
> {
    rules: Vec<RewriteRule<Node>>,
    store: NodeStorage<Node>,
    max_nodes: usize,
    cost_estimator: C,
    goal_checker: G,

    _phantom: std::marker::PhantomData<T>,
}

impl<Node: HashNodeInner + Clone, C: CostEstimator<Node>, T: TruthValue, G: GoalChecker<Node, T>>
    Prover<Node, C, T, G>
{
    /// Create a new prover with the given cost estimator and goal checker.
    pub fn new(max_nodes: usize, cost_estimator: C, goal_checker: G) -> Self {
        Self {
            rules: Vec::new(),
            store: NodeStorage::new(),
            max_nodes,
            cost_estimator,
            goal_checker,

            _phantom: std::marker::PhantomData,
        }
    }

    /// Add a rewrite rule to this prover.
    pub fn add_rule(&mut self, rule: RewriteRule<Node>) {
        self.rules.push(rule);
    }

    /// Attempt to prove that lhs and rhs are equivalent.
    ///
    /// Uses A* search with bidirectional rewriting. Returns `Some(ProofResult)`
    /// if a proof is found within `max_nodes` states, otherwise `None`.
    pub fn prove(
        &self,
        initial_lhs: &HashNode<Node>,
        initial_rhs: &HashNode<Node>,
    ) -> Option<ProofResult<Node, T>>
    where
        HashNode<Node>: SubtermRewritable<Expr = Node>,
    {
        let mut heap = BinaryHeap::new();
        let mut visited = HashSet::new();
        let mut nodes_explored = 0usize;

        let initial_cost = self.cost_estimator.estimate_cost(initial_lhs, initial_rhs);
        let initial_state = ProofState {
            expr: initial_lhs.clone(),
            steps: Vec::new(),
            estimated_cost: initial_cost,
        };

        heap.push(initial_state);

        while let Some(state) = heap.pop() {
            nodes_explored += 1;

            if nodes_explored > self.max_nodes {
                return None;
            }

            if let Some(truth) = self.goal_checker.check(&state.expr) {
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

            for rule in self.rules.iter() {
                for successor in state
                    .expr
                    .get_all_rewrites(&self.store, &|node| rule.apply(node, &self.store))
                {
                    heap.push(ProofState {
                        expr: successor.clone(),
                        steps: {
                            let mut new_steps = state.steps.clone();
                            new_steps.push(ProofStep {
                                rule_name: rule.name.clone(),
                                old_expr: state.expr.clone(),
                                new_expr: successor.clone(),
                            });
                            new_steps
                        },
                        estimated_cost: self.cost_estimator.estimate_cost(&successor, &initial_rhs),
                    });
                }
            }
        }

        None
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
/// Cost = size(LHS) + size(RHS)
///
/// This encourages exploring smaller expressions first as smaller likely indicates
/// simpler forms.
pub struct SizeCostEstimator;

impl<T: HashNodeInner> CostEstimator<T> for SizeCostEstimator {
    fn estimate_cost(&self, lhs: &HashNode<T>, rhs: &HashNode<T>) -> u64 {
        let lhs_size = lhs.size();
        let rhs_size = rhs.size();
        lhs_size + rhs_size
    }
}

/// Default goal checker: axiom matching
///
/// Checks if the expression provided exactly matches a member of the set of axioms;
pub struct AxiomMatchChecker<'a, Node>
where
    Node: HashNodeInner + Clone,
{
    axioms: &'a [RewriteRule<Node>],
}

impl<'a, Node: HashNodeInner + Clone> AxiomMatchChecker<'a, Node> {
    /// Create a new axiom match checker with the given axioms.
    pub fn new(axioms: &'a [RewriteRule<Node>]) -> Self {
        Self { axioms }
    }
}

impl<'a, Node: HashNodeInner + Clone> GoalChecker<Node, BinaryTruth>
    for AxiomMatchChecker<'a, Node>
{
    fn check(&self, expr: &HashNode<Node>) -> Option<BinaryTruth> {
        for axiom in self.axioms.iter() {
            if axiom.apply(expr, &NodeStorage::new()).is_some() {
                return Some(BinaryTruth::True);
            }
        }
        
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cost_estimator() {
        let store = NodeStorage::new();
        let lhs = HashNode::from_store(42u64, &store);
        let rhs = HashNode::from_store(42u64, &store);
        let estimator = SizeCostEstimator;

        let cost = estimator.estimate_cost(&lhs, &rhs);
        // Same value should have low cost (size + size + 0 hash distance)
        assert_eq!(cost, 2); // 1 + 1 + 0
    }

    // #[test]
    // fn test_goal_checker() {
    //     let store = NodeStorage::new();
    //     let expr = HashNode::from_store(42u64, &store);
    //     let checker = AxiomMatchChecker;

    //     assert!(checker.check(&expr, &expr));
    // }
}
