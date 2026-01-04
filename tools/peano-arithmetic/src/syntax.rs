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
    Term(Term),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Term {
    Successor(HashNode<Term>),
    Number(HashNode<u64>),
    DeBruijn(HashNode<u32>),
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
            ArithmeticExpression::Term(term) => write!(f, "{}", term),
        }
    }
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Term::Successor(inner) => write!(f, "S({})", inner),
            Term::Number(n) => write!(f, "{}", n),
            Term::DeBruijn(idx) => write!(f, "/{}", idx),
        }
    }
}

impl HashNodeInner for PeanoContent {
    fn hash(&self) -> u64 {
        match self {
            PeanoContent::Equals(left, right) => {
                let hashes = vec![left.hash(), right.hash()];
                Hashing::root_hash(1, &hashes)
            }
        }
    }

    fn size(&self) -> u64 {
        match self {
            PeanoContent::Equals(left, right) => 1 + left.size() + right.size(),
        }
    }
}

impl HashNodeInner for ArithmeticExpression {
    fn hash(&self) -> u64 {
        match self {
            ArithmeticExpression::Add(left, right) => {
                Hashing::root_hash(8, &[left.hash(), right.hash()])
            }
            ArithmeticExpression::Term(term) => Hashing::root_hash(9, &[term.hash()]),
        }
    }

    fn size(&self) -> u64 {
        match self {
            ArithmeticExpression::Add(left, right) => 1 + left.size() + right.size(),
            ArithmeticExpression::Term(term) => 1 + term.size(),
        }
    }
}

impl HashNodeInner for Term {
    fn hash(&self) -> u64 {
        match self {
            Term::Successor(inner) => Hashing::root_hash(10, &[inner.hash()]),
            Term::Number(n) => Hashing::root_hash(11, &[*n.value]),
            Term::DeBruijn(idx) => Hashing::root_hash(12, &[*idx.value as u64]),
        }
    }

    fn size(&self) -> u64 {
        match self {
            Term::Successor(inner) => 1 + inner.size(),
            Term::Number(_) => 1,
            Term::DeBruijn(_) => 1,
        }
    }
}
