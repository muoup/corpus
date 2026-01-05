use corpus_core::base::nodes::{HashNode, NodeStorage};
use peano_arithmetic::parsing::Parser;
use peano_arithmetic::prover::{prove_pa, ProofResultExt};
use peano_arithmetic::syntax::{PeanoContent, PeanoExpression};
use peano_arithmetic::axioms::peano_arithmetic_rules;

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

            // Extract the PeanoContent (equality expression) from the DomainExpression
            let peano_content = match extract_equality_content(proposition) {
                Ok(content) => content,
                Err(e) => {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                }
            };

            println!("Theorem: {}", peano_content);
            println!();

            println!("Loading Peano axioms...");
            let arithmetic_rules = peano_arithmetic_rules();
            for rule in &arithmetic_rules {
                println!("  - {}", rule.name);
            }
            println!();

            // Create a NodeStorage for PeanoContent
            let store = NodeStorage::new();

            println!("Searching for proof (max 10000 nodes)...");
            match prove_pa(&peano_content, &store, 10000) {
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

fn extract_equality_content(
    proposition: HashNode<PeanoExpression>,
) -> Result<HashNode<PeanoContent>, String> {
    match proposition.value.as_domain() {
        Some(content) => Ok(content.clone()),
        None => Err("Theorem must be an equality (EQ ...).".to_string()),
    }
}
