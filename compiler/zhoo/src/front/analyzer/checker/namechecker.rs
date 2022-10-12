use crate::front::analyzer::context::Context;

use crate::front::parser::tree::ast::{
  Arg, Block, Decl, Expr, ExprKind, Ext, Program, Prototype, Stmt, StmtKind,
};

use crate::front::parser::tree::PBox;
use crate::util::error::{Report, Reporter, Result, SemanticKind};
use crate::util::span::Span;

use inflector::cases::pascalcase::{is_pascal_case, to_pascal_case};

use inflector::cases::screamingsnakecase::{
  is_screaming_snake_case, to_screaming_snake_case,
};

use inflector::cases::snakecase::{is_snake_case, to_snake_case};

use std::fmt;

// TODO: check more cases.

enum NamingConvention {
  Pascal,
  Snake,
  SnakeScreaming,
}

impl fmt::Display for NamingConvention {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Self::Pascal => write!(f, "pascal case"),
      Self::Snake => write!(f, "snake case"),
      Self::SnakeScreaming => write!(f, "screaming snake case"),
    }
  }
}

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
    StmtKind::Val(decl) => check_stmt_decl(context, decl),
    _ => {}
  }
}

fn check_stmt_ext(context: &Context, ext: &Ext) {
  check_prototype(context, &ext.prototype);

  let Some(body) = &ext.body else { return; };

  check_block(context, body);
}

fn check_prototype(context: &Context, prototype: &Prototype) {
  verify_pascal_case(
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

fn check_block(context: &Context, block: &Block) {
  for expr in &block.exprs {
    check_expr(context, expr);
  }
}

fn check_stmt_decl(context: &Context, decl: &Decl) {
  check_decl(context, decl)
}

fn check_decl(context: &Context, decl: &Decl) {
  verify_snake_case_screaming(
    &context.program.reporter,
    decl.pattern.to_string(),
    decl.pattern.span,
  );

  check_expr(context, &decl.value);
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

fn verify_pascal_case(reporter: &Reporter, name: String, span: Span) {
  if !is_pascal_case(&name) {
    add_report_naming_convention(
      reporter,
      name,
      NamingConvention::Pascal,
      span,
    );
  }
}

fn verify_snake_case(reporter: &Reporter, name: String, span: Span) {
  if !is_snake_case(&name) {
    add_report_naming_convention(reporter, name, NamingConvention::Snake, span)
  }
}

fn verify_snake_case_screaming(reporter: &Reporter, name: String, span: Span) {
  if !is_screaming_snake_case(&name) {
    add_report_naming_convention(
      reporter,
      name,
      NamingConvention::SnakeScreaming,
      span,
    )
  }
}

fn add_report_naming_convention(
  reporter: &Reporter,
  name: String,
  naming: NamingConvention,
  span: Span,
) {
  let name = match naming {
    NamingConvention::Pascal => to_pascal_case(&name),
    NamingConvention::Snake => to_snake_case(&name),
    NamingConvention::SnakeScreaming => to_screaming_snake_case(&name),
  };

  reporter.add_report(Report::Semantic(SemanticKind::NamingConvention(
    name,
    naming.to_string(),
    span,
  )));
}
