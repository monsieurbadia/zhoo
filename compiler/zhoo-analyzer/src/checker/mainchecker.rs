use crate::context::Context;

use zhoo_ast::ast::{Program, Stmt, StmtKind};
use zhoo_ast::ptr::Fsp;
use zhoo_errors::{Report, Reporter, Result, SemanticKind};
use zhoo_helper::constant::ENTRY_POINT;
use zhoo_span::span::Span;

pub(crate) fn check(program: &Program) -> Result<()> {
  let context = Context::new(program);

  if !context
    .program
    .stmts
    .iter()
    .any(has_main(&context.program.reporter))
  {
    let entry_point = context.program.reporter.path(context.program.span);

    context.program.reporter.add_report(Report::Semantic(
      SemanticKind::MainNotFound(
        Span::new(
          context.program.span.lo as usize,
          context.program.span.hi as usize,
        ),
        entry_point.display().to_string(),
      ),
    ));
  }

  context.program.reporter.abort_if_has_error();

  Ok(())
}

fn has_main<'a>(
  reporter: &'a Reporter,
) -> Box<impl FnMut(&'a Fsp<Stmt>) -> bool + 'a> {
  Box::new(move |item: &'a Fsp<Stmt>| {
    if let StmtKind::Fun(fun) = &item.kind {
      if fun.prototype.name.to_string() == ENTRY_POINT {
        // todo: check main return type should be equal to `int`
        if !fun.prototype.inputs.is_empty() {
          let inputs = &fun.prototype.inputs;
          let single_span = fun.prototype.inputs[0].span;

          let merged_span = inputs
            .iter()
            .fold(single_span, |acc, value| Span::merge(&acc, &value.span));

          let inputs = fun
            .prototype
            .inputs
            .iter()
            .map(|input| input.ty.to_string())
            .collect::<Vec<_>>()
            .join(", ");

          reporter.add_report(Report::Semantic(SemanticKind::MainHasInputs(
            inputs,
            merged_span,
          )));
        }

        return true;
      }
    }

    false
  })
}
