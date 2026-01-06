//! Concrete logical expression implementation for classical logic.
//!
//! This module provides the concrete `ClassicalLogicalExpression` type that
//! implements the `LogicalExpression` trait from `corpus_core`. It also provides
//! the `DomainContent` trait which is specific to classical logic systems.

use corpus_core::base::expression::LogicalExpression as LogicalExpressionTrait;
use corpus_core::base::logic::LogicalOperator;
use corpus_core::nodes::{HashNode, HashNodeInner, NodeStorage, Hashing};
use corpus_core::truth::TruthValue;
use std::fmt::{Debug, Display};
use std::marker::PhantomData;

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
pub enum ClassicalLogicalExpression<T: TruthValue, D: DomainContent<T>, Op: LogicalOperator<T>>
where
    T: HashNodeInner,
    D: HashNodeInner,
    Op: HashNodeInner,
{
    /// Atomic domain expression (leaf node)
    Atomic(HashNode<D>),
    /// Compound expression with operator and operands
    Compound {
        operator: Op,
        operands: Vec<HashNode<Self>>,
        _phantom: PhantomData<T>,
    },
}

impl<T: TruthValue, D: DomainContent<T>, Op: LogicalOperator<T>> ClassicalLogicalExpression<T, D, Op>
where
    T: HashNodeInner,
    D: HashNodeInner,
    Op: HashNodeInner,
{
    /// Create an atomic expression from domain content.
    pub fn atomic(value: HashNode<D>) -> Self {
        Self::Atomic(value)
    }

    /// Create a compound expression from an operator and operands.
    pub fn compound(operator: Op, operands: Vec<HashNode<Self>>) -> Self {
        Self::Compound {
            operator,
            operands,
            _phantom: PhantomData,
        }
    }

    /// Get the operator if this is a compound expression.
    pub fn operator(&self) -> Option<&Op> {
        match self {
            Self::Compound { operator, .. } => Some(operator),
            _ => None,
        }
    }

    /// Get the operands if this is a compound expression.
    pub fn operands(&self) -> Option<&Vec<HashNode<Self>>> {
        match self {
            Self::Compound { operands, .. } => Some(operands),
            _ => None,
        }
    }
}

impl<T: TruthValue, D: DomainContent<T>, Op: LogicalOperator<T>> HashNodeInner
    for ClassicalLogicalExpression<T, D, Op>
where
    T: HashNodeInner,
    D: HashNodeInner,
    Op: HashNodeInner,
{
    fn hash(&self) -> u64 {
        match self {
            Self::Atomic(value) => Hashing::root_hash(0, &[value.hash()]),
            Self::Compound { operator, operands, .. } => {
                let mut all_hashes = vec![operator.hash()];
                all_hashes.extend(operands.iter().map(|n| n.hash()));
                Hashing::root_hash(1, &all_hashes)
            }
        }
    }

    fn size(&self) -> u64 {
        match self {
            Self::Atomic(value) => 1 + value.size(),
            Self::Compound { operator, operands, .. } => {
                1 + operator.size() + operands.iter().map(|n| n.size()).sum::<u64>()
            }
        }
    }

    fn decompose(&self) -> Option<(u64, Vec<HashNode<Self>>)> {
        match self {
            Self::Compound { operator, operands, .. } => {
                Some((operator.hash(), operands.clone()))
            }
            Self::Atomic(_) => None,
        }
    }

    fn construct_from_parts(
        opcode: u64,
        children: Vec<HashNode<Self>>,
        _store: &NodeStorage<Self>,
    ) -> Option<HashNode<Self>> {
        // This requires being able to map from opcode to operator, which
        // depends on the specific operator type.
        // For now, this returns None - implementations should use the
        // explicit constructors (atomic, compound) instead.
        None
    }
}

impl<T: TruthValue, D: DomainContent<T>, Op: LogicalOperator<T>> LogicalExpressionTrait
    for ClassicalLogicalExpression<T, D, Op>
where
    T: HashNodeInner + Display,
    D: HashNodeInner + Display + Debug + Clone,
    Op: HashNodeInner + Display + Debug,
{
    type TruthValue = T;
}

impl<T: TruthValue, D: DomainContent<T>, Op: LogicalOperator<T>> Display
    for ClassicalLogicalExpression<T, D, Op>
where
    D: Display + HashNodeInner,
    T: Display + HashNodeInner,
    Op: Display + HashNodeInner,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Atomic(value) => write!(f, "{}", value),
            Self::Compound {
                operator, operands, ..
            } => match operator.arity() {
                1 => write!(f, "({} {})", operator, &operands[0]),
                2 => write!(f, "({} {} {})", &operands[0], operator, &operands[1]),
                _ => write!(
                    f,
                    "({} {})",
                    operator,
                    operands
                        .iter()
                        .map(|op| format!("{}", op))
                        .collect::<Vec<_>>()
                        .join(" ")
                ),
            },
        }
    }
}

/// Domain content trait for classical logic.
///
/// This trait is now specific to classical logic systems, having been
/// moved from the core crate. Each domain type (e.g., PeanoContent) implements
/// this trait to specify which logical operators it uses.
///
/// # Type Parameters
///
/// * `T` - The truth value type (e.g., `BinaryTruth`)
pub trait DomainContent<T: TruthValue>
where
    Self: HashNodeInner,
    Self::Operator: HashNodeInner,
{
    /// The logical operator type used with this domain content.
    type Operator: LogicalOperator<T>;
}

/// Type alias for backward compatibility.
///
/// This allows existing code to continue using the name `LogicalExpression`
/// without changes, while the actual implementation is now in classical-logic.
pub type LogicalExpression<T, D, Op> = ClassicalLogicalExpression<T, D, Op>;
