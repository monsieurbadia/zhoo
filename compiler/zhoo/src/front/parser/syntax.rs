use crate::front::grammar::ProgramParser;
use crate::front::parser::tree::ast::Program;

pub fn parse(source: &str) -> Program {
  let parser = ProgramParser::new();

  match parser.parse(source) {
    Ok(stmts) => Program::new(stmts),
    Err(error) => panic!("{:?}", error),
  }
}
