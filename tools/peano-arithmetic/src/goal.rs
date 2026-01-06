//! Goal checking for Peano Arithmetic proofs.
//!
//! This module provides `PeanoGoalChecker`, which wraps the generic
//! `AxiomGoalChecker` from the classical-logic crate. It determines if a
//! theorem has been proven by checking if it matches any PA axiom.

use corpus_classical_logic::{BinaryTruth, ClassicalLogicalExpression, ClassicalOperator, AxiomGoalChecker};
use corpus_core::proving::GoalChecker;
use crate::syntax::{PeanoContent, PeanoLogicalExpression, PeanoLogicalNode};
use crate::axioms::peano_arithmetic_axioms_with_goals;

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
    pub fn with_axioms(axioms: Vec<corpus_core::base::axioms::NamedAxiom<PeanoLogicalExpression>>) -> Self {
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

impl GoalChecker<PeanoLogicalExpression, BinaryTruth> for PeanoGoalChecker {
    fn check(&self, expr: &PeanoLogicalNode) -> Option<BinaryTruth> {
        self.inner.check(expr)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use corpus_core::base::nodes::{HashNode, NodeStorage};
    use corpus_classical_logic::ClassicalLogicalExpression;

    #[test]
    fn test_reflexive_equality_accepted() {
        let checker = PeanoGoalChecker::new();
        let store = NodeStorage::<crate::syntax::PeanoLogicalExpression>::new();
        let arith_store = NodeStorage::<crate::syntax::ArithmeticExpression>::new();

        // Test: /0 = /0 should match the reflexivity axiom pattern (∀x. x = x)
        let var = HashNode::from_store(crate::syntax::ArithmeticExpression::DeBruijn(0), &arith_store);
        let content = HashNode::from_store(
            crate::syntax::PeanoContent::Equals(var.clone(), var),
            &NodeStorage::new()
        );
        let expr = HashNode::from_store(ClassicalLogicalExpression::atomic(content), &store);
        assert_eq!(checker.check(&expr), Some(BinaryTruth::True));
    }

    #[test]
    fn test_contradiction_detected() {
        let checker = PeanoGoalChecker::new();
        let store = NodeStorage::<crate::syntax::PeanoLogicalExpression>::new();
        let arith_store = NodeStorage::<crate::syntax::ArithmeticExpression>::new();

        // Test: /0 = S(/0) should match the negated injectivity axiom pattern (∀x. ¬(x = S(x)))
        let var = HashNode::from_store(crate::syntax::ArithmeticExpression::DeBruijn(0), &arith_store);
        let s_var = HashNode::from_store(
            crate::syntax::ArithmeticExpression::Successor(var.clone()),
            &arith_store
        );
        let content = HashNode::from_store(
            crate::syntax::PeanoContent::Equals(var, s_var),
            &NodeStorage::new()
        );
        let expr = HashNode::from_store(ClassicalLogicalExpression::atomic(content), &store);
        assert_eq!(checker.check(&expr), Some(BinaryTruth::False));
    }
}
