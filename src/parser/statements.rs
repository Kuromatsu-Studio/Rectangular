use crate::{
    ast::{Precedence, Stmt, StmtKind},
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
}
