//! Utilities for working with quantified expressions in Peano Arithmetic.
//!
//! This module provides helper functions for manipulating ClassicalLogicalExpressions
//! that contain quantifiers (∀, ∃), ensuring that quantifier structure is
//! preserved during rewrite operations.

use corpus_classical_logic::{ClassicalLogicalExpression, ClassicalOperator};
use corpus_core::nodes::{HashNode, NodeStorage};

use crate::syntax::PeanoLogicalExpression;
use crate::syntax::PeanoLogicalNode;

/// Apply a function to an expression while preserving quantifier structure.
///
/// This function walks through quantifiers (∀, ∃) and applies the given
/// function to the innermost non-quantified expression. After applying the
/// function, it re-wraps the result in the original quantifier structure.
///
/// # Arguments
///
/// * `expr` - The expression to process
/// * `store` - The node storage for creating new nodes
/// * `f` - A function that takes a non-quantified expression and returns
///         an optional transformed expression
///
/// # Examples
///
/// ```ignore
/// // Apply a rewrite rule under a quantifier
/// let result = apply_under_quantifiers(
///     &forall_x_equals_x,
///     &store,
///     |expr| rule.apply(expr, &store)
/// );
/// ```
pub fn apply_under_quantifiers<F>(
    expr: &PeanoLogicalNode,
    store: &NodeStorage<PeanoLogicalExpression>,
    f: F,
) -> Option<PeanoLogicalNode>
where
    F: Fn(&PeanoLogicalNode) -> Option<PeanoLogicalNode>,
{
    match expr.value.as_ref() {
        // For quantified expressions, recurse into the body
        ClassicalLogicalExpression::Compound { operator, operands, .. }
            if operator.symbol() == "∀" || operator.symbol() == "∃" =>
        {
            if let Some(body) = operands.first() {
                // Recursively apply to the body
                let new_body = apply_under_quantifiers(body, store, f)?;

                // Re-wrap with the quantifier
                let new_expr = ClassicalLogicalExpression::compound(
                    operator.clone(),
                    vec![new_body],
                );
                Some(HashNode::from_store(new_expr, store))
            } else {
                None
            }
        }
        // Base case: apply the function
        _ => f(expr),
    }
}

/// Re-wrap an expression in a quantifier.
///
/// # Arguments
///
/// * `operator` - The quantifier operator (∀ or ∃)
/// * `body` - The expression to wrap
/// * `store` - The node storage for creating new nodes
pub fn wrap_in_quantifier(
    operator: ClassicalOperator,
    body: PeanoLogicalNode,
    store: &NodeStorage<PeanoLogicalExpression>,
) -> PeanoLogicalNode {
    let expr = ClassicalLogicalExpression::compound(
        operator,
        vec![body],
    );
    HashNode::from_store(expr, store)
}

/// Count the number of quantifiers at the outermost level of an expression.
///
/// # Examples
///
/// ```ignore
/// // ∀x.∀y. P(x,y) returns 2
/// // ∀x. (P(x) ∧ Q(x)) returns 1
/// // P(x) returns 0
/// ```
pub fn count_outer_quantifiers(expr: &PeanoLogicalNode) -> usize {
    match expr.value.as_ref() {
        ClassicalLogicalExpression::Compound { operator, operands, .. }
            if operator.symbol() == "∀" || operator.symbol() == "∃" =>
        {
            if let Some(body) = operands.first() {
                1 + count_outer_quantifiers(body)
            } else {
                0
            }
        }
        _ => 0,
    }
}

/// Strip all outer quantifiers from an expression, returning the body and
/// a vector of quantifier operators (outermost first).
///
/// # Examples
///
/// ```ignore
/// // ∀x.∀y. P(x,y) returns (P(x,y), [∀, ∀])
/// ```
pub fn strip_quantifiers(
    expr: &PeanoLogicalNode,
) -> (Option<PeanoLogicalNode>, Vec<ClassicalOperator>) {
    let mut quantifiers = Vec::new();
    let mut current = expr;

    loop {
        match current.value.as_ref() {
            ClassicalLogicalExpression::Compound { operator, operands, .. }
                if operator.symbol() == "∀" || operator.symbol() == "∃" =>
            {
                quantifiers.push(operator.clone());
                if let Some(body) = operands.first() {
                    current = body;
                } else {
                    break;
                }
            }
            _ => break,
        }
    }

    (Some(current.clone()), quantifiers)
}

/// Re-wrap an expression with the given quantifiers (outermost first).
pub fn rewrap_with_quantifiers(
    body: PeanoLogicalNode,
    quantifiers: &[ClassicalOperator],
    store: &NodeStorage<PeanoLogicalExpression>,
) -> PeanoLogicalNode {
    let mut result = body;
    for op in quantifiers.iter().rev() {
        result = wrap_in_quantifier(op.clone(), result, store);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::syntax::{PeanoContent, ArithmeticExpression};
    use corpus_core::nodes::NodeStorage;
    use corpus_classical_logic::BinaryTruth;

    #[test]
    fn test_count_outer_quantifiers() {
        let store = NodeStorage::<PeanoLogicalExpression>::new();

        // Create a simple atomic expression
        let arith_store = NodeStorage::<ArithmeticExpression>::new();
        let zero = HashNode::from_store(ArithmeticExpression::Number(0), &arith_store);
        let content = PeanoContent::Equals(zero.clone(), zero);
        let atomic = content.to_logical(&store);

        assert_eq!(count_outer_quantifiers(&atomic), 0);

        // TODO: Add tests for actual quantified expressions once
        // we have a way to construct them easily
    }

    #[test]
    fn test_strip_and_rewrap_quantifiers() {
        // TODO: Add tests once we can construct quantified expressions
    }
}
