//! Goal checking for Peano Arithmetic proofs.
//!
//! This module provides goal checking implementations for the PA prover.
//!
//! The module includes:
//! - `AxiomPatternChecker`: Legacy checker using hard-coded patterns (for backwards compatibility)
//! - `PeanoGoalChecker`: New checker using generic axiom-based goal checking from classical-logic
//!
//! The `PeanoGoalChecker` is the recommended approach as it uses the generic
//! `AxiomGoalChecker` from the classical-logic crate, which checks theorems against
//! axioms to determine proof completion.

use corpus_classical_logic::{BinaryTruth, ClassicalOperator, AxiomGoalChecker};
use corpus_core::proving::GoalChecker;
use corpus_core::base::nodes::HashNode;
use corpus_core::expression::LogicalExpression;
use crate::syntax::{PeanoContent, ArithmeticExpression, PeanoLogicalNode};
use crate::axioms::peano_arithmetic_axioms_with_goals;

/// Goal checker for Peano Arithmetic equalities.
///
/// For PA equalities, the goal is to check for:
/// - **Reflexive property** (x = x): Returns `Some(True)` when both sides
///   have the same hash, indicating a tautology.
/// - **Contradictions** (n = S(n)): Returns `Some(False)` when a provable
///   contradiction is detected, such as 0 = S(0).
///
/// Note: The PA axioms (additive identity, additive successor) are used as
/// **rewrite rules** for transforming expressions, not as goal patterns.
pub struct AxiomPatternChecker;

impl AxiomPatternChecker {
    pub fn new() -> Self {
        Self
    }
}

impl Default for AxiomPatternChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl GoalChecker<PeanoContent, BinaryTruth> for AxiomPatternChecker {
    fn check(&self, expr: &HashNode<PeanoContent>) -> Option<BinaryTruth> {
        // First check for contradiction (e.g., n = S(n))
        if let Some(result) = check_contradiction(expr) {
            return Some(result);
        }
        // Then check for reflexive equality (x = x)
        check_reflexive_equality(expr)
    }
}

/// Check if the equality is reflexive (x = x), which is the logical basis of equality truth.
///
/// When both sides of an equality have the same hash, they are structurally identical,
/// meaning we've proven the original statement by rewriting it to a tautology.
fn check_reflexive_equality(expr: &HashNode<PeanoContent>) -> Option<BinaryTruth> {
    // This function only handles Equals, not Arithmetic
    let PeanoContent::Equals(left, right) = expr.value.as_ref() else {
        return None;
    };
    // Check if left and right sides have the same hash (are structurally equal)
    if left.hash() == right.hash() {
        return Some(BinaryTruth::True);
    }
    None
}

/// Check if the equality represents a contradiction.
///
/// A contradiction in Peano Arithmetic occurs when we can prove that an
/// equality is always false. The primary contradiction pattern is `n = S(n)`
/// (e.g., `0 = S(0)`), which violates the injectivity of the successor function.
///
/// Returns `Some(BinaryTruth::False)` if a contradiction is detected.
fn check_contradiction(expr: &HashNode<PeanoContent>) -> Option<BinaryTruth> {
    // This function only handles Equals, not Arithmetic
    let PeanoContent::Equals(left, right) = expr.value.as_ref() else {
        return None;
    };

    // Check if this is a direct contradiction like n = S(n)
    if is_successor_contradiction(left, right) || is_successor_contradiction(right, left) {
        return Some(BinaryTruth::False);
    }

    None
}

/// Check if `right` is a direct successor of `left`.
///
/// This detects patterns like `0 = S(0)`, `S(0) = S(S(0))`, etc.
/// These are contradictions because no number equals its own successor
/// (from the PA axioms about successor injectivity).
fn is_successor_contradiction(
    left: &HashNode<ArithmeticExpression>,
    right: &HashNode<ArithmeticExpression>,
) -> bool {
    // Check if right is S(left)
    match right.value.as_ref() {
        ArithmeticExpression::Successor(inner) => inner.hash() == left.hash(),
        _ => false,
    }
}

/// PA goal checker using generic axiom-based goal checking.
///
/// This checker wraps the generic `AxiomGoalChecker` from the classical-logic crate,
/// providing PA-specific axiom configuration. It determines if a theorem has been
/// proven by checking if it matches any PA axiom.
///
/// # Examples
///
/// - Theorem `x = x` matches reflexivity axiom → `True`
/// - Theorem `x = S(x)` matches negated injectivity axiom → `False`
/// - Theorem `0 + S(0) = S(0)` matches additive identity axiom → `True`
///
/// This is the recommended goal checker for Peano Arithmetic proofs, as it properly
/// separates axiom definition from goal checking logic.
pub struct PeanoGoalChecker {
    inner: AxiomGoalChecker<BinaryTruth, PeanoContent, ClassicalOperator>,
}

impl PeanoGoalChecker {
    /// Create a new PA goal checker with standard PA axioms.
    pub fn new() -> Self {
        let axioms = peano_arithmetic_axioms_with_goals();
        Self {
            inner: AxiomGoalChecker::new(axioms),
        }
    }

    /// Create a new PA goal checker with custom axioms.
    pub fn with_axioms(axioms: Vec<corpus_classical_logic::NamedAxiom<BinaryTruth, PeanoContent, ClassicalOperator>>) -> Self {
        Self {
            inner: AxiomGoalChecker::new(axioms),
        }
    }
}

impl Default for PeanoGoalChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl GoalChecker<crate::syntax::PeanoLogicalExpression, BinaryTruth> for PeanoGoalChecker {
    fn check(&self, expr: &PeanoLogicalNode) -> Option<BinaryTruth> {
        self.inner.check(expr)
    }
}

/// Legacy quantified goal checker (DEPRECATED).
///
/// This checker uses hard-coded patterns for reflexivity and contradictions.
/// It is kept for backwards compatibility but should not be used for new code.
/// Use `PeanoGoalChecker` instead, which uses proper axiom-based goal checking.
#[deprecated(note = "Use PeanoGoalChecker for axiom-based goal checking")]
pub struct QuantifiedGoalChecker;

impl QuantifiedGoalChecker {
    pub fn new() -> Self {
        Self
    }
}

impl Default for QuantifiedGoalChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl GoalChecker<crate::syntax::PeanoLogicalExpression, BinaryTruth> for QuantifiedGoalChecker {
    fn check(&self, expr: &PeanoLogicalNode) -> Option<BinaryTruth> {
        match expr.value.as_ref() {
            // Handle compound logical expressions
            LogicalExpression::Compound { operator, operands, .. } => {
                match operator {
                    // Keep quantifiers intact - recursively check the body
                    ClassicalOperator::Forall | ClassicalOperator::Exists => {
                        if operands.len() == 1 {
                            self.check(&operands[0])
                        } else {
                            None
                        }
                    }
                    // Both operands must be true for AND
                    ClassicalOperator::And => {
                        let left = self.check(&operands[0])?;
                        let right = self.check(&operands[1])?;
                        if left == BinaryTruth::True && right == BinaryTruth::True {
                            Some(BinaryTruth::True)
                        } else {
                            None
                        }
                    }
                    // For implication, check if consequent is provable
                    ClassicalOperator::Implies => {
                        self.check(&operands[1])
                    }
                    // For iff, both directions must give same result
                    ClassicalOperator::Iff => {
                        let left = self.check(&operands[0])?;
                        let right = self.check(&operands[1])?;
                        if left == right {
                            Some(left)
                        } else {
                            None
                        }
                    }
                    _ => None,
                }
            }
            // Handle atomic domain expressions
            LogicalExpression::Atomic(domain) => check_atomic_goal(domain),
        }
    }
}

/// Check if an atomic domain expression is a goal.
///
/// This handles the base case of domain content (equalities or arithmetic).
/// NOTE: This uses hard-coded patterns. Use `PeanoGoalChecker` for proper
/// axiom-based goal checking.
fn check_atomic_goal(domain: &HashNode<PeanoContent>) -> Option<BinaryTruth> {
    match domain.value.as_ref() {
        PeanoContent::Equals(left, right) => {
            // Check for reflexive equality (x = x)
            if left.hash() == right.hash() {
                return Some(BinaryTruth::True);
            }
            // Check for contradiction (n = S(n))
            if is_successor_contradiction(left, right) || is_successor_contradiction(right, left) {
                return Some(BinaryTruth::False);
            }
            None
        }
        PeanoContent::Arithmetic(_) => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use corpus_core::base::nodes::NodeStorage;

    #[test]
    fn test_reflexive_equality_accepted() {
        let checker = AxiomPatternChecker::new();
        let store = NodeStorage::<PeanoContent>::new();
        let arith_store = NodeStorage::<ArithmeticExpression>::new();

        // Test 1: S(0) = S(0) should be accepted (reflexive)
        let s_zero = HashNode::from_store(
            ArithmeticExpression::Successor(
                HashNode::from_store(ArithmeticExpression::Number(0), &arith_store)
            ),
            &arith_store
        );
        let expr = HashNode::from_store(PeanoContent::Equals(s_zero.clone(), s_zero), &store);
        assert_eq!(checker.check(&expr), Some(BinaryTruth::True));

        // Test 2: 0 = 0 should be accepted (reflexive)
        let zero = HashNode::from_store(ArithmeticExpression::Number(0), &arith_store);
        let expr = HashNode::from_store(PeanoContent::Equals(zero.clone(), zero), &store);
        assert_eq!(checker.check(&expr), Some(BinaryTruth::True));
    }

    #[test]
    fn test_false_equalities_rejected() {
        let checker = AxiomPatternChecker::new();
        let store = NodeStorage::<PeanoContent>::new();
        let arith_store = NodeStorage::<ArithmeticExpression>::new();

        // Test 1: 0 = 1 should NOT be accepted (different numbers)
        let zero = HashNode::from_store(ArithmeticExpression::Number(0), &arith_store);
        let one = HashNode::from_store(ArithmeticExpression::Number(1), &arith_store);
        let expr = HashNode::from_store(PeanoContent::Equals(zero, one), &store);
        assert_eq!(checker.check(&expr), None); // Should NOT accept

        // Test 2: S(0) = 0 should be detected as contradiction (0 = S(0) pattern)
        let s_zero = HashNode::from_store(
            ArithmeticExpression::Successor(
                HashNode::from_store(ArithmeticExpression::Number(0), &arith_store)
            ),
            &arith_store
        );
        let zero = HashNode::from_store(ArithmeticExpression::Number(0), &arith_store);
        let expr = HashNode::from_store(PeanoContent::Equals(s_zero, zero), &store);
        assert_eq!(checker.check(&expr), Some(BinaryTruth::False)); // Should detect as contradiction
    }

    #[test]
    fn test_complex_false_equality_rejected() {
        let checker = AxiomPatternChecker::new();
        let store = NodeStorage::<PeanoContent>::new();
        let arith_store = NodeStorage::<ArithmeticExpression>::new();

        // Test: S(0) + S(0) = S(S(S(0))) should NOT be accepted
        // This is the original bug example - previously incorrectly returned True
        let zero = HashNode::from_store(ArithmeticExpression::Number(0), &arith_store);
        let s_zero = HashNode::from_store(
            ArithmeticExpression::Successor(zero.clone()),
            &arith_store
        );

        // S(0) + S(0)
        let left = HashNode::from_store(
            ArithmeticExpression::Add(s_zero.clone(), s_zero.clone()),
            &arith_store
        );

        // S(S(S(0)))
        let right = HashNode::from_store(
            ArithmeticExpression::Successor(
                HashNode::from_store(
                    ArithmeticExpression::Successor(
                        HashNode::from_store(
                            ArithmeticExpression::Successor(zero),
                            &arith_store
                        )
                    ),
                    &arith_store
                )
            ),
            &arith_store
        );

        let expr = HashNode::from_store(PeanoContent::Equals(left, right), &store);
        assert_eq!(checker.check(&expr), None); // Should NOT accept
    }

    #[test]
    fn test_contradiction_detected() {
        let checker = AxiomPatternChecker::new();
        let store = NodeStorage::<PeanoContent>::new();
        let arith_store = NodeStorage::<ArithmeticExpression>::new();

        // Test 1: 0 = S(0) should be detected as contradiction (return Some(False))
        let zero = HashNode::from_store(ArithmeticExpression::Number(0), &arith_store);
        let s_zero = HashNode::from_store(
            ArithmeticExpression::Successor(zero.clone()),
            &arith_store
        );
        let expr = HashNode::from_store(PeanoContent::Equals(zero, s_zero), &store);
        assert_eq!(checker.check(&expr), Some(BinaryTruth::False));

        // Test 2: S(0) = S(S(0)) should be detected as contradiction
        let zero = HashNode::from_store(ArithmeticExpression::Number(0), &arith_store);
        let s_zero = HashNode::from_store(
            ArithmeticExpression::Successor(zero.clone()),
            &arith_store
        );
        let ss_zero = HashNode::from_store(
            ArithmeticExpression::Successor(s_zero.clone()),
            &arith_store
        );
        let expr = HashNode::from_store(PeanoContent::Equals(s_zero, ss_zero), &store);
        assert_eq!(checker.check(&expr), Some(BinaryTruth::False));

        // Test 3: S(0) = 0 should also be detected (reverse order)
        let zero = HashNode::from_store(ArithmeticExpression::Number(0), &arith_store);
        let s_zero = HashNode::from_store(
            ArithmeticExpression::Successor(zero.clone()),
            &arith_store
        );
        let expr = HashNode::from_store(PeanoContent::Equals(s_zero, zero), &store);
        assert_eq!(checker.check(&expr), Some(BinaryTruth::False));
    }

    #[test]
    fn test_non_contradiction_returns_none() {
        let checker = AxiomPatternChecker::new();
        let store = NodeStorage::<PeanoContent>::new();
        let arith_store = NodeStorage::<ArithmeticExpression>::new();

        // 0 = S(S(0)) should NOT be a contradiction (0 is not S(S(0)))
        let zero = HashNode::from_store(ArithmeticExpression::Number(0), &arith_store);
        let ss_zero = HashNode::from_store(
            ArithmeticExpression::Successor(
                HashNode::from_store(
                    ArithmeticExpression::Successor(zero.clone()),
                    &arith_store
                )
            ),
            &arith_store
        );
        let expr = HashNode::from_store(PeanoContent::Equals(zero, ss_zero), &store);
        // Should return None, not Some(False)
        assert_eq!(checker.check(&expr), None);
    }
}
