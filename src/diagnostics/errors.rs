use crate::diagnostics::Span;

///This communicates the severity of the error
pub enum Severity {
    Error,   //User errors
    Warning, //User warnings
    Ice,     //Internal Compiler Error
}

///This is the structure of the compiler error that all phases send to the diagnostics
pub struct CompilerError {
    pub message: String, //The error message
    pub span: Option<Span>,  //The span of the error it is optional in cases where u lack a span
    pub severity: Severity,
    pub hint: Option<String>, //The hint attached to this particular error it is optional incase
                              //there is no hint
}

impl CompilerError {
    ///This is the helper for passing user errors
    pub fn error(message: String, span: Option<Span>, hint: Option<String>) -> Self {
        CompilerError {
            message,
            severity: Severity::Error,
            span,
            hint,
        }
    }

    ///This is the helper for passing user warnings
    pub fn warning(message: String, span: Option<Span>, hint: Option<String>) -> CompilerError {
        CompilerError {
            message,
            severity: Severity::Warning,
            span,
            hint,
        }
    }

    ///This is the helper for passing ice errors
    pub fn ice(message: String, span: Option<Span>, hint: Option<String>) -> CompilerError {
        CompilerError {
            message,
            severity: Severity::Ice,
            span,
            hint,
        }
    }
}
