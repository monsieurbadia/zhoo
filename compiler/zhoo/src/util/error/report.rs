use super::generate::{write_generate_report, GenerateKind};
use super::semantic::{write_semantic_report, SemanticKind};
use super::syntax::{write_syntax_report, SyntaxKind};

use crate::util::source::SourceMap;
use crate::util::span::Span;

use ariadne::ReportBuilder;

use std::cell::Cell;
use std::default::Default;
use std::path::{Path, PathBuf};
use std::{io, process};

type Labels = Vec<(Span, String, ariadne::Color)>;
type Notes = Vec<String>;
type Helps = Vec<String>;

pub type ReportMessage = (String, Labels, Notes, Helps);

static EXIT_FAILURE: i32 = 1;

pub enum Report {
  Io(io::Error),
  Syntax(SyntaxKind),
  Semantic(SemanticKind),
  Generate(GenerateKind),
}

impl Report {
  fn as_code(&self) -> i32 {
    match self {
      Self::Io(_) => 0, // this case will never be used
      Self::Syntax(_) => 1,
      Self::Semantic(_) => 2,
      Self::Generate(_) => 3,
    }
  }
}

#[derive(Debug)]
pub struct Reporter {
  has_errors: Cell<bool>,
  pub source_map: SourceMap,
}

impl Reporter {
  pub fn add_source<P: Into<PathBuf>>(&mut self, path: P) -> io::Result<u32> {
    self.source_map.add(path.into())
  }

  pub fn code(&self, source_id: u32) -> &str {
    self.source_map.code(source_id)
  }

  pub fn source(&self, span: Span) -> u32 {
    self.source_map.source(span)
  }

  pub fn path(&self, span: Span) -> &Path {
    self.source_map.path(span)
  }

  pub fn add_report(&self, report: Report) {
    let (message, labels, notes, helps) = match report {
      Report::Syntax(ref kind) => write_syntax_report(kind),
      Report::Semantic(ref kind) => write_semantic_report(kind),
      Report::Generate(ref kind) => write_generate_report(kind),
      Report::Io(error) => panic!("{error}"),
    };

    let span = labels.first().map(|label| label.0).unwrap_or(Span::ZERO);
    let source_id = self.source(span);
    let code = self.code(source_id);
    let code = if code.is_empty() { " " } else { code };
    let path = self.path(span);

    let mut report: ReportBuilder<(String, std::ops::Range<usize>)> =
      ariadne::Report::build(
        ariadne::ReportKind::Error,
        path.display().to_string(),
        span.lo as usize,
      )
      .with_code(report.as_code())
      .with_message(message);

    for (x, (span, message, color)) in labels.into_iter().enumerate() {
      report = report.with_label(
        ariadne::Label::new((path.display().to_string(), span.into()))
          .with_message(message)
          .with_order(x as i32)
          .with_color(color),
      )
    }

    for note in notes {
      report = report.with_note(note);
    }

    for help in helps {
      report = report.with_help(help);
    }

    eprintln!();
    report
      .with_config(ariadne::Config::default())
      .finish()
      .print((path.display().to_string(), ariadne::Source::from(code)))
      .unwrap();

    self.has_errors.set(true);
  }

  pub fn raise(&self, report: Report) -> ! {
    self.add_report(report);
    self.abort()
  }

  pub fn abort_if_has_error(&self) {
    if self.has_errors.get() {
      self.abort();
    }
  }

  pub fn abort(&self) -> ! {
    process::exit(EXIT_FAILURE)
  }
}

impl Default for Reporter {
  fn default() -> Self {
    Self {
      has_errors: Cell::new(false),
      source_map: SourceMap::default(),
    }
  }
}
