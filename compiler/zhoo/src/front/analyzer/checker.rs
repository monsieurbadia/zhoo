mod mainchecker;

use crate::front::parser::tree::ast::Program;
use crate::util::error::Result;

pub fn analyze(program: &Program) -> Result<()> {
  mainchecker::check(program)?;

  Ok(())
}
