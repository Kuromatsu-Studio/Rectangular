use crate::{
    ast::ASTExpr,
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

    pub fn report(&mut self, message: String, span: Option<Span>) {
        self.corrupted = true;
        self.diagnostics
            .borrow_mut()
            .report(CompilerError::error(message, span, None));
    }
}
