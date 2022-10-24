//! this module is used for the semantic analysis phase of the zhoo compiler

use super::report::ReportMessage;

use crate::util::span::Span;

/// the semantic kind enumeration
#[derive(Debug)]
pub enum SemanticKind {
  ArgumentsMismatch(Span, String, usize, usize, String),
  FunctionNotFound(Span, String),
  IdentifierNotFound(Span, String),
  InvalidIndex(Span, String),
  MainNotFound(Span, String),
  MainHasInputs(String, Span),
  NameClash(Span, String),
  NamingConvention(String, String, Span),
  OutOfLoop(Span, String),
  TypeMismatch(Span, String, String),
}

/// get the error messages
pub(crate) fn semantic_report(kind: &SemanticKind) -> ReportMessage {
  use super::report::{ReportKind, REPORT_ERROR, REPORT_WARNING};

  use crate::util::color::Color;
  use crate::util::strcase;

  use ariadne::Fmt;

  match kind {
    SemanticKind::ArgumentsMismatch(
      span,
      inputs,
      expected_len,
      actual_len,
      should_be
    ) => (
      ReportKind::Error(REPORT_ERROR),
      format!("{}", "arguments mismatch".fg(Color::title())),
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
          "🤖 this function takes {expected_len} {expected_argument} but {actual_len} {actual_argument} were supplied.",
          expected_len = expected_len.fg(Color::note()),
          actual_len = actual_len.fg(Color::note()),
          expected_argument = strcase::to_plural_or_singular(*expected_len, "argument").fg(Color::note()),
          actual_argument = strcase::to_plural_or_singular(*actual_len, "argument").fg(Color::note()),
        )
      ],
      vec![
        format!("👉 {}", format_args!("try this: {should_be}").fg(Color::help())),
      ],
    ),
    SemanticKind::FunctionNotFound(span, name) => (
      ReportKind::Error(REPORT_ERROR),
      format!("{}", format_args!("function {} not found", format_args!("`{name}`").fg(Color::hint())).fg(Color::error())),
      vec![(
        *span,
        "this call requires a function that does not exist in this scope.".to_string(),
        Color::error(),
      )],
      vec![format!("🤖 are you sure you have defined it correctly because i'm stumped")],
      vec![],
    ),
    SemanticKind::IdentifierNotFound(span, name) => (
      ReportKind::Error(REPORT_ERROR),
      format!("{}", format_args!("identifier {} not found", format_args!("`{name}`").fg(Color::hint())).fg(Color::error())),
      vec![(
        *span,
        "this identifier do no exist in this scope".to_string(),
        Color::error(),
      )],
      vec![format!("🤖 are you sure you have defined it correctly because i'm stumped")],
      vec![],
    ),
    SemanticKind::InvalidIndex(span, ty) => (
      ReportKind::Error(REPORT_ERROR),
      format!("{}", "invalid index".fg(Color::title())),
      vec![(
        *span,
        format!(
          "{}",
          format_args!("array indices are always of type `int`, got: `{ty}`").fg(Color::error()),
        ),
        Color::error(),
      )],
      vec![],
      vec![],
    ),
    SemanticKind::MainNotFound(span, entry_point) => (
      ReportKind::Error(REPORT_ERROR),
      format!(
        "{} {}",
        "`main`".fg(Color::hint()),
        "function not found".fg(Color::title()),
      ),
      vec![(
        *span,
        format!("to compile a program, i need a main function, add a `main` function to {entry_point}")
          .fg(Color::error()).to_string(),
        Color::error(),
      )],
      vec![format!(
        "🤖 add the following code {} to your entry file",
        "`fun main() {}`".fg(Color::note()),
      )],
      vec![],
    ),
    SemanticKind::MainHasInputs(inputs, span) => (
      ReportKind::Error(REPORT_ERROR),
      format!(
        "{} {}",
        "`main`".fg(Color::hint()),
        "function defined with arguments".fg(Color::title()),
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
        "🤖 expected `fun()` \n\t        actual `fun({})`",
        inputs.fg(Color::note())
      )],
      vec![],
    ),
    SemanticKind::NameClash(span, name) => (
      ReportKind::Error(REPORT_ERROR),
      format!("the name `{}` already exist", name.fg(Color::hint())),
      vec![(
        *span,
        format!(
          "{}",
          "this name is already declared in the scope".fg(Color::error())
        ),
        Color::error(),
      )],
      vec![
        format!("🤖 i'm not sure which one you want to use? rename one of them!"),
      ],
      vec![],
    ),
    SemanticKind::NamingConvention(identifier, naming, span) => (
      ReportKind::Warning(REPORT_WARNING),
      format!(
        "{} {} {} {}",
        "variable".fg(Color::title()),
        format!("`{identifier}`").fg(Color::hint()),
        "should have a".fg(Color::title()),
        format!("`{naming}`").fg(Color::title()),
      ),
      vec![(
        *span,
        format!("change this identifier to {naming} convention: `{identifier}`")
          .fg(Color::warning()).to_string(),
        Color::warning(),
      )],
      vec![],
      vec![],
    ),
    SemanticKind::OutOfLoop(span, behavior) => (
      ReportKind::Error(REPORT_ERROR),
      format!("{} {}", format_args!("`{}`", behavior.fg(Color::hint())), "outside of the loop".fg(Color::title())),
      vec![(
        *span,
        format!("cannot `{behavior}` out of the loop").fg(Color::error()).to_string(),
        Color::error(),
      )],
      vec![],
      vec![],
    ),
    SemanticKind::TypeMismatch(span, t1, t2) => (
      ReportKind::Error(REPORT_ERROR),
      format!("{}", "type mismatch".fg(Color::title())),
      vec![(
        *span,
        format!("expected `{t1}`, found `{t2}`").fg(Color::error()).to_string(),
        Color::error(),
      )],
      vec![],
      vec![],
    ),
  }
}
