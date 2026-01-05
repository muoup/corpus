use corpus_core::nodes::{Hashing};
use crate::syntax::ArithmeticExpression;
use corpus_core::rewriting::{RewriteRule, Pattern, RewriteDirection};

pub fn peano_arithmetic_rules() -> Vec<RewriteRule<ArithmeticExpression>> {
    vec![
        axiom2_rule(),
        axiom3_rule(),
        axiom4_rule(),
    ]
}

fn axiom2_rule() -> RewriteRule<ArithmeticExpression> {
    let sx = Pattern::compound(Hashing::opcode("successor"), vec![Pattern::var(0)]);
    let sy = Pattern::compound(Hashing::opcode("successor"), vec![Pattern::var(1)]);
    let pattern = Pattern::compound(Hashing::opcode("equals"), vec![sx.clone(), sy.clone()]);

    let x = Pattern::var(0);
    let y = Pattern::var(1);
    let replacement = Pattern::compound(Hashing::opcode("equals"), vec![x, y]);

    RewriteRule::bidirectional("axiom2_successor_injectivity", pattern, replacement)
}

fn axiom3_rule() -> RewriteRule<ArithmeticExpression> {
    let x = Pattern::var(0);
    let zero = Pattern::constant(ArithmeticExpression::Number(0));
    let pattern = Pattern::compound(Hashing::opcode("add"), vec![x.clone(), zero]);

    let replacement = x;

    RewriteRule::new("axiom3_additive_identity", pattern, replacement, RewriteDirection::Forward)
}

fn axiom4_rule() -> RewriteRule<ArithmeticExpression> {
    let x = Pattern::var(0);
    let y = Pattern::var(1);
    let sy = Pattern::compound(Hashing::opcode("successor"), vec![y.clone()]);
    let pattern = Pattern::compound(Hashing::opcode("add"), vec![x.clone(), sy]);

    let x_plus_y = Pattern::compound(Hashing::opcode("add"), vec![x, y]);
    let replacement = Pattern::compound(Hashing::opcode("successor"), vec![x_plus_y]);

    RewriteRule::new("axiom4_additive_successor", pattern, replacement, RewriteDirection::Forward)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_axiom_rules_creation() {
        let rules = peano_arithmetic_rules();
        assert_eq!(rules.len(), 3);
    }
}
