use crate::truth::TruthValue;
use std::fmt::Debug;

pub trait LogicalOperator<T: TruthValue>: Clone + Debug + Send + Sync {
    type Symbol: Clone + Debug + PartialEq;

    fn symbol(&self) -> Self::Symbol;
    fn arity(&self) -> usize;
    fn apply(&self, operands: &[T]) -> T;

    fn is_unary(&self) -> bool {
        self.arity() == 1
    }

    fn is_binary(&self) -> bool {
        self.arity() == 2
    }

    fn is_nary(&self) -> bool {
        self.arity() > 2
    }
}

pub struct LogicalOperatorSet<T: TruthValue, Op: LogicalOperator<T>> {
    operators: Vec<Op>,
    _phantom: std::marker::PhantomData<T>,
}

impl<T: TruthValue, Op: LogicalOperator<T>> LogicalOperatorSet<T, Op> {
    pub fn new() -> Self {
        Self {
            operators: Vec::new(),
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn add_operator(&mut self, operator: Op) {
        self.operators.push(operator);
    }

    pub fn find_operator(&self, symbol: &Op::Symbol) -> Option<&Op> {
        self.operators.iter().find(|op| op.symbol() == *symbol)
    }

    pub fn operators(&self) -> &[Op] {
        &self.operators
    }
}

impl<T: TruthValue, Op: LogicalOperator<T>> Default for LogicalOperatorSet<T, Op> {
    fn default() -> Self {
        Self::new()
    }
}
