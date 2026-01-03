use core::fmt;

use corpus_core::nodes::{HashNode, HashNodeInner, Hashing};

pub enum SumNode {
    Proposition(Proposition),
    Expression(Expression),
    Term(Term),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Proposition {
    And(HashNode<Proposition>, HashNode<Proposition>),
    Or(HashNode<Proposition>, HashNode<Proposition>),
    Implies(HashNode<Proposition>, HashNode<Proposition>),
    Not(HashNode<Proposition>),
    Forall(HashNode<Proposition>),
    Exists(HashNode<Proposition>),
    Equals(HashNode<Expression>, HashNode<Expression>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Add(HashNode<Expression>, HashNode<Expression>),
    Term(Term),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Term {
    Successor(HashNode<Term>),
    Number(HashNode<u64>),
    DeBruijn(HashNode<u32>),
}

impl fmt::Display for Proposition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Proposition::And(left, right) => write!(f, "({} ∧ {})", left, right),
            Proposition::Or(left, right) => write!(f, "({} ∨ {})", left, right),
            Proposition::Implies(left, right) => write!(f, "({} -> {})", left, right),
            Proposition::Not(inner) => write!(f, "¬{}", inner),
            Proposition::Forall(inner) => write!(f, "∀({})", inner),
            Proposition::Exists(inner) => write!(f, "∃({})", inner),
            Proposition::Equals(left, right) => write!(f, "{} = {}", left, right),
        }
    }
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Add(left, right) => write!(f, "({} + {})", left, right),
            Expression::Term(term) => write!(f, "{}", term),
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

impl HashNodeInner for Proposition {
    fn hash(&self) -> u64 {
        match self {
            Proposition::And(left, right) => Hashing::root_hash(1, &[left.hash, right.hash]),
            Proposition::Or(left, right) => Hashing::root_hash(2, &[left.hash, right.hash]),
            Proposition::Implies(left, right) => Hashing::root_hash(3, &[left.hash, right.hash]),
            Proposition::Not(inner) => Hashing::root_hash(4, &[inner.hash]),
            Proposition::Forall(inner) => Hashing::root_hash(5, &[inner.hash]),
            Proposition::Exists(inner) => Hashing::root_hash(6, &[inner.hash]),
            Proposition::Equals(left, right) => Hashing::root_hash(7, &[left.hash, right.hash]),
        }
    }
    
    fn size(&self) -> u64 {
        match self {
            Proposition::And(left, right) => 1 + left.size() + right.size(),
            Proposition::Or(left, right) => 1 + left.size() + right.size(),
            Proposition::Implies(left, right) => 1 + left.size() + right.size(),
            Proposition::Not(inner) => 1 + inner.size(),
            Proposition::Forall(inner) => 1 + inner.size(),
            Proposition::Exists(inner) => 1 + inner.size(),
            Proposition::Equals(left, right) => 1 + left.size() + right.size(),
        }
    }
}

impl HashNodeInner for Expression {
    fn hash(&self) -> u64 {
        match self {
            Expression::Add(left, right) => Hashing::root_hash(8, &[left.hash, right.hash]),
            Expression::Term(term) => Hashing::root_hash(9, &[term.hash()]),
        }
    }
    
    fn size(&self) -> u64 {
        match self {
            Expression::Add(left, right) => 1 + left.size() + right.size(),
            Expression::Term(term) => 1 + term.size(),
        }
    }
}

impl HashNodeInner for Term {
    fn hash(&self) -> u64 {
        match self {
            Term::Successor(inner) => Hashing::root_hash(10, &[inner.hash]),
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

impl HashNodeInner for SumNode {
    fn hash(&self) -> u64 {
        match self {
            SumNode::Proposition(p) => p.hash(),
            SumNode::Expression(e) => e.hash(),
            SumNode::Term(t) => t.hash(),
        }
    }
    
    fn size(&self) -> u64 {
        match self {
            SumNode::Proposition(p) => p.size(),
            SumNode::Expression(e) => e.size(),
            SumNode::Term(t) => t.size(),
        }
    }
}