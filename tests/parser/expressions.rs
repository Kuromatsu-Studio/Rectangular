use crate::common::{make_diag, make_lexer};

use rectangular::ast::ASTExpr;
use rectangular::diagnostics::{Diag};
use rectangular::parser::Parser;
use std::{rc::Rc};

fn parse_source(src: &str) -> (Vec<ASTExpr>, Diag){
    let diag = make_diag(src);
    let mut lexer = make_lexer(src, Rc::clone(&diag));

    let tokens =  lexer.tokenize();
        
    let mut parser = Parser::new(tokens, Rc::clone(&diag));
    let tree = parser.parse();

    (tree, diag)
}
