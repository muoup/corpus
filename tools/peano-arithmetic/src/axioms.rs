//! Peano Arithmetic axioms using string-based parsing.
//!
//! This module defines PA axioms as concise string declarations that are
//! parsed into first-class `NamedAxiom` instances.

use corpus_core::base::axioms::NamedAxiom;
use corpus_core::nodes::Hashing;
use corpus_core::rewriting::{Pattern, RewriteDirection, RewriteRule};
use corpus_classical_logic::{BinaryTruth, ClassicalOperator};
use crate::parsing::{parse_axiom, AxiomStores};
use crate::syntax::{ArithmeticExpression, PeanoContent};

/// PA axioms as first-class NamedAxiom instances.
///
/// Uses string-based parsing for clean, readable axiom declarations.
///
/// # Syntax
/// - `EQ (<left>) (<right>)` - equality
/// - `-> (<antecedent>) (<consequent>)` - implication
/// - `PLUS (<left>) (<right>)` - addition
/// - `S (<arg>)` - successor function
/// - `/0`, `/1`, `/2` - De Bruijn indices for variables
///
/// # Examples
/// ```ignore
/// // Successor injectivity: S(x) = S(y) -> x = y
/// "-> (EQ (S (/0)) (S (/1))) (EQ (/0) (/1))"
///
/// // Additive identity: x + 0 = x
/// "EQ (PLUS (/0) (0)) (/0)"
///
/// // Additive successor: x + S(y) = S(x + y)
/// "EQ (PLUS (/0) (S (/1))) (S (PLUS (/0) (/1)))"
/// ```
///
/// Note:
/// - PA axioms are implicitly universal (apply to all variable values)
/// - Implication `->` requires both antecedent and consequent to be
///   parenthesized separately
/// - Quantifiers are not needed in axiom strings since rewrite rules
///   implicitly apply universally
pub fn peano_arithmetic_axioms() -> Vec<NamedAxiom<BinaryTruth, PeanoContent, ClassicalOperator>> {
    let stores = AxiomStores::new();

    vec![
        // Axiom 2: Successor injectivity
        // S(x) = S(y) -> x = y
        parse_axiom(
            "-> (EQ (S (/0)) (S (/1))) (EQ (/0) (/1))",
            "axiom2_successor_injectivity",
            &stores,
        )
        .expect("Failed to parse axiom2_successor_injectivity"),

        // Axiom 3: Additive identity
        // x + 0 = x
        parse_axiom(
            "EQ (PLUS (/0) (0)) (/0)",
            "axiom3_additive_identity",
            &stores,
        )
        .expect("Failed to parse axiom3_additive_identity"),

        // Axiom 4: Additive successor
        // x + S(y) = S(x + y)
        parse_axiom(
            "EQ (PLUS (/0) (S (/1))) (S (PLUS (/0) (/1)))",
            "axiom4_additive_successor",
            &stores,
        )
        .expect("Failed to parse axiom4_additive_successor"),
    ]
}

/// Generate arithmetic rewrite rules from PA axioms.
///
/// This function bridges the gap between the conceptual axiom system
/// (which operates on LogicalExpressions) and the concrete arithmetic
/// rewrite rules needed by the PA prover (which operate on ArithmeticExpressions).
///
/// The rules are hard-coded patterns that correspond to the three PA axioms:
/// - Axiom 2: S(x) = S(y) -> x = y (successor injectivity)
/// - Axiom 3: x + 0 = x (additive identity)
/// - Axiom 4: x + S(y) = S(x + y) (additive successor)
pub fn peano_arithmetic_rules() -> Vec<RewriteRule<ArithmeticExpression>> {
    vec![
        // Axiom 2: S(x) = S(y) -> x = y (bidirectional)
        {
            let sx = Pattern::compound(Hashing::opcode("successor"), vec![Pattern::var(0)]);
            let sy = Pattern::compound(Hashing::opcode("successor"), vec![Pattern::var(1)]);
            let pattern = Pattern::compound(Hashing::opcode("equals"), vec![sx, sy]);

            let x = Pattern::var(0);
            let y = Pattern::var(1);
            let replacement = Pattern::compound(Hashing::opcode("equals"), vec![x, y]);

            RewriteRule::bidirectional("axiom2_successor_injectivity", pattern, replacement)
        },
        // Axiom 3: x + 0 = x (forward)
        {
            let x = Pattern::var(0);
            let zero = Pattern::constant(ArithmeticExpression::Number(0));
            let pattern = Pattern::compound(Hashing::opcode("add"), vec![x.clone(), zero]);

            let replacement = x;

            RewriteRule::new("axiom3_additive_identity", pattern, replacement, RewriteDirection::Forward)
        },
        // Axiom 4: x + S(y) = S(x + y) (forward)
        {
            let x = Pattern::var(0);
            let y = Pattern::var(1);
            let sy = Pattern::compound(Hashing::opcode("successor"), vec![y.clone()]);
            let pattern = Pattern::compound(Hashing::opcode("add"), vec![x.clone(), sy]);

            let x_plus_y = Pattern::compound(Hashing::opcode("add"), vec![x, y]);
            let replacement = Pattern::compound(Hashing::opcode("successor"), vec![x_plus_y]);

            RewriteRule::new("axiom4_additive_successor", pattern, replacement, RewriteDirection::Forward)
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use corpus_core::base::axioms::Axiom;

    #[test]
    fn test_axioms_creation() {
        let axioms = peano_arithmetic_axioms();
        assert_eq!(axioms.len(), 3);

        // Verify axiom names
        let names: Vec<_> = axioms.iter().map(|a| a.name()).collect();
        assert!(names.contains(&"axiom2_successor_injectivity"));
        assert!(names.contains(&"axiom3_additive_identity"));
        assert!(names.contains(&"axiom4_additive_successor"));
    }

    #[test]
    fn test_axioms_are_valid() {
        let axioms = peano_arithmetic_axioms();

        for axiom in axioms {
            assert!(axiom.is_valid(), "Axiom {} should be valid", axiom.name());
            assert!(axiom.operator().is_some(), "Axiom {} should have an operator", axiom.name());
        }
    }

    #[test]
    fn test_axioms_generate_rewrite_rules() {
        let axioms = peano_arithmetic_axioms();

        for axiom in axioms {
            let rules = axiom.to_rewrite_rules();
            // Each axiom should generate at least one rewrite rule
            assert!(!rules.is_empty(), "Axiom {} should generate rewrite rules", axiom.name());
        }
    }

    #[test]
    fn test_axiom2_successor_injectivity() {
        let stores = AxiomStores::new();
        let axiom = parse_axiom(
            "-> (EQ (S (/0)) (S (/1))) (EQ (/0) (/1))",
            "test_axiom2",
            &stores,
        )
        .expect("Failed to parse axiom2");

        assert_eq!(axiom.name(), "test_axiom2");
        assert!(axiom.is_valid());

        let rules = axiom.to_rewrite_rules();
        assert!(!rules.is_empty());
    }

    #[test]
    fn test_axiom3_additive_identity() {
        let stores = AxiomStores::new();
        let axiom = parse_axiom(
            "EQ (PLUS (/0) (0)) (/0)",
            "test_axiom3",
            &stores,
        )
        .expect("Failed to parse axiom3");

        assert_eq!(axiom.name(), "test_axiom3");
        assert!(axiom.is_valid());

        let rules = axiom.to_rewrite_rules();
        assert!(!rules.is_empty());
    }

    #[test]
    fn test_axiom4_additive_successor() {
        let stores = AxiomStores::new();
        let axiom = parse_axiom(
            "EQ (PLUS (/0) (S (/1))) (S (PLUS (/0) (/1)))",
            "test_axiom4",
            &stores,
        )
        .expect("Failed to parse axiom4");

        assert_eq!(axiom.name(), "test_axiom4");
        assert!(axiom.is_valid());

        let rules = axiom.to_rewrite_rules();
        assert!(!rules.is_empty());
    }

    #[test]
    fn test_parse_error_invalid_syntax() {
        let stores = AxiomStores::new();
        let result = parse_axiom("invalid syntax", "test", &stores);
        assert!(result.is_err());
    }
}
