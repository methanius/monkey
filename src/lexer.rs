mod token;

pub mod lexer {
    use super::token::{Token, TokenType};
    struct Lexer<'a> {
        input: &'a str,
    }

    impl<'a> Lexer<'a> {
        pub fn new(code: &'a str) -> Self {
            Lexer { input: code }
        }
        pub fn read_next_char(self) -> &'a str {
            &self.input[..1]
        }

        pub fn next_token(code: &'a str) -> (Token<'a>, Lexer<'a>) {
            let token = Token::from_string(code);
            (token, Lexer::new(&code[..]))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use token::TokenType;
    #[test]
    fn test_next_token() {
        const INPUT: &str = "=+(){};";
        assert_eq!(
            lexer::lex(INPUT),
            vec![
                (TokenType::ASSIGN, "="),
                (TokenType::PLUS, "+"),
                (TokenType::LPAREN, "("),
                (TokenType::RPAREN, ")"),
                (TokenType::LBRACE, "{"),
                (TokenType::RBRACE, "}"),
                (TokenType::COMMA, ","),
                (TokenType::SEMICOLON, ";"),
                (TokenType::EOF, "")
            ]
        );
    }
}
