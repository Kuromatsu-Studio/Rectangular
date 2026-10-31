use crate::{
    ast::{ASTExpr, ASTExprKind, ASTType, ASTTypeKind, Precedence},
    diagnostics::Span,
    lexer::TType,
    parser::Parser,
};

impl Parser {
    pub fn parse_type_expr(&mut self) -> Option<ASTExpr> {
        let ty = self.parse_type()?;
        let span = ty.span.clone();
        Some(ASTExpr::new(ASTExprKind::TypeExpr(Box::new(ty)), span))
    }

    ///The main type sub parser
    fn parse_type(&mut self) -> Option<ASTType> {
        let ty_token = self.current_token()?.clone();
        let span = ty_token.span.clone();
        match ty_token.token_type {
            TType::I8Key
            | TType::U8Key
            | TType::I16Key
            | TType::U16Key
            | TType::I32Key
            | TType::U32Key
            | TType::I64Key
            | TType::U64Key
            | TType::F32Key
            | TType::F64Key
            | TType::BoolKey
            | TType::TypeKey
            | TType::UnitKey => {
                self.advance();
                Some(ASTType::basic(&ty_token))
            }
            TType::Identifier => {
                let name = self.current_token()?.lexeme.clone();
                let span = self.current_token()?.span.clone();
                self.advance();
                Some(ASTType::new(ASTTypeKind::Custom(name), span))
            }
            TType::Asterisk => {
                let start = self.current_token()?.span.start;
                self.advance();
                let next = self.current_token()?.token_type.clone();
                if matches!(
                    next,
                    TType::End
                        | TType::Semicolon
                        | TType::Assign
                        | TType::BindAssign
                        | TType::Comma
                        | TType::Rparen
                        | TType::Rbrace
                        | TType::Rbracket
                        | TType::Arrow
                ) {
                    self.report(
                        "expected a type after '*'".to_string(),
                        Some(Span::new(start, self.last_end)),
                    );
                    return None;
                }
                let inner = Self::expr_to_type(self.parse_expr(Precedence::Call)?);
                let end = self.last_end;
                let span = Span::new(start, end);
                Some(ASTType::new(ASTTypeKind::Pointer(Box::new(inner)), span))
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

    pub fn parse_type_annotation(&mut self) -> Option<ASTType> {
        Some(Self::expr_to_type(self.parse_expr(Precedence::Assignment)?))
    }

    fn expr_to_type(expr: ASTExpr) -> ASTType {
        let span = expr.span.clone();
        match expr.kind {
            ASTExprKind::TypeExpr(ty) => *ty,
            ASTExprKind::Identifier(name) => ASTType::new(ASTTypeKind::Custom(name), span),
            kind => ASTType::new(
                ASTTypeKind::Expr(Box::new(ASTExpr::new(kind, span.clone()))),
                span,
            ),
        }
    }

    /*fn parse_array_type(&mut self) -> Option<ASTType> {
        let start = self.current_token()?.span.start;
        self.expect_token(TType::Lbracket)?;
        let element_type = self.parse_type()?;
        self.expect_token(TType::Semicolon)?;
        let array_size = self.parse_expr(Precedence::Lowest)?;
        let end = self.current_token()?.span.end;
        self.expect_token(TType::Rbracket)?;
        let span = Span::new(start, end);
        Some(ASTType::new(
            ASTTypeKind::Array(Box::new(element_type), Box::new(array_size)),
            span,
        ))
    }*/
}
