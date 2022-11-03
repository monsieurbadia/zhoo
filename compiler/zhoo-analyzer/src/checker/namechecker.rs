use crate::context::Context;

use zhoo_ast::ast::{
  Arg, Block, Decl, Expr, ExprKind, Ext, Fun, Program, Prototype, Stmt,
  StmtKind, TyAlias, Unit,
};

use zhoo_ast::ptr::Fsp;
use zhoo_errors::{Report, Reporter, Result, SemanticKind};
use zhoo_helper::strcase;
use zhoo_helper::strcase::StrCase;
use zhoo_span::span::Span;

pub(crate) fn check(program: &Program) -> Result<()> {
  let context = Context::new(program);

  for stmt in &context.program.stmts {
    check_stmt(&context, stmt);
  }

  Ok(())
}

fn check_stmt(context: &Context, stmt: &Stmt) {
  match &stmt.kind {
    StmtKind::Ext(ext) => check_stmt_ext(context, ext),
    StmtKind::TyAlias(ty_alias) => check_stmt_ty_alias(context, ty_alias),
    StmtKind::Val(decl) => check_stmt_decl(context, decl),
    StmtKind::Fun(fun) => check_stmt_fun(context, fun),
    StmtKind::Unit(unit) => check_stmt_unit(context, unit),
  }
}

fn check_stmt_ext(context: &Context, ext: &Ext) {
  check_prototype(context, &ext.prototype);

  let Some(body) = &ext.body else { return; };

  check_block(context, body);
}

fn check_prototype(context: &Context, prototype: &Prototype) {
  verify_snake_case(
    &context.program.reporter,
    prototype.name.span,
    prototype.name.to_string(),
  );

  check_prototype_inputs(context, &prototype.inputs);
}

fn check_prototype_inputs(context: &Context, inputs: &Vec<Fsp<Arg>>) {
  for input in inputs {
    verify_snake_case(
      &context.program.reporter,
      input.pattern.span,
      input.pattern.to_string(),
    );
  }
}

fn check_block(context: &Context, block: &Block) {
  for expr in &block.exprs {
    check_expr(context, expr);
  }
}

fn check_stmt_ty_alias(context: &Context, ty_alias: &TyAlias) {
  verify_pascal_case(
    &context.program.reporter,
    ty_alias.span,
    ty_alias.name.to_string(),
  );
}

fn check_stmt_decl(context: &Context, decl: &Decl) {
  verify_snake_screaming_case(
    &context.program.reporter,
    decl.pattern.span,
    decl.pattern.to_string(),
  );

  check_expr(context, &decl.value);
}

fn check_stmt_fun(context: &Context, fun: &Fun) {
  check_prototype(context, &fun.prototype);
  check_block(context, &fun.body);
}

fn check_stmt_unit(context: &Context, unit: &Unit) {
  for stmt in &unit.binds {
    check_stmt(context, stmt);
  }

  for fun in &unit.mocks {
    check_stmt_fun(context, fun);
  }

  for fun in &unit.tests {
    check_stmt_fun(context, fun);
  }
}

fn check_expr(context: &Context, expr: &Expr) {
  match &expr.kind {
    ExprKind::Stmt(stmt) => check_expr_stmt(context, stmt),
    ExprKind::Decl(decl) => check_expr_decl(context, decl),
    _ => {}
  }
}

fn check_expr_stmt(context: &Context, stmt: &Stmt) {
  check_stmt(context, stmt);
}

fn check_expr_decl(context: &Context, decl: &Decl) {
  check_decl(context, decl);
}

fn check_decl(context: &Context, decl: &Decl) {
  verify_snake_case(
    &context.program.reporter,
    decl.pattern.span,
    decl.pattern.to_string(),
  );

  check_expr(context, &decl.value);
}

fn verify_pascal_case(reporter: &Reporter, span: Span, name: String) {
  if !strcase::is_pascal_case(&name) {
    add_report_naming_convention(reporter, name, span, StrCase::Pascal);
  }
}

fn verify_snake_case(reporter: &Reporter, span: Span, name: String) {
  if !strcase::is_snake_case(&name) {
    add_report_naming_convention(reporter, name, span, StrCase::Snake)
  }
}

fn verify_snake_screaming_case(reporter: &Reporter, span: Span, name: String) {
  if !strcase::is_snake_screaming_case(&name) {
    add_report_naming_convention(reporter, name, span, StrCase::SnakeScreaming)
  }
}

fn add_report_naming_convention(
  reporter: &Reporter,
  name: String,
  span: Span,
  naming: StrCase,
) {
  let name = match naming {
    StrCase::Pascal => strcase::to_pascal_case(&name),
    StrCase::Snake => strcase::to_snake_case(&name),
    StrCase::SnakeScreaming => strcase::to_snake_screaming_case(&name),
  };

  reporter.add_report(Report::Semantic(SemanticKind::NamingConvention(
    name,
    naming.to_string(),
    span,
  )));
}
