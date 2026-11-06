use std::fmt::{self};

use crate::ast::ast::{BinaryOp, UnaryOp};
use crate::ast::{
    ASTConstructorMember, ASTExpr, ASTExprKind, ASTExprLiteral, ASTParam, ASTType, ASTTypeKind,
};

use crate::diagnostics::Span;

fn indent(f: &mut fmt::Formatter, depth: usize) -> fmt::Result {
    for _ in 0..depth {
        write!(f, "    ")?;
    }
    Ok(())
}

impl fmt::Display for Span {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}..{}", self.start, self.end)
    }
}

impl fmt::Display for ASTExprLiteral {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ASTExprLiteral::I8(v) => write!(f, "{}i8", v),
            ASTExprLiteral::U8(v) => write!(f, "{}u8", v),
            ASTExprLiteral::I16(v) => write!(f, "{}i16", v),
            ASTExprLiteral::U16(v) => write!(f, "{}u16", v),
            ASTExprLiteral::I32(v) => write!(f, "{}i32", v),
            ASTExprLiteral::U32(v) => write!(f, "{}u32", v),
            ASTExprLiteral::I64(v) => write!(f, "{}i64", v),
            ASTExprLiteral::U64(v) => write!(f, "{}u64", v),
            ASTExprLiteral::F32(v) => write!(f, "{}f32", v),
            ASTExprLiteral::F64(v) => write!(f, "{}f64", v),
            ASTExprLiteral::Bool(v) => write!(f, "{}", v),
            ASTExprLiteral::Str(v) => write!(f, "{:?}", v),
            ASTExprLiteral::Char8(v) => write!(f, "{:?}c8", *v as char),
            ASTExprLiteral::Char16(v) => {
                write!(
                    f,
                    "{:?}c16",
                    char::from_u32(*v as u32).unwrap_or('\u{FFFD}')
                )
            }
            ASTExprLiteral::Char32(v) => write!(f, "{:?}c32", v),
        }
    }
}

impl BinaryOp {
    fn as_str(&self) -> &str {
        match self {
            BinaryOp::Add => "+",
            BinaryOp::Minus => "-",
            BinaryOp::Mul => "*",
            BinaryOp::Div => "/",
            BinaryOp::Gt => ">",
            BinaryOp::Gte => ">=",
            BinaryOp::Lt => "<",
            BinaryOp::Lte => "<=",
            BinaryOp::Neq => "!=",
            BinaryOp::Eq => "==",
            BinaryOp::Modulus => "%",
            BinaryOp::Assign => "=",
            BinaryOp::AddAssign => "+=",
            BinaryOp::SubAssign => "-=",
            BinaryOp::MulAssign => "*=",
            BinaryOp::DivAssign => "/=",
            BinaryOp::LogicalOr => "||",
            BinaryOp::LogicalAnd => "&&",
            BinaryOp::BitAnd => "&",
            BinaryOp::BitOr => "|",
            BinaryOp::Shr => ">>",
            BinaryOp::Shl => "<<",
            BinaryOp::BitXor => "^",
            BinaryOp::Range => "..",
            BinaryOp::Invalid => "<??>",
        }
    }
}

impl UnaryOp {
    fn as_str(&self) -> &str {
        match self {
            UnaryOp::Neg => "-",
            UnaryOp::Not => "!",
            UnaryOp::BitNot => "~",
            UnaryOp::Invalid => "<??>",
        }
    }
}

impl fmt::Display for ASTType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            ASTTypeKind::I8 => write!(f, "i8"),
            ASTTypeKind::U8 => write!(f, "u8"),
            ASTTypeKind::I16 => write!(f, "i16"),
            ASTTypeKind::U16 => write!(f, "u16"),
            ASTTypeKind::I32 => write!(f, "i32"),
            ASTTypeKind::U32 => write!(f, "u32"),
            ASTTypeKind::I64 => write!(f, "i64"),
            ASTTypeKind::U64 => write!(f, "u64"),
            ASTTypeKind::F32 => write!(f, "f32"),
            ASTTypeKind::C8 => write!(f, "c8"),
            ASTTypeKind::C16 => write!(f, "c16"),
            ASTTypeKind::C32 => write!(f, "c32"),
            ASTTypeKind::F64 => write!(f, "f64"),
            ASTTypeKind::Bool => write!(f, "bool"),
            ASTTypeKind::Unit => write!(f, "unit"),
            ASTTypeKind::Type => write!(f, "type"),
            ASTTypeKind::Expr(inner) => write!(f, "{}", inner),
            ASTTypeKind::Pointer(inner) => write!(f, "*{}", inner),
            ASTTypeKind::None => write!(f, "<?>"),
            ASTTypeKind::Custom(name) => write!(f, "{}", name),
        }
    }
}

impl fmt::Display for ASTParam {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Param {}: {}", self.name.as_ref(), self.ty.as_ref())
    }
}

impl fmt::Display for ASTConstructorMember {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Member {} : {}", self.name.as_ref(), self.val.as_ref())
    }
}

impl ASTExpr {
    fn fmt_pretty(&self, f: &mut fmt::Formatter, depth: usize) -> fmt::Result {
        match &self.kind {
            ASTExprKind::Literal(lit) => {
                indent(f, depth)?;
                write!(f, "Literal({}) @ {}", lit, self.span)
            }

            ASTExprKind::Identifier(name) => {
                indent(f, depth)?;
                write!(f, "Identifier({}) @ {}", name, self.span)
            }

            ASTExprKind::Break => {
                indent(f, depth)?;
                write!(f, "Break @{}", self.span)
            }

            ASTExprKind::Continue => {
                indent(f, depth)?;
                write!(f, "Continue @{}", self.span)
            }

            ASTExprKind::TypeExpr(ty) => {
                indent(f, depth)?;
                write!(f, "TypeExpr: {} @ {}", ty, self.span)
            }

            ASTExprKind::Dereference(inner) => {
                indent(f, depth)?;
                writeln!(f, "Dereference @ {} {{", self.span)?;
                inner.fmt_pretty(f, depth + 1)?;
                writeln!(f)?;
                indent(f, depth)?;
                write!(f, "}}")
            }

            ASTExprKind::AddressOf(inner) => {
                indent(f, depth)?;
                writeln!(f, "AddressOf @ {} {{", self.span)?;
                inner.fmt_pretty(f, depth + 1)?;
                writeln!(f)?;
                indent(f, depth)?;
                write!(f, "}}")
            }

            ASTExprKind::DollarExpr(expr) => {
                indent(f, depth)?;
                writeln!(f, "DollarExpr @ {} {{", self.span)?;
                expr.fmt_pretty(f, depth + 1)?;
                writeln!(f)?;
                indent(f, depth)?;
                write!(f, "}}")
            }

            ASTExprKind::DoubleDollar(expr) => {
                indent(f, depth)?;
                writeln!(f, "DoubleDollarExpr @ {} {{", self.span)?;
                expr.fmt_pretty(f, depth + 1)?;
                writeln!(f)?;
                indent(f, depth)?;
                write!(f, "}}")
            }

            ASTExprKind::Attributed {
                attributes,
                binding,
            } => {
                indent(f, depth)?;
                writeln!(f, "Attributed @ {} {{", self.span)?;
                indent(f, depth + 1)?;
                write!(f, "Attributes: ")?;
                for (i, attribute) in attributes.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", attribute)?;
                }
                writeln!(f)?;
                indent(f, depth + 1)?;
                writeln!(f, "Binding:")?;
                binding.fmt_pretty(f, depth + 2)?;
                writeln!(f)?;
                indent(f, depth)?;
                write!(f, "}}")
            }

            ASTExprKind::Lambda {
                params,
                ret_ty,
                body,
            } => {
                indent(f, depth)?;
                write!(f, "Lambda @ {} (", self.span)?;
                for (i, param) in params.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", param)?;
                }
                write!(f, ")")?;

                if let Some(ty) = ret_ty {
                    write!(f, " -> {}", ty)?;
                }

                writeln!(f, " {{")?;
                body.fmt_pretty(f, depth + 1)?;
                writeln!(f)?;
                indent(f, depth)?;
                write!(f, "}}")
            }

            ASTExprKind::Bind { names, ty, init } => {
                indent(f, depth)?;
                write!(f, "Bind @ {} (names: [", self.span)?;
                for (i, name) in names.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", name)?;
                }
                write!(f, "]")?;

                if let Some(ty_val) = ty {
                    write!(f, ", type: {}", ty_val)?;
                }

                if let Some(init_val) = init {
                    writeln!(f, ") {{")?;
                    init_val.fmt_pretty(f, depth + 1)?;
                    writeln!(f)?;
                    indent(f, depth)?;
                    write!(f, "}}")
                } else {
                    write!(f, ")")
                }
            }

            ASTExprKind::Struct { members } => {
                indent(f, depth)?;
                if members.is_empty() {
                    write!(f, "Struct {{}} @ {}", self.span)
                } else {
                    writeln!(f, "Struct @ {} {{", self.span)?;
                    for (i, member) in members.iter().enumerate() {
                        if i > 0 {
                            writeln!(f)?;
                        }
                        indent(f, depth + 1)?;
                        write!(f, "{}", member)?;
                    }
                    writeln!(f)?;
                    indent(f, depth)?;
                    write!(f, "}}")
                }
            }

            ASTExprKind::StructConstructor { name, members } => {
                indent(f, depth)?;
                if members.is_empty() {
                    write!(f, "StructConstructor({}) @ {}", name, self.span)
                } else {
                    writeln!(f, "Struct Constructor: {} @ {} {{", name, self.span)?;
                    for (i, member) in members.iter().enumerate() {
                        if i > 0 {
                            writeln!(f)?;
                        }
                        indent(f, depth + 1)?;
                        write!(f, "{}", member)?;
                    }
                    writeln!(f)?;
                    indent(f, depth)?;
                    write!(f, "}}")
                }
            }

            ASTExprKind::IfExpr {
                condition,
                if_body,
                else_body,
            } => {
                indent(f, depth)?;
                writeln!(f, "IfExpr @ {} {{", self.span)?;
                indent(f, depth + 1)?;
                writeln!(f, "Condition:")?;
                condition.fmt_pretty(f, depth + 2)?;
                writeln!(f)?;
                indent(f, depth + 1)?;
                writeln!(f, "Body:")?;
                if_body.fmt_pretty(f, depth + 2)?;
                if let Some(else_bod) = else_body {
                    writeln!(f)?;
                    indent(f, depth + 1)?;
                    writeln!(f, "Else:")?;
                    else_bod.fmt_pretty(f, depth + 2)?;
                }
                writeln!(f)?;
                indent(f, depth)?;
                write!(f, "}}")
            }

            ASTExprKind::WhileExpr { condition, body } => {
                indent(f, depth)?;
                writeln!(f, "WhileExpr @ {} {{", self.span)?;
                indent(f, depth + 1)?;
                writeln!(f, "Condition:")?;
                condition.fmt_pretty(f, depth + 2)?;
                writeln!(f)?;
                indent(f, depth + 1)?;
                writeln!(f, "Body:")?;
                body.fmt_pretty(f, depth + 2)?;
                writeln!(f)?;
                indent(f, depth)?;
                write!(f, "}}")
            }

            ASTExprKind::Unary(op, expr) => {
                indent(f, depth)?;
                writeln!(f, "Unary({}) @ {} {{", op.as_str(), self.span)?;
                expr.fmt_pretty(f, depth + 1)?;
                writeln!(f)?;
                indent(f, depth)?;
                write!(f, "}}")
            }

            ASTExprKind::Binary(left, op, right) => {
                indent(f, depth)?;
                writeln!(f, "Binary({}) @ {} {{", op.as_str(), self.span)?;
                left.fmt_pretty(f, depth + 1)?;
                writeln!(f)?;
                right.fmt_pretty(f, depth + 1)?;
                writeln!(f)?;
                indent(f, depth)?;
                write!(f, "}}")
            }

            ASTExprKind::Block(exprs) => {
                indent(f, depth)?;
                if exprs.is_empty() {
                    write!(f, "Block {{}} @ {}", self.span)
                } else {
                    writeln!(f, "Block @ {} {{", self.span)?;
                    for (i, e) in exprs.iter().enumerate() {
                        if i > 0 {
                            writeln!(f)?;
                        }
                        e.fmt_pretty(f, depth + 1)?;
                    }
                    writeln!(f)?;
                    indent(f, depth)?;
                    write!(f, "}}")
                }
            }

            ASTExprKind::Call(func, args) => {
                indent(f, depth)?;
                writeln!(f, "Call @ {} {{", self.span)?;
                indent(f, depth + 1)?;
                writeln!(f, "Func:")?;
                func.fmt_pretty(f, depth + 2)?;
                if !args.is_empty() {
                    writeln!(f)?;
                    indent(f, depth + 1)?;
                    writeln!(f, "Args:")?;
                    for arg in args {
                        writeln!(f)?;
                        arg.fmt_pretty(f, depth + 2)?;
                    }
                }
                writeln!(f)?;
                indent(f, depth)?;
                write!(f, "}}")
            }

            ASTExprKind::Index(base, idx) => {
                indent(f, depth)?;
                writeln!(f, "Index @ {} {{", self.span)?;
                indent(f, depth + 1)?;
                writeln!(f, "Base:")?;
                base.fmt_pretty(f, depth + 2)?;
                writeln!(f)?;
                indent(f, depth + 1)?;
                writeln!(f, "Index:")?;
                idx.fmt_pretty(f, depth + 2)?;
                writeln!(f)?;
                indent(f, depth)?;
                write!(f, "}}")
            }

            ASTExprKind::Access(left, right) => {
                indent(f, depth)?;
                writeln!(f, "Access @ {} {{", self.span)?;
                indent(f, depth + 1)?;
                writeln!(f, "Left: ")?;
                left.fmt_pretty(f, depth + 2)?;
                writeln!(f)?;
                indent(f, depth + 1)?;
                writeln!(f, "Right: ")?;
                right.fmt_pretty(f, depth + 2)?;
                writeln!(f)?;
                indent(f, depth)?;
                write!(f, "}}")
            }

            ASTExprKind::Array(exprs) => {
                indent(f, depth)?;
                if exprs.is_empty() {
                    write!(f, "Array[] @ {}", self.span)
                } else {
                    writeln!(f, "Array @ {} {{", self.span)?;
                    for (i, e) in exprs.iter().enumerate() {
                        if i > 0 {
                            writeln!(f)?;
                        }
                        e.fmt_pretty(f, depth + 1)?;
                    }
                    writeln!(f)?;
                    indent(f, depth)?;
                    write!(f, "}}")
                }
            }

            ASTExprKind::Return(expr) => {
                indent(f, depth)?;
                if let Some(expr_val) = expr {
                    writeln!(f, "Return @ {} {{", self.span)?;
                    expr_val.fmt_pretty(f, depth + 1)?;
                    writeln!(f)?;
                    indent(f, depth)?;
                    write!(f, "}}")
                } else {
                    write!(f, "Return @ {} (void)", self.span)
                }
            }
        }
    }
}

impl fmt::Display for ASTExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_pretty(f, 0)
    }
}

pub fn pretty_module(exprs: Vec<ASTExpr>) -> String {
    let mut out = String::new();
    for (i, stmt) in exprs.iter().enumerate() {
        if i > 0 {
            out.push_str("\n\n");
        }
        out.push_str(&format!("{}", stmt));
    }
    out
}
