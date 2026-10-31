use crate::{
    ast::{ASTExpr, Precedence},
    diagnostics::{CompilerError, Diag, Span},
    lexer::{TType, Token},
};

pub struct Parser {
    tokens: Vec<Token>,
    current_pos: usize,
    pub last_end: usize,
    diagnostics: Diag,
    pub corrupted: bool,
}

impl Parser {
    pub fn new(tokens: Vec<Token>, diagnostics: Diag) -> Self {
        Parser {
            tokens,
            current_pos: 0,
            diagnostics,
            last_end: 0,
            corrupted: false,
        }
    }

    pub fn parse(&mut self) -> Vec<ASTExpr> {
        let mut stmts = Vec::new();
        loop {
            let tt = match self.current_token() {
                Some(t) => t.token_type.clone(),
                None => break,
            };

            if tt == TType::End {
                break;
            }
            if tt == TType::Semicolon {
                self.advance();
                continue;
            }

            let Some(stmt) = self.parse_expr(Precedence::Lowest) else {
                break;
            };
            stmts.push(stmt);

            match self.current_token().map(|t| t.token_type.clone()) {
                Some(TType::Semicolon) => self.advance(),
                Some(TType::End) | None => {}
                Some(_) => {
                    let span = self.current_token().map(|t| t.span.clone());
                    self.report("expected ';' or end of input".to_string(), span);
                    break;
                }
            }
        }
        stmts
    }
    pub fn advance(&mut self) {
        if self.current_pos < self.tokens.len() {
            self.last_end = self.tokens[self.current_pos].span.end;
            self.current_pos += 1;
        }
    }

    pub fn current_token(&self) -> Option<&Token> {
        self.tokens.get(self.current_pos)
    }

    pub fn peek_token(&self) -> Option<&Token> {
        self.tokens.get(self.current_pos + 1)
    }

    pub fn peek_nth(&self, n: usize) -> Option<&Token> {
        self.tokens.get(self.current_pos + n)
    }

    pub fn expect_token(&mut self, expected: TType) -> Option<()> {
        let token = self.current_token()?.clone();
        if token.token_type == expected {
            self.advance();
            Some(())
        } else {
            let span = token.span;
            self.report(
                format!("Expected {:?}, found {:?}", expected, token.token_type),
                Some(span),
            );
            None
        }
    }

    pub fn lparen_starts_function(&self) -> bool {
        let mut depth = 0;
        let mut i = self.current_pos;
        while let Some(t) = self.tokens.get(i) {
            match t.token_type {
                TType::Lparen => depth += 1,
                TType::Rparen => {
                    depth -= 1;
                    if depth == 0 {
                        return matches!(
                            self.tokens.get(i + 1).map(|t| &t.token_type),
                            Some(TType::Arrow) | Some(TType::Colon)
                        );
                    }
                }
                TType::End => return false,
                _ => {}
            }
            i += 1;
        }
        false
    }

    pub fn operand_missing(&self) -> bool {
        matches!(
            self.current_token().map(|t| &t.token_type),
            Some(
                TType::End
                    | TType::Semicolon
                    | TType::Assign
                    | TType::BindAssign
                    | TType::Comma
                    | TType::Rparen
                    | TType::Rbrace
                    | TType::Rbracket
                    | TType::Arrow
            ) | None
        )
    }

    pub fn report(&mut self, message: String, span: Option<Span>) {
        self.corrupted = true;
        self.diagnostics
            .borrow_mut()
            .report(CompilerError::error(message, span, None));
    }
}
