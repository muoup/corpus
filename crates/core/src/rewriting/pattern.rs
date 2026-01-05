use crate::base::nodes::HashNodeInner;
use std::fmt::{self, Debug, Display};

pub enum QuantifierType {
    ForAll,
    Exists,
}

pub enum Pattern<T: HashNodeInner + Clone> {
    Variable(u32),
    Wildcard,
    Constant(T),
    Compound {
        opcode: u64,
        args: Vec<Pattern<T>>,
    },
}

impl<T: HashNodeInner + Clone> Pattern<T> {
    pub fn var(index: u32) -> Self {
        Pattern::Variable(index)
    }

    pub fn wildcard() -> Self {
        Pattern::Wildcard
    }

    pub fn constant(value: T) -> Self {
        Pattern::Constant(value)
    }

    pub fn compound(opcode: u64, args: Vec<Pattern<T>>) -> Self {
        Pattern::Compound { opcode, args }
    }

    pub fn is_variable(&self) -> bool {
        matches!(self, Pattern::Variable(_))
    }

    pub fn is_wildcard(&self) -> bool {
        matches!(self, Pattern::Wildcard)
    }

    pub fn is_constant(&self) -> bool {
        matches!(self, Pattern::Constant(_))
    }

    pub fn is_compound(&self) -> bool {
        matches!(self, Pattern::Compound { .. })
    }

    pub fn vars(&self) -> Vec<u32> {
        let mut vars = Vec::new();
        self.collect_vars(&mut vars);
        vars
    }

    fn collect_vars(&self, vars: &mut Vec<u32>) {
        match self {
            Pattern::Variable(idx) => {
                if !vars.contains(idx) {
                    vars.push(*idx);
                }
            }
            Pattern::Wildcard => {}
            Pattern::Constant(_) => {}
            Pattern::Compound { args, .. } => {
                for arg in args {
                    arg.collect_vars(vars);
                }
            }
        }
    }

    pub fn size(&self) -> usize {
        match self {
            Pattern::Variable(_) => 1,
            Pattern::Wildcard => 1,
            Pattern::Constant(t) => t.size() as usize,
            Pattern::Compound { args, .. } => {
                1 + args.iter().map(|a| a.size()).sum::<usize>()
            }
        }
    }
}

impl<T: HashNodeInner + Clone> Clone for Pattern<T> {
    fn clone(&self) -> Self {
        match self {
            Pattern::Variable(idx) => Pattern::Variable(*idx),
            Pattern::Wildcard => Pattern::Wildcard,
            Pattern::Constant(c) => Pattern::Constant(c.clone()),
            Pattern::Compound { opcode, args } => Pattern::Compound {
                opcode: *opcode,
                args: args.clone(),
            },
        }
    }
}

impl<T: HashNodeInner + Clone + Display> Display for Pattern<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Pattern::Variable(idx) => write!(f, "/{}", idx),
            Pattern::Wildcard => write!(f, "_"),
            Pattern::Constant(t) => write!(f, "{}", t),
            Pattern::Compound { opcode, args } => {
                write!(f, "({} {})", opcode, args.iter().map(|a| format!("{}", a)).collect::<Vec<_>>().join(" "))
            }
        }
    }
}

impl<T: HashNodeInner + Clone + Debug> Debug for Pattern<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Pattern::Variable(idx) => write!(f, "Variable({})", idx),
            Pattern::Wildcard => write!(f, "Wildcard"),
            Pattern::Constant(t) => write!(f, "Constant({:?})", t),
            Pattern::Compound { opcode, args } => {
                write!(f, "Compound(opcode={}, args={:?})", opcode, args)
            }
        }
    }
}
