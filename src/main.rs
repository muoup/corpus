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
        Ok(ast) => println!("Success: {}", ast),
        Err(e) => println!("Error: {}", e),
    }

    // Example 2: S(0) + S(0) = S(S(0))
    // EQ ( PLUS ( S(0) ) ( S(0) ) ) ( S(S(0)) )
    let input2 = "EQ ( PLUS ( S(0) ) ( S(0) ) ) ( S(S(0)) )";
    println!("\nParsing: {}", input2);
    let mut parser2 = Parser::new(input2);
    match parser2.parse_proposition() {
        Ok(ast) => println!("Success: {}", ast),
        Err(e) => println!("Error: {}", e),
    }
}