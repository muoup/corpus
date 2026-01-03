mod ast;
mod parser;

use parser::Parser;

fn main() {
    // Example: ∀x (S(x) = 0)
    // In our grammar (De Bruijn, Prefix):
    // FORALL ( EQ ( S(/0) ) ( 0 ) )
    
    let input = "FORALL ( EQ ( S(/0) ) ( 0 ) )";
    println!("Parsing: {}", input);

    let mut parser = Parser::new(input);
    match parser.parse_proposition() {
        Ok(ast) => {
            println!("Success: {}", ast);
            let stats = parser.store_stats();
            println!("Store stats - Propositions: {}, Expressions: {}, Terms: {}, u64: {}, u32: {}", 
                     stats.0, stats.1, stats.2, stats.3, stats.4);
        },
        Err(e) => println!("Error: {}", e),
    }

    // Example 2: S(0) + S(0) = S(S(0))
    // EQ ( PLUS ( S(0) ) ( S(0) ) ) ( S(S(0)) )
    let input2 = "EQ ( PLUS ( S(0) ) ( S(0) ) ) ( S(S(0)) )";
    println!("\nParsing: {}", input2);
    let mut parser2 = Parser::new(input2);
    match parser2.parse_proposition() {
        Ok(ast) => {
            println!("Success: {}", ast);
            let stats = parser2.store_stats();
            println!("Store stats - Propositions: {}, Expressions: {}, Terms: {}, u64: {}, u32: {}", 
                     stats.0, stats.1, stats.2, stats.3, stats.4);
        },
        Err(e) => println!("Error: {}", e),
    }

    // Test deduplication: Parse the same expression in different parsers
    println!("\n=== Testing Deduplication ===");
    
    // Parse the same complex expression multiple times in the same parser
    let input3 = "EQ ( PLUS ( S(0) ) ( S(0) ) ) ( S(S(0)) )";
    println!("Parsing '{}' twice in same parser:", input3);
    
    let mut parser3 = Parser::new(input3);
    let prop1 = parser3.parse_proposition().unwrap();
    println!("Parse 1: {} (hash: {})", prop1, prop1.hash);
    
    let mut parser4 = Parser::new(input3);
    let prop2 = parser4.parse_proposition().unwrap();
    println!("Parse 2: {} (hash: {})", prop2, prop2.hash);
    
    println!("Same hash: {}", prop1.hash == prop2.hash);
    
    let stats3 = parser3.store_stats();
    let stats4 = parser4.store_stats();
    println!("Parser 3 stats - Propositions: {}, Expressions: {}, Terms: {}, u64: {}, u32: {}", 
             stats3.0, stats3.1, stats3.2, stats3.3, stats3.4);
    println!("Parser 4 stats - Propositions: {}, Expressions: {}, Terms: {}, u64: {}, u32: {}", 
             stats4.0, stats4.1, stats4.2, stats4.3, stats4.4);
    
    // Test deduplication within a single expression
    println!("\n=== Testing Internal Deduplication ===");
    let input4 = "EQ ( S(0) ) ( S(0) )";
    println!("Parsing '{}':", input4);
    let mut parser5 = Parser::new(input4);
    let prop3 = parser5.parse_proposition().unwrap();
    println!("Result: {}", prop3);
    
    let stats5 = parser5.store_stats();
    println!("Stats - Terms: {}, u64: {} (should be 2 terms, 1 u64)", stats5.2, stats5.3);
}