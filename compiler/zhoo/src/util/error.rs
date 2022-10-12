mod generate;
mod report;
mod semantic;
mod syntax;

use super::color;

pub use generate::GenerateKind;
pub use report::{Report, ReportMessage, Reporter};
pub use semantic::SemanticKind;
pub use syntax::SyntaxKind;

pub type Result<T> = std::result::Result<T, Report>;
