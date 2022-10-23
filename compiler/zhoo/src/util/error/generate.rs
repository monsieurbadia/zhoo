use super::report::ReportMessage;

use crate::util::span::Span;

// todo (?): #1
//
// normally, there should be no similar errors in the
// type checking phase, is right?

#[derive(Debug)]
pub enum GenerateKind {
  CallFunctionNotFound(Span, String),
  IdentifierNotFound(String),
  InvalidBinOp(Span, String, String),
  ArgumentsMismatch(Span),
}

pub fn generate_report(kind: &GenerateKind) -> ReportMessage {
  use super::report::{ReportKind, REPORT_ERROR};

  use crate::util::color::Color;

  use ariadne::Fmt;

  match kind {
    GenerateKind::CallFunctionNotFound(_span, name) => (
      ReportKind::Error(REPORT_ERROR),
      format!(
        "call {} not found",
        format_args!("`{}`", name.fg(Color::error())).fg(Color::error())
      ),
      vec![],
      vec![],
      vec![],
    ),
    GenerateKind::IdentifierNotFound(name) => (
      ReportKind::Error(REPORT_ERROR),
      format!(
        "identifier {} not found",
        format_args!("`{}`", name.fg(Color::error())).fg(Color::error())
      ),
      vec![],
      vec![],
      vec![],
    ),
    GenerateKind::InvalidBinOp(span, lhs, rhs) => (
      ReportKind::Error(REPORT_ERROR),
      format!("{}", "binary operation not valid".fg(Color::title())),
      vec![(
        *span,
        format!(
          "{}",
          format_args!(
            "this binary operation is avoid: {lhs}(left) {rhs}(right)"
          )
          .fg(Color::error())
        ),
        Color::error(),
      )],
      vec![],
      vec![],
    ),
    GenerateKind::ArgumentsMismatch(_span) => (
      ReportKind::Error(REPORT_ERROR),
      format!("{}", "arguments mismatch".fg(Color::title())),
      vec![],
      vec![],
      vec![],
    ),
  }
}
