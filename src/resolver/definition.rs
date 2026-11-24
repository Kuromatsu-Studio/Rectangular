use crate::{
    hir::{ExprId, HIRExpr, HIRExprKind, HIRModule},
    resolver::{
        Resolver,
        core::{DefId, DefKind, Definition, ScopeId},
    },
};

impl Resolver {
    pub fn lookup(&self, name: &str) -> Option<DefId> {
        for id in self.stack.iter().rev() {
            if let Some(def) = self.scopes[id.0].names.get(name) {
                return Some(*def);
            }
        }
        None
    }

    fn define_expr(&mut self, expr: &HIRExpr, scope: ScopeId) {
        let def_id = DefId(self.definitions.len());

        let (name, kind) = match &expr.kind {
            HIRExprKind::Bind { name, .. } => (name.clone(), DefKind::Bind { id: expr.id }),
            _ => return,
        };

        if self.scopes[scope.0].names.contains_key(&name) {
            self.report(
                format!("Redefinition of '{}' in the same scope", name),
                Some(expr.span.clone()),
            );
            return;
        }

        let definition = Definition {
            id: def_id,
            name: name.clone(),
            scope,
            kind,
        };

        self.scopes[scope.0]
            .names
            .insert(definition.name.clone(), def_id);

        self.definitions.push(definition);
    }

    pub fn define_scope(&mut self, module: &HIRModule, exprs: &[ExprId]) {
        let current_scope_id = match self.stack.last() {
            Some(id) => *id,
            None => return,
        };

        for expr_id in exprs {
            let expr = module.exprs.get(expr_id.0);
            match expr {
                Some(e) => self.define_expr(e, current_scope_id),
                None => continue,
            }
        }
    }
}
