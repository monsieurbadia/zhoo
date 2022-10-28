mod generate;
mod report;
mod semantic;
mod syntax;

pub use generate::GenerateKind;
pub use report::{Report, Reporter};
pub use semantic::SemanticKind;
pub use syntax::SyntaxKind;

/// the result type of the `zhoo` compiler
pub type Result<T> = std::result::Result<T, Report>;
