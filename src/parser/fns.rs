use crate::{
    ast::{ASTExpr, ASTExprKind, ASTParam, ASTType, ASTTypeKind, Precedence},
    diagnostics::Span,
    lexer::TType,
    parser::Parser,
};

enum FnParam {
    Named(ASTParam),
    Bare(ASTType),
}

struct FnSignature {
    start: usize,
    params: Vec<FnParam>,
    ret_ty: Option<ASTType>,
}

impl Parser {
    fn parse_lambda_signature(&mut self) -> Option<FnSignature> {
        let start = self.current_token()?.span.start;
        self.expect_token(TType::Fn)?;
        self.expect_token(TType::Lparen)?;

        let mut params = Vec::new();
        loop {
            if self.current_token()?.token_type == TType::Rparen {
                break;
            }

            let named = self.current_token()?.token_type == TType::Identifier
                && self
                    .peek_token()
                    .map(|t| t.token_type == TType::Colon)
                    .unwrap_or(false);

            if named {
                params.push(FnParam::Named(self.parse_param()?));
            } else {
                params.push(FnParam::Bare(self.parse_type()?));
            }

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

        Some(FnSignature {
            start,
            params,
            ret_ty,
        })
    }

    pub fn parse_lambda_expr(&mut self) -> Option<ASTExpr> {
        let sig = self.parse_lambda_signature()?;

        let mut saw_named = false;
        let mut saw_bare = false;
        let mut params = Vec::new();
        let mut type_params = Vec::new();

        for param in sig.params {
            match param {
                FnParam::Named(p) => {
                    saw_named = true;
                    params.push(p);
                }
                FnParam::Bare(ty) => {
                    saw_bare = true;
                    type_params.push(ty);
                }
            }
        }

        if self.current_token()?.token_type == TType::Arrow {
            if saw_bare {
                self.report(
                    "lambda parameters need a name: 'name: Type'".to_string(),
                    Some(Span::new(sig.start, self.last_end)),
                );
            }
            self.advance();

            let body = self.parse_expr(Precedence::Lowest)?;
            let end = self.last_end;
            let span = Span::new(sig.start, end);

            return Some(ASTExpr::new(
                ASTExprKind::Lambda {
                    params,
                    ret_ty: sig.ret_ty,
                    body: Some(Box::new(body)),
                },
                span,
            ));
        } else {
            let ret_ty = match sig.ret_ty {
                Some(ty) => ty,
                None => {
                    self.report(
                        "lambda types and declarations must always have a return type annotation"
                            .to_string(),
                        Some(Span::new(sig.start, self.last_end)),
                    );
                    return None;
                }
            };

            let end = self.last_end;
            let span = Span::new(sig.start, end);
            if saw_named {
                return Some(ASTExpr::new(
                    ASTExprKind::Lambda {
                        params,
                        ret_ty: Some(ret_ty),
                        body: None,
                    },
                    span,
                ));
            } else {
                let ty = ASTType::new(
                    ASTTypeKind::FunctionPtr {
                        type_params,
                        return_type: Box::new(ret_ty),
                    },
                    span.clone(),
                );

                return Some(ASTExpr::new(ASTExprKind::TypeExpr(Box::new(ty)), span));
            }
        };
    }
}
