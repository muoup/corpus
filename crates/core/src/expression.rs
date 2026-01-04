use crate::logic::LogicalOperator;
use crate::nodes::{HashNode, HashNodeInner, Hashing};
use crate::truth::TruthValue;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq)]
pub enum LogicalExpression<T: TruthValue, Op: LogicalOperator<T>>
where
    T: HashNodeInner,
    Op: HashNodeInner,
{
    Atomic(T),
    Compound {
        operator: Op,
        operands: Vec<HashNode<Self>>,
    },
}

impl<T: TruthValue, Op: LogicalOperator<T>> LogicalExpression<T, Op>
where
    T: HashNodeInner,
    Op: HashNodeInner,
{
    pub fn atomic(value: T) -> Self {
        LogicalExpression::Atomic(value)
    }

    pub fn compound(operator: Op, operands: Vec<HashNode<Self>>) -> Self {
        LogicalExpression::Compound { operator, operands }
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

    pub fn evaluate(&self) -> T {
        match self {
            LogicalExpression::Atomic(value) => value.clone(),
            LogicalExpression::Compound { operator, operands } => {
                let evaluated_operands: Vec<T> =
                    operands.iter().map(|node| node.value.evaluate()).collect();
                operator.apply(&evaluated_operands)
            }
        }
    }
}

impl<T: TruthValue, Op: LogicalOperator<T>> Display for LogicalExpression<T, Op>
where
    T: Display + HashNodeInner,
    Op: Display + HashNodeInner,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            LogicalExpression::Atomic(value) => write!(f, "{}", value),
            LogicalExpression::Compound { operator, operands } => {
                if operator.is_unary() {
                    write!(f, "({} {})", operator, &operands[0])
                } else if operator.is_binary() {
                    write!(f, "({} {} {})", operands[0], operator, operands[1])
                } else {
                    write!(
                        f,
                        "({} {})",
                        operator,
                        operands
                            .iter()
                            .map(|op| format!("{}", op))
                            .collect::<Vec<_>>()
                            .join(" ")
                    )
                }
            }
        }
    }
}

impl<T: TruthValue, Op: LogicalOperator<T>> HashNodeInner for LogicalExpression<T, Op>
where
    T: HashNodeInner,
    Op: HashNodeInner,
{
    fn hash(&self) -> u64 {
        match self {
            LogicalExpression::Atomic(value) => Hashing::root_hash(0, &[value.hash()]),
            LogicalExpression::Compound { operator, operands } => {
                let mut all_hashes = vec![operator.hash()];
                all_hashes.extend(operands.iter().map(|node| node.hash));
                Hashing::root_hash(1, &all_hashes)
            }
        }
    }

    fn size(&self) -> u64 {
        match self {
            LogicalExpression::Atomic(value) => 1 + value.size(),
            LogicalExpression::Compound { operator, operands } => {
                1 + operator.size() + operands.iter().map(|node| node.size()).sum::<u64>()
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum DomainExpression<T: TruthValue, D: DomainContent<T>>
where
    T: HashNodeInner,
    D: HashNodeInner,
{
    Domain(D),
    Logical(LogicalExpression<T, D::Operator>),
}

impl<T: TruthValue, D: DomainContent<T>> DomainExpression<T, D>
where
    T: HashNodeInner,
    D: HashNodeInner,
{
    pub fn domain(content: D) -> Self {
        DomainExpression::Domain(content)
    }

    pub fn logical(expr: LogicalExpression<T, D::Operator>) -> Self {
        DomainExpression::Logical(expr)
    }

    pub fn is_domain(&self) -> bool {
        matches!(self, DomainExpression::Domain(_))
    }

    pub fn is_logical(&self) -> bool {
        matches!(self, DomainExpression::Logical(_))
    }

    pub fn as_domain(&self) -> Option<&D> {
        match self {
            DomainExpression::Domain(content) => Some(content),
            _ => None,
        }
    }

    pub fn as_logical(&self) -> Option<&LogicalExpression<T, D::Operator>> {
        match self {
            DomainExpression::Logical(expr) => Some(expr),
            _ => None,
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
