//! Peano Arithmetic axioms using string-based parsing.
//!
//! This module defines PA axioms as concise string declarations that are
//! parsed into rewrite rules.

use crate::{
    PeanoStores,
    parsing::PeanoParser,
    syntax::PeanoLogicalExpression,
};
use corpus_classical_logic::BinaryTruth;
use corpus_core::{
    RewriteDirection,
    rewriting::{
        RewriteRule,
        patterns::{AsRewriteRules, Rewritable},
    },
};

struct AxiomFormat {
    name: &'static str,
    content: &'static str,
}

/// PA axioms as rewrite rules.
///
/// Uses string-based parsing for clean, readable axiom declarations.
///
/// # Syntax
/// - `EQ (<left>) (<right>)` - equality
/// - `PLUS (<left>) (<right>)` - addition
/// - `S (<arg>)` - successor function
/// - `/0`, `/1`, `/2` - De Bruijn indices for variables
/// - `FORALL (<expr>)` - universal quantifier
///
/// # Examples
/// ```ignore
/// // Additive identity: x + 0 = x
/// "FORALL (EQ (PLUS (/0) (0)) (/0))"
///
/// // Additive successor: x + S(y) = S(x + y)
/// "FORALL (FORALL (EQ (PLUS (/0) (S (/1))) (S (PLUS (/0) (/1)))))"
/// ```
///
/// Note:
/// - PA axioms use universal quantifiers which are stripped to extract equality patterns
/// - Each equality becomes a bidirectional rewrite rule
pub fn pa_axiom_rules(stores: &PeanoStores) -> Vec<RewriteRule<PeanoLogicalExpression>> {
    let axioms = vec![
        // Reflexivity: x = x
        // Pattern: EQ(x, x) -> Replacement: True
        AxiomFormat {
            name: "axiom1_reflexivity",
            content: "FORALL (EQ (/0) (/0))",
        },
        // Symmetry: if x = y, then y = x
        // Pattern: EQ(x, y) -> Replacement: EQ(y, x)
        AxiomFormat {
            name: "axiom2_symmetry",
            content: "FORALL (FORALL (IMPLIES (EQ (/0) (/1)) (EQ (/1) (/0))))",
        },
        // Additive identity: if x + 0 = y, then x = y
        // Pattern: EQ(PLUS(x, 0), y) -> Replacement: EQ(x, y)
        AxiomFormat {
            name: "axiom3_additive_identity",
            content: "FORALL (FORALL (IMPLIES (EQ (PLUS (/0) (0)) (/1)) (EQ (/0) (/1))))",
        },
        // Additive successor: if x + S(y) = z, then S(x + y) = z
        // Pattern: EQ(PLUS(x, S(y)), z) -> Replacement: EQ(S(PLUS(x, y)), z)
        AxiomFormat {
            name: "axiom4_additive_successor",
            content: "FORALL (FORALL (FORALL (IMPLIES (EQ (PLUS (/0) (S (/1))) (/2)) (EQ (S (PLUS (/0) (/1))) (/2)))))",
        },
        // No variable is the successor of itself: ¬(x = S(x))
        // Pattern: EQ(x, S(x)) -> Replacement: False
        AxiomFormat {
            name: "axiom5_variable_not_successor_of_self",
            content: "FORALL (NOT (EQ (/0) (S (/0))))",
        },
        // Successor equality: if S(x) = S(y), then x = y
        // Pattern: EQ(S(x), S(y)) -> Replacement: EQ(x, y)
        AxiomFormat {
            name: "axiom6_successor_equality",
            content: "FORALL (FORALL (IMPLIES (EQ (S (/0)) (S (/1))) (EQ (/0) (/1))))",
        }
    ];

    axioms
        .into_iter()
        .flat_map(|axiom| generate_axiom_rewrites(axiom.name, axiom.content, stores))
        .collect()
}

pub(crate) fn generate_axiom_rewrites(
    axiom_name: &str,
    axiom_content: &str,
    stores: &PeanoStores,
) -> Vec<RewriteRule<PeanoLogicalExpression>> {
    let parsed = PeanoParser::parse(axiom_content, stores).expect("Parsing failed");
    let mut pattern = parsed
        .value
        .decompose_to_rewrite_rules(axiom_name, &stores.storage);

    let truthy_pattern = parsed.value.decompose_to_pattern(&stores.storage);
    let truth =
        PeanoLogicalExpression::BooleanConstant(BinaryTruth::True).decompose_to_pattern(&stores.storage);

    pattern.push(RewriteRule::new(
        axiom_name,
        truthy_pattern,
        truth,
        RewriteDirection::Forward,
    ));

    pattern
}