use super::checker::mainchecker;
use super::checker::namechecker;
use super::checker::typechecker;

use zhoo_ast::ast::Program;
use zhoo_errors::Result;

pub fn analyze(program: &Program) -> Result<()> {
  mainchecker::check(program)?;
  namechecker::check(program)?;
  typechecker::check(program)?;

  Ok(())
}
