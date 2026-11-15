use crate::{
    ast::ASTTypeKind,
    diagnostics::Span,
    hir::hir::{ExprId, TypeNodeId},
};

#[derive(Debug)]
pub enum Prim {
    I8,
    U8,
    I16,
    U16,
    I32,
    U32,
    I64,
    U64,
    F32,
    F64,
    C8,
    C16,
    C32,
    Bool,
    Unit,
    Type,
    Error,
}

#[derive(Debug)]
pub enum HIRTypeKind {
    Primitive(Prim),
    Path(Vec<String>),
    Pointer(TypeNodeId),
    Function {
        params: Vec<TypeNodeId>,
        return_ty: TypeNodeId,
    },
    Applied {
        callee: TypeNodeId,
        args: Vec<TypeNodeId>,
    },
    Array {
        ty: TypeNodeId,
        length: ExprId,
    },
}

#[derive(Debug)]
pub struct HIRType {
    pub id: TypeNodeId,
    pub kind: HIRTypeKind,
    pub span: Span,
}

impl Prim {
    pub fn new(ty_kind: ASTTypeKind) -> Prim {
        match ty_kind {
            ASTTypeKind::I8 => Prim::I8,
            ASTTypeKind::U8 => Prim::U8,
            ASTTypeKind::I16 => Prim::I16,
            ASTTypeKind::U16 => Prim::U16,
            ASTTypeKind::U32 => Prim::U32,
            ASTTypeKind::I32 => Prim::I32,
            ASTTypeKind::I64 => Prim::I64,
            ASTTypeKind::U64 => Prim::U64,
            ASTTypeKind::F32 => Prim::F32,
            ASTTypeKind::F64 => Prim::F64,
            ASTTypeKind::C8 => Prim::C8,
            ASTTypeKind::C16 => Prim::C16,
            ASTTypeKind::C32 => Prim::C32,
            ASTTypeKind::Bool => Prim::Bool,
            ASTTypeKind::Unit => Prim::Unit,
            ASTTypeKind::Type => Prim::Type,
            _ => Prim::Error,
        }
    }
}

impl HIRType {
    pub fn new(id: TypeNodeId, kind: HIRTypeKind, span: Span) -> HIRType {
        HIRType { id, kind, span }
    }
}
