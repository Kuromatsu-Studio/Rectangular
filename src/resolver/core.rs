use std::collections::HashMap;

use crate::{
    diagnostics::{CompilerError, Diag, Span},
    hir::{ExprId, HIRExprKind, HIRModule, HIRParam, HIRTypeKind, TypeNodeId},
    resolver::expr_children,
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
    pub type_refs: Vec<Option<DefId>>,
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
            type_refs: Vec::new(),
            corrupted: false,
        }
    }

    pub fn resolve(&mut self) {
        let module = std::mem::take(&mut self.hir_module);
        self.refs = vec![None; module.exprs.len()];
        self.type_refs = vec![None; module.types.len()];

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
                HIRExprKind::Bind { ty, init, .. } => {
                    if let Some(ty) = ty {
                        self.resolve_type(module, *ty);
                    }
                    if let Some(init) = init {
                        self.resolve_expr(module, *init);
                    }
                }

                HIRExprKind::Block(exprs) => {
                    self.push_scope();
                    self.define_scope(module, exprs);
                    for e in exprs {
                        self.resolve_expr(module, *e);
                    }
                    self.pop_scope();
                }
                HIRExprKind::TypeValue(ty) => self.resolve_type(module, *ty),
                HIRExprKind::Lambda { params, body, ret } => {
                    self.push_scope();

                    self.resolve_params(module, params, root);

                    if let Some(ret_ty) = ret {
                        self.resolve_type(module, *ret_ty);
                    }

                    self.define_scope(module, &[*body]);
                    self.resolve_expr(module, *body);

                    self.pop_scope();
                }
                HIRExprKind::LambdaDecl { params, ret } => {
                    self.push_scope();
                    self.resolve_params(module, params, root);

                    self.resolve_type(module, *ret);
                    self.pop_scope();
                }
                HIRExprKind::Struct { members } => {
                    for member in members {
                        self.resolve_type(module, member.ty);
                    }
                }
                HIRExprKind::Identifier(name) => {
                    let def = self.lookup(name.as_str());
                    if def.is_none() {
                        self.report(format!("Undefined name '{}'", name), Some(e.span.clone()));
                    }
                    self.refs[root.0] = def;
                }
                _ => {
                    let mut children = Vec::new();
                    expr_children(e, &mut children);
                    for child in children {
                        self.resolve_expr(module, child);
                    }
                }
            },
            None => (),
        }
    }

    fn resolve_type(&mut self, module: &HIRModule, id: TypeNodeId) {
        let ty_node = match module.types.get(id.0) {
            Some(node) => node,
            None => return,
        };

        match &ty_node.kind {
            HIRTypeKind::Primitive(_) => {}
            HIRTypeKind::Path(segments) => {
                if segments.len() != 1 {
                    return;
                }
                let def = self.lookup(&segments[0]);
                if def.is_none() {
                    self.report(
                        format!("Undefined type '{}'", segments[0]),
                        Some(ty_node.span.clone()),
                    );
                }
                self.type_refs[id.0] = def;
            }
            HIRTypeKind::Pointer(inner) => self.resolve_type(module, *inner),
            HIRTypeKind::Function { params, return_ty } => {
                for param in params {
                    self.resolve_type(module, *param);
                }
                self.resolve_type(module, *return_ty);
            }
            HIRTypeKind::Applied { callee, args } => {
                self.resolve_type(module, *callee);
                for arg in args {
                    self.resolve_type(module, *arg);
                }
            }
            HIRTypeKind::Array { ty, length } => {
                self.resolve_type(module, *ty);
                self.resolve_expr(module, *length);
            }
        }
    }

    fn resolve_params(&mut self, module: &HIRModule, params: &Vec<HIRParam>, root: ExprId) {
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
            self.resolve_type(module, param.ty);

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
