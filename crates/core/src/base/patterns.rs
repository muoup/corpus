//! Pattern decomposition for converting expressions to patterns.
//!
//! This trait provides a generic interface for converting logical expressions
//! (potentially with quantifiers) into pattern matching structures.
//!
//! Note: This module now works with the trait-based `LogicalExpression`
//! abstraction. Each logical system provides their own decomposer implementation.

use crate::expression::LogicalExpression;
use crate::nodes::{HashNode, NodeStorage};
use crate::rewriting::Pattern;
use std::collections::HashSet;

/// Trait for converting expressions with quantifiers into patterns.
///
/// This trait handles the conversion from logical expressions that may contain
/// quantifiers (∀, ∃) into pattern matching structures. Quantifiers introduce
/// variable bindings that need to be tracked during the conversion.
///
/// This trait is now generic over any type that implements `LogicalExpression`,
/// allowing each logical system to provide their own decomposer implementation.
///
/// # Type Parameters
///
/// * `Expr` - The logical expression type (must implement `LogicalExpression`)
///
/// # Example
///
/// ```rust,ignore
/// use corpus_core::patterns::PatternDecomposer;
/// use corpus_classical_logic::ClassicalLogicalExpression;
///
/// struct PeanoPatternDecomposer;
///
/// impl PatternDecomposer<ClassicalLogicalExpression<BinaryTruth, PeanoContent, ClassicalOperator>>
///     for PeanoPatternDecomposer
/// {
///     fn expression_to_pattern(&self, expr: &HashNode<Expr>, store: &...) -> Pattern<Expr> {
///         // Convert expression to pattern, handling quantifiers
///         todo!()
///     }
/// }
/// ```
pub trait PatternDecomposer<Expr: LogicalExpression> {
    /// Convert a logical expression to a pattern, handling quantifiers.
    ///
    /// This function walks the expression tree and converts it to a pattern.
    /// When it encounters a quantifier (Forall, Exists), it introduces a new
    /// variable binding and adjusts the variable indices in the body accordingly.
    ///
    /// # Arguments
    ///
    /// * `expr` - The logical expression to convert
    /// * `store` - The node storage for creating new nodes if needed
    ///
    /// # Returns
    ///
    /// A `Pattern` that represents the structure of the expression with
    /// variables in place of quantified bindings.
    fn expression_to_pattern(
        &self,
        expr: &HashNode<Expr>,
        store: &NodeStorage<Expr>,
    ) -> Pattern<Expr>;

    /// Extract variable bindings from quantified expressions.
    ///
    /// Returns the list of variable indices that are bound by quantifiers
    /// in the given expression, in the order they appear (outermost first).
    ///
    /// For example, `∀x.∃y.(x = y)` would return `[0, 1]` (x is bound at depth 0, y at depth 1).
    fn extract_quantified_variables(
        &self,
        expr: &HashNode<Expr>,
    ) -> Vec<u32>;

    /// Get the binding depth of a variable occurrence.
    ///
    /// Returns `Some(depth)` indicating how many quantifiers scope over this
    /// variable, or `None` if the variable is not bound in this expression.
    ///
    /// # Example
    ///
    /// In `∀x.∀y.(x = y)`:
    /// - The first occurrence of `x` has depth 0 (bound by the outer quantifier)
    /// - The occurrence of `y` has depth 1 (bound by the inner quantifier)
    fn variable_depth(
        &self,
        expr: &HashNode<Expr>,
        var_index: u32,
    ) -> Option<u32>;

    /// Get all free variables in an expression.
    ///
    /// Free variables are those that are not bound by any quantifier in the
    /// given expression. Returns a set of variable indices.
    ///
    /// The default implementation assumes all variables are bound.
    /// Subtraits should override this for their specific representation.
    fn free_variables(
        &self,
        _expr: &HashNode<Expr>,
    ) -> HashSet<u32> {
        // Default implementation assumes all variables are bound
        // Subtraits should override this for their specific representation
        HashSet::new()
    }
}
