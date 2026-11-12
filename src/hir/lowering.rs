use crate::{
    ast::ASTExpr,
    diagnostics::{CompilerError, Diag, Span},
    hir::hir::{ExprId, HIRModule, TypeNodeId},
};

pub struct HIRLowering {
    type_id_counter: usize,
    expr_id_counter: usize,
    ast: Vec<ASTExpr>,
    pub hir_module: HIRModule,
    diagnostics: Diag,
    corrupted: bool,
}

impl HIRLowering {
    pub fn new(ast: Vec<ASTExpr>, diagnostics: Diag) -> HIRLowering {
        HIRLowering {
            type_id_counter: 0,
            expr_id_counter: 0,
            ast,
            hir_module: HIRModule::new(),
            diagnostics,
            corrupted: false,
        }
    }

    pub fn type_id(&mut self) -> TypeNodeId {
        let id = TypeNodeId(self.type_id_counter);
        self.type_id_counter += 1;
        id
    }

    pub fn expr_id(&mut self) -> ExprId {
        let id = ExprId(self.expr_id_counter);
        self.expr_id_counter += 1;
        id
    }

    pub fn report(&mut self, message: String, span: Option<Span>) {
        self.corrupted = true;
        self.diagnostics
            .borrow_mut()
            .report(CompilerError::error(message, span, None));
    }
}
