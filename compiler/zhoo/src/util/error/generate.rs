use crate::util::color::Color;
use crate::util::error::ReportMessage;
use crate::util::span::Span;

pub enum GenerateKind {
  CallFunctionNotFound(Span, String),
  IdentifierNotFound(String),
  InvalidBinOp(Span, String, String),
  ArgumentsMismatch(Span),
}

pub fn write_generate_report(kind: &GenerateKind) -> ReportMessage {
  use ariadne::Fmt;

  match kind {
    GenerateKind::CallFunctionNotFound(_span, name) => (
      ariadne::ReportKind::Error,
      format!(
        "call {} not found",
        format_args!("`{}`", name.fg(Color::error())).fg(Color::error())
      ),
      vec![],
      vec![],
      vec![],
    ),
    GenerateKind::IdentifierNotFound(name) => (
      ariadne::ReportKind::Error,
      format!(
        "identifier {} not found",
        format_args!("`{}`", name.fg(Color::error())).fg(Color::error())
      ),
      vec![],
      vec![],
      vec![],
    ),
    GenerateKind::InvalidBinOp(span, lhs, rhs) => (
      ariadne::ReportKind::Error,
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
      ariadne::ReportKind::Error,
      format!("{}", "arguments mismatch".fg(Color::title())),
      vec![],
      vec![],
      vec![],
    ),
  }
}
