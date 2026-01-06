//! Proof context for tracking quantifier scope during proof search.
//!
//! This module provides types for tracking the current proof context,
//! particularly the scope of quantifiers (forall, exists) which affects
//! which rewrite rules can be applied.
//!
//! Note: This module now works with the trait-based `LogicalExpression`
//! abstraction. Each logical system provides their own context extractor.

use crate::base::expression::LogicalExpression;
use crate::base::nodes::HashNode;
use std::clone::Clone;
use std::fmt::Debug;

/// Information about a quantifier in the current scope.
#[derive(Debug, Clone, PartialEq)]
pub struct QuantifierInfo {
    /// The quantifier operator (forall or exists)
    pub operator: QuantifierOperator,
    /// The variable bound by this quantifier
    pub variable: String,
    /// The nesting depth of this quantifier
    pub depth: usize,
}

/// Type of quantifier operator.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuantifierOperator {
    Forall,
    Exists,
}

/// Tracks the current proof context for conditional rule application.
///
/// The proof context maintains a stack of active quantifiers, allowing
/// axioms to check whether they apply in the current scope.
#[derive(Debug, Clone, PartialEq)]
pub struct ProofContext {
    /// Stack of active quantifiers (e.g., Forall(x), Exists(y))
    quantifier_stack: Vec<QuantifierInfo>,
}

impl ProofContext {
    /// Create a new empty proof context.
    pub fn new() -> Self {
        Self {
            quantifier_stack: Vec::new(),
        }
    }

    /// Enter a quantified scope.
    pub fn push_quantifier(&mut self, operator: QuantifierOperator, variable: impl Into<String>) {
        self.quantifier_stack.push(QuantifierInfo {
            operator,
            variable: variable.into(),
            depth: self.quantifier_stack.len(),
        });
    }

    /// Exit the current quantified scope.
    pub fn pop_quantifier(&mut self) -> Option<QuantifierInfo> {
        self.quantifier_stack.pop()
    }

    /// Get the current quantifier depth.
    pub fn depth(&self) -> usize {
        self.quantifier_stack.len()
    }

    /// Check if a variable with the given name is bound in the current scope.
    pub fn is_bound(&self, variable: &str) -> bool {
        self.quantifier_stack
            .iter()
            .any(|q| q.variable == variable)
    }

    /// Get all bound variables in the current scope.
    pub fn bound_variables(&self) -> Vec<&str> {
        self.quantifier_stack
            .iter()
            .map(|q| q.variable.as_str())
            .collect()
    }

    /// Check if we're currently inside an Exists quantifier.
    pub fn in_exists_scope(&self) -> bool {
        self.quantifier_stack
            .iter()
            .any(|q| q.operator == QuantifierOperator::Exists)
    }

    /// Check if we're currently inside a Forall quantifier.
    pub fn in_forall_scope(&self) -> bool {
        self.quantifier_stack
            .iter()
            .any(|q| q.operator == QuantifierOperator::Forall)
    }

    /// Check if a variable is existentially quantified.
    pub fn is_existentially_bound(&self, variable: &str) -> bool {
        self.quantifier_stack
            .iter()
            .any(|q| q.variable == variable && q.operator == QuantifierOperator::Exists)
    }

    /// Check if a variable is universally quantified.
    pub fn is_universally_bound(&self, variable: &str) -> bool {
        self.quantifier_stack
            .iter()
            .any(|q| q.variable == variable && q.operator == QuantifierOperator::Forall)
    }
}

impl Default for ProofContext {
    fn default() -> Self {
        Self::new()
    }
}

/// Extension trait for extracting proof context from expressions.
///
/// This trait is now generic over any type that implements `LogicalExpression`,
/// allowing each logical system to provide their own context extraction logic.
///
/// # Type Parameters
///
/// * `Expr` - The logical expression type (must implement `LogicalExpression`)
pub trait ProofContextExtractor<Expr: LogicalExpression> {
    /// Extract the proof context from an expression by analyzing its quantifiers.
    fn extract_context(&self) -> ProofContext;
}

/// Default implementation of ProofContextExtractor for HashNode<Expr>.
///
/// This provides a basic implementation that walks the expression tree.
/// Each logical system may provide a more specialized implementation
/// if needed for their specific operator types.
impl<Expr: LogicalExpression> ProofContextExtractor<Expr> for HashNode<Expr> {
    fn extract_context(&self) -> ProofContext {
        let mut context = ProofContext::new();
        // Note: The actual context extraction requires knowledge of
        // quantifier operators which is specific to each logical system.
        // This is a placeholder implementation.
        context
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_context() {
        let ctx = ProofContext::new();
        assert_eq!(ctx.depth(), 0);
        assert!(!ctx.is_bound("x"));
        assert!(!ctx.in_exists_scope());
        assert!(!ctx.in_forall_scope());
    }

    #[test]
    fn test_push_pop_quantifier() {
        let mut ctx = ProofContext::new();
        ctx.push_quantifier(QuantifierOperator::Forall, "x");
        assert_eq!(ctx.depth(), 1);
        assert!(ctx.is_bound("x"));
        assert!(ctx.is_universally_bound("x"));
        assert!(!ctx.is_existentially_bound("x"));

        ctx.push_quantifier(QuantifierOperator::Exists, "y");
        assert_eq!(ctx.depth(), 2);
        assert!(ctx.is_bound("y"));
        assert!(ctx.is_existentially_bound("y"));

        ctx.pop_quantifier();
        assert_eq!(ctx.depth(), 1);
        assert!(!ctx.is_bound("y"));

        ctx.pop_quantifier();
        assert_eq!(ctx.depth(), 0);
    }

    #[test]
    fn test_bound_variables() {
        let mut ctx = ProofContext::new();
        ctx.push_quantifier(QuantifierOperator::Forall, "x");
        ctx.push_quantifier(QuantifierOperator::Exists, "y");

        let vars = ctx.bound_variables();
        assert_eq!(vars.len(), 2);
        assert!(vars.contains(&"x"));
        assert!(vars.contains(&"y"));
    }
}
