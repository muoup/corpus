//! Traits for domain-specific pattern matching and construction.
//!
//! These traits allow cross-level rewrite rules to work with different
//! domain types (like PeanoContent) without hardcoding their structure.

use crate::base::nodes::HashNode;

/// A domain that can be checked for equality structure.
///
/// For example, `PeanoContent::Equals(left, right)` implements this trait.
pub trait EqualityExtractable: crate::base::nodes::HashNodeInner {
    /// The type of expressions that appear in the equality.
    type SubExpr: crate::base::nodes::HashNodeInner;

    /// If this domain value represents an equality, return its left and right sides.
    fn as_equals(&self) -> Option<(&HashNode<Self::SubExpr>, &HashNode<Self::SubExpr>)>;
}

/// A domain that can be checked for successor structure.
///
/// For example, `ArithmeticExpression::Successor(inner)` implements this trait.
pub trait SuccessorExtractable: crate::base::nodes::HashNodeInner {
    type SubExpr: crate::base::nodes::HashNodeInner;

    /// If this domain value represents a successor, return its inner expression.
    fn as_successor(&self) -> Option<&HashNode<Self::SubExpr>>;
}

/// A domain that can be checked for addition structure.
///
/// For example, `ArithmeticExpression::Add(left, right)` implements this trait.
pub trait AddExtractable: crate::base::nodes::HashNodeInner {
    type SubExpr: crate::base::nodes::HashNodeInner;

    /// If this domain value represents addition, return its left and right sides.
    fn as_add(&self) -> Option<(&HashNode<Self::SubExpr>, &HashNode<Self::SubExpr>)>;
}
