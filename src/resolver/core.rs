use std::collections::HashMap;

use crate::{
    diagnostics::Diag,
    hir::{ExprId, HIRExpr, HIRExprKind, HIRModule},
};

pub struct DefId(usize);
pub struct ScopeId(usize);

pub struct Scope {
    parent_scope_id: Option<ScopeId>,
    scope_id: ScopeId,
    names: HashMap<String, DefId>,
}

pub enum DefKind {
    Bind { id: ExprId },
}

pub struct Definition {
    id: DefId,
    scope: ScopeId,
    name: String,
    kind: DefKind,
}

pub struct Resolver {
    hir_module: HIRModule,
    diagnostics: Diag,
    corrupted: bool,
}

impl Resolver {
    pub fn new(hir_module: HIRModule, diagnostics: Diag) -> Self {
        Resolver {
            hir_module,
            diagnostics,
            corrupted: false,
        }
    }

    pub fn resolve(&mut self) {
        let module = std::mem::take(&mut self.hir_module);
        for expr in module.exprs {
            self.define_and_resolve(expr);
        }
    }

    pub fn define_and_resolve(&self, expr: HIRExpr) {
        match expr.kind {
            HIRExprKind::Block(exprs) => (),
            HIRExprKind::Lambda { params, ret, body } => (),
            _ => (),
        }
    }

    fn create_scope() {}
}
