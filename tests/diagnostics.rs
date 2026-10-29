mod common;

use common::make_diag;
use rectangular::diagnostics::{CompilerError, Span};

// get_line_col tests
#[test]
fn line_col_first_character(){
    let diag = make_diag("ab\ncd\nef");
    assert_eq!(diag.borrow().source_map.get_line_col(0), (1, 1));
}

#[test]
fn line_col_on_the_newline_itself(){
    let diag = make_diag("ab\ncd\nef");
    assert_eq!(diag.borrow().source_map.get_line_col(2), (1, 3));
}

#[test]
fn line_col_first_character_after_newline(){
    let diag = make_diag("ab\ncd\nef");
    assert_eq!(diag.borrow().source_map.get_line_col(3), (2, 1));
}

#[test]
fn line_col_last_line(){
    let diag = make_diag("ab\ncd\nef");
    assert_eq!(diag.borrow().source_map.get_line_col(7), (3, 2));
}

#[test]
fn line_col_one_past_end(){
    let diag = make_diag("ab\ncd\nef");
    assert_eq!(diag.borrow().source_map.get_line_col(8), (3, 3));
}

#[test]
fn line_col_empty_source(){
    let diag = make_diag("");
    assert_eq!(diag.borrow().source_map.get_line_col(0), (1, 1));
}

// Span Tests
#[test]
fn snippet_in_bounds(){
    let diag = make_diag("let x = 5");
    let snippet = diag.borrow().source_map.get_snippet(&Span::new(4, 5));
    assert_eq!(snippet, "x");
}

#[test]
fn snippet_end_past_source_is_clamped(){
    let diag = make_diag("let x = 5");
    let snippet = diag.borrow().source_map.get_snippet(&Span::new(4, 100));
    assert_eq!(snippet, "x = 5");
}

#[test]
fn snippet_start_past_source_is_empty(){
    let diag = make_diag("let x = 5");
    let snippet = diag.borrow().source_map.get_snippet(&Span::new(50, 100));
    assert_eq!(snippet, "");
}

#[test]
fn line_snippet_first_line(){
    let diag = make_diag("ab\ncd\nef");
    assert_eq!(diag.borrow().source_map.get_line_snippet(0), "ab");
}

#[test]
fn line_snippet_middle_line(){
    let diag = make_diag("ab\ncd\nef");
    assert_eq!(diag.borrow().source_map.get_line_snippet(4), "cd");
}

#[test]
fn line_snippet_last_line(){
    let diag = make_diag("ab\ncd\nef");
    assert_eq!(diag.borrow().source_map.get_line_snippet(7), "ef");
}

// compilererror tests
#[test]
fn report_error_goes_to_errors(){
    let diag = make_diag("let x = 5");
    diag.borrow_mut().report(CompilerError::error("bad thing".to_string(), None, None));

    assert_eq!(diag.borrow().errors.len(), 1);
    assert_eq!(diag.borrow().warnings.len(), 0);
}

// compilerwarnings test
#[test]
fn report_warning_goes_to_warnings(){
    let diag = make_diag("let x = 5");
    diag.borrow_mut().report(CompilerError::warning("careful".to_string(), None, None));

    assert_eq!(diag.borrow().warnings.len(), 1);
    assert_eq!(diag.borrow().errors.len(), 0);
}
