use crate::{
    ast::ASTExpr,
    diagnostics::Span,
    lexer::{TType, Token},
};

#[derive(Debug)]
pub enum ASTTypeKind {
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
    Bool,
    Custom(String),
    Expr(Box<ASTExpr>), // any expression that evaluates to a type, like List(i32)
    Pointer(Box<ASTType>),
    Unit,
    Type,
    None,
}

#[derive(Debug)]
pub struct ASTType {
    pub kind: ASTTypeKind,
    pub span: Span,
}

impl ASTType {
    pub fn new(kind: ASTTypeKind, span: Span) -> Self {
        ASTType { kind, span }
    }

    ///Checks if the token qualifies to start a type
    pub fn is_type(token_type: TType) -> bool {
        match token_type {
            TType::I8Key
            | TType::U8Key
            | TType::I16Key
            | TType::U16Key
            | TType::I32Key
            | TType::U32Key
            | TType::I64Key
            | TType::U64Key
            | TType::F32Key
            | TType::F64Key
            | TType::BoolKey
            | TType::TypeKey
            | TType::Asterisk
            | TType::Identifier
            | TType::UnitKey => true,
            _ => false,
        }
    }

    pub fn basic(token: &Token) -> Self {
        let kind = match token.token_type {
            TType::I8Key => ASTTypeKind::I8,
            TType::U8Key => ASTTypeKind::U8,
            TType::I16Key => ASTTypeKind::I16,
            TType::U16Key => ASTTypeKind::U16,
            TType::I32Key => ASTTypeKind::I32,
            TType::U32Key => ASTTypeKind::U32,
            TType::I64Key => ASTTypeKind::I64,
            TType::U64Key => ASTTypeKind::U64,
            TType::BoolKey => ASTTypeKind::Bool,
            TType::F32Key => ASTTypeKind::F32,
            TType::F64Key => ASTTypeKind::F64,
            TType::TypeKey => ASTTypeKind::Type,
            TType::UnitKey => ASTTypeKind::Unit,
            _ => ASTTypeKind::None,
        };

        ASTType {
            kind,
            span: token.clone().span,
        }
    }
}
