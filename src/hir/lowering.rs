use crate::{
    ast::{ASTExpr, ASTExprKind},
    diagnostics::{CompilerError, Diag, Span},
    hir::hir::HIRModule,
};

pub struct HIRLowering {
    ast: Vec<ASTExpr>,
    pub hir_module: HIRModule,
    diagnostics: Diag,
    corrupted: bool,
}

impl HIRLowering {
    pub fn new(ast: Vec<ASTExpr>, diagnostics: Diag) -> HIRLowering {
        HIRLowering {
            ast,
            hir_module: HIRModule::new(),
            diagnostics,
            corrupted: false,
        }
    }

    pub fn lower(&mut self) {
        let ast = std::mem::take(&mut self.ast);
        for expr in ast {
            if let Some(id) = self.lower_expr(expr) {
                self.hir_module.roots.push(id);
            }
        }
    }

    pub fn get_string(&mut self, expr: ASTExpr) -> Option<String> {
        let span = expr.span.clone();
        match expr.kind {
            ASTExprKind::Identifier(name) => Some(name),
            _ => {
                self.report(
                    "Cannot extract string from invalid node".to_string(),
                    Some(span),
                );
                None
            }
        }
    }

    pub fn report(&mut self, message: String, span: Option<Span>) {
        self.corrupted = true;
        self.diagnostics
            .borrow_mut()
            .report(CompilerError::error(message, span, None));
    }
}
