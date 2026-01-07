//! Peano Arithmetic axioms using string-based parsing.
//!
//! This module defines PA axioms as concise string declarations that are
//! parsed into rewrite rules.

use crate::{PeanoStores, parsing::PeanoParser, syntax::PeanoLogicalExpression};
use corpus_core::rewriting::{RewriteRule, patterns::AsRewriteRules};

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
        AxiomFormat {
            name: "axiom1_reflexivity",
            content: "FORALL (EQ (/0) (/0))",
        },
        AxiomFormat {
            name: "axiom2_symmetry",
            content: "FORALL (FORALL (IMPLIES (EQ (/0) (/1)) (EQ (/1) (/0))))",
        },
        AxiomFormat {
            name: "axiom3_additive_identity",
            content: "FORALL (EQ (PLUS (/0) (0)) (/0))",
        },
        AxiomFormat {
            name: "axiom4_additive_successor",
            content: "FORALL (FORALL (EQ (PLUS (/0) (S (/1))) (S (PLUS (/0) (/1)))))",
        },
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
    let pattern = parsed
        .value
        .decompose_to_rewrite_rules(axiom_name, &stores.storage);
    
    pattern
}
