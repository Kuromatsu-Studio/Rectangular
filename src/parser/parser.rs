use crate::{
    ast::{ASTType, Stmt},
    diagnostics::{CompilerError, Diag, Span},
    lexer::{TType, Token},
};

pub struct Parser {
    tokens: Vec<Token>,
    current_pos: usize,
    diagnostics: Diag,
    pub corrupted: bool,
}

impl Parser {
    pub fn new(tokens: Vec<Token>, diagnostics: Diag) -> Self {
        Parser {
            tokens,
            current_pos: 0,
            diagnostics,
            corrupted: false,
        }
    }

    pub fn parse(&mut self) -> Vec<Stmt> {
        let mut stmts = Vec::new();
        while self.current_pos < self.tokens.len() {
            if let Some(token) = self.current_token().clone() {
                if token.token_type == TType::End {
                    break;
                }
                if let Some(stmt) = self.parse_stmt() {
                    stmts.push(stmt);
                } else {
                    break;
                }
            }
        }
        stmts
    }

    pub fn advance(&mut self) {
        if self.current_pos < self.tokens.len() {
            self.current_pos += 1;
        }
    }

    pub fn current_token(&self) -> Option<&Token> {
        self.tokens.get(self.current_pos)
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

    pub fn parse_type(&mut self) -> Option<ASTType> {
        let ty_token = self.current_token()?.clone();
        let span = ty_token.span.clone();
        match ty_token.token_type {
            TType::I8Key
            | TType::U8Key
            | TType::I16Key
            | TType::U16Key
            | TType::I32Key
            | TType::U32Key => {
                self.advance();
                Some(ASTType::basic(&ty_token))
            }
            _ => {
                self.report(
                    format!("Invalid type token {:?}", ty_token.token_type),
                    Some(span.clone()),
                );
                None
            }
        }
    }

    pub fn report(&mut self, message: String, span: Option<Span>) {
        self.corrupted = true;
        self.diagnostics
            .borrow_mut()
            .report(CompilerError::error(message, span, None));
    }
}
