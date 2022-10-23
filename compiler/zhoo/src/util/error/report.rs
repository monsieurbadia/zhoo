use super::generate::{generate_report, GenerateKind};
use super::semantic::{semantic_report, SemanticKind};
use super::syntax::{syntax_report, SyntaxKind};

use crate::util::color::Color;
use crate::util::source::SourceMap;
use crate::util::span::Span;

use std::cell::Cell;
use std::default::Default;
use std::path::{Path, PathBuf};
use std::{fmt, io, process};

const EXIT_FAILURE: i32 = 1;
const NEW_LINE: &str = "\n";

pub const REPORT_ERROR: &str = "error";
pub const REPORT_WARNING: &str = "warning";

type Kind = ReportKind;
type Labels = Vec<(Span, String, ariadne::Color)>;
type Notes = Vec<String>;
type Helps = Vec<String>;

pub type ReportMessage = (Kind, String, Labels, Notes, Helps);

#[derive(Debug)]
pub enum Report {
  Io(io::Error),
  Syntax(SyntaxKind),
  Semantic(SemanticKind),
  Generate(GenerateKind),
}

impl fmt::Display for Report {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", format_args!("{:03}", self.as_code()))
  }
}

impl Report {
  fn as_code(&self) -> i32 {
    match self {
      // this case will never be used because in io case we just panic
      Self::Io(_) => 0,
      Self::Syntax(_) => 1,
      Self::Semantic(_) => 2,
      Self::Generate(_) => 3,
    }
  }
}

pub enum ReportKind {
  Error(&'static str),
  Warning(&'static str),
}

impl From<ReportKind> for ariadne::ReportKind {
  fn from(kind: ReportKind) -> Self {
    match kind {
      ReportKind::Error(title) => {
        ariadne::ReportKind::Custom(title, Color::error())
      }
      ReportKind::Warning(title) => {
        ariadne::ReportKind::Custom(title, Color::warning())
      }
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
    let (kind, message, labels, notes, helps) = match report {
      Report::Syntax(ref kind) => syntax_report(kind),
      Report::Semantic(ref kind) => semantic_report(kind),
      Report::Generate(ref kind) => generate_report(kind),
      Report::Io(error) => panic!("{error}"),
    };

    let span = labels.first().map(|label| label.0).unwrap_or(Span::ZERO);
    let source_id = self.source(span);
    let code = self.code(source_id);
    let code = if code.is_empty() { NEW_LINE } else { code };
    let path = self.path(span).display();

    let mut report: ariadne::ReportBuilder<(String, std::ops::Range<usize>)> =
      ariadne::Report::build(kind.into(), path.to_string(), span.lo as usize)
        .with_code(report.to_string())
        .with_message(message);

    for (x, (span, message, color)) in labels.into_iter().enumerate() {
      report = report.with_label(
        ariadne::Label::new((path.to_string(), span.into()))
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
      .write(
        (path.to_string(), ariadne::Source::from(code)),
        std::io::stderr(),
      )
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
