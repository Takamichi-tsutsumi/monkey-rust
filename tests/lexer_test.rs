extern crate monkey_lib;

use monkey_lib::token::*;
use monkey_lib::lexer::*;


#[test]
fn testNextToken() {
    let input = "=+(){},;";
    let tests: Vec<Token> = vec![Token::new(TokenType::ASSIGN, "="),
                                 Token::new(TokenType::PLUS, "+"),
                                 Token::new(TokenType::LPAREN, "("),
                                 Token::new(TokenType::RPAREN, ")"),
                                 Token::new(TokenType::LBRACE, "{"),
                                 Token::new(TokenType::RBRACE, "}"),
                                 Token::new(TokenType::COMMA, ","),
                                 Token::new(TokenType::SEMICOLON, ";"),
                                 Token::new(TokenType::EOF, "")];
    let l = Lexer::new(input);

    for i in 0..tests.len() {
        let tok: Token = l.NextToken();
        let tt: Token = tests[i];

        assert!(tok.token_type == tt.token_type,
                "tests[{}] - token type wrong. expected={}, got={}",
                i,
                tt.token_type,
                tok.token_type);
        assert!(tok.literal == tt.literal,
                "tests[{}] - literal wrong. expected={}, got={}",
                i,
                tt.literal,
                tok.literal);
    }
}
