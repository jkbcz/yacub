use std::fs;

use crate::token::{Token, TokenType};

pub struct Scanner {
    source: String,
    current: usize,
    start: usize,
    line: usize,
    line_start: usize,
}

impl Scanner {
    pub fn new(filename: String) -> Result<Self, std::io::Error> {
        let source = fs::read_to_string(filename)?;

        Ok(Scanner {
            source,
            current: 0,
            start: 0,
            line: 1,
            line_start: 0,
        })
    }

    pub fn source(&self) -> &str {
        &self.source
    }

    pub fn scan_all(&mut self) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();

        loop {
            let token = self.scan()?;
            let is_eof = token.token_type == TokenType::Eof;
            tokens.push(token);

            if is_eof {
                break;
            }
        }

        Ok(tokens)
    }

    fn scan(&mut self) -> Result<Token, String> {
        self.start = self.current;

        if self.is_at_end() {
            return Ok(self.token(TokenType::Eof, None));
        }

        let ch = self.advance();
        match ch {
            '(' => Ok(self.token(TokenType::LeftParen, None)),
            ')' => Ok(self.token(TokenType::RightParen, None)),
            '-' => Ok(self.token(TokenType::Minus, None)),
            '+' => Ok(self.token(TokenType::Plus, None)),
            '*' => Ok(self.token(TokenType::Star, None)),
            '/' => Ok(self.token(TokenType::Slash, None)),
            '!' => {
                let typ = self.match2('=', TokenType::BangEqual, TokenType::Bang);
                Ok(self.token(typ, None))
            }
            '=' => {
                let typ = self.match2('=', TokenType::EqualEqual, TokenType::Equal);
                Ok(self.token(typ, None))
            }
            '<' => {
                let typ = self.match2('=', TokenType::LessEqual, TokenType::Less);
                Ok(self.token(typ, None))
            }
            '>' => {
                let typ = self.match2('=', TokenType::GreaterEqual, TokenType::Greater);
                Ok(self.token(typ, None))
            }
            ' ' | '\r' | '\t' => self.scan(), // Skip whitespace, recursively scan next token
            '\n' => {
                self.line += 1;
                self.line_start = self.current;
                self.scan() // Recursively scan next token
            }
            '0'..='9' => self.scan_number(),
            ch if ch.is_alphabetic() => self.scan_identifier(),
            _ => Err(self.error(format!("Invalid character: '{}'", ch))),
        }
    }

    fn advance(&mut self) -> char {
        let ch = self.source[self.current..].chars().next().unwrap();
        self.current += ch.len_utf8();
        ch
    }

    fn peek(&self) -> Option<char> {
        self.source[self.current..].chars().next()
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_number(&mut self) -> Result<Token, String> {
        while let Some(ch) = self.peek() {
            if ch.is_ascii_digit() {
                self.advance();
            } else {
                break;
            }
        }

        let lexeme = &self.source[self.start..self.current];

        let value = lexeme
            .parse::<i32>()
            .map_err(|e| self.error(format!("Invalid number: {}", e)))?;

        Ok(self.token(TokenType::Number, Some(value)))
    }

    fn scan_identifier(&mut self) -> Result<Token, String> {
        while let Some(ch) = self.peek() {
            if ch.is_alphanumeric() || ch == '_' {
                self.advance();
            } else {
                break;
            }
        }

        Ok(self.token(TokenType::Identifier, None))
    }

    fn match2(&mut self, expected: char, if_match: TokenType, if_not: TokenType) -> TokenType {
        if self.is_at_end() {
            return if_not;
        }

        if self.peek() != Some(expected) {
            return if_not;
        }

        self.advance();
        if_match
    }

    fn token(&self, typ: TokenType, literal: Option<i32>) -> Token {
        let lexeme = self.source[self.start..self.current].to_string();
        Token {
            token_type: typ,
            lexeme,
            literal,
            line: self.line,
            column: self.column(),
        }
    }

    fn column(&self) -> usize {
        self.start - self.line_start + 1
    }

    fn error(&self, msg: String) -> String {
        format!("{}:{}: {}", self.line, self.column(), msg)
    }
}
