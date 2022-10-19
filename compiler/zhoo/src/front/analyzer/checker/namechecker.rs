use crate::front::analyzer::context::Context;

use crate::front::parser::tree::ast::{
  Arg, Block, Decl, Expr, ExprKind, Ext, Fun, Program, Prototype, Stmt,
  StmtKind, TyAlias,
};

use crate::front::parser::tree::PBox;
use crate::util::error::{Report, Reporter, Result, SemanticKind};
use crate::util::span::Span;
use crate::util::strcase;
use crate::util::strcase::StrCase;

#[inline]
pub fn check(program: &Program) -> Result<()> {
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
    StmtKind::Vals(decls) => check_stmt_decls(context, decls),
    StmtKind::Fun(fun) => check_stmt_fun(context, fun),
    _ => {}
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
    prototype.pattern.to_string(),
    prototype.pattern.span,
  );

  check_prototype_inputs(context, &prototype.inputs);
}

fn check_prototype_inputs(context: &Context, inputs: &Vec<PBox<Arg>>) {
  for input in inputs {
    verify_snake_case(
      &context.program.reporter,
      input.pattern.to_string(),
      input.pattern.span,
    );
  }
}

fn check_stmt_ty_alias(context: &Context, ty_alias: &TyAlias) {
  verify_pascal_case(
    &context.program.reporter,
    ty_alias.name.to_string(),
    ty_alias.span,
  );
}

fn check_block(context: &Context, block: &Block) {
  for expr in &block.exprs {
    check_expr(context, expr);
  }
}

fn check_stmt_decl(context: &Context, decl: &Decl) {
  verify_snake_screaming_case(
    &context.program.reporter,
    decl.pattern.to_string(),
    decl.pattern.span,
  );

  check_expr(context, &decl.value);
}

fn check_stmt_decls(context: &Context, decls: &Vec<PBox<Decl>>) {
  for decl in decls {
    verify_snake_screaming_case(
      &context.program.reporter,
      decl.pattern.to_string(),
      decl.pattern.span,
    );

    check_expr(context, &decl.value);
  }
}

fn check_stmt_fun(context: &Context, fun: &Fun) {
  check_prototype(context, &fun.prototype);
  check_block(context, &fun.body);
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
    decl.pattern.to_string(),
    decl.pattern.span,
  );

  check_expr(context, &decl.value);
}

fn verify_pascal_case(reporter: &Reporter, name: String, span: Span) {
  if !strcase::is_pascal_case(&name) {
    add_report_naming_convention(reporter, name, StrCase::Pascal, span);
  }
}

fn verify_snake_case(reporter: &Reporter, name: String, span: Span) {
  if !strcase::is_snake_case(&name) {
    add_report_naming_convention(reporter, name, StrCase::Snake, span)
  }
}

fn verify_snake_screaming_case(reporter: &Reporter, name: String, span: Span) {
  println!("{}", name);
  if !strcase::is_snake_screaming_case(&name) {
    add_report_naming_convention(reporter, name, StrCase::SnakeScreaming, span)
  }
}

fn add_report_naming_convention(
  reporter: &Reporter,
  name: String,
  naming: StrCase,
  span: Span,
) {
  let name = match naming {
    StrCase::Pascal => strcase::to_pascal_case(&name),
    StrCase::Snake => strcase::to_snake_case(&name),
    StrCase::SnakeScreaming => strcase::to_snake_case_screaming(&name),
  };

  reporter.add_report(Report::Semantic(SemanticKind::NamingConvention(
    name,
    naming.to_string(),
    span,
  )));
}
