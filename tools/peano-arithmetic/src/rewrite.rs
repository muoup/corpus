use corpus_core::nodes::{HashNode, NodeStorage};
use corpus_rewriting::RewriteRule;
use corpus_unification::{Pattern, Substitution};

use crate::syntax::ArithmeticExpression;

pub fn apply_rule(
    rule: &RewriteRule<ArithmeticExpression>,
    term: &HashNode<ArithmeticExpression>,
    store: &NodeStorage<ArithmeticExpression>,
) -> Option<HashNode<ArithmeticExpression>> {
    let subst = rule.try_match(term, store).ok()?;
    Some(apply_substitution_to_pattern(&rule.replacement, &subst, store))
}

pub fn apply_rule_reverse(
    rule: &RewriteRule<ArithmeticExpression>,
    term: &HashNode<ArithmeticExpression>,
    store: &NodeStorage<ArithmeticExpression>,
) -> Option<HashNode<ArithmeticExpression>> {
    let subst = rule.try_match_reverse(term, store).ok()?;
    Some(apply_substitution_to_pattern(&rule.pattern, &subst, store))
}

fn apply_substitution_to_pattern(
    pattern: &Pattern<ArithmeticExpression>,
    subst: &Substitution<ArithmeticExpression>,
    store: &NodeStorage<ArithmeticExpression>,
) -> HashNode<ArithmeticExpression> {
    match pattern {
        Pattern::Variable(idx) => subst
            .get(*idx)
            .cloned()
            .expect(&format!("Variable /{} should be bound in substitution", idx)),
        Pattern::Wildcard => {
            panic!("Wildcard should not appear in replacement pattern")
        }
        Pattern::Constant(c) => HashNode::from_store(c.clone(), store),
        Pattern::Compound { opcode, args } => {
            let substituted_args: Vec<HashNode<ArithmeticExpression>> = args
                .iter()
                .map(|arg| apply_substitution_to_pattern(arg, subst, store))
                .collect();
            construct_compound(*opcode, &substituted_args, store)
        }
    }
}

fn construct_compound(
    opcode: u8,
    args: &[HashNode<ArithmeticExpression>],
    store: &NodeStorage<ArithmeticExpression>,
) -> HashNode<ArithmeticExpression> {
    match opcode {
        8 if args.len() == 2 => {
            let expr = ArithmeticExpression::Add(args[0].clone(), args[1].clone());
            HashNode::from_store(expr, store)
        }
        10 if args.len() == 1 => {
            let inner_term = match &*args[0].value {
                ArithmeticExpression::Term(t) => t.clone(),
                other => {
                    eprintln!("Expected term for successor, got: {:?}", other);
                    panic!("Expected term for successor")
                }
            };
            let expr = ArithmeticExpression::Term(crate::syntax::Term::Successor(
                HashNode::from_store(inner_term, &NodeStorage::new()),
            ));
            HashNode::from_store(expr, store)
        }
        1 if args.len() == 2 => {
            let expr = ArithmeticExpression::Add(args[0].clone(), args[1].clone());
            HashNode::from_store(expr, store)
        }
        _ => panic!("Unexpected opcode: {} with {} args", opcode, args.len()),
    }
}
