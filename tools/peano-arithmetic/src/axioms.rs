use corpus_core::nodes::{HashNode, NodeStorage};
use crate::syntax::{ArithmeticExpression, Term};
use corpus_rewriting::RewriteRule;
use corpus_unification::Pattern;

pub fn peano_arithmetic_rules(
    store: &NodeStorage<ArithmeticExpression>,
) -> Vec<RewriteRule<ArithmeticExpression>> {
    let mut rules = vec![];

    // Axiom 1: ∀x ¬(S(x) = 0) - no rewrite rule needed (prevents S(x) → 0)

    // Axiom 2: ∀x ∀y (S(x) = S(y) → x = y)
    // Rewrite: (S(x) = S(y)) ↔ (x = y)
    // Note: This is bidirectional but only works when both sides are successor functions
    rules.push(axiom2_rule(store));

    // Axiom 3: ∀x (x + 0 = x)
    // Rewrite: (x + 0) ↔ x
    rules.push(axiom3_rule(store));

    // Axiom 4: ∀x ∀y (x + S(y) = S(x + y))
    // Rewrite: (x + S(y)) ↔ S(x + y)
    rules.push(axiom4_rule(store));

    rules
}

fn axiom2_rule(_store: &NodeStorage<ArithmeticExpression>) -> RewriteRule<ArithmeticExpression> {
    // Pattern: (S(x) = S(y))
    // Replacement: (x = y)
    let sx = Pattern::compound(10, vec![Pattern::var(0)]);
    let sy = Pattern::compound(10, vec![Pattern::var(1)]);
    let pattern = Pattern::compound(1, vec![sx.clone(), sy.clone()]);

    let x = Pattern::var(0);
    let y = Pattern::var(1);
    let replacement = Pattern::compound(1, vec![x, y]);

    RewriteRule::bidirectional("axiom2_successor_injectivity", pattern, replacement)
}

fn axiom3_rule(store: &NodeStorage<ArithmeticExpression>) -> RewriteRule<ArithmeticExpression> {
    // Pattern: (x + 0)
    // Replacement: x
    let x = Pattern::var(0);
    let zero = Pattern::constant(create_zero_term(store));
    let pattern = Pattern::compound(8, vec![x.clone(), zero]);

    let replacement = x;

    RewriteRule::bidirectional("axiom3_additive_identity", pattern, replacement)
}

fn axiom4_rule(_store: &NodeStorage<ArithmeticExpression>) -> RewriteRule<ArithmeticExpression> {
    // Pattern: (x + S(y))
    // Replacement: S(x + y)
    let x = Pattern::var(0);
    let y = Pattern::var(1);
    let sy = Pattern::compound(10, vec![y.clone()]);
    let pattern = Pattern::compound(8, vec![x.clone(), sy]);

    let x_plus_y = Pattern::compound(8, vec![x, y]);
    let replacement = Pattern::compound(10, vec![x_plus_y]);

    RewriteRule::bidirectional("axiom4_additive_successor", pattern, replacement)
}

fn create_zero_term(_store: &NodeStorage<ArithmeticExpression>) -> ArithmeticExpression {
    ArithmeticExpression::Term(Term::Number(HashNode::from_store(0u64, &NodeStorage::new())))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_axiom_rules_creation() {
        let store = NodeStorage::new();
        let rules = peano_arithmetic_rules(&store);
        assert_eq!(rules.len(), 3);
    }
}
