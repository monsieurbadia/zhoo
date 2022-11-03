use crate::grammar::ProgramParser;

use zhoo_ast::ast::Program;
use zhoo_errors::Reporter;

use std::path::PathBuf;

pub fn parse<P: Into<PathBuf>>(pathname: P) -> Program {
  let mut reporter = Reporter::default();
  let source_id = reporter.add_source(pathname.into()).unwrap();
  let source_code = reporter.code(source_id);
  let parser = ProgramParser::new();

  match parser.parse(source_code) {
    Ok(node) => Program::new(node.0, node.1, reporter),
    Err(error) => panic!("{error}"),
  }
}
