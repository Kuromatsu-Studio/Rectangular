use crate::{
    ast::{
        ASTConstructorMember, ASTExpr, ASTExprKind, ASTExprLiteral, ASTParam, BinaryOp, Precedence,
        UnaryOp,
    },
    diagnostics::Span,
    lexer::{TType, Token},
    parser::parser::Parser,
};

impl Parser {
    pub fn parse_expr(&mut self, min_prec: Precedence) -> Option<ASTExpr> {
        let mut left = self.parse_prefix_with_postfix()?;
        while let Some(token) = self.current_token() {
            if token.token_type == TType::End {
                break;
            }

            if !BinaryOp::is_valid(&token.token_type) {
                break;
            }

            let op_prec = Precedence::prec(&token.token_type);
            if op_prec <= min_prec {
                break;
            }

            left = self.parse_binary(left)?;
        }

        Some(left)
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

    fn parse_lambda(&mut self) -> Option<ASTExpr> {
        let start = self.current_token()?.span.start;
        self.expect_token(TType::Lparen)?;
        let mut params = Vec::new();
        loop {
            let param = self.parse_param()?;
            params.push(param);
            if self.current_token()?.token_type == TType::Comma {
                self.advance();
            } else {
                break;
            }
        }
        self.expect_token(TType::Rparen)?;

        let ret_ty = if self.current_token()?.token_type == TType::Colon {
            self.advance();
            Some(self.parse_type()?)
        } else {
            None
        };

        self.expect_token(TType::Arrow)?;
        let body = self.parse_expr(Precedence::Lowest)?;
        let end = self.last_end;
        let span = Span::new(start, end);

        Some(ASTExpr::new(
            ASTExprKind::Lambda {
                params,
                ret_ty,
                body: Box::new(body),
            },
            span,
        ))
    }

    fn parse_bind(&mut self) -> Option<ASTExpr> {
        let start = self.current_token()?.span.start;
        self.expect_token(TType::Bind)?;
        let mut names = Vec::new();

        loop {
            let name = self.parse_identifier()?;
            names.push(name);
            if self.current_token()?.token_type == TType::Comma {
                self.advance();
            } else {
                break;
            }
        }

        let ty = if self.current_token()?.token_type == TType::Colon {
            self.advance();
            Some(self.parse_type()?)
        } else {
            None
        };

        let mutbind = self.current_token()?.token_type == TType::Assign;
        let immutbind = self.current_token()?.token_type == TType::BindAssign;
        let mut mutable = false;

        let init = if mutbind || immutbind {
            mutable = mutbind;
            self.advance();
            Some(Box::new(self.parse_expr(Precedence::Lowest)?))
        } else {
            None
        };

        let end = self.last_end;
        let span = Span::new(start, end);

        Some(ASTExpr::new(
            ASTExprKind::Bind {
                mutable,
                names,
                ty,
                init,
            },
            span,
        ))
    }

    fn parse_if_expr(&mut self) -> Option<ASTExpr> {
        let start = self.current_token()?.span.start;
        self.expect_token(TType::If)?;
        let condition = self.parse_expr(Precedence::Lowest)?;
        let if_body = self.parse_block()?;

        let else_body = if self.current_token()?.token_type == TType::Else {
            self.advance();
            let body = self.parse_block()?;
            Some(Box::new(body))
        } else {
            None
        };

        let end = self.last_end;
        let span = Span::new(start, end);

        Some(ASTExpr::new(
            ASTExprKind::IfExpr {
                condition: Box::new(condition),
                if_body: Box::new(if_body),
                else_body,
            },
            span,
        ))
    }

    fn parse_while_expr(&mut self) -> Option<ASTExpr> {
        let start = self.current_token()?.span.start;
        self.expect_token(TType::While)?;
        let condition = self.parse_expr(Precedence::Lowest)?;
        let body = self.parse_block()?;
        let end = self.last_end;
        let span = Span::new(start, end);
        Some(ASTExpr::new(
            ASTExprKind::WhileExpr {
                condition: Box::new(condition),
                body: Box::new(body),
            },
            span,
        ))
    }

    fn parse_return(&mut self) -> Option<ASTExpr> {
        let start = self.current_token()?.span.start;
        self.expect_token(TType::Return)?;
        if self.current_token()?.token_type == TType::Semicolon
            || self.current_token()?.token_type == TType::Rbrace
            || self.current_token()?.token_type == TType::End
        {
            let end = self.last_end;
            let span = Span::new(start, end);
            Some(ASTExpr::new(ASTExprKind::Return(None), span))
        } else {
            let value = self.parse_expr(Precedence::Lowest)?;
            let end = self.last_end;
            let span = Span::new(start, end);
            Some(ASTExpr::new(
                ASTExprKind::Return(Some(Box::new(value))),
                span,
            ))
        }
    }

    fn parse_array_literal(&mut self) -> Option<ASTExpr> {
        let start = self.current_token()?.span.start;
        self.expect_token(TType::Lbracket)?;
        let mut elements = Vec::new();
        while self.current_token()?.token_type != TType::End
            && self.current_token()?.token_type != TType::Rbracket
        {
            let expr = self.parse_expr(Precedence::Lowest)?;
            elements.push(expr);
            if self.current_token()?.token_type == TType::Comma {
                self.advance();
            }
        }
        let end = self.current_token()?.span.end;
        self.expect_token(TType::Rbracket)?;
        let span = Span::new(start, end);
        Some(ASTExpr::new(ASTExprKind::Array(elements), span))
    }

    pub fn parse_block(&mut self) -> Option<ASTExpr> {
        let start = self.current_token()?.span.start;
        self.expect_token(TType::Lbrace)?;

        let mut exprs = Vec::new();

        while self.current_token()?.token_type != TType::Rbrace {
            if self.current_token()?.token_type == TType::Semicolon {
                self.advance();
                continue;
            }
            let expr = self.parse_expr(Precedence::Lowest)?;
            exprs.push(expr);

            if self.current_token()?.token_type == TType::Semicolon {
                self.advance();
            } else if self.current_token()?.token_type != TType::Rbrace {
                self.report(
                    "expected ';' or '}'".to_string(),
                    Some(self.current_token()?.span.clone()),
                );
                return None;
            }
        }

        let end = self.current_token()?.span.end;
        self.expect_token(TType::Rbrace)?;

        Some(ASTExpr::new(ASTExprKind::Block(exprs), Span { start, end }))
    }

    fn parse_binary(&mut self, left: ASTExpr) -> Option<ASTExpr> {
        let start = left.span.start;
        let operator = self.current_token()?.clone();
        self.advance();

        let op = BinaryOp::new(&operator.token_type);
        let op_prec = if op.is_assignment() {
            Precedence::Lowest
        } else {
            Precedence::prec(&operator.token_type)
        };

        let right = self.parse_expr(op_prec)?;
        let end = self.last_end;
        let span = Span { start, end };
        Some(ASTExpr::new(
            ASTExprKind::Binary(Box::new(left), op, Box::new(right)),
            span,
        ))
    }

    fn parse_prefix(&mut self) -> Option<ASTExpr> {
        let token = self.current_token()?.clone();
        match token.token_type {
            TType::I8Literal
            | TType::U8Literal
            | TType::I16Literal
            | TType::U16Literal
            | TType::I32Literal
            | TType::U32Literal
            | TType::I64Literal
            | TType::U64Literal
            | TType::F32Literal
            | TType::F64Literal
            | TType::IntLiteral
            | TType::FloatLiteral
            | TType::StringLiteral
            | TType::Char8Literal
            | TType::Char16Literal
            | TType::Char32Literal
            | TType::True
            | TType::False => self.parse_literal(),
            TType::Bind => self.parse_bind(),
            TType::Struct => self.parse_struct_expr(),
            TType::If => self.parse_if_expr(),
            TType::While => self.parse_while_expr(),
            TType::Return => self.parse_return(),
            TType::Break => {
                let span = self.current_token()?.span.clone();
                self.advance();
                Some(ASTExpr::new(ASTExprKind::Break, span))
            }
            TType::Continue => {
                let span = self.current_token()?.span.clone();
                self.advance();
                Some(ASTExpr::new(ASTExprKind::Continue, span))
            }
            TType::Lparen => {
                if self.lparen_starts_function() {
                    return self.parse_lambda();
                } else {
                    return self.parse_grouped();
                }
            }
            TType::Bang | TType::Minus => self.parse_unary(),
            TType::Identifier => {
                let tt = |n| self.peek_nth(n).map(|t| t.token_type.clone());
                if tt(1) == Some(TType::Lparen)
                    && tt(2) == Some(TType::Identifier)
                    && tt(3) == Some(TType::Colon)
                {
                    self.parse_struct_constructor()
                } else {
                    self.parse_identifier()
                }
            }
            TType::Lbrace => self.parse_block(),
            TType::Lbracket => self.parse_array_literal(),
            TType::Dollar => self.parse_dollar_expr(),
            TType::DoubleDollar => self.parse_double_dollar_expr(),
            _ => {
                let span = token.span;
                self.report(
                    format!("Unexpected prefix token: {:?}", token.token_type),
                    Some(span),
                );
                self.advance();
                None
            }
        }
    }

    fn parse_dollar_expr(&mut self) -> Option<ASTExpr> {
        let start = self.current_token()?.span.start;
        self.expect_token(TType::Dollar)?;
        let expr = self.parse_expr(Precedence::Lowest)?;
        let end = self.last_end;
        let span = Span::new(start, end);
        Some(ASTExpr::new(ASTExprKind::DollarExpr(Box::new(expr)), span))
    }

    fn parse_double_dollar_expr(&mut self) -> Option<ASTExpr> {
        let start = self.current_token()?.span.start;
        self.expect_token(TType::DoubleDollar)?;
        let expr = self.parse_expr(Precedence::Lowest)?;
        let end = self.last_end;
        let span = Span::new(start, end);
        Some(ASTExpr::new(
            ASTExprKind::DoubleDollar(Box::new(expr)),
            span,
        ))
    }

    fn parse_grouped(&mut self) -> Option<ASTExpr> {
        self.expect_token(TType::Lparen)?;
        let inner_expr = self.parse_expr(Precedence::Lowest)?;
        self.expect_token(TType::Rparen)?;
        Some(inner_expr)
    }

    fn parse_unary(&mut self) -> Option<ASTExpr> {
        let start = self.current_token()?.span.start;
        let ttype = self.current_token()?.token_type.clone();
        let op = UnaryOp::new(&ttype);
        if op == UnaryOp::Invalid {
            self.report(
                format!("Invalid unary operator {:?}", ttype),
                Some(self.current_token()?.span.clone()),
            );
        }
        self.advance();
        let expr = self.parse_expr(Precedence::Call)?;
        let end = expr.span.end;
        let span = Span::new(start, end);
        Some(ASTExpr::new(ASTExprKind::Unary(op, Box::new(expr)), span))
    }

    fn parse_prefix_with_postfix(&mut self) -> Option<ASTExpr> {
        let mut expr = self.parse_prefix()?;
        let valid_postfix = |t: TType| match t {
            TType::Lparen | TType::Lbracket => true,
            _ => false,
        };
        while let Some(token) = self.current_token() {
            if valid_postfix(token.token_type.clone()) {
                expr = self.parse_postfix(expr)?;
            } else {
                break;
            }
        }
        Some(expr)
    }

    fn parse_postfix(&mut self, left: ASTExpr) -> Option<ASTExpr> {
        let token = self.current_token()?.clone();
        let span = Span {
            start: left.span.start,
            end: token.span.end,
        };
        match token.token_type {
            TType::Lparen => self.parse_call(left),
            TType::Lbracket => self.parse_index(left),
            _ => {
                self.report("Invalid postfix expression".to_string(), Some(span));
                None
            }
        }
    }

    fn parse_index(&mut self, left: ASTExpr) -> Option<ASTExpr> {
        let start = left.span.start;
        self.expect_token(TType::Lbracket)?;
        let index = self.parse_expr(Precedence::Lowest)?;
        self.expect_token(TType::Rbracket)?;
        let end = self.last_end;
        let span = Span::new(start, end);
        Some(ASTExpr::new(
            ASTExprKind::Index(Box::new(left), Box::new(index)),
            span,
        ))
    }

    fn parse_call_arguments(&mut self) -> Option<Vec<ASTExpr>> {
        let mut args = Vec::new();
        self.expect_token(TType::Lparen)?;
        while self.current_token()?.token_type != TType::Rparen
            && self.current_token()?.token_type != TType::End
        {
            if self.current_token()?.token_type == TType::Comma {
                self.advance();
                continue;
            }
            let expr = self.parse_expr(Precedence::Lowest)?;
            args.push(expr);
        }

        self.expect_token(TType::Rparen)?;
        Some(args)
    }

    fn parse_call(&mut self, left: ASTExpr) -> Option<ASTExpr> {
        let args = self.parse_call_arguments()?;
        let start = left.span.start;
        let end = self.last_end;
        let span = Span::new(start, end);
        Some(ASTExpr::new(ASTExprKind::Call(Box::new(left), args), span))
    }

    pub fn parse_identifier(&mut self) -> Option<ASTExpr> {
        let token = self.current_token()?.clone();
        let name = token.lexeme.clone();
        let span = token.span;
        self.expect_token(TType::Identifier)?;
        Some(ASTExpr::new(ASTExprKind::Identifier(name), span))
    }

    fn parse_struct_expr(&mut self) -> Option<ASTExpr> {
        let start = self.current_token()?.span.start;
        self.expect_token(TType::Struct)?;
        self.expect_token(TType::Lbrace)?;
        let mut members = Vec::new();
        loop {
            if self.current_token()?.token_type == TType::Rbrace {
                break;
            }

            let member = self.parse_param()?;
            members.push(member);
            if self.current_token()?.token_type == TType::Comma {
                self.advance();
            } else {
                break;
            }
        }
        self.expect_token(TType::Rbrace)?;
        let end = self.last_end;
        let span = Span::new(start, end);
        Some(ASTExpr::new(ASTExprKind::Struct { members }, span))
    }

    fn parse_struct_constructor(&mut self) -> Option<ASTExpr> {
        let start = self.current_token()?.span.start;
        let name = self.parse_identifier()?;
        self.expect_token(TType::Lparen)?;

        let mut members = Vec::new();

        loop {
            if self.current_token()?.token_type == TType::Rparen {
                break;
            }

            let member_name = self.parse_identifier()?;
            self.expect_token(TType::Colon)?;
            let value = self.parse_expr(Precedence::Lowest)?;

            members.push(ASTConstructorMember {
                name: Box::new(member_name),
                val: Box::new(value),
            });

            if self.current_token()?.token_type == TType::Comma {
                self.advance();
            } else {
                break;
            }
        }

        self.expect_token(TType::Rparen)?;
        let end = self.last_end;
        let span = Span::new(start, end);

        Some(ASTExpr::new(
            ASTExprKind::StructConstructor {
                name: Box::new(name),
                members,
            },
            span,
        ))
    }
}
