//! First-class axiom representation for automatic rewrite rule generation.
//!
//! Axioms are logical statements that can be converted to rewrite rules
//! based on their logical operators. This module provides the trait
//! abstraction and core conversion logic.
//!
//! Note: This module now works with the trait-based `LogicalExpression`
//! abstraction. Domain-specific implementations (like `DomainContent`)
//! are defined in logical system crates (e.g., `corpus_classical_logic`).

use std::fmt::Debug;

/// Error types for axiom conversion.
#[derive(Debug, Clone, PartialEq)]
pub enum StandardAxiomError {
    /// Not an axiom (atomic expression)
    NotAnAxiom,
    /// Unsupported operator for axiom conversion
    UnsupportedOperator,
    /// Malformed axiom (wrong arity)
    MalformedAxiom {
        expected: usize,
        found: usize,
    },
    /// Missing variables in pattern
    MissingVariables(Vec<String>),
    /// Parse error with location information
    ParseError {
        message: String,
        position: Option<usize>,
    },
    /// Invalid operator for axiom-level declaration
    InvalidTopLevelOperator {
        operator: String,
    },
    /// Variable binding error (e.g., unbound De Bruijn index)
    UnboundVariable {
        index: u32,
    },
}

impl std::fmt::Display for StandardAxiomError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StandardAxiomError::NotAnAxiom => write!(f, "Expression is not an axiom (atomic)"),
            StandardAxiomError::UnsupportedOperator => write!(f, "Operator does not support axiom conversion"),
            StandardAxiomError::MalformedAxiom { expected, found } => {
                write!(f, "Malformed axiom: expected {} operands, found {}", expected, found)
            }
            StandardAxiomError::MissingVariables(vars) => {
                write!(f, "Missing variables in pattern: {}", vars.join(", "))
            }
            StandardAxiomError::ParseError { message, position } => {
                if let Some(pos) = position {
                    write!(f, "Parse error at position {}: {}", pos, message)
                } else {
                    write!(f, "Parse error: {}", message)
                }
            }
            StandardAxiomError::InvalidTopLevelOperator { operator } => {
                write!(f, "Invalid top-level operator for axiom: '{}'", operator)
            }
            StandardAxiomError::UnboundVariable { index } => {
                write!(f, "Unbound variable in axiom: /{}", index)
            }
        }
    }
}

impl std::error::Error for StandardAxiomError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_axiom_error_display() {
        let err = StandardAxiomError::NotAnAxiom;
        assert_eq!(format!("{}", err), "Expression is not an axiom (atomic)");

        let err = StandardAxiomError::MalformedAxiom { expected: 2, found: 1 };
        assert_eq!(format!("{}", err), "Malformed axiom: expected 2 operands, found 1");
    }
}
