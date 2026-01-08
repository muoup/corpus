use std::{iter::Peekable, str::Chars};

use corpus_classical_logic::ClassicalLogicalExpression;
use corpus_core::nodes::HashNode;

use crate::{
    PeanoStores,
    syntax::{PeanoArithmeticExpression, PeanoDomainExpression, PeanoLogicalExpression},
};

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

pub struct PeanoParser<'a> {
    tokens: Peekable<Lexer<'a>>,
}

impl<'a> PeanoParser<'a> {
    pub fn parse(
        input: &'a str,
        storage: &PeanoStores,
    ) -> Result<HashNode<PeanoLogicalExpression>, String> {
        let mut parser = PeanoParser::new(input);
        parser.parse_proposition(storage)
    }

    pub fn new(input: &'a str) -> Self {
        Self {
            tokens: Lexer::new(input).peekable(),
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
    fn parse_parenthesized<F, T>(&mut self, storage: &PeanoStores, parser: F) -> Result<T, String>
    where
        F: FnOnce(&mut Self, &PeanoStores) -> Result<T, String>,
    {
        self.expect(Token::LParen)?;
        let result = parser(self, storage)?;
        self.expect(Token::RParen)?;
        Ok(result)
    }

    pub fn parse_proposition(
        &mut self,
        storage: &PeanoStores,
    ) -> Result<HashNode<PeanoLogicalExpression>, String> {
        let token = self
            .tokens
            .next()
            .ok_or("Unexpected EOF expecting Proposition")?;
        match token {
            Token::And => {
                let left = self.parse_parenthesized(storage, Self::parse_proposition)?;
                let right = self.parse_parenthesized(storage, Self::parse_proposition)?;
                let logical_expr = ClassicalLogicalExpression::And(left, right);
                let logical_node = HashNode::from_store(logical_expr, &storage.storage.logical_storage);
                Ok(logical_node)
            }
            Token::Or => {
                let left = self.parse_parenthesized(storage, Self::parse_proposition)?;
                let right = self.parse_parenthesized(storage, Self::parse_proposition)?;
                let logical_expr = ClassicalLogicalExpression::Or(left, right);
                let logical_node = HashNode::from_store(logical_expr, &storage.storage.logical_storage);
                Ok(logical_node)
            }
            Token::Implies => {
                let left = self.parse_parenthesized(storage, Self::parse_proposition)?;
                let right = self.parse_parenthesized(storage, Self::parse_proposition)?;
                let logical_expr = ClassicalLogicalExpression::Imply(left, right);
                let logical_node = HashNode::from_store(logical_expr, &storage.storage.logical_storage);
                Ok(logical_node)
            }
            Token::Not => {
                let inner = self.parse_parenthesized(storage, Self::parse_proposition)?;
                let logical_expr = ClassicalLogicalExpression::Not(inner);
                let logical_node = HashNode::from_store(logical_expr, &storage.storage.logical_storage);
                Ok(logical_node)
            }
            Token::Forall => {
                let inner = self.parse_parenthesized(storage, Self::parse_proposition)?;
                let logical_expr = ClassicalLogicalExpression::ForAll(inner);
                let logical_node = HashNode::from_store(logical_expr, &storage.storage.logical_storage);
                Ok(logical_node)
            }
            Token::Exists => {
                let inner = self.parse_parenthesized(storage, Self::parse_proposition)?;
                let logical_expr = ClassicalLogicalExpression::Exists(inner);
                let logical_node = HashNode::from_store(logical_expr, &storage.storage.logical_storage);
                Ok(logical_node)
            }
            Token::Eq => {
                let left = self.parse_parenthesized(storage, Self::parse_arithmetic_expr)?;
                let right = self.parse_parenthesized(storage, Self::parse_arithmetic_expr)?;

                // Create domain-level Equality expression
                let domain_expr = PeanoDomainExpression::Equality(left, right);
                let domain_node = HashNode::from_store(domain_expr, &storage.pa_storage().domain_content_storage);

                // Wrap in DomainContent to create a logical expression
                let logical_expr = ClassicalLogicalExpression::DomainContent(domain_node);
                let logical_node = HashNode::from_store(logical_expr, &storage.storage.logical_storage);
                Ok(logical_node)
            }
            _ => Err(format!(
                "Unexpected token {:?} for start of Proposition",
                token
            )),
        }
    }

    pub fn parse_arithmetic_expr(
        &mut self,
        storage: &PeanoStores,
    ) -> Result<HashNode<PeanoArithmeticExpression>, String> {
        let token = self
            .tokens
            .peek()
            .cloned()
            .ok_or("Unexpected EOF expecting Expression")?;

        match token {
            Token::Plus => {
                self.tokens.next();
                let left = self.parse_parenthesized(storage, Self::parse_arithmetic_expr)?;
                let right = self.parse_parenthesized(storage, Self::parse_arithmetic_expr)?;
                let expr = PeanoArithmeticExpression::Add(left, right);
                Ok(HashNode::from_store(expr, &storage.pa_storage().arithmetic_storage))
            }
            Token::Successor => {
                self.tokens.next();
                let inner = self.parse_parenthesized(storage, Self::parse_arithmetic_expr)?;
                let expr = PeanoArithmeticExpression::Successor(inner);
                Ok(HashNode::from_store(expr, &storage.pa_storage().arithmetic_storage))
            }
            Token::Number(n) => {
                self.tokens.next();
                let expr = PeanoArithmeticExpression::Number(n);
                Ok(HashNode::from_store(expr, &storage.pa_storage().arithmetic_storage))
            }
            Token::DeBruijn(n) => {
                self.tokens.next();
                let expr = PeanoArithmeticExpression::DeBruijn(n);
                Ok(HashNode::from_store(expr, &storage.pa_storage().arithmetic_storage))
            }
            _ => Err(format!(
                "Unexpected token {:?} for start of Expression",
                token
            )),
        }
    }
}
