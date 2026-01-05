//! Goal checking for Peano Arithmetic proofs.
//!
//! This module provides goal checking implementations for the PA prover,
//! specifically for checking when an equality is reflexive (x = x).

use corpus_classical_logic::BinaryTruth;
use corpus_core::proving::GoalChecker;
use corpus_core::base::nodes::HashNode;
use crate::syntax::{PeanoContent, ArithmeticExpression};

/// Goal checker for Peano Arithmetic equalities.
///
/// For PA equalities, the goal is to check for the **reflexive property** (x = x).
/// When both sides of an equality have the same hash, the equality is a tautology,
/// which means we've successfully proven the original statement by rewriting it
/// to a trivially true form.
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
        // Check if the equality is reflexive (x = x)
        check_reflexive_equality(expr)
    }
}

/// Check if the equality is reflexive (x = x), which is the logical basis of equality truth.
///
/// When both sides of an equality have the same hash, they are structurally identical,
/// meaning we've proven the original statement by rewriting it to a tautology.
fn check_reflexive_equality(expr: &HashNode<PeanoContent>) -> Option<BinaryTruth> {
    if let PeanoContent::Equals(left, right) = expr.value.as_ref() {
        // Check if left and right sides have the same hash (are structurally equal)
        if left.hash() == right.hash() {
            return Some(BinaryTruth::True);
        }
    }
    None
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

        // Test 2: S(0) = 0 should NOT be accepted
        let s_zero = HashNode::from_store(
            ArithmeticExpression::Successor(
                HashNode::from_store(ArithmeticExpression::Number(0), &arith_store)
            ),
            &arith_store
        );
        let zero = HashNode::from_store(ArithmeticExpression::Number(0), &arith_store);
        let expr = HashNode::from_store(PeanoContent::Equals(s_zero, zero), &store);
        assert_eq!(checker.check(&expr), None); // Should NOT accept
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
}
