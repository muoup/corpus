//! Classical logic axiom implementations.
//!
//! This module provides concrete implementations for classical logical operators.

use crate::expression::{ClassicalLogicalExpression, DomainContent, LogicalStorage};
use crate::BinaryTruth;
use corpus_core::base::nodes::{HashNode, HashNodeInner};
use corpus_core::rewriting::patterns::Rewritable;
use corpus_core::rewriting::Pattern;
use std::clone::Clone;

pub enum ClassicalLogicPattern<D: DomainContent> {
    Compound {
        opcode: u64,
        operands: Vec<Self>,
    },
    BooleanConstant(BinaryTruth),
    DomainPattern(D::AsPattern),
}

impl<D: DomainContent> Rewritable for ClassicalLogicalExpression<D> {
    type AsPattern = ClassicalLogicPattern<D>;
    type Storage = LogicalStorage<D>;

    fn decompose_to_pattern(
        &self,
        expr: &HashNode<Self>,
        store: &LogicalStorage<D>,
    ) -> ClassicalLogicPattern<D> {
        expression_to_pattern(expr, store)
    }

    fn try_rewrite(
        &self,
        from: &Self::AsPattern,
        to: &Self::AsPattern,
        store: &Self::Storage,
    ) -> Option<HashNode<Self>> {
        todo!()
    }

    fn get_recursive_rewrites(&self, store: &Self::Storage) -> Vec<HashNode<Self>> {
        todo!()
    }
}

/// Convert a ClassicalLogicalExpression to a Pattern.
fn expression_to_pattern<D: DomainContent>(
    expr: &HashNode<ClassicalLogicalExpression<D>>,
    store: &LogicalStorage<D>,
) -> ClassicalLogicPattern<D>
where
    D: HashNodeInner + Clone,
{
    todo!()
}
