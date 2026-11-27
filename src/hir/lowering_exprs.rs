use crate::{
    ast::{ASTConstructorMember, ASTExpr, ASTExprKind, ASTExprLiteral, ASTParam, ASTType},
    diagnostics::Span,
    hir::{
        expr::{ComptimeKind, ConstructorMember, HIRExpr, HIRExprKind, HIRExprLiteral, HIRParam},
        hir::{AttrId, ExprId},
        lowering::HIRLowering,
    },
};

impl HIRLowering {
    fn push_expr(&mut self, kind: HIRExprKind, span: Span) -> ExprId {
        let id = ExprId(self.hir_module.exprs.len());
        self.hir_module.exprs.push(HIRExpr::new(id, kind, span));
        id
    }

    pub fn expr_ids(&mut self, exprs: Vec<ASTExpr>) -> Option<Vec<ExprId>> {
        exprs.into_iter().map(|e| self.lower_expr(e)).collect()
    }

    fn bind_kind(
        &mut self,
        attrs: Vec<AttrId>,
        name: ASTExpr,
        ty: Option<ASTType>,
        init: Option<Box<ASTExpr>>,
    ) -> Option<HIRExprKind> {
        let name = self.get_string(name)?;

        let ty = match ty {
            Some(t) => Some(self.lower_type(t)?),
            None => None,
        };

        let init = match init {
            Some(expr) => Some(self.lower_expr(*expr)?),
            None => None,
        };

        Some(HIRExprKind::Bind {
            attrs,
            name,
            ty,
            init,
        })
    }

    pub fn lower_expr(&mut self, expr: ASTExpr) -> Option<ExprId> {
        let span = expr.span.clone();

        let kind = match expr.kind {
            ASTExprKind::Block(exprs) => {
                let expr_ids = self.expr_ids(exprs)?;
                HIRExprKind::Block(expr_ids)
            }
            ASTExprKind::Binary(left, op, right) => {
                let left_id = self.lower_expr(*left)?;
                let right_id = self.lower_expr(*right)?;
                HIRExprKind::Binary {
                    op,
                    lhs: left_id,
                    rhs: right_id,
                }
            }
            ASTExprKind::Call(name, args) => {
                let name_id = self.lower_expr(*name)?;
                let hir_args = self.expr_ids(args)?;
                HIRExprKind::Call {
                    callee: name_id,
                    args: hir_args,
                }
            }
            ASTExprKind::Access(left, right) => {
                let left_id = self.lower_expr(*left)?;
                let right_id = self.lower_expr(*right)?;
                HIRExprKind::Access {
                    left: left_id,
                    right: right_id,
                }
            }
            ASTExprKind::Unary(op, operand) => {
                let operand_id = self.lower_expr(*operand)?;
                HIRExprKind::Unary {
                    op,
                    operand: operand_id,
                }
            }
            ASTExprKind::Defer(inner) => {
                let inner_id = self.lower_expr(*inner)?;
                HIRExprKind::Defer(inner_id)
            }
            ASTExprKind::Array(elements) => {
                let elem_ids = self.expr_ids(elements)?;
                HIRExprKind::Array(elem_ids)
            }
            ASTExprKind::Dereference(inner) => {
                let inner_id = self.lower_expr(*inner)?;
                HIRExprKind::Dereference(inner_id)
            }
            ASTExprKind::TypeExpr(ty) => {
                let ty_id = self.lower_type(*ty)?;
                HIRExprKind::TypeValue(ty_id)
            }
            ASTExprKind::Bind { name, ty, init } => self.bind_kind(Vec::new(), *name, ty, init)?,
            ASTExprKind::Attributed {
                attributes,
                binding,
            } => {
                let attrs = self.attr_ids(attributes)?;
                let binding_span = binding.span.clone();
                let ASTExpr { kind, .. } = *binding;

                match kind {
                    ASTExprKind::Bind { name, ty, init } => {
                        self.bind_kind(attrs, *name, ty, init)?
                    }
                    _ => {
                        self.report(
                            "attributes can only be attached to a bind or parameter".to_string(),
                            Some(binding_span),
                        );
                        return None;
                    }
                }
            }
            ASTExprKind::IfExpr {
                condition,
                if_body,
                else_body,
            } => {
                let cond = self.lower_expr(*condition)?;
                let if_bod = self.lower_expr(*if_body)?;
                let el_body = match else_body {
                    Some(el) => Some(self.lower_expr(*el)?),
                    None => None,
                };
                HIRExprKind::If {
                    condition: cond,
                    if_body: if_bod,
                    else_body: el_body,
                }
            }
            ASTExprKind::WhileExpr { condition, body } => {
                let cond_id = self.lower_expr(*condition)?;
                let body_id = self.lower_expr(*body)?;
                HIRExprKind::While {
                    condition: cond_id,
                    body: body_id,
                }
            }
            ASTExprKind::Literal(lit) => {
                let hir_lit = self.lower_literal(lit);
                HIRExprKind::Literal(hir_lit)
            }
            ASTExprKind::Break => HIRExprKind::Break,
            ASTExprKind::Continue => HIRExprKind::Continue,
            ASTExprKind::Return(inner) => {
                let expr_id = match inner {
                    Some(valid) => self.lower_expr(*valid),
                    None => None,
                };
                HIRExprKind::Return(expr_id)
            }
            ASTExprKind::Lambda {
                params,
                ret_ty,
                body,
            } => {
                let hir_params = params
                    .into_iter()
                    .map(|p| self.lower_param(p))
                    .collect::<Option<Vec<_>>>()?;

                let r_ty = match ret_ty {
                    Some(r) => self.lower_type(r),
                    None => None,
                };

                match body {
                    Some(b) => {
                        let body_id = self.lower_expr(*b)?;
                        HIRExprKind::Lambda {
                            params: hir_params,
                            ret: r_ty,
                            body: body_id,
                        }
                    }
                    None => match r_ty {
                        Some(ret) => HIRExprKind::LambdaDecl {
                            params: hir_params,
                            ret,
                        },
                        None => {
                            self.report(
                                "a bodyless lambda declaration requires a return type".to_string(),
                                Some(span.clone()),
                            );
                            return None;
                        }
                    },
                }
            }
            ASTExprKind::DollarExpr(inner) => {
                let inner_id = self.lower_expr(*inner)?;
                HIRExprKind::Comptime {
                    kind: ComptimeKind::Read,
                    expr: inner_id,
                }
            }
            ASTExprKind::DoubleDollar(inner) => {
                let inner_id = self.lower_expr(*inner)?;
                HIRExprKind::Comptime {
                    kind: ComptimeKind::Evaluate,
                    expr: inner_id,
                }
            }
            ASTExprKind::AddressOf(inner) => {
                let inner_id = self.lower_expr(*inner)?;
                HIRExprKind::AddressOf(inner_id)
            }
            ASTExprKind::Index(base, index) => {
                let hir_base = self.lower_expr(*base)?;
                let hir_index = self.lower_expr(*index)?;
                HIRExprKind::Index {
                    base: hir_base,
                    index: hir_index,
                }
            }
            ASTExprKind::Struct { members } => {
                let struct_members = members
                    .into_iter()
                    .map(|p| self.lower_param(p))
                    .collect::<Option<Vec<_>>>()?;

                HIRExprKind::Struct {
                    members: struct_members,
                }
            }
            ASTExprKind::Identifier(ident) => HIRExprKind::Identifier(ident),
            ASTExprKind::StructConstructor { name, members } => {
                let name = self.lower_expr(*name)?;

                let struct_members = members
                    .into_iter()
                    .map(|m| self.lower_constructor_member(m))
                    .collect::<Option<Vec<_>>>()?;

                HIRExprKind::Constructor {
                    name,
                    members: struct_members,
                }
            }
        };

        Some(self.push_expr(kind, span))
    }

    fn lower_constructor_member(
        &mut self,
        member: ASTConstructorMember,
    ) -> Option<ConstructorMember> {
        let name = self.get_string(*member.name)?;
        let val = self.lower_expr(*member.val)?;

        Some(ConstructorMember {
            name,
            val,
            span: member.span,
        })
    }

    fn lower_param(&mut self, param: ASTParam) -> Option<HIRParam> {
        let param_name = self.get_string(*param.name)?;
        let attributes = self.attr_ids(param.attributes)?;
        let param_ty = self.lower_type(*param.ty)?;

        Some(HIRParam {
            attributes,
            name: param_name,
            ty: param_ty,
            span: param.span,
        })
    }

    fn lower_literal(&mut self, literal: ASTExprLiteral) -> HIRExprLiteral {
        match literal {
            ASTExprLiteral::I8(v) => HIRExprLiteral::I8(v),
            ASTExprLiteral::U8(v) => HIRExprLiteral::U8(v),
            ASTExprLiteral::I16(v) => HIRExprLiteral::I16(v),
            ASTExprLiteral::U16(v) => HIRExprLiteral::U16(v),
            ASTExprLiteral::I32(v) => HIRExprLiteral::I32(v),
            ASTExprLiteral::U32(v) => HIRExprLiteral::U32(v),
            ASTExprLiteral::I64(v) => HIRExprLiteral::I64(v),
            ASTExprLiteral::U64(v) => HIRExprLiteral::U64(v),
            ASTExprLiteral::F32(f) => HIRExprLiteral::F32(f),
            ASTExprLiteral::F64(f) => HIRExprLiteral::F64(f),
            ASTExprLiteral::Bool(b) => HIRExprLiteral::Bool(b),
            ASTExprLiteral::Str(s) => HIRExprLiteral::Str(s),
            ASTExprLiteral::Char8(c) => HIRExprLiteral::Char8(c),
            ASTExprLiteral::Char16(c) => HIRExprLiteral::Char16(c),
            ASTExprLiteral::Char32(c) => HIRExprLiteral::Char32(c),
        }
    }
}
