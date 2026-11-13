use crate::{
    ast::{ASTExpr, ASTExprKind},
    diagnostics::Span,
    hir::{
        expr::{HIRExpr, HIRExprKind},
        hir::ExprId,
        lowering::HIRLowering,
    },
};

impl HIRLowering {
    fn push_expr(&mut self, kind: HIRExprKind, span: Span) -> ExprId {
        let id = ExprId(self.hir_module.exprs.len());
        self.hir_module.exprs.push(HIRExpr { id, kind, span });
        id
    }

    fn expr_ids(&mut self, exprs: Vec<ASTExpr>) -> Option<Vec<ExprId>> {
        exprs.into_iter().map(|e| self.lower_expr(e)).collect()
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
            /*ASTExprKind::Bind { name, ty, init } => {
                let name = self.lower_expr(&*name)?.id;
                let ty_id = match ty {
                    Some(t) => Some(self.lower_type(t)?.id),
                    None => None,
                };

                let init = match init {
                    Some(expr) => Some(self.lower_expr(&expr)?.id),
                    None => None,
                };

                HIRExprKind::Bind {
                    attrs: (),
                    name: (),
                    ty: (),
                    init,
                }
            }*/
            ASTExprKind::WhileExpr { condition, body } => {
                let cond_id = self.lower_expr(*condition)?;
                let body_id = self.lower_expr(*body)?;
                HIRExprKind::While {
                    condition: cond_id,
                    body: body_id,
                }
            }
        };

        Some(self.push_expr(kind, span))
    }
}
