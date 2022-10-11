mod mainchecker;

use crate::front::parser::tree::ast::Program;

pub fn analyze(program: &Program) -> Result<(), String> {
  mainchecker::check(program);

  Ok(())
}
