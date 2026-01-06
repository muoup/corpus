use core::fmt;

use corpus_classical_logic::{ClassicalLogicalExpression, DomainContent};
use corpus_core::{NodeStorage, nodes::{HashNode, HashNodeInner, Hashing}};

use crate::PeanoStores;

// NOTE: PeanoExpression (DomainExpression) has been removed as DomainExpression
// is no longer in the core crate. Domain-specific expressions should use
// ClassicalLogicalExpression directly.

/// Logical expression type for Peano Arithmetic with full first-order logic support.
/// This wraps PeanoContent in ClassicalLogicalExpression to enable quantifiers (∀, ∃) and
/// mixed logical operators (→, ∧, ∨, ¬, ↔).
pub type PeanoLogicalExpression = ClassicalLogicalExpression<PeanoArithmeticExpression>;

/// Hash node containing a Peano logical expression.
pub type PeanoLogicalNode = HashNode<PeanoLogicalExpression>;

#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum PeanoArithmeticExpression {
    Add(
        HashNode<PeanoArithmeticExpression>,
        HashNode<PeanoArithmeticExpression>,
    ),
    Successor(HashNode<PeanoArithmeticExpression>),
    Number(u64),
    DeBruijn(u32),
}

impl DomainContent for PeanoArithmeticExpression {}

impl Rewritable for PeanoArithmeticExpression {
    type AsPattern = PeanoArithmeticExpression;
    type Storage = NodeStorage<LogicalStorage<PeanoStores>>;

    fn decompose_to_pattern(
        &self,
        expr: &HashNode<Self>,
        _store: &Self::Storage,
    ) -> PeanoArithmeticExpression {
        expr.as_ref().clone()
    }

    fn try_rewrite(
        &self,
        _from: &Self::AsPattern,
        _to: &Self::AsPattern,
        _store: &Self::Storage,
    ) -> Option<HashNode<Self>> {
        None
    }

    fn get_recursive_rewrites(&self, _store: &Self::Storage) -> Vec<HashNode<Self>> {
        vec![]
    }
}

impl fmt::Display for PeanoArithmeticExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PeanoArithmeticExpression::Add(left, right) => write!(f, "({} + {})", left, right),
            PeanoArithmeticExpression::Successor(inner) => write!(f, "S({})", inner),
            PeanoArithmeticExpression::Number(n) => write!(f, "{}", n),
            PeanoArithmeticExpression::DeBruijn(idx) => write!(f, "/{}", idx),
        }
    }
}

impl HashNodeInner for PeanoArithmeticExpression {
    fn hash(&self) -> u64 {
        match self {
            PeanoArithmeticExpression::Add(left, right) => {
                Hashing::root_hash(Hashing::opcode("add"), &[left.hash(), right.hash()])
            }
            PeanoArithmeticExpression::Successor(inner) => {
                Hashing::root_hash(Hashing::opcode("successor"), &[inner.hash()])
            }
            PeanoArithmeticExpression::Number(n) => Hashing::root_hash(Hashing::opcode("number"), &[*n]),
            PeanoArithmeticExpression::DeBruijn(idx) => {
                Hashing::root_hash(Hashing::opcode("debruijn"), &[*idx as u64])
            }
        }
    }

    fn size(&self) -> u64 {
        match self {
            PeanoArithmeticExpression::Add(left, right) => 1 + left.size() + right.size(),
            PeanoArithmeticExpression::Successor(inner) => 1 + inner.size(),
            PeanoArithmeticExpression::Number(_) => 1,
            PeanoArithmeticExpression::DeBruijn(_) => 1,
        }
    }
}
