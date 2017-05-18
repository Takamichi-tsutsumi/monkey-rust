pub struct Token {
    Type: TokenType,
    Literal: String,
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
