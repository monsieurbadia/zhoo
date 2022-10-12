use super::color::Color;

use crate::util::error::ReportMessage;
use crate::util::span::Span;
use crate::util::strcase;

pub enum SemanticKind {
  MainNotFound(Span, String),
  MainHasInputs(String, Span),
  NameClash(Span, String),
  NamingConvention(String, String, Span),
  OutOfLoop(Span, String),
  TypeMismatch(Span, String, String),
  WrongInputCount(Span, String, usize, usize, String),
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
          "rule number 1, no arguments should be given to the main function".fg(Color::error()),
        ),
        Color::error(),
      )],
      vec![format!(
        "expected `fun()` \n\t     actual `fun({})`",
        inputs.fg(Color::hint())
      )],
    ),
    SemanticKind::NameClash(span, name) => (
      format!("variable `{}` already exist", name.fg(Color::hint())),
      vec![(
        *span,
        format!("{}", "this name is already declared in the scope".fg(Color::error())),
        Color::error(),
      )],
      vec![],
    ),
    SemanticKind::NamingConvention(identifier, naming, span) => (
      format!("{} {} {} {}", "variable".fg(Color::title()), format!("`{identifier}`").fg(Color::hint()),  "should have a".fg(Color::title()), format!("`{naming}`").fg(Color::title())),
      vec![(
        *span,
        format!("change this identifier to {naming} convention: `{identifier}`").fg(Color::error()).to_string(),
        Color::error(),
      )],
      vec![],
    ),
    SemanticKind::OutOfLoop(span, behavior) => (
      format!("{} {}", format_args!("`{}`", behavior.fg(Color::hint())), "outside of the loop".fg(Color::title())),
      vec![(
        *span,
        format!("cannot `{behavior}` out of the loop").fg(Color::error()).to_string(),
        Color::error(),
      )],
      vec![],
    ),
    SemanticKind::TypeMismatch(span, t1, t2) => (
      format!("{}", "type mismatch".fg(Color::title())),
      vec![(
        *span,
        format!("expected `{t1}`, found `{t2}`").fg(Color::error()).to_string(),
        Color::error(),
      )],
      vec![],
    ),
    SemanticKind::WrongInputCount(span, inputs, expected_len, actual_len, should_be) => (
      format!("{}", "missing input arguments".fg(Color::title())),
      vec![(
        *span,
        format!(
          "the input {argument} of type ({inputs}) are required",
          argument = strcase::to_plural_or_singular(*expected_len, "argument"),
        ).fg(Color::error()).to_string(),
        Color::error(),
      )],
      vec![
        format!(
          "this function takes {expected_len} {expected_argument} but {actual_len} {actual_argument} were supplied. try this: {should_be}",
          expected_argument = strcase::to_plural_or_singular(*expected_len, "argument"),
          actual_argument = strcase::to_plural_or_singular(*actual_len, "argument"),
        )
      ],
    )
  }
}
