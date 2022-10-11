use crate::front::analyzer::context::Context;
use crate::front::parser::tree::ast::{Program, Stmt, StmtKind};
use crate::front::parser::tree::PBox;
use crate::util::constant::PROGRAM_ENTRY;

// TODO #1
//
// handle nice and clear error. in my opinion, `ariadne` is an
// excellent candidat.

pub fn check(program: &Program) {
  let context = Context::new(program);

  if !context.program.stmts.iter().any(has_main(&context)) {
    panic!("to compile a program, i need a main function"); // TODO #1
  }
}

fn has_main<'a>(
  _context: &'a Context,
) -> Box<impl FnMut(&'a PBox<Stmt>) -> bool + 'a> {
  Box::new(move |item: &'a PBox<Stmt>| {
    if let StmtKind::Fun(fun) = &item.kind {
      if fun.prototype.pattern.to_string() == PROGRAM_ENTRY {
        if !fun.prototype.inputs.is_empty() {
          // TODO #1
          panic!(
            "rule number 1, no arguments should be given to the main function"
          );
        }

        return true;
      }
    }

    false
  })
}
