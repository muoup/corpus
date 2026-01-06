use std::{iter::Peekable, str::Chars};

use corpus_classical_logic::{BinaryTruth, ClassicalOperator, ClassicalLogicalExpression};
use corpus_core::nodes::{HashNode, NodeStorage};

use crate::syntax::{ArithmeticExpression, PeanoContent, PeanoLogicalExpression};

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    LParen,
    RParen,
    And,
    Or,
    Implies,
    Not,
    Forall,
    Exists,
    Eq,
    Plus,
    Successor,
    Number(u64),
    DeBruijn(u32),
}

pub struct Lexer<'a> {
    chars: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            chars: input.chars().peekable(),
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(&c) = self.chars.peek() {
            if c.is_whitespace() {
                self.chars.next();
            } else {
                break;
            }
        }
    }

    fn parse_number_or_debruijn(&mut self) -> Option<Token> {
        let mut s = String::new();
        let is_debruijn = if let Some(&'/') = self.chars.peek() {
            self.chars.next(); // consume '/'
            true
        } else {
            false
        };

        while let Some(&c) = self.chars.peek() {
            if c.is_ascii_digit() {
                s.push(self.chars.next().unwrap());
            } else {
                break;
            }
        }

        if s.is_empty() {
            return None; // Should not happen if called correctly
        }

        if is_debruijn {
            Some(Token::DeBruijn(s.parse().ok()?))
        } else {
            Some(Token::Number(s.parse().ok()?))
        }
    }

    fn parse_keyword_or_symbol(&mut self) -> Option<Token> {
        let c = self.chars.peek()?;
        if *c == '(' {
            self.chars.next();
            return Some(Token::LParen);
        }
        if *c == ')' {
            self.chars.next();
            return Some(Token::RParen);
        }

        // Symbols
        match *c {
            '∧' => {
                self.chars.next();
                return Some(Token::And);
            }
            '∨' => {
                self.chars.next();
                return Some(Token::Or);
            }
            '→' => {
                self.chars.next();
                return Some(Token::Implies);
            }
            '¬' => {
                self.chars.next();
                return Some(Token::Not);
            }
            '∀' => {
                self.chars.next();
                return Some(Token::Forall);
            }
            '∃' => {
                self.chars.next();
                return Some(Token::Exists);
            }
            '=' => {
                self.chars.next();
                return Some(Token::Eq);
            }
            '+' => {
                self.chars.next();
                return Some(Token::Plus);
            }
            _ => {}
        }

        // Multi-char symbols or keywords
        // Simple heuristic: read alphanumeric chars
        let mut s = String::new();
        while let Some(&peep) = self.chars.peek() {
            if peep.is_alphanumeric() || peep == '-' || peep == '>' {
                s.push(self.chars.next().unwrap());
            } else {
                break;
            }
        }

        match s.as_str() {
            "AND" => Some(Token::And),
            "OR" => Some(Token::Or),
            "IMPLIES" | "->" => Some(Token::Implies),
            "NOT" => Some(Token::Not),
            "FORALL" => Some(Token::Forall),
            "EXISTS" => Some(Token::Exists),
            "EQ" => Some(Token::Eq),
            "PLUS" => Some(Token::Plus),
            "S" => Some(Token::Successor), // 'S' is a keyword for Successor
            _ => None,                     // parsing error or empty
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.skip_whitespace();
        if let Some(&c) = self.chars.peek() {
            if c.is_ascii_digit() || c == '/' {
                return self.parse_number_or_debruijn();
            }
            return self.parse_keyword_or_symbol();
        }
        None
    }
}

pub struct Parser<'a> {
    tokens: Peekable<Lexer<'a>>,

    pub expression_store: NodeStorage<ArithmeticExpression>,
    pub content_store: NodeStorage<PeanoContent>,
    pub logical_store: NodeStorage<PeanoLogicalExpression>,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            tokens: Lexer::new(input).peekable(),
            expression_store: NodeStorage::new(),
            content_store: NodeStorage::new(),
            logical_store: NodeStorage::new(),
        }
    }

    fn expect(&mut self, expected: Token) -> Result<(), String> {
        match self.tokens.next() {
            Some(t) if t == expected => Ok(()),
            Some(t) => Err(format!("Expected {:?}, found {:?}", expected, t)),
            None => Err(format!("Expected {:?}, found EOF", expected)),
        }
    }

    // Helper to consume optional surrounding parentheses for an argument
    // The grammar says: <op> (<arg>) (<arg>)
    // So we basically expect a LParen, parse, then RParen.
    fn parse_parenthesized<F, T>(&mut self, parser: F) -> Result<T, String>
    where
        F: FnOnce(&mut Self) -> Result<T, String>,
    {
        self.expect(Token::LParen)?;
        let result = parser(self)?;
        self.expect(Token::RParen)?;
        Ok(result)
    }

    pub fn parse_proposition(&mut self) -> Result<HashNode<PeanoLogicalExpression>, String> {
        let token = self
            .tokens
            .next()
            .ok_or("Unexpected EOF expecting Proposition")?;
        match token {
            Token::And => {
                let left = self.parse_parenthesized(Self::parse_proposition)?;
                let right = self.parse_parenthesized(Self::parse_proposition)?;
                let logical_expr = ClassicalLogicalExpression::compound(
                    ClassicalOperator::And,
                    vec![
                        left,
                        right,
                    ],
                );
                let logical_node = HashNode::from_store(logical_expr, &self.logical_store);
                Ok(logical_node)
            }
            Token::Or => {
                let left = self.parse_parenthesized(Self::parse_proposition)?;
                let right = self.parse_parenthesized(Self::parse_proposition)?;
                let logical_expr = ClassicalLogicalExpression::compound(
                    ClassicalOperator::Or,
                    vec![
                        left,
                        right,
                    ],
                );
                let logical_node = HashNode::from_store(logical_expr, &self.logical_store);
                Ok(logical_node)
            }
            Token::Implies => {
                let left = self.parse_parenthesized(Self::parse_proposition)?;
                let right = self.parse_parenthesized(Self::parse_proposition)?;
                let logical_expr = ClassicalLogicalExpression::compound(
                    ClassicalOperator::Implies,
                    vec![
                        left,
                        right,
                    ],
                );
                let logical_node = HashNode::from_store(logical_expr, &self.logical_store);
                Ok(logical_node)
            }
            Token::Not => {
                let inner = self.parse_parenthesized(Self::parse_proposition)?;
                let logical_expr = ClassicalLogicalExpression::compound(
                    ClassicalOperator::Not,
                    vec![inner],
                );
                let logical_node = HashNode::from_store(logical_expr, &self.logical_store);
                Ok(logical_node)
            }
            Token::Forall => {
                let inner = self.parse_parenthesized(Self::parse_proposition)?;
                let logical_expr = ClassicalLogicalExpression::compound(
                    ClassicalOperator::Forall,
                    vec![inner],
                );
                let logical_node = HashNode::from_store(logical_expr, &self.logical_store);
                Ok(logical_node)
            }
            Token::Exists => {
                let inner = self.parse_parenthesized(Self::parse_proposition)?;
                let logical_expr = ClassicalLogicalExpression::compound(
                    ClassicalOperator::Exists,
                    vec![inner]
                );
                let logical_node = HashNode::from_store(logical_expr, &self.logical_store);
                Ok(logical_node)
            }
            Token::Eq => {
                let left = self.parse_parenthesized(Self::parse_expression)?;
                let right = self.parse_parenthesized(Self::parse_expression)?;
                let content_node = HashNode::from_store(PeanoContent::Equals(left, right), &self.content_store);
                let logical_expr = ClassicalLogicalExpression::atomic(content_node);
                let logical_node = HashNode::from_store(logical_expr, &self.logical_store);
                Ok(logical_node)
            }
            _ => Err(format!(
                "Unexpected token {:?} for start of Proposition",
                token
            )),
        }
    }

    pub fn parse_expression(&mut self) -> Result<HashNode<ArithmeticExpression>, String> {
        let token = self
            .tokens
            .peek()
            .cloned()
            .ok_or("Unexpected EOF expecting Expression")?;

        match token {
            Token::Plus => {
                self.tokens.next();
                let left = self.parse_parenthesized(Self::parse_expression)?;
                let right = self.parse_parenthesized(Self::parse_expression)?;
                let expr = ArithmeticExpression::Add(left, right);
                Ok(HashNode::from_store(expr, &self.expression_store))
            }
            Token::Successor => {
                self.tokens.next();
                let inner = self.parse_parenthesized(Self::parse_expression)?;
                let expr = ArithmeticExpression::Successor(inner);
                Ok(HashNode::from_store(expr, &self.expression_store))
            }
            Token::Number(n) => {
                self.tokens.next();
                let expr = ArithmeticExpression::Number(n);
                Ok(HashNode::from_store(expr, &self.expression_store))
            }
            Token::DeBruijn(n) => {
                self.tokens.next();
                let expr = ArithmeticExpression::DeBruijn(n);
                Ok(HashNode::from_store(expr, &self.expression_store))
            }
            _ => Err(format!(
                "Unexpected token {:?} for start of Expression",
                token
            )),
        }
    }

    pub fn store_stats(&self) -> (usize, usize, usize) {
        (
            self.content_store.len(),
            self.expression_store.len(),
            self.logical_store.len(),
        )
    }
}

// ============================================================================
// Axiom Parsing Support
// ============================================================================

/// Storage instances for axiom parsing.
///
/// This struct holds the various NodeStorage instances needed during
/// axiom parsing, allowing external management of storage lifetime.
pub struct AxiomStores {
    pub expression_store: NodeStorage<ArithmeticExpression>,
    pub content_store: NodeStorage<PeanoContent>,
    pub logical_store: NodeStorage<PeanoLogicalExpression>,
}

impl AxiomStores {
    pub fn new() -> Self {
        Self {
            expression_store: NodeStorage::new(),
            content_store: NodeStorage::new(),
            logical_store: NodeStorage::new(),
        }
    }
}

/// Parse an axiom from a string with explicit quantifiers.
///
/// # Syntax
/// - Quantifiers: `forall /0, forall /1.` or `∀/0, ∀/1.`
/// - De Bruijn indices: `/0`, `/1`, `/2`
/// - Arithmetic: `S(...)`, `+`, numbers
/// - Logical: `=`, `->` (impllication), `<->` (iff)
///
/// # Examples
/// ```ignore
/// // Successor injectivity
/// let axiom = parse_axiom(
///     "FORALL (FORALL (EQ (S (/0)) (S (/1)) -> EQ (/0) (/1)))",
///     "axiom2_successor_injectivity",
///     &stores
/// )?;
///
/// // Additive identity
/// let axiom = parse_axiom(
///     "FORALL (EQ (PLUS (/0) (0)) (/0))",
///     "axiom3_additive_identity",
///     &stores
/// )?;
/// ```
///
/// Note: The current implementation uses S-expression style parsing.
/// The syntax is: `<operator> (<operand>) (<operand>)`.
pub fn parse_axiom(
    input: &str,
    name: &str,
    _stores: &AxiomStores,
) -> Result<
    corpus_core::base::axioms::NamedAxiom<PeanoLogicalExpression>,
    corpus_core::base::axioms::AxiomError,
> {
    use corpus_core::base::axioms::{AxiomError, NamedAxiom};

    // Parse the input using the existing parser infrastructure
    let mut parser = Parser::new(input);

    // Try to parse as a proposition (logical expression)
    let logical_expr = parser.parse_proposition().map_err(|e| AxiomError::ParseError {
        message: e,
        position: None,
    })?;

    // Create the NamedAxiom with the ClassicalAxiomConverter
    Ok(NamedAxiom::new_with_converter(
        name,
        logical_expr,
        Box::new(corpus_classical_logic::axioms::ClassicalAxiomConverter),
    ))
}
