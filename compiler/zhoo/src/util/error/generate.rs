use crate::util::color::Color;
use crate::util::error::ReportMessage;
use crate::util::span::Span;

pub enum GenerateKind {
  CallNotFound(Span, String),
  IdentifierNotFound(String),
  InvalidBinOp(Span, String, String),
  WrongInputCount(Span),
}

pub fn write_generate_report(kind: &GenerateKind) -> ReportMessage {
  use ariadne::Fmt;

  match kind {
    GenerateKind::CallNotFound(_span, name) => (
      format!(
        "call {} not found",
        format_args!("`{}`", name.fg(Color::error())).fg(Color::error())
      ),
      vec![],
      vec![],
      vec![],
    ),
    GenerateKind::IdentifierNotFound(name) => (
      format!(
        "identifier {} not found",
        format_args!("`{}`", name.fg(Color::error())).fg(Color::error())
      ),
      vec![],
      vec![],
      vec![],
    ),
    GenerateKind::InvalidBinOp(span, lhs, rhs) => (
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
    GenerateKind::WrongInputCount(_span) => (
      format!("{}", "missing input arguments".fg(Color::title())),
      vec![],
      vec![],
      vec![],
    ),
  }
}
