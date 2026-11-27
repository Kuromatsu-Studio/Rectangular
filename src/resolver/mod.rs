mod core;
mod definition;

pub use core::Resolver;

use crate::hir::{ExprId, HIRExpr, HIRExprKind};

pub fn expr_children(expr: &HIRExpr, out: &mut Vec<ExprId>) {
    match &expr.kind {
        HIRExprKind::Binary { lhs, rhs, .. } => {
            out.push(*lhs);
            out.push(*rhs);
        }
        HIRExprKind::While { condition, body } => {
            out.push(*condition);
            out.push(*body);
        }
        HIRExprKind::If {
            condition,
            if_body,
            else_body,
        } => {
            out.push(*condition);
            out.push(*if_body);
            if let Some(el) = else_body {
                out.push(*el);
            }
        }
        HIRExprKind::Unary { operand, .. } => out.push(*operand),
        HIRExprKind::Index { base, index } => {
            out.push(*base);
            out.push(*index);
        }
        HIRExprKind::Call { callee, args } => {
            out.push(*callee);
            out.extend(args.iter());
        }
        HIRExprKind::Constructor { name, members, .. } => {
            out.push(*name);
            out.extend(members.iter().map(|member| member.val));
        }

        HIRExprKind::Comptime { expr: inner, .. } => out.push(*inner),
        HIRExprKind::Array(elems) => out.extend(elems.iter()),
        HIRExprKind::Access { left, .. } => out.push(*left),
        HIRExprKind::Return(inner) => {
            if let Some(e) = inner {
                out.push(*e);
            }
        }
        HIRExprKind::Dereference(inner)
        | HIRExprKind::AddressOf(inner)
        | HIRExprKind::Defer(inner) => out.push(*inner),

        HIRExprKind::Literal(_)
        | HIRExprKind::Identifier(_)
        | HIRExprKind::Lambda { .. }
        | HIRExprKind::LambdaDecl { .. }
        | HIRExprKind::Struct { .. }
        | HIRExprKind::TypeValue(_)
        | HIRExprKind::Block(_)
        | HIRExprKind::Bind { .. }
        | HIRExprKind::Break
        | HIRExprKind::Continue => {}
    }
}
