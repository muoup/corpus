use corpus_core::nodes::{HashNode, NodeStorage};
use corpus_unification::{Pattern, Substitution};

use crate::syntax::{ArithmeticExpression, Term};

pub fn apply_substitution(
    pattern: &Pattern<ArithmeticExpression>,
    subst: &Substitution<ArithmeticExpression>,
    store: &NodeStorage<ArithmeticExpression>,
) -> HashNode<ArithmeticExpression> {
    match pattern {
        Pattern::Variable(idx) => {
            let term = ArithmeticExpression::Term(create_debruijn_term(*idx));
            HashNode::from_store(term, store)
        }
        Pattern::Wildcard => {
            let term = ArithmeticExpression::Term(Term::Number(HashNode::from_store(0u64, &NodeStorage::new())));
            HashNode::from_store(term, store)
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
                8 if applied_args.len() == 2 => {
                    let term = ArithmeticExpression::Add(applied_args[0].clone(), applied_args[1].clone());
                    HashNode::from_store(term, store)
                }
                10 if applied_args.len() == 1 => {
                    let inner = match &*applied_args[0].value {
                        ArithmeticExpression::Term(t) => t.clone(),
                        _ => panic!("Expected term for successor"),
                    };
                    let term = ArithmeticExpression::Term(Term::Successor(HashNode::from_store(inner, &NodeStorage::new())));
                    HashNode::from_store(term, store)
                }
                _ => panic!("Unexpected opcode: {}", opcode),
            }
        }
    }
}

fn create_debruijn_term(idx: u32) -> Term {
    use corpus_core::nodes::NodeStorage;
    Term::DeBruijn(HashNode::from_store(idx, &NodeStorage::new()))
}