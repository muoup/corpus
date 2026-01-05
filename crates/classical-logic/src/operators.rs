use corpus_core::truth::TruthValue;
use std::fmt::{Debug, Display};

/// Classical logical operators for binary truth systems
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ClassicalOperator {
    Equals,
    And,
    Or,
    Implies,
    Not,
    Iff,
    Forall,
    Exists,
}

impl Display for ClassicalOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.symbol())
    }
}

impl ClassicalOperator {
    pub fn symbol(&self) -> &'static str {
        match self {
            ClassicalOperator::Equals => "=",
            ClassicalOperator::And => "∧",
            ClassicalOperator::Or => "∨",
            ClassicalOperator::Implies => "->",
            ClassicalOperator::Not => "¬",
            ClassicalOperator::Iff => "<->",
            ClassicalOperator::Forall => "∀",
            ClassicalOperator::Exists => "∃",
        }
    }

    pub fn arity(&self) -> usize {
        match self {
            ClassicalOperator::Equals => 2,
            ClassicalOperator::And => 2,
            ClassicalOperator::Or => 2,
            ClassicalOperator::Implies => 2,
            ClassicalOperator::Iff => 2,
            ClassicalOperator::Not => 1,
            ClassicalOperator::Forall => 1,
            ClassicalOperator::Exists => 1,
        }
    }
}

impl<T: TruthValue> corpus_core::logic::LogicalOperator<T> for ClassicalOperator {
    type Symbol = &'static str;

    fn symbol(&self) -> Self::Symbol {
        self.symbol()
    }

    fn arity(&self) -> usize {
        self.arity()
    }
}

impl corpus_core::nodes::HashNodeInner for ClassicalOperator {
    fn hash(&self) -> u64 {
        match self {
            ClassicalOperator::And => 1,
            ClassicalOperator::Or => 2,
            ClassicalOperator::Implies => 3,
            ClassicalOperator::Not => 4,
            ClassicalOperator::Iff => 5,
            ClassicalOperator::Forall => 6,
            ClassicalOperator::Exists => 7,
        }
    }

    fn size(&self) -> u64 {
        1
    }
}
