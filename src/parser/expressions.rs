use crate::{
    ast::{Expr, ExprKind, ExprLiteral},
    lexer::{TType, Token},
    parser::parser::Parser,
};

impl Parser {
    fn parse_prefix(&mut self) -> Option<Expr> {
        let token = self.current_token()?.clone();
        match token.token_type {
            TType::I8Literal
            | TType::U8Literal
            | TType::I16Literal
            | TType::U16Literal
            | TType::I32Literal
            | TType::U32Literal
            | TType::I64Literal
            | TType::U64Literal
            | TType::F32Literal
            | TType::F64Literal
            | TType::IntLiteral
            | TType::FloatLiteral
            | TType::True
            | TType::False => self.parse_literal(),
            _ => {
                let span = token.span;
                self.report(
                    format!("Unexpected prefix token: {:?}", token.token_type),
                    Some(span),
                );
                self.advance();
                None
            }
        }
    }

    fn parse_literal(&mut self) -> Option<Expr> {
        let token = self.current_token()?.clone();
        let span = self.current_token()?.span.clone();

        match token.token_type {
            TType::True => {
                self.advance();
                Some(Expr::new(ExprKind::Literal(ExprLiteral::Bool(true)), span))
            }
            TType::False => {
                self.advance();
                Some(Expr::new(ExprKind::Literal(ExprLiteral::Bool(false)), span))
            }
            TType::I8Literal
            | TType::U8Literal
            | TType::I16Literal
            | TType::U16Literal
            | TType::I32Literal
            | TType::U32Literal
            | TType::I64Literal
            | TType::U64Literal
            | TType::IntLiteral
            | TType::F32Literal
            | TType::F64Literal
            | TType::FloatLiteral => {
                let lit = self.get_value(&token)?;
                self.advance();
                Some(Expr::new(ExprKind::Literal(lit), span))
            }
            _ => {
                self.report(
                    format!("Expected literal, found {:?}", token.token_type),
                    Some(span),
                );
                self.advance();
                None
            }
        }
    }

    fn parse_number(&self, num_str: &str) -> Option<i64> {
        if num_str.starts_with("0x") || num_str.starts_with("0X") {
            let hex_str = &num_str[2..];
            i64::from_str_radix(hex_str, 16).ok()
        } else if num_str.starts_with("0b") || num_str.starts_with("0B") {
            let bin_str = &num_str[2..];
            i64::from_str_radix(bin_str, 2).ok()
        } else if num_str.starts_with("0o") || num_str.starts_with("0O") {
            let oct_str = &num_str[2..];
            i64::from_str_radix(oct_str, 8).ok()
        } else {
            num_str.parse::<i64>().ok()
        }
    }

    fn parse_float(&self, num_str: &str) -> Option<f64> {
        num_str.parse::<f64>().ok()
    }

    pub fn get_value(&mut self, token: &Token) -> Option<ExprLiteral> {
        let lexeme = &token.lexeme;
        match token.token_type {
            TType::I8Literal => {
                let value = self.parse_number(lexeme)? as i8;
                Some(ExprLiteral::I8(value))
            }
            TType::U8Literal => {
                let value = self.parse_number(lexeme)? as u8;
                Some(ExprLiteral::U8(value))
            }
            TType::I16Literal => {
                let value = self.parse_number(lexeme)? as i16;
                Some(ExprLiteral::I16(value))
            }
            TType::U16Literal => {
                let value = self.parse_number(lexeme)? as u16;
                Some(ExprLiteral::U16(value))
            }
            TType::I32Literal => {
                let value = self.parse_number(lexeme)? as i32;
                Some(ExprLiteral::I32(value))
            }
            TType::U32Literal => {
                let value = self.parse_number(lexeme)? as u32;
                Some(ExprLiteral::U32(value))
            }
            TType::I64Literal => {
                let value = self.parse_number(lexeme)? as i64;
                Some(ExprLiteral::I64(value))
            }
            TType::U64Literal => {
                let value = self.parse_number(lexeme)? as u64;
                Some(ExprLiteral::U64(value))
            }
            TType::IntLiteral => {
                let value = self.parse_number(lexeme)? as i32;
                Some(ExprLiteral::I32(value))
            }
            TType::F32Literal => {
                let value = self.parse_float(lexeme)? as f32;
                Some(ExprLiteral::F32(value))
            }
            TType::F64Literal => {
                let value = self.parse_float(lexeme)? as f64;
                Some(ExprLiteral::F64(value))
            }
            TType::FloatLiteral => {
                let value = self.parse_float(lexeme)? as f32;
                Some(ExprLiteral::F32(value))
            }
            _ => {
                let span = token.clone().span;
                self.report(
                    format!("Unexpected token type: {:?}", token.token_type),
                    Some(span),
                );
                None
            }
        }
    }
}
