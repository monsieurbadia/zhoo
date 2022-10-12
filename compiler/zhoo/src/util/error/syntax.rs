use crate::util::error::ReportMessage;
use crate::util::span::Span;

pub enum SyntaxKind {
  UnrecognizedToken(String, Vec<String>, Span), // character, expected characters, span
}

pub fn write_syntax_report(_kind: &SyntaxKind) -> ReportMessage {
  todo!()
}
