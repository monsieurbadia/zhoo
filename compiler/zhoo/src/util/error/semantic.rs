use super::color::Color;

use crate::util::error::ReportMessage;
use crate::util::span::Span;

pub enum SemanticKind {
  MainNotFound(Span, String),
  MainHasInputs(String, Span),
  NamingConvention(String, String, Span),
}

pub fn write_semantic_report(kind: &SemanticKind) -> ReportMessage {
  use ariadne::Fmt;

  match kind {
    SemanticKind::MainNotFound(span, entry_point) => (
      format!(
        "{} {}",
        "`main`".fg(Color::hint()),
        "function not found".fg(Color::title()),
      ),
      vec![(
        *span,
        format!("to compile a program, i need a main function, add a `main` function to {entry_point}").fg(Color::error()).to_string(),
        Color::error(),
      )],
      vec![format!(
        "add the following code {} to your entry file",
        "`fun main() {}`".fg(Color::hint()),
      )],
    ),
    SemanticKind::MainHasInputs(inputs, span) => (
      format!(
        "{} {}",
        "`main`".fg(Color::hint()),
        "function defined with args".fg(Color::title()),
      ),
      vec![(
        *span,
        format!(
          "{}",
          "rule number 1, no arguments should be given to the main function",
        ).fg(Color::error()).to_string(),
        Color::error(),
      )],
      vec![format!(
        "expected `fun()` \n\t     actual `fun({})`",
        inputs.fg(Color::hint())
      )],
    ),
    SemanticKind::NamingConvention(identifier, naming, span) => (
      format!("{} {} {} {}", "variable".fg(Color::title()), format!("`{identifier}`").fg(Color::hint()),  "should have a".fg(Color::title()), format!("`{naming}`").fg(Color::title())),
      vec![(
        *span,
        format!("change this identifier to {naming} convention: `{identifier}`").fg(Color::error()).to_string(),
        Color::error(),
      )],
      vec![],
    )
  }
}
