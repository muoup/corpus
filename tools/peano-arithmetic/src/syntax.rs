use core::fmt;

use corpus_classical_logic::{BinaryTruth, ClassicalOperator};
use corpus_core::expression::{DomainContent, DomainExpression};
use corpus_core::nodes::{HashNode, HashNodeInner, Hashing};

pub type PeanoExpression = DomainExpression<BinaryTruth, PeanoContent>;

#[derive(Debug, Clone, PartialEq)]
pub enum PeanoContent {
    Equals(
        HashNode<ArithmeticExpression>,
        HashNode<ArithmeticExpression>,
    ),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ArithmeticExpression {
    Add(
        HashNode<ArithmeticExpression>,
        HashNode<ArithmeticExpression>,
    ),
    Successor(HashNode<ArithmeticExpression>),
    Number(u64),
    DeBruijn(u32),
}

impl fmt::Display for PeanoContent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PeanoContent::Equals(left, right) => write!(f, "{} = {}", left, right),
        }
    }
}

impl DomainContent<BinaryTruth> for PeanoContent {
    type Operator = ClassicalOperator;
}

impl fmt::Display for ArithmeticExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ArithmeticExpression::Add(left, right) => write!(f, "({} + {})", left, right),
            ArithmeticExpression::Successor(inner) => write!(f, "S({})", inner),
            ArithmeticExpression::Number(n) => write!(f, "{}", n),
            ArithmeticExpression::DeBruijn(idx) => write!(f, "/{}", idx),
        }
    }
}

impl HashNodeInner for PeanoContent {
    fn hash(&self) -> u64 {
        match self {
            PeanoContent::Equals(left, right) => {
                let hashes = vec![left.hash(), right.hash()];
                Hashing::root_hash(Hashing::opcode("equals"), &hashes)
            }
        }
    }

    fn size(&self) -> u64 {
        match self {
            PeanoContent::Equals(left, right) => 1 + left.size() + right.size(),
        }
    }

    fn decompose(&self) -> Option<(u8, Vec<HashNode<Self>>)> {
        None
    }
}

impl HashNodeInner for ArithmeticExpression {
    fn hash(&self) -> u64 {
        match self {
            ArithmeticExpression::Add(left, right) => {
                Hashing::root_hash(Hashing::opcode("add"), &[left.hash(), right.hash()])
            }
            ArithmeticExpression::Successor(inner) => {
                Hashing::root_hash(Hashing::opcode("successor"), &[inner.hash()])
            }
            ArithmeticExpression::Number(n) => Hashing::root_hash(Hashing::opcode("number"), &[*n]),
            ArithmeticExpression::DeBruijn(idx) => {
                Hashing::root_hash(Hashing::opcode("debruijn"), &[*idx as u64])
            }
        }
    }

    fn size(&self) -> u64 {
        match self {
            ArithmeticExpression::Add(left, right) => 1 + left.size() + right.size(),
            ArithmeticExpression::Successor(inner) => 1 + inner.size(),
            ArithmeticExpression::Number(_) => 1,
            ArithmeticExpression::DeBruijn(_) => 1,
        }
    }

    fn decompose(&self) -> Option<(u8, Vec<HashNode<Self>>)> {
        match self {
            ArithmeticExpression::Add(left, right) => {
                Some((Hashing::opcode("add"), vec![left.clone(), right.clone()]))
            }
            ArithmeticExpression::Successor(inner) => {
                Some((Hashing::opcode("successor"), vec![inner.clone()]))
            }
            ArithmeticExpression::Number(_) | ArithmeticExpression::DeBruijn(_) => None,
        }
    }
}
