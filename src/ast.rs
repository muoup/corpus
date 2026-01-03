use std::{fmt, hash::Hasher, rc::Rc};

pub trait Hashable {
    fn hash(&self) -> u64;
}

#[derive(Debug, Clone)]
pub struct HashNode<T: Hashable> {
    pub value: Rc<T>,
    pub hash: u64,
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

impl<T: Hashable> PartialEq for HashNode<T> {
    fn eq(&self, other: &Self) -> bool {
        self.hash == other.hash
    }
}

impl<T: fmt::Display + Hashable> fmt::Display for HashNode<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
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

fn hash_combine(seed: u64, value: u64) -> u64 {
    const MAGIC: u64 = 0x9e3779b9;

    seed ^ (value
        .wrapping_add(MAGIC)
        .wrapping_add(seed << 6)
        .wrapping_add(seed >> 2))
}

fn operator_hash(op_code: u64, hashes: &[u64]) -> u64 {
    let mut result = op_code;
    for h in hashes {
        result = hash_combine(result, *h);
    }
    result
}

impl Hashable for Proposition {
    fn hash(&self) -> u64 {
        match self {
            Proposition::And(left, right) => operator_hash(1, &[left.hash, right.hash]),
            Proposition::Or(left, right) => operator_hash(2, &[left.hash, right.hash]),
            Proposition::Implies(left, right) => operator_hash(3, &[left.hash, right.hash]),
            Proposition::Not(inner) => operator_hash(4, &[inner.hash]),
            Proposition::Forall(inner) => operator_hash(5, &[inner.hash]),
            Proposition::Exists(inner) => operator_hash(6, &[inner.hash]),
            Proposition::Equals(left, right) => operator_hash(7, &[left.hash, right.hash]),
        }
    }
}

impl Hashable for Expression {
    fn hash(&self) -> u64 {
        match self {
            Expression::Add(left, right) => operator_hash(8, &[left.hash, right.hash]),
            Expression::Term(term) => term.hash(),
        }
    }
}

impl Hashable for Term {
    fn hash(&self) -> u64 {
        match self {
            Term::Successor(inner) => operator_hash(9, &[inner.hash]),
            Term::Number(n) => operator_hash(10, &[*n.value]),
            Term::DeBruijn(idx) => operator_hash(11, &[*idx.value as u64]),
        }
    }
}

impl Hashable for u64 {
    fn hash(&self) -> u64 {
        *self
    }
}

impl Hashable for u32 {
    fn hash(&self) -> u64 {
        *self as u64
    }
}

impl<T: Hashable> std::hash::Hash for HashNode<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u64(self.hash);
    }
}

impl<T: Hashable> From<T> for HashNode<T> {
    fn from(value: T) -> Self {
        let hash = value.hash();
        HashNode {
            value: Rc::new(value),
            hash,
        }
    }
}
