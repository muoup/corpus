use corpus_core::DomainExpression;
use corpus_core::base::nodes::HashNode;
use corpus_core::expression::LogicalExpression;
use corpus_core::proving::{Prover, SizeCostEstimator};
use peano_arithmetic::parsing::Parser;
use peano_arithmetic::goal::PeanoGoalChecker;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("Usage: {} <theorem>", args[0]);
        println!();
        println!("Example: {} \"EQ (PLUS (S(0)) (0)) (S(0))\"", args[0]);
        println!();
        std::process::exit(1);
    }

    let theorem = &args[1];
    println!("Parsing theorem: {}", theorem);

    let mut parser = Parser::new(theorem);
    
    match parser.parse_proposition() {
        Ok(proposition) => {
            println!("Theorem: {}", proposition);
            println!();

            println!("Searching for proof (max 10000 nodes)...");

            // Create the prover with PeanoGoalChecker (axiom-based goal checking)
            let goal_checker = PeanoGoalChecker::new();
            let prover = Prover::new(10000, SizeCostEstimator, goal_checker);
            
            let proposition_as_domain = match proposition.value.as_ref() {
                DomainExpression::Logical(logical_expr) => logical_expr.clone(),
                DomainExpression::Domain(domain) => HashNode::from_store(LogicalExpression::Atomic(domain.clone()), &parser.logical_store)
            };

            match prover.prove(&proposition_as_domain) {
                Some(_) => {
                    println!("");
                    println!("✓ Theorem proved!");
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
