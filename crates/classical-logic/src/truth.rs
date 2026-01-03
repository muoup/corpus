use corpus_core::nodes::HashNodeInner;
use std::fmt::{Debug, Display};

/// Classical binary truth values
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum BinaryTruth {
    True,
    #[default]
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

impl From<bool> for BinaryTruth {
    fn from(value: bool) -> Self {
        if value {
            BinaryTruth::True
        } else {
            BinaryTruth::False
        }
    }
}

impl From<BinaryTruth> for bool {
    fn from(value: BinaryTruth) -> Self {
        matches!(value, BinaryTruth::True)
    }
}

impl corpus_core::truth::TruthValue for BinaryTruth {
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
        Self::from(value)
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

// Const-optimized versions for compile-time evaluation
impl BinaryTruth {
    pub const fn const_and(self, other: Self) -> Self {
        match (self, other) {
            (BinaryTruth::True, BinaryTruth::True) => BinaryTruth::True,
            _ => BinaryTruth::False,
        }
    }

    pub const fn const_or(self, other: Self) -> Self {
        match (self, other) {
            (BinaryTruth::False, BinaryTruth::False) => BinaryTruth::False,
            _ => BinaryTruth::True,
        }
    }

    pub const fn const_not(self) -> Self {
        match self {
            BinaryTruth::True => BinaryTruth::False,
            BinaryTruth::False => BinaryTruth::True,
        }
    }

    pub const fn const_implies(self, other: Self) -> Self {
        match (self, other) {
            (BinaryTruth::False, _) => BinaryTruth::True,
            (_, BinaryTruth::True) => BinaryTruth::True,
            (BinaryTruth::True, BinaryTruth::False) => BinaryTruth::False,
        }
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
