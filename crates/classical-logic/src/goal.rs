//! Generic axiom-based goal checking for classical logic systems.
//!
//! This module provides a generic goal checker that determines whether a theorem
//! has been proven by checking if it matches any axiom. Since axioms are true
//! by definition:
//! - Direct match: theorem matches axiom → True
//! - Negation match: theorem matches ¬(axiom_body) → False

use corpus_core::base::axioms::NamedAxiom;
use corpus_core::base::nodes::HashNodeInner;
use corpus_core::expression::{DomainContent, LogicalExpression};
use corpus_core::logic::LogicalOperator;
use corpus_core::nodes::HashNode;
use corpus_core::proving::GoalChecker;
use corpus_core::truth::TruthValue;
use std::sync::Arc;

/// Generic goal checker that matches theorems against axioms.
///
/// This checker determines if a proof is complete by checking whether the
/// current theorem state matches any known axiom pattern:
/// - Direct match → Theorem is provably true (matches an axiom)
/// - Negation match → Theorem is provably false (contradicts an axiom)
///
/// # Type Parameters
///
/// * `T` - Truth value type (e.g., `BinaryTruth`)
/// * `D` - Domain content type (e.g., `PeanoContent`)
/// * `Op` - Logical operator type (e.g., `ClassicalOperator`)
pub struct AxiomGoalChecker<T, D, Op>
where
    T: TruthValue + HashNodeInner,
    D: DomainContent<T> + Clone + std::fmt::Debug,
    Op: LogicalOperator<T> + HashNodeInner,
{
    axioms: Arc<Vec<NamedAxiom<T, D, Op>>>,
}

impl<T, D, Op> AxiomGoalChecker<T, D, Op>
where
    T: TruthValue + HashNodeInner,
    D: DomainContent<T> + Clone + std::fmt::Debug,
    Op: LogicalOperator<T> + HashNodeInner,
    Op::Symbol: AsRef<str>,
{
    /// Create a new axiom goal checker with the given axioms.
    pub fn new(axioms: Vec<NamedAxiom<T, D, Op>>) -> Self {
        Self {
            axioms: Arc::new(axioms),
        }
    }

    /// Get the axioms used by this checker.
    pub fn axioms(&self) -> &[NamedAxiom<T, D, Op>] {
        &self.axioms
    }
}

impl<T, D, Op> GoalChecker<LogicalExpression<T, D, Op>, T> for AxiomGoalChecker<T, D, Op>
where
    T: TruthValue + PartialEq + HashNodeInner,
    D: DomainContent<T> + Clone + std::fmt::Debug,
    Op: LogicalOperator<T> + Clone + HashNodeInner,
    Op::Symbol: AsRef<str>,
{
    fn check(&self, theorem: &HashNode<LogicalExpression<T, D, Op>>) -> Option<T> {
        // Check each axiom for a match
        for axiom in &*self.axioms {
            if let Some(result) = check_axiom_match(theorem, axiom) {
                return Some(result);
            }
        }
        None
    }
}

/// Check if a theorem matches an axiom pattern.
///
/// Returns:
/// - `Some(True)` if theorem directly matches the axiom
/// - `Some(False)` if theorem matches negation of axiom body
/// - `None` if no match
fn check_axiom_match<T, D, Op>(
    theorem: &HashNode<LogicalExpression<T, D, Op>>,
    axiom: &NamedAxiom<T, D, Op>,
) -> Option<T>
where
    T: TruthValue + PartialEq + HashNodeInner,
    D: DomainContent<T> + Clone + std::fmt::Debug,
    Op: LogicalOperator<T> + Clone + HashNodeInner,
    Op::Symbol: AsRef<str>,
{
    use LogicalExpression::*;

    match axiom.expression.value.as_ref() {
        // Handle quantified axioms: ∀x. P(x) or ∃x. P(x)
        Compound { operator, operands, .. } if is_quantifier::<T, D, Op>(operator) => {
            // Strip quantifiers and check body match
            if let Some(axiom_body) = operands.first() {
                // Check if theorem matches the axiom body (ignoring quantifiers)
                if expressions_match(theorem, axiom_body) {
                    return Some(T::from_bool(true));
                }
                // Check if theorem is negation of axiom body
                if let Some(negated) = extract_negation(theorem) {
                    if expressions_match(&negated, axiom_body) {
                        return Some(T::from_bool(false));
                    }
                }
            }
        }
        // Handle simple axioms without quantifiers
        _ => {
            // Direct match
            if expressions_match(theorem, &axiom.expression) {
                return Some(T::from_bool(true));
            }
            // Negation match
            if let Some(negated) = extract_negation(theorem) {
                if expressions_match(&negated, &axiom.expression) {
                    return Some(T::from_bool(false));
                }
            }
        }
    }
    None
}

/// Check if an operator is a quantifier.
fn is_quantifier<T, D, Op>(op: &Op) -> bool
where
    T: TruthValue,
    D: DomainContent<T>,
    Op: LogicalOperator<T>,
    Op::Symbol: AsRef<str>,
{
    let symbol = op.symbol();
    (symbol.as_ref() == "∀") || (symbol.as_ref() == "∃")
}

/// Check if two expressions structurally match for goal checking purposes.
///
/// For goal checking, we care about whether the FORM matches, not specific
/// variable bindings. We use hash-based structural equality as a starting point.
fn expressions_match<T, D, Op>(
    a: &HashNode<LogicalExpression<T, D, Op>>,
    b: &HashNode<LogicalExpression<T, D, Op>>,
) -> bool
where
    T: TruthValue + HashNodeInner,
    D: DomainContent<T> + HashNodeInner,
    Op: LogicalOperator<T> + HashNodeInner,
{
    // Hash-based structural matching
    // TODO: This could be refined to handle variable bindings more carefully
    // For now, if two expressions have the same hash, they're structurally identical
    a.hash() == b.hash()
}

/// Extract the body of a negation: ¬P → P
fn extract_negation<T, D, Op>(
    expr: &HashNode<LogicalExpression<T, D, Op>>,
) -> Option<HashNode<LogicalExpression<T, D, Op>>>
where
    T: TruthValue + HashNodeInner,
    D: DomainContent<T> + HashNodeInner,
    Op: LogicalOperator<T> + HashNodeInner,
    Op::Symbol: AsRef<str>,
{
    let symbol = expr.value.as_ref().operator()?.symbol();
    if symbol.as_ref() == "¬" {
        if let LogicalExpression::Compound { operands, .. } = expr.value.as_ref() {
            return operands.first().cloned();
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::BinaryTruth;
    use corpus_core::base::nodes::HashNodeInner;
    use corpus_core::expression::DomainContent;

    #[test]
    fn test_extract_negation() {
        // TODO: Add tests for negation extraction once we have proper test setup
    }

    #[test]
    fn test_is_quantifier() {
        use crate::ClassicalOperator;

        // Create a minimal DomainContent implementation for testing
        struct TestDomain;
        impl DomainContent<BinaryTruth> for TestDomain {
            type Operator = ClassicalOperator;
        }
        impl HashNodeInner for TestDomain {
            fn hash(&self) -> u64 {
                0
            }
            fn size(&self) -> u64 {
                1
            }
        }

        assert!(is_quantifier::<BinaryTruth, TestDomain, ClassicalOperator>(&ClassicalOperator::Forall));
        assert!(is_quantifier::<BinaryTruth, TestDomain, ClassicalOperator>(&ClassicalOperator::Exists));
    }
}
