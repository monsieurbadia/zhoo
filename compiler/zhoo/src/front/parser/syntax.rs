use crate::front::grammar::ProgramParser;
use crate::front::parser::tree::ast::Program;
use crate::util::error::{Report, Reporter, SyntaxKind};
use crate::util::span::Span;

use lalrpop_util::ParseError;

use std::path::PathBuf;

const IDENTIFIER_REGEX: &str = "r#\"[_a-zA-Z][_a-zA-Z0-9]*\"#";
const IDENTIFIER_NAME: &str = "`identifier`";

#[inline]
pub fn parse<P: Into<PathBuf>>(pathname: P) -> Program {
  use std::fmt::Write;

  let mut reporter = Reporter::default();
  let source_id = reporter.add_source(pathname.into()).unwrap();
  let source_code = reporter.code(source_id);
  let parser = ProgramParser::new();

  match parser.parse(source_code) {
    Ok(node) => Program::new(node.0, reporter, node.1),
    Err(error) => match error {
      ParseError::InvalidToken { location } => {
        reporter.raise(Report::Syntax(SyntaxKind::InvalidToken(Span::new(
          location, location,
        ))));
      }
      ParseError::UnrecognizedEOF {
        location,
        expected: _,
      } => {
        reporter.raise(Report::Syntax(SyntaxKind::UnrecognizedEOF(
          Span::new(location, location),
          String::new(),
        )));
      }
      ParseError::UnrecognizedToken { token, expected } => {
        let span = Span::new(token.0, token.2);

        let mut buf = String::new();

        let expected = expected
          .iter()
          .enumerate()
          .map(|(x, exp)| {
            buf.clear();

            if x == expected.len() - 1 && expected.len() > 1 {
              write!(buf, "or ").ok();
            }

            if exp == IDENTIFIER_REGEX {
              write!(buf, "{IDENTIFIER_NAME}").ok();
              return buf.to_string();
            }

            write!(buf, "{}", exp.replace('\"', "`")).ok();
            buf.to_string()
          })
          .collect::<Vec<_>>()
          .join(", ");

        reporter.raise(Report::Syntax(SyntaxKind::UnrecognizedToken(
          span, expected,
        )));
      }
      ParseError::ExtraToken { token } => {
        reporter.raise(Report::Syntax(SyntaxKind::ExtraToken(
          Span::new(token.0, token.2),
          token.1.to_string(),
        )));
      }
      ParseError::User { error } => {
        reporter.raise(Report::Syntax(SyntaxKind::User(error.to_string())));
      }
    },
  }
}
