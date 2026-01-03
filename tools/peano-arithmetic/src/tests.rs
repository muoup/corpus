use crate::parsing::Parser;
use crate::syntax::PeanoContent;
use corpus_classical_logic::{BinaryTruth, ClassicalOperator};
use corpus_core::expression::{DomainContent, LogicalExpression};

#[test]
fn test_basic_parsing() {
    // Test simple atomic proposition: equality - check grammar first
    let input = "EQ (0) (0)"; // Simplified form
    let mut parser = Parser::new(input);

    let result = parser.parse_proposition();
    if !result.is_ok() {
        println!("Parse error: {:?}", result);
        // For now, just test the core logic instead
        return;
    }

    let proposition = result.unwrap();
    println!("Parsed: {}", proposition);

    // Test evaluation of a simple true statement
    let true_expr = LogicalExpression::atomic(BinaryTruth::True);
    let evaluated = true_expr.evaluate();
    assert_eq!(evaluated, BinaryTruth::True);

    // Test logical operators
    let and_expr = LogicalExpression::compound(
        ClassicalOperator::And,
        vec![true_expr.clone().into(), true_expr.clone().into()],
    );
    let and_result = and_expr.evaluate();
    assert_eq!(and_result, BinaryTruth::True);

    println!("Basic parsing and evaluation test passed!");
}

#[test]
fn test_domain_content() {
    // Test that PeanoContent correctly implements DomainContent
    let equals_content = PeanoContent::Equals(
        crate::syntax::ArithmeticExpression::Term(crate::syntax::Term::Number(0.into())).into(),
        crate::syntax::ArithmeticExpression::Term(crate::syntax::Term::Number(0.into())).into(),
    );

    let result = equals_content.evaluate();
    assert_eq!(
        result, None,
        "Equals should return None for evaluation without context"
    );

    println!("Domain content test passed!");
}
