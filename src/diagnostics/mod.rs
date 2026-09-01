mod diagnostics;
mod errors;
mod source_map;
mod span;

pub use diagnostics::{Diag, Diagnostics};
pub use errors::CompilerError;
pub use span::Span;
