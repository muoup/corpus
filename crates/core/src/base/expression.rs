use crate::logic::LogicalOperator;
use crate::nodes::{HashNode, HashNodeInner, Hashing, NodeStorage};
use crate::truth::TruthValue;
use std::fmt::{Display, Formatter};
use std::marker::PhantomData;

#[derive(Debug, Clone, PartialEq)]
pub enum LogicalExpression<T: TruthValue, D: DomainContent<T>, Op: LogicalOperator<T>>
where
    T: HashNodeInner,
    Op: HashNodeInner,
{
    Atomic(HashNode<D>),
    Compound {
        operator: Op,
        operands: Vec<HashNode<Self>>,

        _phantom: std::marker::PhantomData<T>,
    },
}

impl<T: TruthValue, D: DomainContent<T>, Op: LogicalOperator<T>> LogicalExpression<T, D, Op>
where
    T: HashNodeInner,
    Op: HashNodeInner,
{
    pub fn atomic(value: HashNode<D>) -> Self {
        LogicalExpression::Atomic(value)
    }

    pub fn compound(operator: Op, operands: Vec<HashNode<Self>>) -> Self {
        LogicalExpression::Compound {
            operator,
            operands,
            _phantom: PhantomData,
        }
    }

    pub fn is_atomic(&self) -> bool {
        matches!(self, LogicalExpression::Atomic(_))
    }

    pub fn is_compound(&self) -> bool {
        matches!(self, LogicalExpression::Compound { .. })
    }

    pub fn operator(&self) -> Option<&Op> {
        match self {
            LogicalExpression::Compound { operator, .. } => Some(operator),
            _ => None,
        }
    }

    pub fn operands(&self) -> Option<&Vec<HashNode<Self>>> {
        match self {
            LogicalExpression::Compound { operands, .. } => Some(operands),
            _ => None,
        }
    }
}

impl<T: TruthValue, D: DomainContent<T>, Op: LogicalOperator<T>> Display
    for LogicalExpression<T, D, Op>
where
    D: Display + HashNodeInner,
    T: Display + HashNodeInner,
    Op: Display + HashNodeInner,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            LogicalExpression::Atomic(value) => write!(f, "{}", value),
            LogicalExpression::Compound {
                operator, operands, ..
            } => match operator.arity() {
                1 => write!(f, "({} {})", operator, &operands[0]),
                2 => write!(f, "({} {} {})", &operands[0], operator, &operands[1]),
                _ => write!(
                    f,
                    "({} {})",
                    operator,
                    operands
                        .iter()
                        .map(|op| format!("{}", op))
                        .collect::<Vec<_>>()
                        .join(" ")
                ),
            },
        }
    }
}

impl<T: TruthValue, D: DomainContent<T>, Op: LogicalOperator<T>> HashNodeInner
    for LogicalExpression<T, D, Op>
where
    T: HashNodeInner,
    Op: HashNodeInner,
{
    fn hash(&self) -> u64 {
        match self {
            LogicalExpression::Atomic(value) => Hashing::root_hash(0, &[value.hash()]),
            LogicalExpression::Compound {
                operator, operands, ..
            } => {
                let mut all_hashes = vec![operator.hash()];
                all_hashes.extend(operands.iter().map(|node| node.hash()));
                Hashing::root_hash(1, &all_hashes)
            }
        }
    }

    fn size(&self) -> u64 {
        match self {
            LogicalExpression::Atomic(value) => 1 + value.size(),
            LogicalExpression::Compound {
                operator, operands, ..
            } => 1 + operator.size() + operands.iter().map(|node| node.size()).sum::<u64>(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum DomainExpression<T: TruthValue, D: DomainContent<T>>
where
    T: HashNodeInner,
    D: HashNodeInner,
{
    Domain(HashNode<D>),
    Logical(HashNode<LogicalExpression<T, D, D::Operator>>),
}

impl<T: TruthValue, D: DomainContent<T>> DomainExpression<T, D>
where
    T: HashNodeInner,
    D: HashNodeInner,
{
    pub fn domain(content: HashNode<D>) -> Self {
        DomainExpression::Domain(content)
    }

    pub fn logical(expr: HashNode<LogicalExpression<T, D, D::Operator>>) -> Self {
        DomainExpression::Logical(expr)
    }

    pub fn is_domain(&self) -> bool {
        matches!(self, DomainExpression::Domain(_))
    }

    pub fn is_logical(&self) -> bool {
        matches!(self, DomainExpression::Logical(_))
    }

    pub fn as_domain(&self) -> Option<&HashNode<D>> {
        match self {
            DomainExpression::Domain(content) => Some(content),
            _ => None,
        }
    }

    pub fn as_logical(
        &self,
        storage: &NodeStorage<LogicalExpression<T, D, D::Operator>>,
    ) -> HashNode<LogicalExpression<T, D, D::Operator>> {
        match self {
            DomainExpression::Logical(expr) => expr.clone(),
            DomainExpression::Domain(domain) => {
                HashNode::from_store(LogicalExpression::atomic(domain.clone()), storage)
            }
        }
    }
}

pub trait DomainContent<T: TruthValue>
where
    Self: HashNodeInner,
    Self::Operator: HashNodeInner,
{
    type Operator: LogicalOperator<T>;
}

impl<T: TruthValue, D: DomainContent<T>> HashNodeInner for DomainExpression<T, D>
where
    T: HashNodeInner,
    D: HashNodeInner,
    D::Operator: HashNodeInner,
{
    fn hash(&self) -> u64 {
        match self {
            DomainExpression::Domain(content) => Hashing::root_hash(0, &[content.hash()]),
            DomainExpression::Logical(expr) => Hashing::root_hash(1, &[expr.hash()]),
        }
    }

    fn size(&self) -> u64 {
        match self {
            DomainExpression::Domain(content) => 1 + content.size(),
            DomainExpression::Logical(expr) => 1 + expr.size(),
        }
    }
}

impl<T: TruthValue, D: DomainContent<T>> Display for DomainExpression<T, D>
where
    T: Display + HashNodeInner,
    D: Display + HashNodeInner,
    D::Operator: Display + HashNodeInner,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DomainExpression::Domain(content) => write!(f, "{}", content),
            DomainExpression::Logical(expr) => write!(f, "{}", expr),
        }
    }
}
