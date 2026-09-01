use std::collections::HashMap;

use crate::{
    diagnostics::Span,
    lexer::token::{TType, Token},
};

pub struct Lexer<'a> {
    pos: usize,
    keywords: HashMap<String, TType>,
    src: &'a str,
}

impl<'a> Lexer<'a> {
    pub fn new(src: &'a str) -> Self {
        let keywords = Self::load_keywords();
        Lexer {
            pos: 0,
            keywords,
            src,
        }
    }

    fn load_keywords() -> HashMap<String, TType> {
        let keywords = HashMap::from([("let".to_string(), TType::Let)]);
        keywords
    }

    fn current_char(&self) -> Option<char> {
        self.src.chars().nth(self.pos)
    }

    fn peek_char(&self) -> Option<char> {
        self.src.chars().nth(self.pos + 1)
    }

    fn advance(&mut self) {
        self.pos += 1;
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.current_char() {
            if ch.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        let start = self.pos;

        match self.current_char() {
            Some('+') => {
                self.advance();
                Token::new(TType::Plus, "+".to_string(), Span::new(start, self.pos))
            }
            Some('-') => {
                self.advance();
                Token::new(TType::Minus, "-".to_string(), Span::new(start, self.pos))
            }
            Some('*') => {
                self.advance();
                Token::new(TType::Asterisk, "*".to_string(), Span::new(start, self.pos))
            }
            Some('/') => {
                self.advance();
                Token::new(TType::Slash, "/".to_string(), Span::new(start, self.pos))
            }
            None =>Token::new(TType::End, "".to_string(), Span::new(start, self.pos)),
            _ => panic!("Invalid char"),
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        loop {
            let token = self.next_token();
            let is_end = token.token_type == TType::End;
            tokens.push(token);
            if is_end {
                break;
            }
        }
        tokens
    }
}
