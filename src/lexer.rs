mod token;

use core::panic;

use token::Token;
pub struct Lexer<'lexing> {
    input: &'lexing str,
}

impl<'lexing> Lexer<'lexing> {
    pub fn new(code: &'lexing str) -> Self {
        Lexer { input: code }
    }

    pub fn next_token(self) -> (Token<'lexing>, Lexer<'lexing>) {
        let lexer = self.skip_whitespace();
        if let Some(c) = lexer.input.chars().next() {
            let advanced_lexer = Lexer::new(&lexer.input[1..]);
            let (token, lexer) = match c {
                '=' => (Token::Assign, advanced_lexer),
                '+' => (Token::Plus, advanced_lexer),
                ',' => (Token::Comma, advanced_lexer),
                ';' => (Token::Semicolon, advanced_lexer),
                '(' => (Token::Lparen, advanced_lexer),
                ')' => (Token::Rparen, advanced_lexer),
                '{' => (Token::Lbrace, advanced_lexer),
                '}' => (Token::Rbrace, advanced_lexer),
                '0' => (Token::Eof, advanced_lexer),
                x if x.is_ascii_alphabetic() => {
                    let ident_name_len: usize =
                        lexer.input.chars().take_while(is_valid_ident_char).count();
                    let ident_name = &lexer.input[..ident_name_len];
                    let advanced_lexer = Lexer::new(&lexer.input[ident_name_len..]);
                    let token_match: Token = KEYWORD_TOKENS
                        .iter()
                        .filter(|(word, _)| *word == ident_name)
                        .map(|t| t.1.clone())
                        .next()
                        .unwrap_or(Token::Ident(ident_name));
                    (token_match, advanced_lexer)
                }
                '0'..='9' => {
                    let integer_len = lexer
                        .input
                        .chars()
                        .take_while(|c| c.is_ascii_digit())
                        .count();
                    (
                        Token::Int(&lexer.input[..integer_len]),
                        Lexer::new(&lexer.input[integer_len..]),
                    )
                }
                _ => (Token::Illegal(&lexer.input[..1]), advanced_lexer),
            };
            (token, lexer)
        } else {
            (Token::Eof, Lexer::new(""))
        }
    }

    fn skip_whitespace(self) -> Self {
        Lexer::new(&self.input[self.input.chars().take_while(is_whitespace).count()..])
    }
}

const KEYWORD_TOKENS: [(&str, Token); 2] = [("fn", Token::Function), ("let", Token::Let)];

fn is_whitespace(c: &char) -> bool {
    *c == ' ' || *c == '\t' || *c == '\n' || *c == '\r'
}

fn is_valid_ident_char(c: &char) -> bool {
    c.is_ascii_alphabetic() || *c == '_'
}

pub fn lex(code: &str) -> Vec<Token> {
    let mut lexer = Lexer::new(code);
    let mut token;
    let mut token_vec: Vec<Token> = vec![];
    loop {
        (token, lexer) = lexer.next_token();
        token_vec.push(token);
        match token_vec.last().unwrap() {
            Token::Eof => return token_vec,
            Token::Illegal(c) => panic!("Illegal token encountered, char {c:?}"),
            _ => (),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use token::Token;
    #[test]
    fn test_next_single_literal_token() {
        const INPUT: &str = "=+(){},;";
        let token_vec = lex(INPUT);
        assert_eq!(
            token_vec,
            vec![
                Token::Assign,
                Token::Plus,
                Token::Lparen,
                Token::Rparen,
                Token::Lbrace,
                Token::Rbrace,
                Token::Comma,
                Token::Semicolon,
                Token::Eof,
            ]
        );
    }

    #[test]
    fn test_next_multi_literal_token() {
        const INPUT: &str = "let five = 5;
    let ten = 10;
    let add = fn(x, y) {
    x + y;
    };
    let result = add(five, ten);";
        let token_vec = lex(INPUT);
        assert_eq!(
            token_vec,
            vec![
                Token::Let,
                Token::Ident("five"),
                Token::Assign,
                Token::Int("5"),
                Token::Semicolon,
                Token::Let,
                Token::Ident("ten"),
                Token::Assign,
                Token::Int("10"),
                Token::Semicolon,
                Token::Let,
                Token::Ident("add"),
                Token::Assign,
                Token::Function,
                Token::Lparen,
                Token::Ident("x"),
                Token::Comma,
                Token::Ident("y"),
                Token::Rparen,
                Token::Lbrace,
                Token::Ident("x"),
                Token::Plus,
                Token::Ident("y"),
                Token::Semicolon,
                Token::Rbrace,
                Token::Semicolon,
                Token::Let,
                Token::Ident("result"),
                Token::Assign,
                Token::Ident("add"),
                Token::Lparen,
                Token::Ident("five"),
                Token::Comma,
                Token::Ident("ten"),
                Token::Rparen,
                Token::Semicolon,
                Token::Eof,
            ]
        )
    }
}
