use crate::ast::{Expression, HashNode, Proposition, Term};
use std::iter::Peekable;
use std::str::Chars;

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
            '∧' => { self.chars.next(); return Some(Token::And); }
            '∨' => { self.chars.next(); return Some(Token::Or); }
            '→' => { self.chars.next(); return Some(Token::Implies); }
            '¬' => { self.chars.next(); return Some(Token::Not); }
            '∀' => { self.chars.next(); return Some(Token::Forall); }
            '∃' => { self.chars.next(); return Some(Token::Exists); }
            '=' => { self.chars.next(); return Some(Token::Eq); }
            '+' => { self.chars.next(); return Some(Token::Plus); }
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
            _ => None, // parsing error or empty
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
}

impl<'a> Parser<'a> {
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
    fn parse_parenthesized<F, T>(&mut self, parser: F) -> Result<T, String>
    where
        F: FnOnce(&mut Self) -> Result<T, String>,
    {
        self.expect(Token::LParen)?;
        let result = parser(self)?;
        self.expect(Token::RParen)?;
        Ok(result)
    }

    pub fn parse_proposition(&mut self) -> Result<HashNode<Proposition>, String> {
        let token = self.tokens.next().ok_or("Unexpected EOF expecting Proposition")?;
        match token {
            Token::And => {
                let left = self.parse_parenthesized(Self::parse_proposition)?;
                let right = self.parse_parenthesized(Self::parse_proposition)?;
                Ok(Proposition::And(left, right).into())
            }
            Token::Or => {
                let left = self.parse_parenthesized(Self::parse_proposition)?;
                let right = self.parse_parenthesized(Self::parse_proposition)?;
                Ok(Proposition::Or(left, right).into())
            }
            Token::Implies => {
                let left = self.parse_parenthesized(Self::parse_proposition)?;
                let right = self.parse_parenthesized(Self::parse_proposition)?;
                Ok(Proposition::Implies(left, right).into())
            }
            Token::Not => {
                let inner = self.parse_parenthesized(Self::parse_proposition)?;
                Ok(Proposition::Not(inner).into())
            }
            Token::Forall => {
                // Forall (<prop>)
                let inner = self.parse_parenthesized(Self::parse_proposition)?;
                Ok(Proposition::Forall(inner).into())
            }
            Token::Exists => {
                let inner = self.parse_parenthesized(Self::parse_proposition)?;
                Ok(Proposition::Exists(inner).into())
            }
            Token::Eq => {
                let left = self.parse_parenthesized(Self::parse_expression)?;
                let right = self.parse_parenthesized(Self::parse_expression)?;
                Ok(Proposition::Equals(left, right).into())
            }
            _ => Err(format!("Unexpected token {:?} for start of Proposition", token)),
        }
    }

    pub fn parse_expression(&mut self) -> Result<Expression, String> {
        // Peek to decide if it's an Op (Plus) or a Term start
        let token = self.tokens.peek().cloned().ok_or("Unexpected EOF expecting Expression")?;
        
        match token {
            Token::Plus => {
                self.tokens.next(); // consume PLUS
                let left = self.parse_parenthesized(Self::parse_expression)?;
                let right = self.parse_parenthesized(Self::parse_expression)?;
                Ok(Expression::Add(Box::new(left), Box::new(right)))
            }
            // S, Number, DeBruijn are Term starters
            Token::Successor | Token::Number(_) | Token::DeBruijn(_) => {
                let term = self.parse_term()?;
                Ok(Expression::Term(term))
            }
            _ => Err(format!("Unexpected token {:?} for start of Expression", token)),
        }
    }

    pub fn parse_term(&mut self) -> Result<Term, String> {
        let token = self.tokens.next().ok_or("Unexpected EOF expecting Term")?;
        match token {
            Token::Successor => {
                // S(<term>) - grammar says S(<term>) but other ops use ( arg )
                // Let's assume standard function call syntax S(term) or S (term)
                // The parse_parenthesized expects ( ... )
                let inner = self.parse_parenthesized(Self::parse_term)?;
                Ok(Term::Successor(Box::new(inner)))
            }
            Token::Number(n) => Ok(Term::Number(n)),
            Token::DeBruijn(n) => Ok(Term::DeBruijn(n)),
            _ => Err(format!("Unexpected token {:?} for Term", token)),
        }
    }
}
