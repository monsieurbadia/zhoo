mod mainchecker;
mod namechecker;

use crate::front::parser::tree::ast::Program;
use crate::util::error::Result;

pub fn analyze(program: &Program) -> Result<()> {
  mainchecker::check(program)?;
  namechecker::check(program)?;

  Ok(())
}
