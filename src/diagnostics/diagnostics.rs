use std::{cell::RefCell, rc::Rc};

use crate::diagnostics::{
    errors::{CompilerError, Severity},
    source_map::SourceMap,
};
use colored::*;

pub type Diag = Rc<RefCell<Diagnostics>>;

///This is the main diagnostics engine it is responsible for collecting errors and printing them
pub struct Diagnostics {
    pub source_map: SourceMap,
    pub filename: String,
    pub errors: Vec<CompilerError>,
    pub warnings: Vec<CompilerError>,
}

impl Diagnostics {
    pub fn new(filename: String, source: String) -> Self {
        Diagnostics {
            source_map: SourceMap::new(source),
            filename,
            errors: Vec::new(),
            warnings: Vec::new(),
        }
    }

    ///This takes the compiler error and routes it according to severity
    pub fn report(&mut self, error: CompilerError) {
        match error.severity {
            Severity::Error => self.errors.push(error),
            Severity::Warning => self.warnings.push(error),
            Severity::Ice => self.report_ice(error),
        }
    }

    ///This is the method called by the main compiler driver to dump the errors
    pub fn dump(&self) {
        for warning in &self.warnings {
            self.print_error(&warning);
        }

        for error in &self.errors {
            self.print_error(&error);
        }

        let err_count = self.errors.len();
        let warnings_count = self.warnings.len();
        if warnings_count > 0 || err_count > 0 {
            println!(
                "{}",
                format!(
                    "aborting due to {} error(s), {} warning(s)",
                    err_count, warnings_count
                )
                .bold()
                .red()
            );
        }
    }

    ///Special method that takes an ice error reports it and panics immediately
    fn report_ice(&mut self, error: CompilerError) -> ! {
        self.print_ice_header();
        self.print_error(&error);
        self.print_ice_footer();
        panic!("ICE");
    }

    ///This method is generally used to print all errors and warnings
    fn print_error(&self, error: &CompilerError) {
        let (severity_label, severity_color) = match error.severity {
            Severity::Error => ("error".to_string(), Color::Red),
            Severity::Warning => ("warning".to_string(), Color::Yellow),
            Severity::Ice => ("ice".to_string(), Color::BrightRed),
        };

        if let Some(span) = &error.span {
            let (line, col) = self.source_map.get_line_col(span.start);
            let length = span.length().max(1);
            let line_num_str = line.to_string();
            let padding = " ".repeat(line_num_str.len());
            let line_snippet = self.source_map.get_line_snippet(span.start);

            println!(
                "{}:{}:{}: {}:{}",
                self.filename.color(Color::Cyan),
                line_num_str.color(Color::Yellow),
                col.to_string().color(Color::Yellow),
                severity_label.color(severity_color),
                error.message.color(Color::White).bold()
            );

            //Show the upper context for up to 2 lines
            let context_before = 2;
            let start_line = line.saturating_sub(context_before);
            for ctx_line in start_line..line {
                if ctx_line == 0 {
                    continue;
                }
                let snippet = self
                    .source_map
                    .get_line_snippet(self.source_map.line_starts[ctx_line - 1]);
                println!(
                    " {} | {}",
                    ctx_line.to_string().color(Color::Cyan).dimmed(),
                    snippet.dimmed()
                );
            }

            //Error line
            println!(
                " {} | {}",
                line_num_str.color(Color::Cyan),
                line_snippet.color(Color::White).bold()
            );

            //Caret undelining
            let mut underline = format!(" {} | ", padding);
            for _ in 0..col.saturating_sub(1) {
                underline.push(' ');
            }
            for _ in 0..length {
                underline.push('^');
            }
            println!("{}", underline.color(severity_color).bold());

            // context after, up to 2 lines
            let context_after = 2;
            let end_line = (line + context_after).min(self.source_map.line_starts.len());
            for ctx_line in (line + 1)..=end_line {
                if ctx_line > self.source_map.line_starts.len() {
                    continue;
                }
                let snippet = self
                    .source_map
                    .get_line_snippet(self.source_map.line_starts[ctx_line - 1]);
                println!(
                    " {} | {}",
                    ctx_line.to_string().color(Color::Cyan).dimmed(),
                    snippet.dimmed()
                );
            }

            // hint if present
            if let Some(hint) = &error.hint {
                println!(" {} = {}: {}", padding, "hint".bold().cyan(), hint);
            }
        } else {
            println!(
                "{}: {}",
                severity_label.color(severity_color).bold(),
                error.message.color(Color::White)
            );
        }
    }

    fn print_ice_header(&self) {
        eprintln!(
            "\n{}",
            "=================================================================="
                .bold()
                .red()
        );
        eprintln!("{}", "  INTERNAL COMPILER ERROR ".bold().bright_red());
        eprintln!(
            "{}",
            "=================================================================="
                .bold()
                .red()
        );
        eprintln!("The compiler encountered an internal state bug and cannot continue.");
        eprintln!(
            "This is {} in the compiler itself, NOT an error in your code.",
            "a bug".bold().underline()
        );
    }

    fn print_ice_footer(&self) {
        eprintln!(
            "\n{}",
            "------------------------------------------------------------------".dimmed()
        );
        eprintln!("{}", "Please consider filing a bug report with:".bold());
        eprintln!("  1. Your source code file (`{}`)", self.filename.cyan());
        eprintln!("  2. The exact error trace shown above");
        eprintln!(
            "  3. Compiler version: {}",
            env!("CARGO_PKG_VERSION").yellow()
        );
        eprintln!("4. To: https://github.com/Black-Pine-Studio/Rectangular/issues");
        eprintln!(
            "{}",
            "==================================================================\n"
                .bold()
                .red()
        );
    }
}
