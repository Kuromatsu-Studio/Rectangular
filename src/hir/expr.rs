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
    pub attributes: Vec<AttrId>,
    pub name: String,
    pub ty: TypeNodeId,
    pub span: Span,
}

#[derive(Debug)]
pub struct ConstructorMember {
    pub name: String,
    pub val: ExprId,
    pub span: Span,
}

#[derive(Debug)]
pub enum HIRExprLiteral {
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
    Literal(HIRExprLiteral),
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
    If {
        condition: ExprId,
        if_body: ExprId,
        else_body: Option<ExprId>,
    },
    Access {
        left: ExprId,
        right: ExprId,
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
    Struct {
        members: Vec<HIRParam>,
    },
    Constructor {
        name: String,
        members: Vec<ConstructorMember>,
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
    pub id: ExprId,
    pub kind: HIRExprKind,
    pub span: Span,
}

impl HIRExpr {
    pub fn new(id: ExprId, kind: HIRExprKind, span: Span) -> HIRExpr {
        HIRExpr { id, kind, span }
    }
}
