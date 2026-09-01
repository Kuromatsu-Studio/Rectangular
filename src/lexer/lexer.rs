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
        let keywords = HashMap::from([
            ("let".to_string(), TType::Let),
            ("mut".to_string(), TType::Mut),
            ("func".to_string(), TType::Func),
            ("union".to_string(), TType::Union),
            ("record".to_string(), TType::Record),
            ("alias".to_string(), TType::Alias),
            ("as".to_string(), TType::As),
            ("while".to_string(), TType::While),
            ("if".to_string(), TType::If),
            ("else".to_string(), TType::Else),
            ("match".to_string(), TType::Match),
            ("for".to_string(), TType::For),
            ("client".to_string(), TType::Client),
            ("server".to_string(), TType::Server),
            ("async".to_string(), TType::Async),
            ("await".to_string(), TType::Await),
            ("true".to_string(), TType::True),
            ("false".to_string(), TType::False),
            ("from".to_string(), TType::From),
            ("import".to_string(), TType::Import),
            ("return".to_string(), TType::Return),
            ("i8".to_string(), TType::I8Key),
            ("u8".to_string(), TType::U8Key),
            ("i16".to_string(), TType::I16Key),
            ("u16".to_string(), TType::U16Key),
            ("i32".to_string(), TType::I32Key),
            ("u32".to_string(), TType::U32Key),
            ("i64".to_string(), TType::I64Key),
            ("u64".to_string(), TType::U64Key),
            ("f32".to_string(), TType::F32Key),
            ("f64".to_string(), TType::F64Key),
            ("str".to_string(), TType::StrKey),
            ("bool".to_string(), TType::BoolKey),
        ]);
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

    fn read_identifier(&mut self) -> Token {
        let start = self.pos;
        while let Some(ch) = self.current_char() {
            if ch.is_ascii_alphanumeric() || ch == '_' {
                self.advance();
            } else {
                break;
            }
        }

        let end = self.pos;
        let lexeme = self.src[start..end].to_string();
        let span = Span { start, end };
        match self.keywords.get(&lexeme) {
            Some(ttype) => Token::new(ttype.clone(), lexeme, span),
            None => Token::new(TType::Identifier, lexeme, span),
        }
    }

    fn read_number(&mut self) -> Token {
        let start = self.pos;

        // Check for hex and binary
        if let Some('0') = self.current_char() {
            if let Some(next) = self.peek_char() {
                if next == 'x' || next == 'X' {
                    return self.read_hex();
                } else if next == 'b' || next == 'B' {
                    return self.read_binary();
                }
            }
        }

        // Read digits
        while let Some(ch) = self.current_char() {
            if ch.is_ascii_digit() {
                self.advance();
            } else if ch == '_' {
                self.advance();
            } else {
                break;
            }
        }

        // Check for float
        if let Some('.') = self.current_char() {
            self.advance();
            while let Some(ch) = self.current_char() {
                if ch.is_ascii_digit() {
                    self.advance();
                } else {
                    break;
                }
            }
            let end = self.pos;
            let lexeme = self.src[start..end].replace('_', "");
            return self.parse_float_suffix(lexeme, Span { start, end });
        }

        let end = self.pos;
        let lexeme = self.src[start..end].replace('_', "");
        self.parse_suffix(lexeme, Span { start, end })
    }

    fn read_hex(&mut self) -> Token {
        let start = self.pos;
        self.advance(); // 0
        self.advance(); // x

        let mut has_digit = false;
        while let Some(ch) = self.current_char() {
            if ch.is_ascii_hexdigit() {
                has_digit = true;
                self.advance();
            } else if ch == '_' {
                self.advance();
            } else {
                break;
            }
        }

        let end = self.pos;
        let lexeme = self.src[start..end].replace('_', "");

        if !has_digit {
            let span = Span { start, end };
            return Token::new(TType::Illegal, lexeme, span);
        }

        self.parse_suffix(lexeme, Span { start, end })
    }

    fn read_binary(&mut self) -> Token {
        let start = self.pos;
        self.advance(); // 0
        self.advance(); // b

        let mut has_digit = false;
        while let Some(ch) = self.current_char() {
            if ch == '0' || ch == '1' {
                has_digit = true;
                self.advance();
            } else if ch == '_' {
                self.advance();
            } else {
                break;
            }
        }

        let end = self.pos;
        let lexeme = self.src[start..end].replace('_', "");

        if !has_digit {
            let _span = Span { start, end };
            panic!("Invalid binary number: expected binary digit after '0b'",);
            //return Token::new(TType::Illegal, lexeme, span);
        }

        self.parse_suffix(lexeme, Span { start, end })
    }

    fn parse_suffix(&mut self, value: String, span: Span) -> Token {
        let start = self.pos;
        while let Some(ch) = self.current_char() {
            if ch.is_ascii_alphabetic() || ch.is_ascii_digit() {
                self.advance();
            } else {
                break;
            }
        }

        let end = self.pos;
        let suffix = self.src[start..end].to_string();

        let token_type = match suffix.as_str() {
            "i64" => TType::I64Literal,
            "u64" => TType::U64Literal,
            "i16" => TType::I64Literal,
            "u16" => TType::U64Literal,
            "i32" => TType::I32Literal,
            "u32" => TType::U32Literal,
            "i8" => TType::I8Literal,
            "u8" => TType::U8Literal,
            _ => TType::IntLiteral,
        };

        Token::new(
            token_type,
            value,
            Span {
                start: span.start,
                end,
            },
        )
    }

    fn parse_float_suffix(&mut self, value: String, span: Span) -> Token {
        let start = self.pos;
        while let Some(ch) = self.current_char() {
            if ch.is_ascii_alphabetic() || ch.is_ascii_digit() {
                self.advance();
            } else {
                break;
            }
        }

        let end = self.pos;
        let suffix = self.src[start..end].to_string();

        let token_type = match suffix.as_str() {
            "f64" => TType::F64Literal,
            "f32" => TType::F32Literal,
            _ => TType::FloatLiteral,
        };

        Token::new(
            token_type,
            value,
            Span {
                start: span.start,
                end,
            },
        )
    }

    fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        let start = self.pos;

        match self.current_char() {
            Some(ch) if ch.is_ascii_alphabetic() || ch == '_' => self.read_identifier(),
            Some(ch) if ch.is_ascii_digit() || ch == '0' => self.read_number(),
            Some('+') => {
                self.advance();
                if let Some('=') = self.current_char() {
                    self.advance();
                    Token::new(
                        TType::AddAssign,
                        "+=".to_string(),
                        Span::new(start, self.pos),
                    )
                } else {
                    Token::new(TType::Plus, "+".to_string(), Span::new(start, self.pos))
                }
            }
            Some('.') => {
                self.advance();
                Token::new(TType::Dot, ".".to_string(), Span::new(start, self.pos))
            }
            Some('|') => {
                self.advance();
                Token::new(TType::Pipe, "|".to_string(), Span::new(start, self.pos))
            }
            Some('&') => {
                self.advance();
                Token::new(
                    TType::Ampersand,
                    "&".to_string(),
                    Span::new(start, self.pos),
                )
            }
            Some('-') => {
                self.advance();
                if let Some('=') = self.current_char() {
                    self.advance();
                    Token::new(
                        TType::SubAssign,
                        "-=".to_string(),
                        Span::new(start, self.pos),
                    )
                } else {
                    Token::new(TType::Minus, "-".to_string(), Span::new(start, self.pos))
                }
            }
            Some('(') => {
                self.advance();
                Token::new(TType::Lparen, "(".to_string(), Span::new(start, self.pos))
            }
            Some(')') => {
                self.advance();
                Token::new(TType::Rparen, ")".to_string(), Span::new(start, self.pos))
            }
            Some('{') => {
                self.advance();
                Token::new(TType::Lbrace, "{".to_string(), Span::new(start, self.pos))
            }
            Some('}') => {
                self.advance();
                Token::new(TType::Rbrace, "}".to_string(), Span::new(start, self.pos))
            }
            Some('*') => {
                self.advance();
                Token::new(TType::Asterisk, "*".to_string(), Span::new(start, self.pos))
            }
            Some('/') => {
                self.advance();
                Token::new(TType::Slash, "/".to_string(), Span::new(start, self.pos))
            }
            Some(',') => {
                self.advance();
                Token::new(TType::Comma, ",".to_string(), Span::new(start, self.pos))
            }
            Some('!') => {
                self.advance();
                if let Some('=') = self.current_char() {
                    self.advance();
                    Token::new(TType::Neq, "!=".to_string(), Span::new(start, self.pos))
                } else {
                    Token::new(TType::Bang, "!".to_string(), Span::new(start, self.pos))
                }
            }
            Some(':') => {
                self.advance();
                if let Some(':') = self.current_char() {
                    self.advance();
                    Token::new(
                        TType::DoubleColon,
                        "::".to_string(),
                        Span::new(start, self.pos),
                    )
                } else {
                    Token::new(TType::Colon, ":".to_string(), Span::new(start, self.pos))
                }
            }
            Some('>') => {
                self.advance();
                if let Some('=') = self.current_char() {
                    self.advance();
                    Token::new(TType::Gte, ">=".to_string(), Span::new(start, self.pos))
                } else {
                    Token::new(TType::Gt, ">".to_string(), Span::new(start, self.pos))
                }
            }
            Some('<') => {
                self.advance();
                if let Some('=') = self.current_char() {
                    self.advance();
                    Token::new(TType::Lte, ">=".to_string(), Span::new(start, self.pos))
                } else {
                    Token::new(TType::Lt, ">".to_string(), Span::new(start, self.pos))
                }
            }
            Some('=') => {
                self.advance();
                if let Some('=') = self.current_char() {
                    self.advance();
                    Token::new(
                        TType::Equality,
                        "==".to_string(),
                        Span::new(start, self.pos),
                    )
                } else if let Some('>') = self.current_char() {
                    self.advance();
                    Token::new(
                        TType::FatArrow,
                        "=>".to_string(),
                        Span::new(start, self.pos),
                    )
                } else {
                    Token::new(TType::Assign, "=".to_string(), Span::new(start, self.pos))
                }
            }
            None => Token::new(TType::End, "".to_string(), Span::new(start, self.pos)),
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
