use std::fmt::{Display, Debug};
use corpus_core::truth::TruthValue;

/// Classical logical operators for binary truth systems
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ClassicalOperator {
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
    
    fn apply(&self, operands: &[T]) -> T {
        match self {
            ClassicalOperator::And => {
                assert_eq!(operands.len(), 2, "And requires exactly 2 operands");
                operands[0].and(&operands[1])
            },
            ClassicalOperator::Or => {
                assert_eq!(operands.len(), 2, "Or requires exactly 2 operands");
                operands[0].or(&operands[1])
            },
            ClassicalOperator::Implies => {
                assert_eq!(operands.len(), 2, "Implies requires exactly 2 operands");
                operands[0].implies(&operands[1])
            },
            ClassicalOperator::Iff => {
                assert_eq!(operands.len(), 2, "Iff requires exactly 2 operands");
                operands[0].implies(&operands[1]).and(&operands[1].implies(&operands[0]))
            },
            ClassicalOperator::Not => {
                assert_eq!(operands.len(), 1, "Not requires exactly 1 operand");
                operands[0].not()
            },
            ClassicalOperator::Forall => {
                assert_eq!(operands.len(), 1, "Forall requires exactly 1 operand");
                operands[0].clone() // No logical change in classical logic
            },
            ClassicalOperator::Exists => {
                assert_eq!(operands.len(), 1, "Exists requires exactly 1 operand");
                operands[0].clone() // No logical change in classical logic
            },
        }
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