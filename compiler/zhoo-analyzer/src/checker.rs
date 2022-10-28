mod mainchecker;
mod namechecker;
mod typechecker;

use zhoo_parser::tree::ast::Program;
use zhoo_util::error::Result;

#[inline]
pub fn analyze(program: &Program) -> Result<()> {
  mainchecker::check(program)?;
  namechecker::check(program)?;
  typechecker::check(program)?;

  Ok(())
}
