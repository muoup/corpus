//! Pattern decomposition for converting expressions to patterns.
//!
//! This trait provides a generic interface for converting logical expressions
//! (potentially with quantifiers) into pattern matching structures.
//!
//! Note: This module now works with the trait-based `LogicalExpression`
//! abstraction. Each logical system provides their own decomposer implementation.

use crate::{HashNodeInner, nodes::HashNode};

pub trait Rewritable : HashNodeInner {
    type AsPattern;
    type Storage;
    
    fn decompose_to_pattern(&self, expr: &HashNode<Self>, store: &Self::Storage) -> Self::AsPattern;
    fn try_rewrite(&self, from: &Self::AsPattern, to: &Self::AsPattern, store: &Self::Storage) -> Option<HashNode<Self>>;
    fn get_recursive_rewrites(&self, store: &Self::Storage) -> Vec<HashNode<Self>>;
}