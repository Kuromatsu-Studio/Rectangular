use rectangular::diagnostics::{Diag, Diagnostics};
use rectangular::lexer::Lexer;
use std::cell::RefCell;
use std::rc::Rc;

pub fn make_diag(source: &str) -> Diag {
    Rc::new(RefCell::new(Diagnostics::new(
                "test.rgl".to_string(), 
                source.to_string(),
            )))
}

pub fn make_lexer<'a>(source: &'a str, diag: Diag) -> Lexer<'a> {
    Lexer::new(source, diag)
}
