use std::collections::HashMap;

use crate::{
    diagnostics::{CompilerError, Diag, Span},
    hir::{ExprId, HIRExpr, HIRExprKind, HIRModule},
};

#[derive(Debug, Clone, Copy)]
pub struct DefId(usize);

#[derive(Clone, Copy)]
pub struct ScopeId(pub usize);

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
    definitions: Vec<Definition>,
    scopes: Vec<Scope>,
    stack: Vec<ScopeId>,
    pub refs: Vec<Option<DefId>>,
    corrupted: bool,
}

impl Resolver {
    pub fn new(hir_module: HIRModule, diagnostics: Diag) -> Self {
        Resolver {
            hir_module,
            diagnostics,
            definitions: Vec::new(),
            scopes: Vec::new(),
            stack: Vec::new(),
            refs: Vec::new(),
            corrupted: false,
        }
    }

    pub fn resolve(&mut self) {
        let module = std::mem::take(&mut self.hir_module);
        self.refs = vec![None; module.exprs.len()];
        self.push_scope();
        self.define_scope(&module, &module.roots);
        for root in &module.roots {
            self.resolve_expr(&module, *root);
        }
        self.pop_scope();

        self.hir_module = module;
    }

    fn resolve_expr(&mut self, module: &HIRModule, root: ExprId) {
        let expr = module.exprs.get(root.0);

        match expr {
            Some(e) => match &e.kind {
                HIRExprKind::Block(exprs) => {
                    self.push_scope();
                    self.define_scope(module, exprs);
                    for e in exprs {
                        self.resolve_expr(module, *e);
                    }
                    self.pop_scope();
                }
                HIRExprKind::Identifier(name) => {
                    let def = self.lookup(name.as_str());
                    if def.is_none() {
                        self.report(format!("Undefined name '{}'", name), Some(e.span.clone()));
                    }
                    self.refs[root.0] = def;
                }
                HIRExprKind::Bind { init, .. } => match init {
                    Some(init) => self.resolve_expr(module, *init),
                    None => (),
                },
                _ => (),
            },
            None => (),
        }
    }

    fn push_scope(&mut self) {
        let id = ScopeId(self.scopes.len());
        let parent_scope_id = self.stack.last().copied();
        let scope = Scope {
            parent_scope_id,
            scope_id: id,
            names: HashMap::new(),
        };
        self.scopes.push(scope);
        self.stack.push(id);
    }

    fn pop_scope(&mut self) {
        self.stack.pop();
    }

    fn lookup(&self, name: &str) -> Option<DefId> {
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

    fn define_scope(&mut self, module: &HIRModule, exprs: &Vec<ExprId>) {
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

    fn report(&mut self, message: String, span: Option<Span>) {
        self.corrupted = true;
        self.diagnostics
            .borrow_mut()
            .report(CompilerError::error(message, span, None));
    }

    pub fn is_corrupted(&self) -> bool {
        self.corrupted
    }
}
