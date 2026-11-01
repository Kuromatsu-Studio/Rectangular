use std::collections::HashMap;

use crate::{
    diagnostics::{CompilerError, Diag, Span},
    lexer::token::{TType, Token, ends_expression},
};

pub struct Lexer<'a> {
    pos: usize,
    keywords: HashMap<String, TType>,
    src: &'a str,
    saw_newline: bool,
    diagnostics: Diag,
    pub corrupted: bool,
}

impl<'a> Lexer<'a> {
    pub fn new(src: &'a str, diagnostics: Diag) -> Self {
        let keywords = Self::load_keywords();
        Lexer {
            pos: 0,
            keywords,
            src,
            diagnostics,
            saw_newline: false,
            corrupted: false,
        }
    }

    fn load_keywords() -> HashMap<String, TType> {
        let keywords = HashMap::from([
            ("true".to_string(), TType::True),
            ("false".to_string(), TType::False),
            ("return".to_string(), TType::Return),
            ("struct".to_string(), TType::Struct),
            ("bind".to_string(), TType::Bind),
            ("if".to_string(), TType::If),
            ("else".to_string(), TType::Else),
            ("break".to_string(), TType::Break),
            ("continue".to_string(), TType::Continue),
            ("while".to_string(), TType::While),
            ("type".to_string(), TType::TypeKey),
            ("unit".to_string(), TType::UnitKey),
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
            ("bool".to_string(), TType::BoolKey),
        ]);
        keywords
    }

    fn char_at(&self, byte: usize) -> Option<char> {
        self.src.get(byte..)?.chars().next()
    }

    fn current_char(&self) -> Option<char> {
        self.char_at(self.pos)
    }

    fn peek_char(&self) -> Option<char> {
        if let Some(c) = self.current_char() {
            self.char_at(self.pos + c.len_utf8())
        } else {
            None
        }
    }

    fn advance(&mut self) {
        if let Some(c) = self.current_char() {
            self.pos += c.len_utf8();
        }
    }

    fn needs_semicolon(&self, prev: Option<&Token>, next: &Token) -> bool {
        let Some(prev) = prev else { return false };
        let at_line_break = self.saw_newline || next.token_type == TType::End;
        at_line_break
            && ends_expression(&prev.token_type)
            && !matches!(
                next.token_type,
                TType::Rbrace | TType::Rparen | TType::DotStar | TType::Rbracket | TType::Else
            )
    }

    fn skip_whitespace(&mut self) {
        self.saw_newline = false;
        while let Some(ch) = self.current_char() {
            if ch == '\n' {
                self.saw_newline = true;
                self.advance();
            } else if ch.is_whitespace() {
                self.advance();
            } else if ch == '/' && matches!(self.peek_char(), Some('/') | Some('*')) {
                self.skip_comment();
            } else {
                break;
            }
        }
    }

    fn skip_comment(&mut self) {
        let start = self.pos;
        self.advance(); // first '/'

        if self.current_char() == Some('/') {
            while let Some(c) = self.current_char() {
                if c == '\n' {
                    break;
                }
                self.advance();
            }
        } else {
            self.advance(); // the '*'
            let mut closed = false;
            while let Some(c) = self.current_char() {
                if c == '*' && self.peek_char() == Some('/') {
                    self.advance();
                    self.advance();
                    closed = true;
                    break;
                }
                if c == '\n' {
                    self.saw_newline = true;
                } // multi-line comment counts as a newline
                self.advance();
            }
            if !closed {
                self.report(
                    "Unterminated block comment".to_string(),
                    Some(Span::new(start, start + 2)),
                );
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
            self.report(
                "Invalid hex number: expected hex digit after '0x'".to_string(),
                Some(span.clone()),
            );
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
            let span = Span { start, end };
            self.report(
                "Invalid binary number: expected binary digit after '0b'".to_string(),
                Some(span.clone()),
            );
            return Token::new(TType::Illegal, lexeme, span);
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
            "i16" => TType::I16Literal,
            "u16" => TType::U16Literal,
            "i32" => TType::I32Literal,
            "u32" => TType::U32Literal,
            "i8" => TType::I8Literal,
            "u8" => TType::U8Literal,
            "" => TType::IntLiteral,
            _ => {
                self.report(
                    format!("Invalid integer suffix: '{}'", suffix),
                    Some(Span {
                        start: span.start,
                        end,
                    }),
                );
                TType::Illegal
            }
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

    fn read_escape(&mut self, esc_start: usize) -> Option<char> {
        let c = self.current_char()?; // EOF, the caller reports it
        if c == '\n' {
            // a backslash at end of line, don't swallow the newline
            self.report(
                "Unfinished escape sequence".to_string(),
                Some(Span::new(esc_start, self.pos)),
            );
            return None;
        }
        self.advance();
        match c {
            'n' => Some('\n'),
            't' => Some('\t'),
            'r' => Some('\r'),
            '0' => Some('\0'),
            '\\' => Some('\\'),
            '"' => Some('"'),
            '\'' => Some('\''),
            'u' => self.read_unicode_escape(esc_start),
            other => {
                self.report(
                    format!("Unknown escape sequence: '\\{}'", other),
                    Some(Span::new(esc_start, self.pos)),
                );
                None
            }
        }
    }

    fn read_unicode_escape(&mut self, esc_start: usize) -> Option<char> {
        if self.current_char() != Some('{') {
            self.report(
                "Expected '{' after '\\u'".to_string(),
                Some(Span::new(esc_start, self.pos)),
            );
            return None;
        }
        self.advance();

        let mut hex = String::new();
        while let Some(c) = self.current_char() {
            if c.is_ascii_hexdigit() {
                hex.push(c);
                self.advance();
            } else {
                break;
            }
        }

        // anything other than '}' here (a quote, a newline, junk) stops the scan
        // without consuming it, so the string can still find its own end
        if self.current_char() != Some('}') {
            self.report(
                "Invalid unicode escape: expected hex digits followed by '}'".to_string(),
                Some(Span::new(esc_start, self.pos)),
            );
            return None;
        }
        self.advance();

        if hex.is_empty() || hex.len() > 6 {
            self.report(
                "Invalid unicode escape: expected 1 to 6 hex digits".to_string(),
                Some(Span::new(esc_start, self.pos)),
            );
            return None;
        }

        match u32::from_str_radix(&hex, 16).ok().and_then(char::from_u32) {
            Some(c) => Some(c),
            None => {
                self.report(
                    format!("Invalid unicode codepoint: U+{}", hex),
                    Some(Span::new(esc_start, self.pos)),
                );
                None
            }
        }
    }

    fn read_string(&mut self, start: usize) -> Token {
        self.advance(); // opening quote
        let mut value = String::new();
        loop {
            match self.current_char() {
                Some('"') => {
                    self.advance();
                    return Token::new(TType::StringLiteral, value, Span::new(start, self.pos));
                }
                Some('\\') => {
                    let esc_start = self.pos;
                    self.advance();
                    if let Some(c) = self.read_escape(esc_start) {
                        value.push(c);
                    }
                }
                Some('\n') | None => {
                    self.report(
                        "Unterminated string literal".to_string(),
                        Some(Span::new(start, start + 1)),
                    );
                    return Token::new(TType::Illegal, value, Span::new(start, self.pos));
                }
                Some(c) => {
                    value.push(c);
                    self.advance();
                }
            }
        }
    }

    fn read_char(&mut self, start: usize) -> Token {
        self.advance(); // opening quote

        let mut bad_escape = false;

        let ch = match self.current_char() {
            None | Some('\n') => {
                self.report(
                    "Unterminated character literal".to_string(),
                    Some(Span::new(start, start + 1)),
                );
                return Token::new(TType::Illegal, String::new(), Span::new(start, self.pos));
            }
            Some('\'') => {
                self.advance();
                self.report(
                    "Empty character literal".to_string(),
                    Some(Span::new(start, self.pos)),
                );
                return Token::new(TType::Illegal, String::new(), Span::new(start, self.pos));
            }
            Some('\\') => {
                let esc_start = self.pos;
                self.advance();
                self.read_escape(esc_start).unwrap_or_else(|| {
                    bad_escape = true;
                    '\u{FFFD}'
                })
            }
            Some(c) => {
                self.advance();
                c
            }
        };

        if self.current_char() == Some('\'') {
            self.advance();
        } else {
            while let Some(c) = self.current_char() {
                if c == '\n' {
                    break;
                }
                self.advance();
                if c == '\'' {
                    break;
                }
            }
            self.report(
                "Character literal must hold exactly one character".to_string(),
                Some(Span::new(start, self.pos)),
            );
            return Token::new(TType::Illegal, ch.to_string(), Span::new(start, self.pos));
        }

        let suffix_start = self.pos;
        while let Some(c) = self.current_char() {
            if c.is_ascii_alphanumeric() {
                self.advance();
            } else {
                break;
            }
        }
        let suffix = self.src[suffix_start..self.pos].to_string();
        let span = Span::new(start, self.pos);

        if bad_escape {
            return Token::new(TType::Illegal, ch.to_string(), span);
        }

        let ttype = match suffix.as_str() {
            "" | "c8" => {
                if !ch.is_ascii() {
                    self.report(
                    format!("Character '{}' does not fit in c8 (the default); use the c16 or c32 suffix", ch),
                    Some(span.clone()),
                    );

                    TType::Illegal

                } else { TType::Char8Literal }
            }
            "c16" => {
                if ch.len_utf16() != 1 {
                    self.report(
                        format!("Character '{}' does not fit in c16; use the c32 suffix", ch),
                        Some(span.clone()),
                    );

                    TType::Illegal

                } else { TType::Char16Literal }
            }
            "c32" => TType::Char32Literal,
            _ => {
                self.report(
                    format!("Unknown character suffix '{}'", suffix),
                    Some(span.clone()),
                );
                TType::Illegal
            }
        };

        Token::new(ttype, ch.to_string(), span)
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
                if let Some('.') = self.current_char() {
                    self.advance();
                    Token::new(TType::Range, "..".to_string(), Span::new(start, self.pos))
                } else if let Some('*') = self.current_char() {
                    self.advance();
                    Token::new(TType::DotStar, ".*".to_string(), Span::new(start, self.pos))
                } else {
                    Token::new(TType::Dot, ".".to_string(), Span::new(start, self.pos))
                }
            }
            Some('|') => {
                self.advance();
                if let Some('|') = self.current_char() {
                    self.advance();
                    Token::new(
                        TType::DoublePipe,
                        "||".to_string(),
                        Span::new(start, self.pos),
                    )
                } else {
                    Token::new(TType::Pipe, "|".to_string(), Span::new(start, self.pos))
                }
            }
            Some('&') => {
                self.advance();
                if let Some('&') = self.current_char() {
                    self.advance();
                    Token::new(
                        TType::DoubleAmpersand,
                        "&&".to_string(),
                        Span::new(start, self.pos),
                    )
                } else {
                    Token::new(
                        TType::Ampersand,
                        "&".to_string(),
                        Span::new(start, self.pos),
                    )
                }
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
                } else if let Some('>') = self.current_char() {
                    self.advance();
                    Token::new(TType::Arrow, "->".to_string(), Span::new(start, self.pos))
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
            Some('[') => {
                self.advance();
                Token::new(TType::Lbracket, "[".to_string(), Span::new(start, self.pos))
            }
            Some(']') => {
                self.advance();
                Token::new(TType::Rbracket, "]".to_string(), Span::new(start, self.pos))
            }
            Some('*') => {
                self.advance();
                if let Some('=') = self.current_char() {
                    self.advance();
                    Token::new(
                        TType::MulAssign,
                        "*=".to_string(),
                        Span::new(start, self.pos),
                    )
                } else {
                    Token::new(TType::Asterisk, "*".to_string(), Span::new(start, self.pos))
                }
            }
            Some('/') => {
                self.advance();
                if let Some('=') = self.current_char() {
                    self.advance();
                    Token::new(
                        TType::DivAssign,
                        "/=".to_string(),
                        Span::new(start, self.pos),
                    )
                } else {
                    Token::new(TType::Slash, "/".to_string(), Span::new(start, self.pos))
                }
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
            Some('%') => {
                self.advance();
                Token::new(TType::Percent, "%".to_string(), Span::new(start, self.pos))
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
                } else if let Some('=') = self.current_char() {
                    self.advance();
                    Token::new(
                        TType::BindAssign,
                        ":=".to_string(),
                        Span::new(start, self.pos),
                    )
                } else {
                    Token::new(TType::Colon, ":".to_string(), Span::new(start, self.pos))
                }
            }
            Some('>') => {
                self.advance();
                if let Some('>') = self.current_char() {
                    self.advance();
                    Token::new(
                        TType::DoubleRAngle,
                        ">>".to_string(),
                        Span::new(start, self.pos),
                    )
                } else if let Some('=') = self.current_char() {
                    self.advance();
                    Token::new(TType::Gte, ">=".to_string(), Span::new(start, self.pos))
                } else {
                    Token::new(TType::Gt, ">".to_string(), Span::new(start, self.pos))
                }
            }
            Some('<') => {
                self.advance();
                if let Some('<') = self.current_char() {
                    self.advance();
                    Token::new(
                        TType::DoubleLAngle,
                        "<<".to_string(),
                        Span::new(start, self.pos),
                    )
                } else if let Some('=') = self.current_char() {
                    self.advance();
                    Token::new(TType::Lte, "<=".to_string(), Span::new(start, self.pos))
                } else {
                    Token::new(TType::Lt, "<".to_string(), Span::new(start, self.pos))
                }
            }
            Some('$') => {
                self.advance();
                if let Some('$') = self.current_char() {
                    self.advance();
                    Token::new(
                        TType::DoubleDollar,
                        "$$".to_string(),
                        Span::new(start, self.pos),
                    )
                } else {
                    Token::new(TType::Dollar, "$".to_string(), Span::new(start, self.pos))
                }
            }
            Some(';') => {
                self.advance();
                Token::new(
                    TType::Semicolon,
                    ";".to_string(),
                    Span::new(start, self.pos),
                )
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
            Some('"') => self.read_string(start),
            Some('\'') => self.read_char(start),
            None => Token::new(TType::End, "".to_string(), Span::new(start, self.pos)),
            Some(ch) => {
                let span = Span {
                    start,
                    end: self.pos + 1,
                };
                self.report(format!("Invalid character: '{}'", ch), Some(span.clone()));
                self.advance();
                Token::new(TType::Illegal, ch.to_string(), span)
            }
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();
        loop {
            let token = self.next_token();
            let is_end = token.token_type == TType::End;

            if self.needs_semicolon(tokens.last(), &token) {
                let at = tokens.last().map(|t| t.span.end).unwrap_or(0);
                tokens.push(Token::new(
                    TType::Semicolon,
                    ";".to_string(),
                    Span::new(at, at),
                ));
            }

            tokens.push(token);
            if is_end {
                break;
            }
        }
        tokens
    }

    pub fn report(&mut self, message: String, span: Option<Span>) {
        self.corrupted = true;
        self.diagnostics
            .borrow_mut()
            .report(CompilerError::error(message, span, None));
    }
}
