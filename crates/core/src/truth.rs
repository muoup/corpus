use std::fmt::{Display, Debug};
use crate::nodes::HashNodeInner;

pub trait TruthValue: Clone + Debug + Display + PartialEq + Send + Sync {
    fn is_true(&self) -> bool;
    fn is_false(&self) -> bool;
    fn as_bool(&self) -> Option<bool>;
    
    fn from_bool(value: bool) -> Self;
    
    fn and(&self, other: &Self) -> Self;
    fn or(&self, other: &Self) -> Self;
    fn not(&self) -> Self;
    fn implies(&self, other: &Self) -> Self;
    
    fn conjunction(values: &[Self]) -> Self;
    fn disjunction(values: &[Self]) -> Self;
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BinaryTruth {
    True,
    False,
}

impl Display for BinaryTruth {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BinaryTruth::True => write!(f, "true"),
            BinaryTruth::False => write!(f, "false"),
        }
    }
}

impl TruthValue for BinaryTruth {
    fn is_true(&self) -> bool {
        matches!(self, BinaryTruth::True)
    }
    
    fn is_false(&self) -> bool {
        matches!(self, BinaryTruth::False)
    }
    
    fn as_bool(&self) -> Option<bool> {
        Some(self.is_true())
    }
    
    fn from_bool(value: bool) -> Self {
        if value {
            BinaryTruth::True
        } else {
            BinaryTruth::False
        }
    }
    
    fn and(&self, other: &Self) -> Self {
        match (self, other) {
            (BinaryTruth::True, BinaryTruth::True) => BinaryTruth::True,
            _ => BinaryTruth::False,
        }
    }
    
    fn or(&self, other: &Self) -> Self {
        match (self, other) {
            (BinaryTruth::False, BinaryTruth::False) => BinaryTruth::False,
            _ => BinaryTruth::True,
        }
    }
    
    fn not(&self) -> Self {
        match self {
            BinaryTruth::True => BinaryTruth::False,
            BinaryTruth::False => BinaryTruth::True,
        }
    }
    
    fn implies(&self, other: &Self) -> Self {
        match (self, other) {
            (BinaryTruth::False, _) => BinaryTruth::True,
            (_, BinaryTruth::True) => BinaryTruth::True,
            (BinaryTruth::True, BinaryTruth::False) => BinaryTruth::False,
        }
    }
    
    fn conjunction(values: &[Self]) -> Self {
        if values.iter().all(|v| v.is_true()) {
            BinaryTruth::True
        } else {
            BinaryTruth::False
        }
    }
    
    fn disjunction(values: &[Self]) -> Self {
        if values.iter().any(|v| v.is_true()) {
            BinaryTruth::True
        } else {
            BinaryTruth::False
        }
    }
}

impl Default for BinaryTruth {
    fn default() -> Self {
        BinaryTruth::False
    }
}

impl From<bool> for BinaryTruth {
    fn from(value: bool) -> Self {
        Self::from_bool(value)
    }
}

impl From<BinaryTruth> for bool {
    fn from(value: BinaryTruth) -> Self {
        value.is_true()
    }
}

impl HashNodeInner for BinaryTruth {
    fn hash(&self) -> u64 {
        match self {
            BinaryTruth::True => 1,
            BinaryTruth::False => 0,
        }
    }
    
    fn size(&self) -> u64 {
        1
    }
}