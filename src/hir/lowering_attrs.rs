use crate::{
    ast::{ASTExpr, ASTExprKind},
    diagnostics::Span,
    hir::{
        attrs::HIRAttr,
        hir::{AttrId, ExprId},
        lowering::HIRLowering,
    },
};

impl HIRLowering {
    pub fn attr_ids(&mut self, attrs: Vec<ASTExpr>) -> Option<Vec<AttrId>> {
        attrs.into_iter().map(|a| self.lower_attr(a)).collect()
    }

    fn push_attr(&mut self, path: Vec<String>, args: Vec<ExprId>, span: Span) -> AttrId {
        let id = AttrId(self.hir_module.attrs.len());
        self.hir_module
            .attrs
            .push(HIRAttr::new(id, path, args, span));
        id
    }

    fn attr_path(&mut self, expr: ASTExpr) -> Option<Vec<String>> {
        let span = expr.span.clone();

        match expr.kind {
            ASTExprKind::Identifier(name) => Some(vec![name]),

            ASTExprKind::Access(base, field) => {
                let mut path = self.attr_path(*base)?;

                match field.kind {
                    ASTExprKind::Identifier(name) => {
                        path.push(name);
                        Some(path)
                    }
                    _ => {
                        self.report(
                            "attribute path segments must be identifiers".to_string(),
                            Some(field.span.clone()),
                        );
                        None
                    }
                }
            }

            _ => {
                self.report(
                    "attribute paths must be identifiers or access chains".to_string(),
                    Some(span),
                );
                None
            }
        }
    }

    fn lower_attr(&mut self, attr: ASTExpr) -> Option<AttrId> {
        let ASTExpr { kind, span } = attr;

        match kind {
            ASTExprKind::Call(callee, args) => {
                let path = self.attr_path(*callee)?;
                let args = self.expr_ids(args)?;
                Some(self.push_attr(path, args, span))
            }

            kind => {
                let path = self.attr_path(ASTExpr {
                    kind,
                    span: span.clone(),
                })?;
                Some(self.push_attr(path, Vec::new(), span))
            }
        }
    }
}
