use crate::{
    ast::{ASTParam, ASTStmt, FuncTarget, Precedence, StmtKind},
    diagnostics::Span,
    lexer::TType,
    parser::parser::Parser,
};

impl Parser {
    pub fn parse_stmt(&mut self) -> Option<ASTStmt> {
        let token = self.current_token()?.clone();
        let ttype = token.token_type;
        let t_span = token.span;
        match ttype {
            TType::Enum => self.parse_enum(),
            TType::Record => self.parse_record(),
            TType::Let => self.parse_let_stmt(),
            TType::Func => self.parse_func(),
            TType::Server | TType::Client => self.parse_target(),
            _ => {
                self.report(
                    format!("Invalid statement encuntered {:?}", ttype),
                    Some(t_span),
                );
                None
            }
        }
    }

    fn parse_enum(&mut self) -> Option<ASTStmt> {
        let start = self.current_token()?.span.start;
        self.expect_token(TType::Enum)?;
        let name = self.parse_identifier()?;
        self.expect_token(TType::Lbrace)?;
        let mut fields = Vec::new();
        while self.current_token()?.token_type != TType::End
            && self.current_token()?.token_type != TType::Rbrace
        {
            let field = self.parse_expr(Precedence::Lowest)?;
            fields.push(field);
            if self.current_token()?.token_type == TType::Comma {
                self.advance();
            }
        }
        let end = self.current_token()?.span.end;
        self.expect_token(TType::Rbrace);
        let span = Span { start, end };

        Some(ASTStmt::new(
            StmtKind::Enum {
                name: Box::new(name),
                block: fields,
            },
            span,
        ))
    }

    pub fn parse_param(&mut self) -> Option<ASTParam> {
        let name = self.parse_identifier()?;
        self.expect_token(TType::Colon)?;
        let ty = self.parse_type()?;
        Some(ASTParam {
            name: Box::new(name),
            ty: Box::new(ty),
        })
    }

    fn parse_func(&mut self) -> Option<ASTStmt> {
        let start = self.current_token()?.span.start;
        self.expect_token(TType::Func)?;
        let name = self.parse_identifier()?;
        let mut params = Vec::new();
        if self.current_token()?.token_type == TType::Lparen {
            self.advance();
            while self.current_token()?.token_type != TType::Rparen
                && self.current_token()?.token_type != TType::End
            {
                let param = self.parse_param()?;
                params.push(param);
                if self.current_token()?.token_type == TType::Comma {
                    self.advance();
                }
            }
            self.expect_token(TType::Rparen)?;
        }

        let ret_ty = if self.current_token()?.token_type == TType::Colon {
            self.advance();
            Some(self.parse_type()?)
        } else {
            None
        };

        let body = self.parse_block()?;
        let end = self.current_token()?.span.end;
        let span = Span::new(start, end);
        Some(ASTStmt::new(
            StmtKind::FuncDef {
                target: FuncTarget::None,
                name: Box::new(name),
                params,
                ret_ty: Box::new(ret_ty),
                body: Box::new(body),
            },
            span,
        ))
    }

    fn parse_record(&mut self) -> Option<ASTStmt> {
        let start = self.current_token()?.span.start;
        self.expect_token(TType::Record)?;
        let name = self.parse_identifier()?;
        self.expect_token(TType::Lbrace)?;
        let mut params = Vec::new();
        while self.current_token()?.token_type != TType::Rbrace
            && self.current_token()?.token_type != TType::End
        {
            let param = self.parse_param()?;
            params.push(param);
            if self.current_token()?.token_type == TType::Comma {
                self.advance();
            }
        }
        let end = self.current_token()?.span.end;
        self.expect_token(TType::Rbrace)?;
        let span = Span::new(start, end);
        Some(ASTStmt::new(
            StmtKind::Record {
                name: Box::new(name),
                params,
            },
            span,
        ))
    }

    fn parse_target(&mut self) -> Option<ASTStmt> {
        let start = self.current_token()?.span.start;
        let tgt = match self.current_token()?.token_type {
            TType::Server => FuncTarget::Server,
            TType::Client => FuncTarget::Client,
            _ => FuncTarget::None,
        };
        self.advance();

        if self.current_token()?.token_type == TType::Func {
            let mut fn_decl = self.parse_func()?;
            fn_decl.kind = match fn_decl.kind {
                StmtKind::FuncDef {
                    name,
                    params,
                    ret_ty,
                    body,
                    ..
                } => StmtKind::FuncDef {
                    target: tgt,
                    name,
                    params,
                    ret_ty,
                    body,
                },
                _ => {
                    self.report("Invalid statement".to_string(), Some(fn_decl.span.clone()));
                    return None;
                }
            };
            return Some(fn_decl);
        }

        if self.current_token()?.token_type == TType::Lbrace {
            self.advance();
            let mut funcs = Vec::new();

            while self.current_token()?.token_type != TType::Rbrace {
                // Optional nested modifier (client inside server block = error later)
                let inner_tgt = match self.current_token()?.token_type {
                    TType::Server => {
                        self.advance();
                        FuncTarget::Server
                    }
                    TType::Client => {
                        self.advance();
                        FuncTarget::Client
                    }
                    _ => tgt.clone(),
                };

                if self.current_token()?.token_type != TType::Func {
                    self.report(
                        "Only function declarations allowed in target blocks".to_string(),
                        Some(self.current_token()?.span.clone()),
                    );
                    self.advance();
                    continue;
                }

                let mut fn_decl = self.parse_func()?;
                fn_decl.kind = match fn_decl.kind {
                    StmtKind::FuncDef {
                        name,
                        params,
                        ret_ty,
                        body,
                        ..
                    } => StmtKind::FuncDef {
                        target: inner_tgt,
                        name,
                        params,
                        ret_ty,
                        body,
                    },
                    _ => continue,
                };
                funcs.push(fn_decl);
            }

            let end = self.current_token()?.span.end;
            self.expect_token(TType::Rbrace)?;
            let span = Span::new(start, end);
            return Some(ASTStmt::new(
                StmtKind::TargetBlock { target: tgt, funcs },
                span,
            ));
        }

        self.report(
            "Expected 'func' or '{' after target modifier".to_string(),
            Some(self.current_token()?.span.clone()),
        );
        None
    }

    fn parse_let_stmt(&mut self) -> Option<ASTStmt> {
        let let_expr = self.parse_let_expr()?;
        let span = let_expr.span.clone();
        Some(ASTStmt::new(StmtKind::Let(Box::new(let_expr)), span))
    }
}
