use std::fmt;

pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}

impl Token {
    pub fn new(typ: TokenType, literal: &str) -> Token {
        Token {
            token_type: typ,
            literal: literal.to_string(),
        }
    }
}

pub enum TokenType {
    ILLEGAL,
    EOF,

    //  Identifiers, Literals
    IDENT,
    INT,

    // Operators
    ASSIGN,
    PLUS,

    // Delimiters
    COMMA,
    SEMICOLON,

    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,

    // Keywords
    FUNCTION,
    LET,
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", "TOKEN")
    }
}

impl PartialEq for TokenType {
    fn eq(&self, other: &TokenType) -> bool {
        self == other
    }
}
