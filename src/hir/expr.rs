use crate::{
    ast::{BinaryOp, UnaryOp},
    diagnostics::Span,
    hir::hir::{AttrId, ExprId, TypeNodeId},
};

#[derive(Debug)]
pub enum ComptimeKind {
    Read,     //$
    Evaluate, //$$
}

#[derive(Debug)]
pub struct HIRParam {
    attributes: Vec<AttrId>,
    name: String,
    ty: TypeNodeId,
}

#[derive(Debug)]
pub enum HirExprLiteral {
    I8(i8),
    U8(u8),
    I16(i16),
    U16(u16),
    I32(i32),
    U32(u32),
    I64(i64),
    U64(u64),
    F32(f32),
    F64(f64),
    Bool(bool),
    Str(String),
    Char8(u8),
    Char16(u16),
    Char32(char),
}

#[derive(Debug)]
pub enum HIRExprKind {
    Literal(HirExprLiteral),
    Identifier(String),
    Binary {
        op: BinaryOp,
        lhs: ExprId,
        rhs: ExprId,
    },
    While {
        condition: ExprId,
        body: ExprId,
    },
    Unary {
        op: UnaryOp,
        operand: ExprId,
    },
    Call {
        callee: ExprId,
        args: Vec<ExprId>,
    },
    Index {
        base: ExprId,
        index: ExprId,
    },
    Comptime {
        kind: ComptimeKind,
        expr: ExprId,
    },
    Bind {
        attrs: Vec<AttrId>,
        name: String,
        ty: Option<TypeNodeId>,
        init: Option<ExprId>,
    },
    Lambda {
        params: Vec<HIRParam>,
        ret: Option<TypeNodeId>,
        body: ExprId,
    },
    LambdaDecl {
        params: Vec<HIRParam>,
        ret: TypeNodeId,
    },
    TypeValue(TypeNodeId),
    Array(Vec<ExprId>),
    Block(Vec<ExprId>),
    AddressOf(ExprId),
    Defer(ExprId),
    Dereference(ExprId),
    Return(Option<ExprId>),
    Break,
    Continue,
}

#[derive(Debug)]
pub struct HIRExpr {
    id: ExprId,
    kind: HIRExprKind,
    span: Span,
}

impl HIRExpr {
    pub fn new(id: ExprId, kind: HIRExprKind, span: Span) -> HIRExpr {
        HIRExpr { id, kind, span }
    }
}
