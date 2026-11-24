use std::collections::HashMap;

use crate::{
    diagnostics::{CompilerError, Diag, Span},
    hir::{ExprId, HIRExprKind, HIRModule},
};

#[derive(Debug, Clone, Copy)]
pub struct DefId(pub usize);

#[derive(Clone, Copy)]
pub struct ScopeId(pub usize);

pub struct Scope {
    parent_scope_id: Option<ScopeId>,
    scope_id: ScopeId,
    pub names: HashMap<String, DefId>,
}

pub enum DefKind {
    Bind { id: ExprId },
    Param { lambda: ExprId, index: usize },
}

pub struct Definition {
    pub id: DefId,
    pub scope: ScopeId,
    pub name: String,
    pub kind: DefKind,
}

pub struct Resolver {
    hir_module: HIRModule,
    diagnostics: Diag,
    pub definitions: Vec<Definition>,
    pub scopes: Vec<Scope>,
    pub stack: Vec<ScopeId>,
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
                HIRExprKind::Lambda { params, body, .. } => {
                    self.push_scope();

                    let scope_id = match self.stack.last() {
                        Some(id) => *id,
                        None => return,
                    };

                    for (index, param) in params.iter().enumerate() {
                        if self.scopes[scope_id.0].names.contains_key(&param.name) {
                            self.report(
                                format!("Redefinition of '{}' in the same scope", param.name),
                                Some(param.span.clone()),
                            );
                            continue;
                        }

                        let def_id = DefId(self.definitions.len());
                        self.definitions.push(Definition {
                            id: def_id,
                            name: param.name.clone(),
                            kind: DefKind::Param {
                                lambda: root,
                                index,
                            },
                            scope: scope_id,
                        });

                        self.scopes[scope_id.0]
                            .names
                            .insert(param.name.clone(), def_id);
                    }

                    self.define_scope(module, &[*body]);
                    self.resolve_expr(module, *body);

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
                HIRExprKind::Literal(_) => (),

                _ => todo!("Add resolution for other HIR expressions"),
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

    pub fn report(&mut self, message: String, span: Option<Span>) {
        self.corrupted = true;
        self.diagnostics
            .borrow_mut()
            .report(CompilerError::error(message, span, None));
    }

    pub fn is_corrupted(&self) -> bool {
        self.corrupted
    }
}
