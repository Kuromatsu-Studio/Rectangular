use crate::{
    ast::{ASTType, ASTTypeKind},
    diagnostics::Span,
    lexer::TType,
    parser::Parser,
};

impl Parser {
    ///The main type sub parser
    pub fn parse_type(&mut self) -> Option<ASTType> {
        let mut ty = self.parse_type_atom()?;
        loop {
            if self.current_token()?.token_type != TType::Lparen {
                break;
            }

            self.advance();
            let mut args = Vec::new();

            loop {
                if self.current_token()?.token_type == TType::Rparen {
                    break;
                }

                args.push(self.parse_type()?);

                if self.current_token()?.token_type == TType::Comma {
                    self.advance();
                } else {
                    break;
                }
            }

            self.expect_token(TType::Rparen)?;
            let span = Span::new(ty.span.start, self.last_end);
            ty = ASTType::new(
                ASTTypeKind::Applied {
                    callee: Box::new(ty),
                    args,
                },
                span,
            );
        }

        Some(ty)
    }

    fn parse_type_atom(&mut self) -> Option<ASTType> {
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
            | TType::C8Key
            | TType::C16Key
            | TType::C32Key
            | TType::BoolKey
            | TType::TypeKey
            | TType::U0Key => {
                self.advance();
                Some(ASTType::basic(&ty_token))
            }
            TType::Identifier => self.parse_ident_type(),
            TType::Asterisk => self.parse_pointer_type(),
            TType::Fn => self.parse_function_pointer_type(),
            _ => {
                self.report(
                    format!("Invalid type token {:?}", ty_token.token_type),
                    Some(span.clone()),
                );
                None
            }
        }
    }

    fn parse_ident_type(&mut self) -> Option<ASTType> {
        let start = self.current_token()?.span.start;
        let first = self.current_token()?.lexeme.clone();

        self.expect_token(TType::Identifier)?;

        let mut path = vec![first];

        while self.current_token()?.token_type == TType::Dot {
            self.advance();
            let field = self.current_token()?.clone();
            if field.token_type != TType::Identifier {
                self.report(
                    "expected an identifier after '.' in a type path".to_string(),
                    Some(field.span),
                );
                return None;
            }
            path.push(field.lexeme);
            self.advance();
        }

        let end = self.last_end;
        Some(ASTType::new(ASTTypeKind::Path(path), Span::new(start, end)))
    }
    fn parse_function_pointer_type(&mut self) -> Option<ASTType> {
        let start = self.current_token()?.span.start;
        self.expect_token(TType::Fn)?;
        self.expect_token(TType::Lparen)?;
        let mut types = Vec::new();
        loop {
            if self.current_token()?.token_type == TType::Rparen {
                break;
            }
            let ty = self.parse_type()?;
            types.push(ty);
            if self.current_token()?.token_type == TType::Comma {
                self.advance();
            } else {
                break;
            }
        }
        self.expect_token(TType::Rparen)?;
        self.expect_token(TType::Colon)?;
        let ret_ty = self.parse_type()?;
        let end = self.last_end;
        let span = Span::new(start, end);
        Some(ASTType::new(
            ASTTypeKind::FunctionPtr {
                type_params: types,
                return_type: Box::new(ret_ty),
            },
            span,
        ))
    }

    fn parse_pointer_type(&mut self) -> Option<ASTType> {
        let start = self.current_token()?.span.start;
        self.expect_token(TType::Asterisk)?;
        if self.operand_missing() {
            self.report(
                "expected a type after '*'".to_string(),
                Some(Span::new(start, self.last_end)),
            );
            return None;
        }
        let inner = self.parse_type()?;
        let end = self.last_end;
        let span = Span::new(start, end);
        Some(ASTType::new(ASTTypeKind::Pointer(Box::new(inner)), span))
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
