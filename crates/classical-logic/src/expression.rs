//! Concrete logical expression implementation for classical logic.
//!
//! This module provides the concrete `ClassicalLogicalExpression` type that
//! implements the `LogicalExpression` trait from `corpus_core`. It also provides
//! the `DomainContent` trait which is specific to classical logic systems.

use corpus_core::{
    nodes::{HashNode, HashNodeInner, Hashing, NodeStorage},
    rewriting::patterns::Rewritable,
};
use std::fmt::Debug;

use crate::BinaryTruth;

/// Concrete logical expression for classical logic.
///
/// This enum replaces the generic `LogicalExpression<T, D, Op>` that was
/// previously in the core crate. It is now a specific implementation for
/// classical logic, with the domain content parameterized by the `D` type.
///
/// # Type Parameters
///
/// * `T` - The truth value type (e.g., `BinaryTruth`)
/// * `D` - The domain content type (e.g., `PeanoContent`)
/// * `Op` - The logical operator type (e.g., `ClassicalOperator`)
#[derive(Debug, Clone, PartialEq)]
pub enum ClassicalLogicalExpression<D: DomainContent> {
    And(HashNode<Self>, HashNode<Self>),
    Or(HashNode<Self>, HashNode<Self>),
    Not(HashNode<Self>),
    Imply(HashNode<Self>, HashNode<Self>),
    Iff(HashNode<Self>, HashNode<Self>),
    ForAll(HashNode<Self>),
    Exists(HashNode<Self>),
    Equals(HashNode<D>, HashNode<D>),

    BooleanConstant(BinaryTruth),

    #[allow(non_camel_case_types)]
    _phantom(),
}

pub struct LogicalStorage<D: DomainContent> {
    pub logical_storage: NodeStorage<ClassicalLogicalExpression<D>>,
    pub domain_storage: D::Storage,
}

/// Content existing in the "Domain of Discourse" for classical logic.
///
/// # Type Parameters
///
/// * `T` - The truth value type (e.g., `BinaryTruth`)
pub trait DomainContent
where
    Self: HashNodeInner,
    Self: Rewritable,
{
}

impl<D: DomainContent> ClassicalLogicalExpression<D> {
    /// Get the operands if this is a compound expression.
    pub fn logical_operands(&self) -> Option<Vec<HashNode<Self>>> {
        match self {
            Self::And(left, right) => Some(vec![left.clone(), right.clone()]),
            Self::Or(left, right) => Some(vec![left.clone(), right.clone()]),
            Self::Not(operand) => Some(vec![operand.clone()]),
            Self::Imply(left, right) => Some(vec![left.clone(), right.clone()]),
            Self::Iff(left, right) => Some(vec![left.clone(), right.clone()]),
            Self::ForAll(operand) => Some(vec![operand.clone()]),
            Self::Exists(operand) => Some(vec![operand.clone()]),

            Self::BooleanConstant(_) => Some(vec![]),
            Self::_phantom(..) | Self::Equals(..) => None,
        }
    }

    pub fn domain_operands(&self) -> Option<Vec<HashNode<D>>> {
        match self {
            Self::Equals(left, right) => Some(vec![left.clone(), right.clone()]),

            _ => None,
        }
    }
}

impl<D: DomainContent> HashNodeInner for ClassicalLogicalExpression<D> {
    fn hash(&self) -> u64 {
        match self {
            Self::Equals(left, right) => {
                let all_hashes = vec![Hashing::opcode("DOMAIN"), left.hash(), right.hash()];
                Hashing::root_hash(1, &all_hashes)
            }

            Self::And(left, right) => {
                let all_hashes = vec![Hashing::opcode("AND"), left.hash(), right.hash()];
                Hashing::root_hash(1, &all_hashes)
            }

            Self::Or(left, right) => {
                let all_hashes = vec![Hashing::opcode("OR"), left.hash(), right.hash()];
                Hashing::root_hash(1, &all_hashes)
            }

            Self::Not(operand) => {
                let all_hashes = vec![Hashing::opcode("NOT"), operand.hash()];
                Hashing::root_hash(1, &all_hashes)
            }

            Self::Imply(left, right) => {
                let all_hashes = vec![Hashing::opcode("IMPLY"), left.hash(), right.hash()];
                Hashing::root_hash(1, &all_hashes)
            }

            Self::Iff(left, right) => {
                let all_hashes = vec![Hashing::opcode("IFF"), left.hash(), right.hash()];
                Hashing::root_hash(1, &all_hashes)
            }

            Self::ForAll(operand) => {
                let all_hashes = vec![Hashing::opcode("FORALL"), operand.hash()];
                Hashing::root_hash(1, &all_hashes)
            }

            Self::Exists(operand) => {
                let all_hashes = vec![Hashing::opcode("EXISTS"), operand.hash()];
                Hashing::root_hash(1, &all_hashes)
            }

            Self::BooleanConstant(value) => {
                let all_hashes = vec![Hashing::opcode("BOOLEAN_CONSTANT"), value.hash()];
                Hashing::root_hash(1, &all_hashes)
            }

            Self::_phantom(..) => {
                unreachable!()
            }
        }
    }

    fn size(&self) -> u64 {
        match self {
            Self::BooleanConstant(_) => 1,

            Self::Equals(lhs, rhs) => 1 + lhs.size() + rhs.size(),

            Self::And(left, right)
            | Self::Or(left, right)
            | Self::Imply(left, right)
            | Self::Iff(left, right) => 1 + left.size() + right.size(),

            Self::Not(operand) | Self::ForAll(operand) | Self::Exists(operand) => {
                1 + operand.size()
            }

            Self::_phantom(..) => unreachable!(),
        }
    }
}
