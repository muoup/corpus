//! First-class axiom representation for automatic rewrite rule generation.
//!
//! Axioms are logical statements that can be converted to rewrite rules
//! based on their logical operators. This module provides the trait
//! abstraction and core conversion logic.
//!
//! Note: This module now works with the trait-based `LogicalExpression`
//! abstraction. Domain-specific implementations (like `DomainContent`)
//! are defined in logical system crates (e.g., `corpus_classical_logic`).

use crate::expression::LogicalExpression;
use crate::nodes::HashNode;
use crate::rewriting::RewriteRule;
use std::fmt::Debug;

/// Trait for types that can act as axioms and generate rewrite rules.
///
/// This trait is now generic over any type that implements the `LogicalExpression`
/// trait, allowing each logical system to provide its own expression type.
///
/// # Type Parameters
///
/// * `Expr` - The logical expression type (must implement `LogicalExpression`)
pub trait Axiom<Expr: LogicalExpression>: Debug {
    /// Convert this axiom to one or more rewrite rules.
    ///
    /// The number and direction of rules depends on the logical operator:
    /// - Equality (=) → 1 bidirectional rule (Both)
    /// - Implication (->) → 1 forward rule (antecedent → consequent)
    /// - Iff (<->) → 1 bidirectional rule (Both)
    fn to_rewrite_rules(&self) -> Vec<RewriteRule<Expr>>;

    /// Get the name/identifier of this axiom.
    fn name(&self) -> &str;

    /// Get the underlying logical expression.
    fn expression(&self) -> &HashNode<Expr>;

    /// Check if this axiom is valid (well-formed).
    ///
    /// Default implementation checks if the expression is compound (has an operator).
    fn is_valid(&self) -> bool {
        self.expression().value.is_compound()
    }
}

/// Direction of inference for logical operators.
///
/// This is used by axiom converters to determine the direction of
/// rewrite rules generated from logical expressions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InferenceDirection {
    /// Bidirectional (e.g., equality, iff)
    Both,
    /// Forward only (e.g., implication: antecedent → consequent)
    Forward,
    /// Backward only (e.g., reverse implication)
    Backward,
}

/// Trait for operators that specify their inference direction.
///
/// This trait allows logical operators to declare how they should
/// be converted to rewrite rules.
pub trait InferenceDirectional {
    /// Get the inference direction for this operator.
    fn inference_direction(&self) -> InferenceDirection;
}

/// Error types for axiom conversion.
#[derive(Debug, Clone, PartialEq)]
pub enum AxiomError {
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

impl std::fmt::Display for AxiomError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AxiomError::NotAnAxiom => write!(f, "Expression is not an axiom (atomic)"),
            AxiomError::UnsupportedOperator => write!(f, "Operator does not support axiom conversion"),
            AxiomError::MalformedAxiom { expected, found } => {
                write!(f, "Malformed axiom: expected {} operands, found {}", expected, found)
            }
            AxiomError::MissingVariables(vars) => {
                write!(f, "Missing variables in pattern: {}", vars.join(", "))
            }
            AxiomError::ParseError { message, position } => {
                if let Some(pos) = position {
                    write!(f, "Parse error at position {}: {}", pos, message)
                } else {
                    write!(f, "Parse error: {}", message)
                }
            }
            AxiomError::InvalidTopLevelOperator { operator } => {
                write!(f, "Invalid top-level operator for axiom: '{}'", operator)
            }
            AxiomError::UnboundVariable { index } => {
                write!(f, "Unbound variable in axiom: /{}", index)
            }
        }
    }
}

impl std::error::Error for AxiomError {}

/// Trait for operator-specific axiom to rewrite rule conversion.
///
/// This trait allows different logical systems to provide their own
/// conversion logic from axioms to rewrite rules. Each logical system
/// implements this trait for their specific operator type.
///
/// # Type Parameters
///
/// * `Expr` - The logical expression type (must implement `LogicalExpression`)
pub trait AxiomConverter<Expr: LogicalExpression> {
    /// Convert a logical expression axiom to rewrite rules based on the operator.
    fn convert_axiom(
        &self,
        expr: &HashNode<Expr>,
        name: &str,
    ) -> Result<Vec<RewriteRule<Expr>>, AxiomError>;
}

/// Wrapper that turns a logical expression into a named axiom.
///
/// This struct provides a name and metadata for a logical expression,
/// allowing it to be used as an axiom that can generate rewrite rules.
///
/// # Type Parameters
///
/// * `Expr` - The logical expression type (must implement `LogicalExpression`)
pub struct NamedAxiom<Expr: LogicalExpression> {
    pub name: String,
    pub expression: HashNode<Expr>,
    pub converter: Option<Box<dyn AxiomConverter<Expr>>>,
}

impl<Expr: LogicalExpression> NamedAxiom<Expr> {
    /// Create a new NamedAxiom with an external converter.
    pub fn new_with_converter(
        name: impl Into<String>,
        expression: HashNode<Expr>,
        converter: Box<dyn AxiomConverter<Expr>>,
    ) -> Self {
        Self {
            name: name.into(),
            expression,
            converter: Some(converter),
        }
    }

    /// Create a new NamedAxiom without a converter (for later use).
    pub fn new(
        name: impl Into<String>,
        expression: HashNode<Expr>,
    ) -> Self {
        Self {
            name: name.into(),
            expression,
            converter: None,
        }
    }
}

impl<Expr: LogicalExpression> Debug for NamedAxiom<Expr> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NamedAxiom")
            .field("name", &self.name)
            .field("expression", &self.expression)
            .finish()
    }
}

impl<Expr: LogicalExpression> Clone for NamedAxiom<Expr> {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            expression: self.expression.clone(),
            converter: None, // Can't clone the trait object
        }
    }
}

/// Simple implementation of Axiom for NamedAxiom that returns empty rules.
///
/// This is a placeholder implementation. Each logical system should
/// provide their own implementation based on their operator types.
impl<Expr: LogicalExpression> Axiom<Expr> for NamedAxiom<Expr> {
    fn to_rewrite_rules(&self) -> Vec<RewriteRule<Expr>> {
        // Try to use the converter if available
        if let Some(converter) = &self.converter {
            match converter.convert_axiom(&self.expression, &self.name) {
                Ok(rules) => rules,
                Err(e) => {
                    eprintln!("Warning: Failed to convert axiom '{}': {}", self.name, e);
                    vec![]
                }
            }
        } else {
            // No converter available, return empty rules
            // Each logical system should provide their own converter
            vec![]
        }
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn expression(&self) -> &HashNode<Expr> {
        &self.expression
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inference_direction_equality() {
        // Test that InferenceDirection values are correctly comparable
        assert_eq!(InferenceDirection::Both, InferenceDirection::Both);
        assert_ne!(InferenceDirection::Forward, InferenceDirection::Backward);
    }

    #[test]
    fn test_axiom_error_display() {
        let err = AxiomError::NotAnAxiom;
        assert_eq!(format!("{}", err), "Expression is not an axiom (atomic)");

        let err = AxiomError::MalformedAxiom { expected: 2, found: 1 };
        assert_eq!(format!("{}", err), "Malformed axiom: expected 2 operands, found 1");
    }
}
