use std::{cell::RefCell, env, fs, rc::Rc};

use crate::{diagnostics::Diagnostics, lexer::Lexer};

mod diagnostics;
mod lexer;

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();
    let mut filename: Option<&String> = None;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            _ => {
                filename = Some(&args[i]);
                i += 1;
            }
        }
    }

    let filename = match filename {
        Some(f) => f,
        None => {
            eprintln!("No source input was specified");
            std::process::exit(1)
        }
    };
    let source = fs::read_to_string(filename)?;

    //Diagnostics
    let diagnostics = Rc::new(RefCell::new(Diagnostics::new(
        filename.clone(),
        source.clone(),
    )));

    //Lexer
    let mut lexer = Lexer::new(&source, Rc::clone(&diagnostics));
    let tokens = lexer.tokenize();
    if lexer.corrupted {
        diagnostics.borrow().dump();
        std::process::exit(1);
    }
    println!("TOKENS : {:?}", tokens);
    Ok(())
}
