//! Peano Arithmetic axioms using string-based parsing.
//!
//! This module defines PA axioms as concise string declarations that are
//! parsed into first-class `NamedAxiom` instances.

use crate::{PeanoStores, parsing::PeanoParser, syntax::PeanoLogicalExpression};
use corpus_core::rewriting::RewriteRule;

struct AxiomFormat {
    name: &'static str,
    content: &'static str,
}

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
/// - `∀ (<var>) (<expr>)` - universal quantifier
/// - `¬ (<expr>)` - negation
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
pub fn pa_axiom_rules(stores: &PeanoStores) -> Vec<RewriteRule<PeanoLogicalExpression>> {
    let axioms = vec![
        AxiomFormat {
            name: "axiom2_successor_injectivity",
            content: "FORALL (FORALL (IMPLIES (EQ (S (/0)) (S (/1))) (EQ (/0) (/1))))",
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
        .map(|axiom| generate_axiom_rewrites(axiom.name, axiom.content, stores))
        .flatten()
        .collect()
}

pub(crate) fn generate_axiom_rewrites(
    axiom_name: &str,
    axiom_content: &str,
    stores: &PeanoStores,
) -> Vec<RewriteRule<PeanoLogicalExpression>> {
    let parsed = PeanoParser::parse(axiom_content, stores).expect("Parsing failed");
    todo!("Should use standard pattern rewriting")
    
    // let rewrites = converter
    //     .convert_axiom(&parsed, axiom_name)
    //     .expect("Axiom conversion failed");

    // rewrites
}
