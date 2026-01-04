use corpus_core::nodes::{HashNode, NodeStorage, Hashing};
use corpus_core::rewriting::{Pattern, Substitution};

use crate::syntax::ArithmeticExpression;

pub fn apply_substitution(
    pattern: &Pattern<ArithmeticExpression>,
    subst: &Substitution<ArithmeticExpression>,
    store: &NodeStorage<ArithmeticExpression>,
) -> HashNode<ArithmeticExpression> {
    match pattern {
        Pattern::Variable(idx) => {
            subst.get(*idx).cloned().expect(&format!("Variable /{} should be bound in substitution", idx))
        }
        Pattern::Wildcard => {
            panic!("Wildcard should not appear in pattern")
        }
        Pattern::Constant(c) => {
            HashNode::from_store(c.clone(), store)
        }
        Pattern::Compound { opcode, args } => {
            let applied_args: Vec<HashNode<ArithmeticExpression>> = args
                .iter()
                .map(|arg| apply_substitution(arg, subst, store))
                .collect();

            match *opcode {
                o if o == Hashing::opcode("add") && applied_args.len() == 2 => {
                    let term = ArithmeticExpression::Add(applied_args[0].clone(), applied_args[1].clone());
                    HashNode::from_store(term, store)
                }
                o if o == Hashing::opcode("successor") && applied_args.len() == 1 => {
                    let term = ArithmeticExpression::Successor(applied_args[0].clone());
                    HashNode::from_store(term, store)
                }
                o if o == Hashing::opcode("equals") && applied_args.len() == 2 => {
                    let term = ArithmeticExpression::Add(applied_args[0].clone(), applied_args[1].clone());
                    HashNode::from_store(term, store)
                }
                _ => panic!("Unexpected opcode: {}", opcode),
            }
        }
    }
}
