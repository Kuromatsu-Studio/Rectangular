use crate::{
    hir::{ExprId, HIRExpr, HIRExprKind, HIRModule},
    resolver::{
        Resolver,
        core::{DefId, DefKind, Definition, ScopeId},
        expr_chilren,
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
        let scope_id = match self.stack.last() {
            Some(id) => *id,
            None => return,
        };

        let mut pending = exprs.to_vec();
        let mut children = Vec::new();

        while let Some(id) = pending.pop() {
            let expr = match module.exprs.get(id.0) {
                Some(e) => e,
                None => continue,
            };

            self.define_expr(expr, scope_id);

            children.clear();
            expr_chilren(expr, &mut children);
            pending.extend(children.iter());
        }
    }
}
