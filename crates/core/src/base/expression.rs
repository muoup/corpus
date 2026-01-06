use crate::nodes::HashNodeInner;
use crate::truth::TruthValue;
use std::fmt::{Debug, Display};

/// Trait for logical expressions that can be used with generic proving/rewriting systems.
///
/// Each logical system (classical, intuitionistic, etc.) implements this trait
/// for their concrete expression type. Uses opcode-based deconstruction similar
/// to `HashNodeInner` for generic operations.
///
/// # Type Parameters
///
/// * `TruthValue` - The truth value type for this logical system (e.g., `BinaryTruth`)
pub trait LogicalExpression: HashNodeInner + Clone + Debug + Display
where
    Self: Sized,
{
    /// The truth value type for this logical system (e.g., BinaryTruth)
    type TruthValue: TruthValue;

    /// Check if this is an atomic (leaf) expression
    fn is_atomic(&self) -> bool {
        self.decompose().is_none()
    }

    /// Check if this is a compound expression with an operator
    fn is_compound(&self) -> bool {
        !self.is_atomic()
    }

    /// Get the opcode if this is a compound expression (for pattern matching)
    fn opcode(&self) -> Option<u64> {
        self.decompose().map(|(op, _)| op)
    }

    /// Get the arity (number of operands) if compound
    fn arity(&self) -> Option<usize> {
        self.decompose().map(|(_, children)| children.len())
    }

    // Note: decompose() and construct_from_parts() are inherited from HashNodeInner
}
// NOTE: DomainExpression and DomainContent have been removed from core.
// These are now implementation-specific, defined in crates like classical-logic.
// This removes the coupling between the core crate and domain-specific concepts.