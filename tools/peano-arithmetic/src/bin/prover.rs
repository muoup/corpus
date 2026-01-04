use corpus_core::base::nodes::HashNode;
use peano_arithmetic::axioms::peano_arithmetic_rules;
use peano_arithmetic::parsing::Parser;
use peano_arithmetic::prover::{create_prover, ProofResultExt};
use peano_arithmetic::syntax::{ArithmeticExpression, PeanoContent, PeanoExpression};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("Usage: {} <theorem>", args[0]);
        println!();
        println!("Example: {} \"EQ (PLUS (S(0)) (0)) (S(0))\"", args[0]);
        println!();
        println!("Theorem format: EQ (left) (right)");
        println!("  left, right: Peano arithmetic expressions");
        println!("  Operators: PLUS, S (successor), numbers (0, 1, 2, ...)");
        println!("  Variables: /0, /1, /2, ... (De Bruijn indices)");
        std::process::exit(1);
    }

    let theorem = &args[1];
    println!("Parsing theorem: {}", theorem);

    let mut parser = Parser::new(theorem);
    match parser.parse_proposition() {
        Ok(proposition) => {
            println!("Parsed: {}", proposition);
            println!();

            let (lhs, rhs) = match extract_equality_operands(proposition) {
                Ok(pair) => pair,
                Err(e) => {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                }
            };

            println!("LHS: {}", lhs);
            println!("RHS: {}", rhs);
            println!();

            let mut prover = create_prover(10000);

            println!("Loading Peano axioms...");
            for rule in peano_arithmetic_rules() {
                println!("  - {}", rule.name);
                prover.add_rule(rule);
            }
            println!();

            println!("Searching for proof (max 10000 nodes)...");
            match prover.prove(&lhs, &rhs) {
                Some(result) => {
                    println!();
                    result.print();
                }
                None => {
                    println!();
                    println!("✗ Could not prove theorem (reached limit)");
                }
            }
        }
        Err(e) => {
            eprintln!("Parse error: {}", e);
            std::process::exit(1);
        }
    }
}

fn extract_equality_operands(
    proposition: HashNode<PeanoExpression>,
) -> Result<
    (
        HashNode<ArithmeticExpression>,
        HashNode<ArithmeticExpression>,
    ),
    String,
> {
    match proposition.value.as_domain().map(|peano_expr| peano_expr.value.as_ref()) {
        Some(PeanoContent::Equals(left, right)) => Ok((left.clone(), right.clone())),
        None => Err("Theorem must be an equality (EQ ...).".to_string()),
    }
}
