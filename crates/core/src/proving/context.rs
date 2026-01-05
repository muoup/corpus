//! Proof context for tracking quantifier scope during proof search.
//!
//! This module provides types for tracking the current proof context,
//! particularly the scope of quantifiers (forall, exists) which affects
//! which rewrite rules can be applied.

use crate::base::expression::{DomainContent, LogicalExpression};
use crate::base::nodes::{HashNode, HashNodeInner};
use crate::logic::LogicalOperator;
use crate::truth::TruthValue;
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
pub trait ProofContextExtractor<T: TruthValue, D: DomainContent<T>, Op: LogicalOperator<T, Symbol = &'static str> + HashNodeInner> {
    /// Extract the proof context from an expression by analyzing its quantifiers.
    fn extract_context(&self) -> ProofContext;
}

impl<T: TruthValue, D: DomainContent<T>, Op: LogicalOperator<T, Symbol = &'static str> + HashNodeInner> ProofContextExtractor<T, D, Op>
    for HashNode<LogicalExpression<T, D, Op>>
where
    T: HashNodeInner + Clone,
    D: HashNodeInner + Clone,
    Op: Clone,
{
    fn extract_context(&self) -> ProofContext {
        let mut context = ProofContext::new();
        extract_context_recursive(self, &mut context);
        context
    }
}

fn extract_context_recursive<T: TruthValue, D: DomainContent<T>, Op: LogicalOperator<T, Symbol = &'static str> + HashNodeInner>(
    expr: &HashNode<LogicalExpression<T, D, Op>>,
    context: &mut ProofContext,
) where
    T: HashNodeInner + Clone,
    D: HashNodeInner + Clone,
    Op: Clone,
{
    match expr.value.as_ref() {
        LogicalExpression::Atomic(_) => {
            // No quantifiers in atomic expressions
        }
        LogicalExpression::Compound { operator, operands, .. } => {
            // Check if this is a quantifier
            let is_forall = operator.symbol() == "∀";
            let is_exists = operator.symbol() == "∃";

            if (is_forall || is_exists) && !operands.is_empty() {
                // Extract variable name from the quantifier
                if let Some(var_name) = extract_variable_name(&operands[0]) {
                    let quantifier_op = if is_forall {
                        QuantifierOperator::Forall
                    } else {
                        QuantifierOperator::Exists
                    };
                    context.push_quantifier(quantifier_op, var_name);
                }
            }

            // Recursively process operands
            for operand in operands {
                extract_context_recursive(operand, context);
            }

            // Pop the quantifier after processing its scope
            if is_forall || is_exists {
                context.pop_quantifier();
            }
        }
    }
}

/// Try to extract a variable name from an expression.
/// This is a simplified version - a real implementation would need
/// to properly handle variable expressions.
fn extract_variable_name<T: TruthValue, D: DomainContent<T>, Op: LogicalOperator<T, Symbol = &'static str> + HashNodeInner>(
    expr: &HashNode<LogicalExpression<T, D, Op>>,
) -> Option<String>
where
    T: HashNodeInner + Clone,
    D: HashNodeInner + Clone,
    Op: Clone,
{
    // For now, return a placeholder. A real implementation would
    // check if the expression is a variable and extract its name.
    match expr.value.as_ref() {
        LogicalExpression::Atomic(domain) => {
            // Try to get variable name from domain content
            Some(format!("var_{}", domain.hash()))
        }
        LogicalExpression::Compound { .. } => None,
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
