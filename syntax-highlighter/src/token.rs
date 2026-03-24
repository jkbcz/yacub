#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenType {
    LeftParen,
    RightParen,
    Minus,
    Plus,
    Slash,
    Star,

    // One or two character tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals
    Identifier,
    Number,

    Eof,
}

pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Option<i32>,
    pub line: usize,
    pub column: usize,
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}:{} {:?} '{}' {}",
            self.line,
            self.column,
            self.token_type,
            self.lexeme,
            self.literal.map_or(String::new(), |v| v.to_string())
        )
    }
}
