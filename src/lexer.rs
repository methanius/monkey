mod token;

use token::Token;
pub struct Lexer<'a> {
    input: &'a str,
}

impl<'a> Lexer<'a> {
    pub fn new(code: &'a str) -> Self {
        Lexer { input: code }
    }

    pub fn next_token(&mut self) -> Token<'a> {
        let token = Token::from_string(self.input);
        let len_token = token.literal.chars().count();
        self.input = &self.input[len_token..];
        token
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use token::{Token, TokenType};
    #[test]
    fn test_next_token() {
        const INPUT: &str = "=+(){};";
        let mut lexer = Lexer::new(INPUT);
        let mut token_vec: Vec<Token> = vec![lexer.next_token()];
        while token_vec.last().unwrap().token_type != TokenType::EOF {
            token_vec.push(lexer.next_token())
        }

        assert_eq!(
            token_vec,
            vec![
                Token {
                    token_type: TokenType::ASSIGN,
                    literal: "="
                },
                Token {
                    token_type: TokenType::PLUS,
                    literal: "+"
                },
                Token {
                    token_type: TokenType::LPAREN,
                    literal: "("
                },
                Token {
                    token_type: TokenType::RPAREN,
                    literal: ")"
                },
                Token {
                    token_type: TokenType::LBRACE,
                    literal: "{"
                },
                Token {
                    token_type: TokenType::RBRACE,
                    literal: "}"
                },
                Token {
                    token_type: TokenType::COMMA,
                    literal: ","
                },
                Token {
                    token_type: TokenType::SEMICOLON,
                    literal: ";"
                },
                Token {
                    token_type: TokenType::EOF,
                    literal: ""
                },
            ]
        );
    }
}
