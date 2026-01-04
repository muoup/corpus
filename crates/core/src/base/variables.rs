//! Variable extraction trait for logical expressions.
//!
//! This trait provides a generic interface for extracting variable bindings
//! from expressions. Different domains may represent variables differently
//! (e.g., de Bruijn indices, named variables, etc.).

use crate::nodes::HashNode;
use std::collections::HashSet;

/// Trait for types that can extract variable bindings from expressions.
///
/// Domains implement this trait to provide their own variable extraction logic.
/// For example, Peano Arithmetic uses de Bruijn indices (`DeBruijn(u32)` nodes),
/// while other domains might use named variables or other representations.
///
/// # Example
///
/// ```rust,ignore
/// use corpus_core::variables::VariableExtractor;
/// use corpus_core::nodes::HashNode;
///
/// struct MyExtractor;
///
/// impl VariableExtractor<MyType> for MyExtractor {
///     fn extract_variables(&self, expr: &HashNode<MyType>) -> HashSet<u32> {
///         // Implementation specific to MyType
///         HashSet::new()
///     }
/// }
/// ```
pub trait VariableExtractor<T: crate::nodes::HashNodeInner> {
    /// Extract all variable indices from an expression.
    ///
    /// Returns a set of variable identifiers (e.g., de Bruijn indices).
    fn extract_variables(&self, expr: &HashNode<T>) -> HashSet<u32>;

    /// Extract variables at a specific binding depth.
    ///
    /// This is useful when working with quantified expressions where variables
    /// at different depths may have different meanings.
    ///
    /// The default implementation filters variables to those >= depth.
    fn extract_variables_at_depth(&self, expr: &HashNode<T>, depth: u32) -> HashSet<u32> {
        self.extract_variables(expr)
            .into_iter()
            .filter(|&idx| idx >= depth)
            .collect()
    }

    /// Count the number of distinct variables in an expression.
    ///
    /// Default implementation simply returns the size of the extracted variables set.
    fn count_variables(&self, expr: &HashNode<T>) -> usize {
        self.extract_variables(expr).len()
    }
}
