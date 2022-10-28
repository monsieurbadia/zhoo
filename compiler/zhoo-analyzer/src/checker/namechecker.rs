use crate::context::Context;

use zhoo_parser::tree::ast::{
  Apply, ApplyKind, Arg, Behavior, Block, Decl, Enum, Expr, ExprKind, Ext, Fun,
  ImplElement, MacroCall, MacroDecl, Program, Prototype, Stmt, StmtKind,
  Struct, TyAlias, Unit,
};

use zhoo_parser::tree::PBox;
use zhoo_util::error::{Report, Reporter, Result, SemanticKind};
use zhoo_util::span::Span;
use zhoo_util::strcase;
use zhoo_util::strcase::StrCase;

#[inline]
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
    StmtKind::MacroDecl(macro_decl) => {
      check_stmt_macro_decl(context, macro_decl)
    }
    StmtKind::MacroCall(macro_call) => {
      check_stmt_macro_call(context, macro_call)
    }
    StmtKind::TyAlias(ty_alias) => check_stmt_ty_alias(context, ty_alias),
    StmtKind::Behavior(behavior) => check_stmt_behavior(context, behavior),
    StmtKind::Enum(enum_def) => check_stmt_enum_def(context, enum_def),
    StmtKind::Struct(struct_def) => check_stmt_struct_def(context, struct_def),
    StmtKind::Apply(apply) => check_stmt_apply(context, apply),
    StmtKind::Val(decl) => check_stmt_decl(context, decl),
    StmtKind::Vals(decls) => check_stmt_decls(context, decls),
    StmtKind::Fun(fun) => check_stmt_fun(context, fun),
    StmtKind::Unit(unit) => check_stmt_unit(context, unit),
  }
}

fn check_stmt_ext(context: &Context, ext: &Ext) {
  check_prototype(context, &ext.prototype);

  let Some(body) = &ext.body else { return; };

  check_block(context, body);
}

fn check_stmt_macro_decl(context: &Context, macro_decl: &MacroDecl) {
  verify_snake_case(
    &context.program.reporter,
    macro_decl.name.span,
    macro_decl.name.to_string(),
  );
}

fn check_stmt_macro_call(context: &Context, macro_call: &MacroCall) {
  verify_snake_case(
    &context.program.reporter,
    macro_call.pattern.span,
    macro_call.pattern.to_string(),
  );
}

fn check_stmt_behavior(context: &Context, behavior: &Behavior) {
  verify_pascal_case(
    &context.program.reporter,
    behavior.identifier.span,
    behavior.identifier.to_string(),
  );
}

fn check_stmt_enum_def(context: &Context, enum_def: &Enum) {
  verify_pascal_case(
    &context.program.reporter,
    enum_def.name.span,
    enum_def.name.to_string(),
  );
}

fn check_stmt_struct_def(context: &Context, struct_def: &Struct) {
  verify_pascal_case(
    &context.program.reporter,
    struct_def.name.span,
    struct_def.name.to_string(),
  );
}

fn check_stmt_apply(context: &Context, apply: &Apply) {
  match &apply.kind {
    ApplyKind::Behavior(maybe_behavior) => {
      let Some(behavior) = maybe_behavior else { return; };

      verify_pascal_case(
        &context.program.reporter,
        behavior.identifier.span,
        behavior.identifier.to_string(),
      );
    }
    ApplyKind::Ty(_ty) => {}
  }

  for impl_element in &apply.elements {
    check_impl_element(context, impl_element)
  }
}

fn check_impl_element(context: &Context, impl_element: &ImplElement) {
  match impl_element {
    ImplElement::Fun(fun) => check_stmt_fun(context, fun),
    ImplElement::TyAlias(ty_alias) => check_stmt_ty_alias(context, ty_alias),
  }
}

fn check_prototype(context: &Context, prototype: &Prototype) {
  verify_snake_case(
    &context.program.reporter,
    prototype.pattern.span,
    prototype.pattern.to_string(),
  );

  check_prototype_inputs(context, &prototype.inputs);
}

fn check_prototype_inputs(context: &Context, inputs: &Vec<PBox<Arg>>) {
  for input in inputs {
    verify_snake_case(
      &context.program.reporter,
      input.pattern.span,
      input.pattern.to_string(),
    );
  }
}

fn check_stmt_ty_alias(context: &Context, ty_alias: &TyAlias) {
  verify_pascal_case(
    &context.program.reporter,
    ty_alias.span,
    ty_alias.name.to_string(),
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
    decl.pattern.span,
    decl.pattern.to_string(),
  );

  check_expr(context, &decl.value);
}

fn check_stmt_decls(context: &Context, decls: &Vec<PBox<Decl>>) {
  for decl in decls {
    verify_snake_screaming_case(
      &context.program.reporter,
      decl.pattern.span,
      decl.pattern.to_string(),
    );

    check_expr(context, &decl.value);
  }
}

fn check_stmt_fun(context: &Context, fun: &Fun) {
  check_prototype(context, &fun.prototype);
  check_block(context, &fun.body);
}

fn check_stmt_unit(context: &Context, unit: &Unit) {
  for mock in &unit.mocks {
    check_stmt_fun(context, mock);
  }

  for test in &unit.tests {
    check_stmt_fun(context, test);
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
  println!("{}", name);
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
