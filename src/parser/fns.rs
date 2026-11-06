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
    fn parse_fn_signature(&mut self) -> Option<FnSignature> {
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

    pub fn parse_fn_expr(&mut self) -> Option<ASTExpr> {
        let sig = self.parse_fn_signature()?;

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

        let is_lambda = self.current_token()?.token_type == TType::Arrow;

        if is_lambda {
            if saw_bare {
                self.report(
                    "lambda parameters need a name: 'name: Type'".to_string(),
                    Some(Span::new(sig.start, self.last_end)),
                );
            }

            self.expect_token(TType::Arrow)?;
            let body = self.parse_expr(Precedence::Lowest)?;
            let end = self.last_end;
            let span = Span::new(sig.start, end);

            return Some(ASTExpr::new(
                ASTExprKind::Lambda {
                    params,
                    ret_ty: sig.ret_ty,
                    body: Box::new(body),
                },
                span,
            ));
        }

        if saw_named {
            self.report(
                "function type parameters must be types, not named parameters".to_string(),
                Some(Span::new(sig.start, self.last_end)),
            );
        }

        let ret_ty = match sig.ret_ty {
            Some(ty) => ty,
            None => {
                let message = if saw_named {
                    "expected '->' after lambda parameters"
                } else {
                    "function type requires a return type: fn(...): T"
                };
                self.report(
                    message.to_string(),
                    Some(Span::new(sig.start, self.last_end)),
                );
                return None;
            }
        };

        let end = self.last_end;
        let span = Span::new(sig.start, end);
        let ty = ASTType::new(
            ASTTypeKind::FunctionPtr {
                type_params,
                return_type: Box::new(ret_ty),
            },
            span.clone(),
        );

        Some(ASTExpr::new(ASTExprKind::TypeExpr(Box::new(ty)), span))
    }
}
