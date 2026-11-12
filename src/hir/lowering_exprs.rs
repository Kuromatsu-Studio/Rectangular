use crate::{
    ast::{ASTExpr, ASTExprKind},
    hir::{
        expr::{HIRExpr, HIRExprKind},
        hir::ExprId,
        lowering::HIRLowering,
    },
};

impl HIRLowering {
    fn expr_ids(&self, exprs: Vec<ASTExpr>) -> Option<Vec<ExprId>> {
        exprs
            .iter()
            .map(|e| self.lower_expr(e).map(|hirexpr| hirexpr.id))
            .collect()
    }

    pub fn lower_expr(&self, expr: &ASTExpr) -> Option<HIRExpr> {
        let span = expr.span.clone();
        let id = self.expr_id();

        let kind = match expr.kind {
            ASTExprKind::Block(exprs) => {
                let expr_ids = self.expr_ids(exprs)?;
                HIRExprKind::Block(expr_ids)
            }
            ASTExprKind::Binary(left, op, right) => {
                let left_id = self.lower_expr(&*left)?.id;
                let right_id = self.lower_expr(&*right)?.id;
                HIRExprKind::Binary {
                    op,
                    lhs: left_id,
                    rhs: right_id,
                }
            }
            ASTExprKind::Unary(op, operand) => {
                let operand_id = self.lower_expr(&*&operand)?.id;
                HIRExprKind::Unary {
                    op,
                    operand: operand_id,
                }
            }
            ASTExprKind::Defer(inner) => {
                let inner_id = self.lower_expr(&*inner)?.id;
                HIRExprKind::Defer(inner_id)
            }
        };

        let hir_expr = HIRExpr::new(id, kind, span);
        self.hir_module.exprs.push(hir_expr.clone());
        Some(hir_expr)
    }
}
