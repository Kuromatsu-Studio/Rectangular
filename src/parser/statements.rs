use crate::{
    ast::{ASTParam, Precedence, Stmt, StmtKind},
    diagnostics::Span,
    lexer::TType,
    parser::parser::Parser,
};

impl Parser {
    pub fn parse_stmt(&mut self) -> Option<Stmt> {
        let token = self.current_token()?.clone();
        let ttype = token.token_type;
        let t_span = token.span;
        match ttype {
            TType::Enum => self.parse_enum(),
            TType::Record => self.parse_record(),
            TType::Let => self.parse_let_stmt(),
            _ => {
                self.report(
                    format!("Invalid statement encuntered {:?}", ttype),
                    Some(t_span),
                );
                None
            }
        }
    }

    fn parse_enum(&mut self) -> Option<Stmt> {
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

        Some(Stmt::new(
            StmtKind::Enum {
                name: Box::new(name),
                block: fields,
            },
            span,
        ))
    }

    fn parse_param(&mut self) -> Option<ASTParam> {
        let name = self.parse_identifier()?;
        self.expect_token(TType::Colon)?;
        let ty = self.parse_type()?;
        Some(ASTParam {
            name: Box::new(name),
            ty: Box::new(ty),
        })
    }

    fn parse_record(&mut self) -> Option<Stmt> {
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
        Some(Stmt::new(
            StmtKind::Record {
                name: Box::new(name),
                params,
            },
            span,
        ))
    }

    fn parse_let_stmt(&mut self) -> Option<Stmt> {
        let let_expr = self.parse_let_expr()?;
        let span = let_expr.span.clone();
        Some(Stmt::new(StmtKind::Let(Box::new(let_expr)), span))
    }
}
