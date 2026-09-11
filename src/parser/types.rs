use crate::{
    ast::{ASTType, ASTTypeKind, Precedence},
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
            TType::Func => self.parse_func_type(),
            TType::Lparen => {
                if ASTType::is_type(self.peek_token()?.token_type.clone()) {
                    self.parse_tuple_type()
                } else {
                    self.parse_unit_type()
                }
            }
            TType::Lbracket => self.parse_array_type(),
            _ => {
                self.report(
                    format!("Invalid type token {:?}", ty_token.token_type),
                    Some(span.clone()),
                );
                None
            }
        }
    }

    ///The sub parser for  (i32,u32) types
    fn parse_tuple_type(&mut self) -> Option<ASTType> {
        let start = self.current_token()?.span.start;
        self.expect_token(TType::Lparen)?;
        let mut types = Vec::new();
        while self.current_token()?.token_type != TType::Rparen
            && self.current_token()?.token_type != TType::End
        {
            let ty = self.parse_type()?;
            types.push(ty);
            if self.current_token()?.token_type == TType::Comma {
                self.advance();
            }
        }
        let end = self.current_token()?.span.end;
        self.expect_token(TType::Rparen)?;
        let span = Span::new(start, end);
        Some(ASTType::new(ASTTypeKind::Tuple(types), span))
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

    fn parse_func_type(&mut self) -> Option<ASTType> {
        let start = self.current_token()?.span.start;
        self.expect_token(TType::Func)?;
        let mut ty_params = Vec::new();
        if self.current_token()?.token_type == TType::Lparen {
            self.advance();
            while self.current_token()?.token_type != TType::End
                && self.current_token()?.token_type != TType::Rparen
            {
                let ty_param = self.parse_type()?;
                ty_params.push(ty_param);
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
        let end = self.current_token()?.span.end;
        let span = Span::new(start, end);

        Some(ASTType::new(
            ASTTypeKind::Func {
                params: ty_params,
                ret_ty: Box::new(ret_ty),
            },
            span,
        ))
    }

    fn parse_array_type(&mut self) -> Option<ASTType> {
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
    }
}
