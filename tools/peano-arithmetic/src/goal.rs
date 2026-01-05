//! Goal checking for Peano Arithmetic proofs.
//!
//! This module provides goal checking implementations for the PA prover,
//! specifically for checking when an equality is reflexive (x = x) or
//! contradictory (n = S(n)).

use corpus_classical_logic::BinaryTruth;
use corpus_core::proving::GoalChecker;
use corpus_core::base::nodes::HashNode;
use crate::syntax::{PeanoContent, ArithmeticExpression};

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
    let PeanoContent::Equals(left, right) = expr.value.as_ref();
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
    let PeanoContent::Equals(left, right) = expr.value.as_ref();

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
