//! Generic proving framework with trait hooks for domain-specific behavior.
//!
//! This module provides a generic prover that can work with any logical system
//! by implementing the `CostEstimator` and `GoalChecker` traits.

use crate::base::nodes::{HashNode, HashNodeInner};
use crate::rewriting::RewriteRule;
use crate::rewriting::patterns::Rewritable;
use crate::{BinaryTruth, TruthValue};
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};
use std::fmt::Display;

/// Trait for domain-specific cost estimation in proof search.
///
/// Implementations define how to estimate the "cost" or "distance to goal" for
/// an expression. Lower costs indicate states that should be explored first.
pub trait CostEstimator<T: Rewritable> {
    /// Estimate the cost of an expression (distance to goal).
    ///
    /// Lower values indicate the expression is "closer" to a goal and should be
    /// prioritized in the A* search.
    fn estimate_cost(&self, previous_state: Option<&ProofState<T>>, expr: &HashNode<T>) -> u64;
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
pub struct ProofState<T: Rewritable> {
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
pub struct Prover<Node: Rewritable, C: CostEstimator<Node>, T: TruthValue, G: GoalChecker<Node, T>>
{
    rules: Vec<RewriteRule<Node>>,
    max_nodes: usize,
    cost_estimator: C,
    goal_checker: G,

    _phantom: std::marker::PhantomData<T>,
}

impl<
    Node: Rewritable + Display,
    C: CostEstimator<Node>,
    T: TruthValue,
    G: GoalChecker<Node, T>,
> Prover<Node, C, T, G>
{
    /// Create a new prover with the given cost estimator and goal checker.
    pub fn new(max_nodes: usize, cost_estimator: C, goal_checker: G) -> Self {
        Self {
            rules: Vec::new(),
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

    /// Add multiple rewrite rules to this prover.
    pub fn add_rules(&mut self, rules: impl IntoIterator<Item = RewriteRule<Node>>) {
        self.rules.extend(rules);
    }

    /// Attempt to prove a statement by rewriting it until a goal is reached.
    ///
    /// Uses A* search to explore possible rewrites. Returns `Some(ProofResult)`
    /// if a proof is found within `max_nodes` states, otherwise `None`.
    pub fn prove(
        &self,
        store: &Node::Storage,
        initial_expr: HashNode<Node>,
    ) -> Option<ProofResult<Node, T>> {
        let mut heap = BinaryHeap::new();
        let mut visited = HashSet::new();
        let mut nodes_explored = 0usize;

        let initial_cost = self.cost_estimator.estimate_cost(None, &initial_expr);
        let initial_state = ProofState {
            expr: initial_expr,
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
                // Try to apply this rule to the current expression (including recursive rewrites)
                let rewrites = rule.apply_recursive(&state.expr, store);
                if !rewrites.is_empty() {
                    println!("Rule {} generated {} rewrites for {}", rule.name, rewrites.len(), state.expr);
                }
                for rewritten in rewrites {
                    println!("  -> {}", rewritten);
                    let cost = self.cost_estimator.estimate_cost(Some(&state), &rewritten);

                    heap.push(ProofState {
                        expr: rewritten.clone(),
                        steps: {
                            let mut new_steps = state.steps.clone();
                            new_steps.push(ProofStep {
                                rule_name: rule.name.clone(),
                                old_expr: state.expr.clone(),
                                new_expr: rewritten,
                            });
                            new_steps
                        },
                        estimated_cost: cost,
                    });
                }
            }
        }

        None
    }
}

// Implement Ord for BinaryHeap (min-heap by cost)
impl<T: Rewritable> PartialEq for ProofState<T> {
    fn eq(&self, other: &Self) -> bool {
        self.estimated_cost == other.estimated_cost
    }
}

impl<T: Rewritable> Eq for ProofState<T> {}

impl<T: Rewritable> PartialOrd for ProofState<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: Rewritable> Ord for ProofState<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.estimated_cost.cmp(&self.estimated_cost) // Reverse for min-heap
    }
}

// ============================================================================
// Default Implementations
// ============================================================================

/// Default cost estimator: based on expression size.
///
/// Lower cost = smaller expression. This encourages exploring smaller
/// expressions first as they likely indicate simpler forms.
pub struct SizeCostEstimator;

impl<T: Rewritable> CostEstimator<T> for SizeCostEstimator {
    fn estimate_cost(&self, previous_state: Option<&ProofState<T>>, expr: &HashNode<T>) -> u64 {
        let expr_size = expr.size();
        let path_length = previous_state
            .map(|s| s.steps.len())
            .unwrap_or(0) as u64;
        
        path_length + expr_size
    }
}

/// Default goal checker: reflexive axiom check for equalities
///
/// For equality expressions, checks if both sides have the same hash (i.e., they're equal),
/// which means the reflexive axiom (x = x) applies.
pub struct ReflexiveGoalChecker;

impl ReflexiveGoalChecker {
    pub fn new() -> Self {
        Self
    }
}

impl Default for ReflexiveGoalChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl<Node: HashNodeInner + Clone> GoalChecker<Node, BinaryTruth> for ReflexiveGoalChecker {
    fn check(&self, _expr: &HashNode<Node>) -> Option<BinaryTruth> {
        // For a generic node, we can't check if it's an equality with two sides.
        // This is meant to be overridden by domain-specific implementations.
        // For PA, this should check if both sides of PeanoContent::Equals are equal.
        None
    }
}