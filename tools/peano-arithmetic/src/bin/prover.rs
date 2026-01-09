use corpus_core::proving::{Prover, SizeCostEstimator};
use corpus_classical_logic::{BinaryTruth, ClassicalTruthChecker};
use peano_arithmetic::{axioms, parsing::PeanoParser, syntax::PeanoLogicalExpression};

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

    let mut parser = PeanoParser::new(theorem);
    let stores = peano_arithmetic::PeanoStores::new();
    
    match parser.parse_proposition(&stores) {
        Ok(proposition) => {
            println!("Theorem: {}", proposition);
            println!();

            println!("Searching for proof (max 10000 nodes)...");

            // Create the prover with PeanoGoalChecker (axiom-based goal checking)
            let goal_checker = ClassicalTruthChecker::new();
            let mut prover: Prover<PeanoLogicalExpression, SizeCostEstimator, BinaryTruth, _> =
                Prover::new(10000, SizeCostEstimator, goal_checker);

            // Load Peano Arithmetic axioms as rewrite rules
            let axiom_rules = axioms::pa_axiom_rules(&stores);
            prover.add_rules(axiom_rules);
            println!();

            match prover.prove(&stores.storage, proposition) {
                Some(result) => {
                    println!();
                    
                    match result.truth_result {
                        BinaryTruth::True => println!("✓ Theorem proved!"),
                        BinaryTruth::False => println!("✗ Theorem disproved!"),
                    };

                    // Print the rewrite path
                    println!();
                    println!("Proof path:");
                    for (i, step) in result.steps.iter().enumerate() {
                        println!("  {}. {} → {}  [{}]", i + 1, step.old_expr, step.new_expr, step.rule_name);
                    }

                    // Show final truth result
                    println!("  → {:?}  [Goal reached]", result.truth_result);
                }
                None => {
                    println!();
                    println!("? No conclusion found (max search depth reached)");
                }
            }
        }
        Err(e) => {
            eprintln!("Parse error: {}", e);
            std::process::exit(1);
        }
    }
}
