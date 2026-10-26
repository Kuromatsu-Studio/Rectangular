use crate::{
    ast::{ASTType, ASTTypeKind},
    diagnostics::Span,
    lexer::TType,
    parser::Parser,
};

impl Parser {
    ///The main type sub parser
    pub fn parse_type(&mut self) -> Option<ASTType> {
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
            | TType::BoolKey => {
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

    ///Sub parser for () type
    fn parse_unit_type(&mut self) -> Option<ASTType> {
        let start = self.current_token()?.span.start;
        self.expect_token(TType::Lparen)?;
        let end = self.current_token()?.span.end;
        self.expect_token(TType::Rparen)?;
        let span = Span::new(start, end);
        Some(ASTType::new(ASTTypeKind::Unit, span))
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
