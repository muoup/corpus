//! Generic axiom-based goal checking for classical logic systems.
//!
//! This module provides a generic goal checker that determines whether a theorem
//! has been proven by checking if it matches any axiom. Since axioms are true
//! by definition:
//! - Direct match: theorem matches axiom → True
//! - Negation match: theorem matches ¬(axiom_body) → False

use crate::expression::{ClassicalLogicalExpression, DomainContent};
use crate::BinaryTruth;
use corpus_core::nodes::HashNode;
use corpus_core::proving::GoalChecker;

/// Tracks the current proof context for conditional rule application.
///
/// The proof context maintains a stack of active quantifiers, allowing
/// axioms to check whether they apply in the current scope.
#[derive(Debug, Clone, PartialEq)]
pub struct ProofContext {
    /// Stack of active quantifiers (e.g., Forall(x), Exists(y))
    quantifier_stack: Vec<QuantifierOperator>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum QuantifierOperator {
    Forall,
    Exists,
}

impl ProofContext {
    /// Create a new empty proof context.
    pub fn new() -> Self {
        Self {
            quantifier_stack: Vec::new(),
        }
    }

    /// Enter a quantified scope.
    pub fn push_quantifier(&mut self, operator: QuantifierOperator) {
        self.quantifier_stack.push(operator);
    }

    /// Exit the current quantified scope.
    pub fn pop_quantifier(&mut self) -> Option<QuantifierOperator> {
        self.quantifier_stack.pop()
    }

    /// Get the current quantifier depth.
    pub fn depth(&self) -> usize {
        self.quantifier_stack.len()
    }

    /// Check if a variable with the given name is bound in the current scope.
    pub fn get(&self, index: u8) -> Option<&QuantifierOperator> {
        self.quantifier_stack
            .get(self.quantifier_stack.len() - 1 - (index as usize))
    }

    /// Check if we're currently inside an Exists quantifier.
    pub fn in_exists_scope(&self) -> bool {
        self.quantifier_stack
            .iter()
            .any(|q| matches!(q, QuantifierOperator::Exists))
    }

    /// Check if we're currently inside a Forall quantifier.
    pub fn in_forall_scope(&self) -> bool {
        self.quantifier_stack
            .iter()
            .any(|q| matches!(q, QuantifierOperator::Forall))
    }

    /// Check if a variable is existentially quantified.
    pub fn is_existentially_bound(&self, variable: u8) -> bool {
        self.get(variable)
            .map_or(false, |q| *q == QuantifierOperator::Exists)
    }

    /// Check if a variable is universally quantified.
    pub fn is_universally_bound(&self, variable: u8) -> bool {
        self.get(variable)
            .map_or(false, |q| *q == QuantifierOperator::Forall)
    }
}

impl Default for ProofContext {
    fn default() -> Self {
        Self::new()
    }
}

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
pub struct ClassicalTruthChecker;

impl ClassicalTruthChecker {
    /// Create a new axiom goal checker with the given axioms.
    pub fn new() -> Self {
        Self
    }
}

impl<'a, D> GoalChecker<ClassicalLogicalExpression<D>, BinaryTruth> for ClassicalTruthChecker
where
    D: DomainContent + Clone + PartialEq,
{
    fn check(&self, theorem: &HashNode<ClassicalLogicalExpression<D>>) -> Option<BinaryTruth> {
        match theorem.value.as_ref() {
            // One main observation of classical logic is maybe best summarized as:
            // "Things which are true, are true; and things which are false, are false."
            ClassicalLogicalExpression::BooleanConstant(truth) => Some(*truth),
            ClassicalLogicalExpression::ForAll(inner) => self.check(inner),

            // Otherwise who's to say
            _ => None,
        }
    }
}
