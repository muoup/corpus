//! First-class axiom representation for automatic rewrite rule generation.
//!
//! Axioms are logical statements that can be converted to rewrite rules
//! based on their logical operators. This module provides the trait
//! abstraction and core conversion logic.

use crate::expression::{DomainContent, LogicalExpression};
use crate::logic::LogicalOperator;
use crate::nodes::{HashNode, HashNodeInner};
use crate::rewriting::{Pattern, RewriteDirection, RewriteRule};
use crate::truth::TruthValue;
use std::clone::Clone;
use std::fmt::Debug;

/// Trait for types that can act as axioms and generate rewrite rules.
///
/// # Type Parameters
///
/// * `T` - Truth value type (e.g., `BinaryTruth`)
/// * `D` - Domain content type (e.g., `PeanoContent`)
/// * `Op` - Logical operator type (e.g., `ClassicalOperator`)
pub trait Axiom<T: TruthValue + HashNodeInner, D: DomainContent<T> + Clone, Op: LogicalOperator<T> + HashNodeInner>: Debug {
    /// Convert this axiom to one or more rewrite rules.
    ///
    /// The number and direction of rules depends on the logical operator:
    /// - Equality (=) → 1 bidirectional rule (Both)
    /// - Implication (->) → 1 forward rule (antecedent → consequent)
    /// - Iff (<->) → 1 bidirectional rule (Both)
    fn to_rewrite_rules(&self) -> Vec<RewriteRule<LogicalExpression<T, D, Op>>>;

    /// Get the name/identifier of this axiom.
    fn name(&self) -> &str;

    /// Get the logical operator used in this axiom.
    fn operator(&self) -> Option<&Op>;

    /// Get the underlying logical expression.
    fn expression(&self) -> &HashNode<LogicalExpression<T, D, Op>>;

    /// Check if this axiom is valid (well-formed).
    fn is_valid(&self) -> bool {
        self.operator().is_some()
    }
}

/// Direction of inference for logical operators.
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
/// conversion logic from axioms to rewrite rules.
pub trait AxiomConverter<T: TruthValue + HashNodeInner, D: DomainContent<T> + Clone, Op: LogicalOperator<T> + HashNodeInner> {
    /// Convert a logical expression axiom to rewrite rules based on the operator.
    fn convert_axiom(
        &self,
        expr: &HashNode<LogicalExpression<T, D, Op>>,
        name: &str,
    ) -> Result<Vec<RewriteRule<LogicalExpression<T, D, Op>>>, AxiomError>;
}

/// Wrapper that turns a logical expression into a named axiom.
///
/// This struct provides a name and metadata for a logical expression,
/// allowing it to be used as an axiom that can generate rewrite rules.
/// It is generic over the operator type `Op`, making it usable with
/// any logical system.
pub struct NamedAxiom<T, D, Op>
where
    T: TruthValue + HashNodeInner,
    D: DomainContent<T> + Clone + Debug,
    Op: LogicalOperator<T> + HashNodeInner,
{
    pub name: String,
    pub expression: HashNode<LogicalExpression<T, D, Op>>,
    pub converter: Option<Box<dyn AxiomConverter<T, D, Op>>>,
}

impl<T, D, Op> NamedAxiom<T, D, Op>
where
    T: TruthValue + HashNodeInner,
    D: DomainContent<T> + Clone + Debug,
    Op: LogicalOperator<T> + HashNodeInner,
{
    /// Create a new NamedAxiom with an external converter.
    pub fn new_with_converter(
        name: impl Into<String>,
        expression: HashNode<LogicalExpression<T, D, Op>>,
        converter: Box<dyn AxiomConverter<T, D, Op>>,
    ) -> Self {
        Self {
            name: name.into(),
            expression,
            converter: Some(converter),
        }
    }

    /// Create a new NamedAxiom without a converter (for later use with operator impl).
    pub fn new(
        name: impl Into<String>,
        expression: HashNode<LogicalExpression<T, D, Op>>,
    ) -> Self {
        Self {
            name: name.into(),
            expression,
            converter: None,
        }
    }
}

impl<T, D, Op> Debug for NamedAxiom<T, D, Op>
where
    T: TruthValue + HashNodeInner,
    D: DomainContent<T> + Clone + Debug,
    Op: LogicalOperator<T> + HashNodeInner,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NamedAxiom")
            .field("name", &self.name)
            .field("expression", &self.expression)
            .finish()
    }
}

impl<T, D, Op> Clone for NamedAxiom<T, D, Op>
where
    T: TruthValue + HashNodeInner,
    D: DomainContent<T> + Clone + Debug,
    Op: LogicalOperator<T> + HashNodeInner,
{
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            expression: self.expression.clone(),
            converter: None, // Can't clone the trait object
        }
    }
}

// Blanket implementation for any type that implements LogicalOperator + InferenceDirectional
// This allows operators to provide their own conversion logic via a static method
impl<T, D, Op> Axiom<T, D, Op> for NamedAxiom<T, D, Op>
where
    T: TruthValue + HashNodeInner,
    D: DomainContent<T> + Clone + Debug,
    Op: LogicalOperator<T> + HashNodeInner + InferenceDirectional,
{
    fn to_rewrite_rules(&self) -> Vec<RewriteRule<LogicalExpression<T, D, Op>>> {
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
            // Fallback: use operator's inference direction for simple equality/implication
            // This is a simplified version - full implementation would be in the converter
            convert_by_inference_direction(&self.expression, &self.name)
        }
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn operator(&self) -> Option<&Op> {
        self.expression.value.operator()
    }

    fn expression(&self) -> &HashNode<LogicalExpression<T, D, Op>> {
        &self.expression
    }
}

/// Fallback conversion using inference direction (simplified).
fn convert_by_inference_direction<T: TruthValue, D: DomainContent<T>, Op: LogicalOperator<T> + HashNodeInner + InferenceDirectional>(
    expr: &HashNode<LogicalExpression<T, D, Op>>,
    name: &str,
) -> Vec<RewriteRule<LogicalExpression<T, D, Op>>>
where
    T: HashNodeInner,
    D: HashNodeInner + Clone,
{
    let expr_ref = expr.value.as_ref();

    // Must be a compound expression
    let LogicalExpression::Compound { operator, operands, .. } = expr_ref else {
        return vec![];
    };

    match operator.inference_direction() {
        InferenceDirection::Both => {
            if operands.len() != 2 {
                return vec![];
            }
            let lhs_pattern = expression_to_pattern(&operands[0]);
            let rhs_pattern = expression_to_pattern(&operands[1]);
            vec![RewriteRule::bidirectional(name, lhs_pattern, rhs_pattern)]
        }
        InferenceDirection::Forward => {
            if operands.len() != 2 {
                return vec![];
            }
            let lhs_pattern = expression_to_pattern(&operands[0]);
            let rhs_pattern = expression_to_pattern(&operands[1]);
            vec![RewriteRule::new(name, lhs_pattern, rhs_pattern, RewriteDirection::Forward)]
        }
        InferenceDirection::Backward => {
            if operands.len() != 2 {
                return vec![];
            }
            let lhs_pattern = expression_to_pattern(&operands[0]);
            let rhs_pattern = expression_to_pattern(&operands[1]);
            vec![RewriteRule::new(name, lhs_pattern, rhs_pattern, RewriteDirection::Backward)]
        }
    }
}

/// Convert a LogicalExpression to a Pattern (simplified).
fn expression_to_pattern<T: TruthValue, D: DomainContent<T>, Op: LogicalOperator<T>>(
    expr: &HashNode<LogicalExpression<T, D, Op>>,
) -> Pattern<LogicalExpression<T, D, Op>>
where
    T: HashNodeInner,
    D: HashNodeInner + Clone,
    Op: HashNodeInner,
{
    match expr.value.as_ref() {
        LogicalExpression::Atomic(_) => {
            Pattern::constant(expr.value.as_ref().clone())
        }
        LogicalExpression::Compound { operator, operands, .. } => {
            let arg_patterns: Vec<_> = operands
                .iter()
                .enumerate()
                .map(|(i, op)| {
                    if matches!(op.value.as_ref(), LogicalExpression::Atomic(_)) {
                        expression_to_pattern(op)
                    } else {
                        Pattern::var(i as u32)
                    }
                })
                .collect();
            Pattern::compound(operator.hash(), arg_patterns)
        }
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
