//! this module is used to display report error messages

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

/// the exit code in failure case
const EXIT_FAILURE: i32 = 1;

/// the new line character
const NEW_LINE: &str = "\n";

/// the report error name
pub(crate) const REPORT_ERROR: &str = "error";

/// the report warning name
pub(crate) const REPORT_WARNING: &str = "warning";

type Kind = ReportKind;
type Labels = Vec<(Span, String, ariadne::Color)>;
type Notes = Vec<String>;
type Helps = Vec<String>;

pub type ReportMessage = (Kind, String, Labels, Notes, Helps);

/// a report enumeration
#[derive(Debug)]
pub enum Report {
  /// a report variant for `io`
  Io(io::Error),

  /// a report variant for the `syntax analysis`
  Syntax(SyntaxKind),

  /// a report variant for the `semantic analysis`
  Semantic(SemanticKind),

  /// a report variant for the `code generation`
  Generate(GenerateKind),
}

impl fmt::Display for Report {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", format_args!("{:03}", self.as_code()))
  }
}

impl Report {
  /// a free conversion of an report to an code number
  fn as_code(&self) -> i32 {
    match self {
      Self::Io(_) => 0, // this case will never be used because in io case we just panic
      Self::Syntax(_) => 1,
      Self::Semantic(_) => 2,
      Self::Generate(_) => 3,
    }
  }
}

/// a report kind enumeration
pub enum ReportKind {
  /// a main title variant for an error
  Error(&'static str),

  /// a main title variant for a warning
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

/// an instance of a reporter
#[derive(Debug)]
pub struct Reporter {
  has_errors: Cell<bool>,
  source_map: SourceMap,
}

impl Reporter {
  /// add a source from a path
  pub(crate) fn add_source<P: Into<PathBuf>>(
    &mut self,
    path: P,
  ) -> io::Result<u32> {
    self.source_map.add(path.into())
  }

  /// get the source code from the source id
  pub(crate) fn code(&self, source_id: u32) -> &str {
    self.source_map.code(source_id)
  }

  /// get the source id from the span
  fn source_id(&self, span: Span) -> u32 {
    self.source_map.source_id(span)
  }

  /// get the path from the span
  pub(crate) fn path(&self, span: Span) -> &Path {
    self.source_map.path(span)
  }

  /// add a report and display it in the std error
  pub fn add_report(&self, report: Report) {
    let (kind, message, labels, notes, helps) = match report {
      Report::Syntax(ref kind) => syntax_report(kind),
      Report::Semantic(ref kind) => semantic_report(kind),
      Report::Generate(ref kind) => generate_report(kind),
      Report::Io(error) => panic!("{error}"),
    };

    let span = labels.first().map(|label| label.0).unwrap_or(Span::ZERO);
    let source_id = self.source_id(span);
    let code = self.code(source_id);
    let code = if code.is_empty() { NEW_LINE } else { code };
    let path = self.path(span).display();

    let mut report =
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

  /// add a report and then abort the program
  pub fn raise(&self, report: Report) -> ! {
    self.add_report(report);
    self.abort()
  }

  /// abort a program if it has errors
  pub fn abort_if_has_error(&self) {
    if self.has_errors.get() {
      self.abort();
    }
  }

  /// abort the program
  fn abort(&self) -> ! {
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
