//! Classical logic axiom implementations.
//!
//! This module provides concrete implementations for classical logical operators.

use crate::expression::{ClassicalLogicalExpression, DomainContent};
use crate::operators::ClassicalOperator;
use corpus_core::base::axioms::{AxiomConverter, InferenceDirection, InferenceDirectional};
use corpus_core::base::nodes::{HashNode, HashNodeInner};
use corpus_core::rewriting::{Pattern, RewriteDirection, RewriteRule};
use corpus_core::truth::TruthValue;
use std::clone::Clone;
use std::fmt::{Debug, Display};

impl InferenceDirectional for ClassicalOperator {
    fn inference_direction(&self) -> InferenceDirection {
        match self {
            ClassicalOperator::Equals => InferenceDirection::Both,
            ClassicalOperator::Iff => InferenceDirection::Both,
            ClassicalOperator::Implies => InferenceDirection::Forward,
            ClassicalOperator::And => InferenceDirection::Both,
            ClassicalOperator::Or => InferenceDirection::Both,
            ClassicalOperator::Not => InferenceDirection::Forward,
            ClassicalOperator::Forall => InferenceDirection::Both,
            ClassicalOperator::Exists => InferenceDirection::Both,
        }
    }
}

/// Classical logic implementation of AxiomConverter.
///
/// This struct provides the conversion logic specific to classical operators.
pub struct ClassicalAxiomConverter;

impl<T: TruthValue, D: DomainContent<T>> AxiomConverter<ClassicalLogicalExpression<T, D, ClassicalOperator>> for ClassicalAxiomConverter
where
    T: HashNodeInner,
    D: HashNodeInner + Clone + Debug + Display,
{
    fn convert_axiom(
        &self,
        expr: &HashNode<ClassicalLogicalExpression<T, D, ClassicalOperator>>,
        name: &str,
    ) -> Result<Vec<RewriteRule<ClassicalLogicalExpression<T, D, ClassicalOperator>>>, corpus_core::base::axioms::AxiomError> {
        convert_classical_axiom_to_rules(expr, name)
    }
}

/// Convert a classical logical expression to rewrite rules based on its operator.
fn convert_classical_axiom_to_rules<T: TruthValue, D: DomainContent<T>>(
    axiom: &HashNode<ClassicalLogicalExpression<T, D, ClassicalOperator>>,
    axiom_name: &str,
) -> Result<Vec<RewriteRule<ClassicalLogicalExpression<T, D, ClassicalOperator>>>, corpus_core::base::axioms::AxiomError>
where
    T: HashNodeInner,
    D: HashNodeInner + Clone,
{
    use corpus_core::base::axioms::AxiomError;

    let expr_ref = axiom.value.as_ref();

    // Must be a compound expression
    let ClassicalLogicalExpression::Compound { operator, operands, .. } = expr_ref else {
        return Err(AxiomError::NotAnAxiom);
    };

    match operator {
        ClassicalOperator::Equals => {
            // Equality: f(x) = g(x) → bidirectional rewrite
            if operands.len() != 2 {
                return Err(AxiomError::MalformedAxiom { expected: 2, found: operands.len() });
            }
            Ok(vec![create_equality_rule(axiom_name, &operands[0], &operands[1])])
        }
        ClassicalOperator::Implies => {
            // Implication: f(x) -> g(x) → forward rewrite
            if operands.len() != 2 {
                return Err(AxiomError::MalformedAxiom { expected: 2, found: operands.len() });
            }
            Ok(vec![create_implication_rule(axiom_name, &operands[0], &operands[1])])
        }
        ClassicalOperator::Iff => {
            // Iff: f(x) <-> g(x) → bidirectional rewrite
            if operands.len() != 2 {
                return Err(AxiomError::MalformedAxiom { expected: 2, found: operands.len() });
            }
            Ok(vec![create_equality_rule(axiom_name, &operands[0], &operands[1])])
        }
        _ => Err(AxiomError::UnsupportedOperator), // Other operators not supported for axioms
    }
}

/// Create a bidirectional rewrite rule from an equality axiom.
fn create_equality_rule<T: TruthValue, D: DomainContent<T>>(
    name: &str,
    lhs: &HashNode<ClassicalLogicalExpression<T, D, ClassicalOperator>>,
    rhs: &HashNode<ClassicalLogicalExpression<T, D, ClassicalOperator>>,
) -> RewriteRule<ClassicalLogicalExpression<T, D, ClassicalOperator>>
where
    T: HashNodeInner,
    D: HashNodeInner + Clone,
{
    let lhs_pattern = expression_to_pattern(lhs);
    let rhs_pattern = expression_to_pattern(rhs);
    RewriteRule::bidirectional(name, lhs_pattern, rhs_pattern)
}

/// Create a forward rewrite rule from an implication axiom.
fn create_implication_rule<T: TruthValue, D: DomainContent<T>>(
    name: &str,
    antecedent: &HashNode<ClassicalLogicalExpression<T, D, ClassicalOperator>>,
    consequent: &HashNode<ClassicalLogicalExpression<T, D, ClassicalOperator>>,
) -> RewriteRule<ClassicalLogicalExpression<T, D, ClassicalOperator>>
where
    T: HashNodeInner,
    D: HashNodeInner + Clone,
{
    let antecedent_pattern = expression_to_pattern(antecedent);
    let consequent_pattern = expression_to_pattern(consequent);
    RewriteRule::new(name, antecedent_pattern, consequent_pattern, RewriteDirection::Forward)
}

/// Convert a ClassicalLogicalExpression to a Pattern.
fn expression_to_pattern<T: TruthValue, D: DomainContent<T>>(
    expr: &HashNode<ClassicalLogicalExpression<T, D, ClassicalOperator>>,
) -> Pattern<ClassicalLogicalExpression<T, D, ClassicalOperator>>
where
    T: HashNodeInner,
    D: HashNodeInner + Clone,
{
    match expr.value.as_ref() {
        ClassicalLogicalExpression::Atomic(_) => {
            Pattern::constant(expr.value.as_ref().clone())
        }
        ClassicalLogicalExpression::Compound { operator, operands, .. } => {
            let arg_patterns: Vec<_> = operands
                .iter()
                .enumerate()
                .map(|(i, op)| {
                    if matches!(op.value.as_ref(), ClassicalLogicalExpression::Atomic(_)) {
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
    fn test_inference_direction_for_operators() {
        assert_eq!(
            ClassicalOperator::Equals.inference_direction(),
            InferenceDirection::Both
        );
        assert_eq!(
            ClassicalOperator::Implies.inference_direction(),
            InferenceDirection::Forward
        );
        assert_eq!(
            ClassicalOperator::Iff.inference_direction(),
            InferenceDirection::Both
        );
        assert_eq!(
            ClassicalOperator::And.inference_direction(),
            InferenceDirection::Both
        );
        assert_eq!(
            ClassicalOperator::Not.inference_direction(),
            InferenceDirection::Forward
        );
    }
}
