use corpus_core::nodes::{HashNode, NodeStorage};
use corpus_core::rewriting::RewriteRule;

use crate::syntax::ArithmeticExpression;
use crate::opcodes::PeanoOpcodeMapper;

pub fn apply_rule(
    rule: &RewriteRule<ArithmeticExpression, PeanoOpcodeMapper>,
    term: &HashNode<ArithmeticExpression>,
    store: &NodeStorage<ArithmeticExpression>,
) -> Option<HashNode<ArithmeticExpression>> {
    rule.apply(term, store)
}

pub fn apply_rule_reverse(
    rule: &RewriteRule<ArithmeticExpression, PeanoOpcodeMapper>,
    term: &HashNode<ArithmeticExpression>,
    store: &NodeStorage<ArithmeticExpression>,
) -> Option<HashNode<ArithmeticExpression>> {
    rule.apply_reverse(term, store)
}

pub fn rewrite_subterms(
    rules: &[RewriteRule<ArithmeticExpression, PeanoOpcodeMapper>],
    term: &HashNode<ArithmeticExpression>,
    store: &NodeStorage<ArithmeticExpression>,
) -> Vec<HashNode<ArithmeticExpression>> {
    let mut results = Vec::new();

    for rule in rules {
        if let Some(new_term) = apply_rule(rule, term, store) {
            results.push(new_term);
        }
        if let Some(new_term) = apply_rule_reverse(rule, term, store) {
            results.push(new_term);
        }
    }

    match term.value.as_ref() {
        ArithmeticExpression::Add(left, right) => {
            results.extend(rewrite_subterms(rules, left, store));
            results.extend(rewrite_subterms(rules, right, store));
        }
        ArithmeticExpression::Successor(inner) => {
            results.extend(rewrite_subterms(rules, inner, store));
        }
        ArithmeticExpression::Number(_) | ArithmeticExpression::DeBruijn(_) => {}
    }

    results
}
