use crate::context::Context;

use zhoo_ast::ast::{
  AsTy, BinOp, BinOpKind, Block, Decl, Expr, ExprKind, Fun, Lit, LitKind,
  PatternKind, Program, Prototype, Stmt, StmtKind, Ty, TyKind, UnOp, UnOpKind,
};

use zhoo_ast::ptr::Fsp;
use zhoo_errors::{Report, Result, SemanticKind};
use zhoo_span::span::Span;

pub(crate) fn check(program: &Program) -> Result<()> {
  let mut context = Context::new(program);

  for stmt in &context.program.stmts {
    match check_stmt(&mut context, stmt) {
      Ok(_ty) => {}
      Err(report) => context.program.reporter.add_report(report),
    };
  }

  context.program.reporter.abort_if_has_error();

  Ok(())
}

fn check_stmt(context: &mut Context, stmt: &Stmt) -> Result<Fsp<Ty>> {
  match &stmt.kind {
    StmtKind::Val(decl) => check_stmt_decl(context, decl),
    StmtKind::Fun(fun) => check_stmt_fun(context, fun),
    _ => unimplemented!(),
  }
}

fn check_stmt_decl(context: &mut Context, decl: &Decl) -> Result<Fsp<Ty>> {
  let ty = if let Some(ty) = &decl.ty {
    ty.clone()
  } else {
    Ty::INFER.into()
  };

  let name = match &decl.pattern.kind {
    PatternKind::Identifier(identifier) => identifier,
    _ => panic!("should be an identifier"),
  };

  let Ok(_ty) = context.scope_map.set_decl(name.to_string(), ty) else {
    return Err(Report::Semantic(SemanticKind::NameClash(name.span, name.to_string())));
  };

  let t1 = check_expr(context, name)?;
  let t2 = check_expr(context, &decl.value)?;

  unify_tys(context, &t1, &t2)?;
  Ok(make_ty_void(decl.span).into())
}

fn check_stmt_fun(context: &mut Context, fun: &Fun) -> Result<Fsp<Ty>> {
  match context.scope_map.set_fun(
    fun.prototype.name.to_string(),
    (fun.prototype.as_inputs_tys(), fun.prototype.as_ty()),
  ) {
    Ok(_fun) => {
      context.scope_map.enter_scope();
      check_prototype(context, &fun.prototype)?;
      check_block(context, &fun.body)?;
      context.scope_map.exit_scope();
      Ok(make_ty_void(fun.span).into())
    }
    Err(_error) => Err(Report::Semantic(SemanticKind::NameClash(
      fun.prototype.name.span,
      fun.prototype.name.to_string(),
    ))),
  }
}

fn check_prototype(context: &mut Context, prototype: &Prototype) -> Result<()> {
  for input in &prototype.inputs {
    if context
      .scope_map
      .set_decl(input.pattern.to_string(), input.ty.to_owned())
      .is_err()
    {
      return Err(Report::Semantic(SemanticKind::NameClash(
        input.span,
        input.to_string(),
      )));
    }
  }

  context.return_ty = prototype.as_ty();

  Ok(())
}

fn check_block(context: &mut Context, block: &Block) -> Result<()> {
  for expr in &block.exprs {
    check_expr(context, expr)?;
  }

  Ok(())
}

fn check_expr(context: &mut Context, expr: &Expr) -> Result<Fsp<Ty>> {
  match &expr.kind {
    ExprKind::Lit(lit) => check_expr_lit(lit),
    ExprKind::Identifier(identifier) => {
      check_expr_identifier(context, expr.span, identifier)
    }
    ExprKind::Call(callee, args) => check_expr_call(context, callee, args),
    ExprKind::UnOp(op, rhs) => check_expr_un_op(context, op, rhs),
    ExprKind::BinOp(lhs, op, rhs) => check_expr_bin_op(context, lhs, op, rhs),
    ExprKind::Decl(decl) => check_expr_decl(context, decl),
    ExprKind::Assign(lhs, op, rhs) => check_expr_assign(context, lhs, op, rhs),
    ExprKind::AssignOp(lhs, op, rhs) => {
      check_expr_assign_op(context, lhs, op, rhs)
    }
    ExprKind::Block(body) => check_expr_block(context, body),
    ExprKind::Loop(body) => check_expr_loop(context, body),
    ExprKind::While(condition, body) => {
      check_expr_while(context, condition, body)
    }
    ExprKind::Until(condition, body) => {
      check_expr_until(context, condition, body)
    }
    ExprKind::Return(maybe_expr) => {
      check_expr_return(context, maybe_expr, expr.span)
    }
    ExprKind::Break(maybe_expr) => check_expr_break(context, maybe_expr, expr),
    ExprKind::Continue => check_expr_continue(context, expr),
    ExprKind::When(condition, consequence, alternative) => {
      check_expr_when(context, condition, consequence, alternative)
    }
    ExprKind::IfElse(condition, consequence, maybe_alternative) => {
      check_expr_if_else(context, condition, consequence, maybe_alternative)
    }
    ExprKind::Array(elements) => check_expr_array(context, expr.span, elements),
    ExprKind::ArrayAccess(indexed, index) => {
      check_expr_array_access(context, expr.span, indexed, index)
    }
    ExprKind::Stmt(stmt) => check_expr_stmt(context, stmt),
    _ => unimplemented!(),
  }
}

fn check_expr_lit(lit: &Lit) -> Result<Fsp<Ty>> {
  match &lit.kind {
    LitKind::Bool(_boolean) => check_expr_lit_bool(lit.span),
    LitKind::Int(_int) => check_expr_lit_int(lit.span),
    LitKind::Real(_real) => check_expr_lit_real(lit.span),
    LitKind::Str(_string) => check_expr_lit_str(lit.span),
  }
}

fn check_expr_lit_bool(span: Span) -> Result<Fsp<Ty>> {
  Ok(make_ty_bool(span).into())
}

fn check_expr_lit_int(span: Span) -> Result<Fsp<Ty>> {
  Ok(make_ty_int(span).into())
}

fn check_expr_lit_real(span: Span) -> Result<Fsp<Ty>> {
  Ok(make_ty_real(span).into())
}

fn check_expr_lit_str(span: Span) -> Result<Fsp<Ty>> {
  Ok(make_ty_str(span).into())
}

fn check_expr_identifier(
  context: &mut Context,
  span: Span,
  identifier: &str,
) -> Result<Fsp<Ty>> {
  if let Some(ty) = context.scope_map.decl(identifier) {
    Ok(ty.clone())
  } else if let Some(ty) = context.scope_map.fun(identifier) {
    Ok(ty.1.clone())
  } else {
    Err(Report::Semantic(SemanticKind::IdentifierNotFound(
      span,
      identifier.to_string(),
    )))
  }
}

fn check_expr_call(
  context: &mut Context,
  callee: &Expr,
  inputs: &[Fsp<Expr>],
) -> Result<Fsp<Ty>> {
  let (fun_inputs_tys, fun_return_ty) =
    match context.scope_map.fun(&callee.to_string()) {
      Some(fun_ty) => fun_ty,
      None => {
        return Err(Report::Semantic(SemanticKind::FunctionNotFound(
          callee.span,
          callee.to_string(),
        )));
      }
    };

  if inputs.len() != fun_inputs_tys.len() {
    let expected_inputs = fun_inputs_tys
      .iter()
      .map(|input| format!("`{}`", input))
      .collect::<Vec<_>>()
      .join(", ");

    let should_be = format!("{}({})", callee, expected_inputs);

    return Err(Report::Semantic(SemanticKind::ArgumentsMismatch(
      callee.span,
      expected_inputs,
      fun_inputs_tys.len(),
      inputs.len(),
      should_be,
    )));
  }

  for (x, input) in inputs.iter().enumerate() {
    if x < fun_inputs_tys.len() {
      ensure_expr_ty(&mut context.clone(), input, &fun_inputs_tys[x])?;
    }
  }

  ensure_expr_ty(&mut context.clone(), callee, fun_return_ty)?;

  Ok(fun_return_ty.clone())
}

fn check_expr_un_op(
  context: &mut Context,
  op: &UnOp,
  rhs: &Expr,
) -> Result<Fsp<Ty>> {
  let t1 = check_expr(context, rhs)?;

  match &op.node {
    UnOpKind::Neg => {
      if !t1.is_numeric() {
        return Err(Report::Semantic(SemanticKind::TypeMismatch(
          Span::merge(&t1.span, &op.span),
          Ty::INT.to_string(),
          t1.to_string(),
        )));
      }

      Ok(make_ty_int(Span::merge(&op.span, &rhs.span)).into())
    }
    UnOpKind::Not => {
      if !t1.is_boolean() {
        return Err(Report::Semantic(SemanticKind::TypeMismatch(
          Span::merge(&t1.span, &op.span),
          Ty::BOOL.to_string(),
          t1.to_string(),
        )));
      }

      Ok(make_ty_bool(Span::merge(&op.span, &rhs.span)).into())
    }
  }
}

// todo: ugly stuff, this will be improve later
fn check_expr_bin_op(
  context: &mut Context,
  lhs: &Expr,
  op: &BinOp,
  rhs: &Expr,
) -> Result<Fsp<Ty>> {
  let t1 = check_expr(context, lhs)?;
  let t2 = check_expr(context, rhs)?;

  match &op.node {
    BinOpKind::Lt | BinOpKind::Le | BinOpKind::Gt | BinOpKind::Ge => {
      if !t1.is_int() || !t2.is_int() {
        return Err(Report::Semantic(SemanticKind::TypeMismatch(
          op.span,
          t1.to_string(),
          t2.to_string(),
        )));
      }

      Ok(make_ty_bool(Span::merge(&lhs.span, &rhs.span)).into())
    }
    BinOpKind::And | BinOpKind::Or => {
      if t1.kind != t2.kind {
        return Err(Report::Semantic(SemanticKind::TypeMismatch(
          op.span,
          t1.to_string(),
          t2.to_string(),
        )));
      }

      Ok(make_ty_bool(Span::merge(&lhs.span, &rhs.span)).into())
    }
    BinOpKind::Eq | BinOpKind::Ne => {
      if t1.kind != t2.kind {
        return Err(Report::Semantic(SemanticKind::TypeMismatch(
          op.span,
          t1.to_string(),
          t2.to_string(),
        )));
      }

      Ok(make_ty_bool(Span::merge(&lhs.span, &rhs.span)).into())
    }
    _ => {
      if t1.kind != t2.kind {
        return Err(Report::Semantic(SemanticKind::TypeMismatch(
          op.span,
          t1.to_string(),
          t2.to_string(),
        )));
      }

      Ok(make_ty_int(Span::merge(&lhs.span, &rhs.span)).into())
    }
  }
}

fn check_expr_assign(
  context: &mut Context,
  lhs: &Expr,
  _op: &BinOp,
  rhs: &Expr,
) -> Result<Fsp<Ty>> {
  let t1 = check_expr(context, lhs)?;

  ensure_expr_ty(context, rhs, &t1)?;
  Ok(make_ty_void(Span::merge(&lhs.span, &rhs.span)).into())
}

fn check_expr_assign_op(
  context: &mut Context,
  lhs: &Expr,
  op: &BinOp,
  rhs: &Expr,
) -> Result<Fsp<Ty>> {
  let t1 = check_expr(context, lhs)?;
  let t2 = check_expr(context, rhs)?;

  if !op.node.is_assign_op() {
    return Err(Report::Semantic(SemanticKind::TypeMismatch(
      op.span,
      t1.to_string(),
      t2.to_string(),
    )));
  }

  expect_equality(context, &t1, &t2);
  Ok(make_ty_void(Span::merge(&lhs.span, &rhs.span)).into())
}

fn check_expr_return(
  context: &mut Context,
  maybe_expr: &Option<Fsp<Expr>>,
  return_span: Span,
) -> Result<Fsp<Ty>> {
  if let Some(expr) = maybe_expr {
    let t1 = check_expr(context, expr)?;

    expect_equality(context, &t1, &context.return_ty.clone());

    return Ok(t1);
  };

  Ok(make_ty_void(return_span).into())
}

fn check_expr_block(context: &mut Context, body: &Block) -> Result<Fsp<Ty>> {
  let mut t1 = make_ty_void(body.span).into();

  for expr in &body.exprs {
    t1 = check_expr(context, expr)?;
  }

  Ok(t1)
}

fn check_expr_loop(context: &mut Context, body: &Block) -> Result<Fsp<Ty>> {
  context.loop_depth += 1;
  check_block(context, body)?;
  context.loop_depth -= 1;

  Ok(make_ty_void(body.span).into())
}

fn check_expr_while(
  context: &mut Context,
  condition: &Expr,
  body: &Block,
) -> Result<Fsp<Ty>> {
  check_expr_while_or_until(context, condition, body)
}

fn check_expr_until(
  context: &mut Context,
  condition: &Expr,
  body: &Block,
) -> Result<Fsp<Ty>> {
  check_expr_while_or_until(context, condition, body)
}

fn check_expr_while_or_until(
  context: &mut Context,
  condition: &Expr,
  body: &Block,
) -> Result<Fsp<Ty>> {
  ensure_expr_ty(context, condition, &make_ty_bool(condition.span))?;
  context.loop_depth += 1;
  check_block(context, body)?;
  context.loop_depth -= 1;

  Ok(make_ty_void(body.span).into())
}

fn check_expr_break(
  context: &mut Context,
  maybe_expr: &Option<Fsp<Expr>>,
  origin: &Expr,
) -> Result<Fsp<Ty>> {
  if context.loop_depth == 0 {
    return Err(Report::Semantic(SemanticKind::OutOfLoop(
      origin.span,
      origin.to_string(),
    )));
  }

  if let Some(expr) = maybe_expr {
    let t1 = check_expr(context, expr)?;

    expect_equality(context, &t1, &context.return_ty.clone());

    return Ok(t1);
  }

  Ok(make_ty_void(origin.span).into())
}

fn check_expr_continue(
  context: &mut Context,
  origin: &Expr,
) -> Result<Fsp<Ty>> {
  if context.loop_depth == 0 {
    return Err(Report::Semantic(SemanticKind::OutOfLoop(
      origin.span,
      origin.to_string(),
    )));
  }

  Ok(make_ty_void(origin.span).into())
}

fn check_expr_when(
  context: &mut Context,
  condition: &Expr,
  consequence: &Expr,
  alternative: &Expr,
) -> Result<Fsp<Ty>> {
  let t1 = check_expr(context, condition)?;
  let t2 = check_expr(context, consequence)?;
  let t3 = check_expr(context, alternative)?;
  let boolean = make_ty_bool(condition.span);

  expect_equality(context, &t1, &boolean);
  unify_tys(context, &t2, &t3)
}

fn check_expr_if_else(
  context: &mut Context,
  condition: &Expr,
  consequence: &Expr,
  maybe_alternative: &Option<Fsp<Expr>>,
) -> Result<Fsp<Ty>> {
  let t1 = check_expr(context, condition)?;
  let t2 = check_expr(context, consequence)?;
  let Some(alternative) = maybe_alternative else { return Ok(t2); };
  let t3 = check_expr(context, alternative)?;

  if !t1.is_boolean() {
    let boolean = make_ty_bool(condition.span);

    return Err(Report::Semantic(SemanticKind::TypeMismatch(
      Span::merge(&t1.span, &boolean.span),
      boolean.to_string(),
      t1.to_string(),
    )));
  }

  expect_equality(context, &t2, &t3);
  Ok(t2)
}

fn check_expr_array(
  context: &mut Context,
  span: Span,
  elements: &[Fsp<Expr>],
) -> Result<Fsp<Ty>> {
  let mut element_tys = elements
    .iter()
    .map(|element| check_expr(context, element).unwrap())
    .collect::<Vec<Fsp<Ty>>>();

  let first_ty = if let Some(last_ty) = element_tys.pop() {
    last_ty
  } else {
    return Ok(make_ty_array(Ty::INFER.into(), None, span).into());
  };

  for ty in element_tys {
    expect_equality(context, &first_ty, &ty);
  }

  Ok(make_ty_array(first_ty, Some(elements.len() as i64), span).into())
}

fn check_expr_array_access(
  context: &mut Context,
  _span: Span,
  indexed: &Expr,
  index: &Expr,
) -> Result<Fsp<Ty>> {
  let _indexed = check_expr(context, indexed)?;
  let index = check_expr(context, index)?;

  if index.kind != Ty::INT.kind {
    return Err(Report::Semantic(SemanticKind::InvalidIndex(
      index.span,
      index.kind.to_string(),
    )));
  }

  Ok(index)
}

fn check_expr_stmt(context: &mut Context, stmt: &Stmt) -> Result<Fsp<Ty>> {
  check_stmt(context, stmt)?;
  Ok(make_ty_void(stmt.span).into())
}

fn check_expr_decl(context: &mut Context, decl: &Decl) -> Result<Fsp<Ty>> {
  let ty = if let Some(ty) = &decl.ty {
    ty.clone()
  } else {
    Ty::INFER.into()
  };

  let name = match &decl.pattern.kind {
    PatternKind::Identifier(identifier) => identifier,
    _ => panic!("should be an identifier"),
  };

  let variable_shadowed = context.scope_map.remove_decl(&name.to_string());
  let _ = context.scope_map.set_decl(name.to_string(), ty);

  let t1 = check_expr(context, name)?;
  let t2 = check_expr(context, &decl.value)?;

  if let Some(variable) = variable_shadowed {
    let _ = context.scope_map.set_decl(name.to_string(), variable);
  }

  unify_tys(context, &t1, &t2)?;
  Ok(make_ty_void(decl.span).into())
}

fn ensure_expr_ty(context: &mut Context, expr: &Expr, t1: &Ty) -> Result<bool> {
  let t2 = check_expr(context, expr)?;

  Ok(expect_equality(context, t1, &t2))
}

fn expect_equality(context: &mut Context, t1: &Ty, t2: &Ty) -> bool {
  if t1.kind != t2.kind {
    context.program.reporter.add_report(Report::Semantic(
      SemanticKind::TypeMismatch(
        Span::merge(&t1.span, &t2.span),
        t1.to_string(),
        t2.to_string(),
      ),
    ));

    false
  } else {
    true
  }
}

fn unify_tys(_context: &mut Context, t1: &Ty, t2: &Ty) -> Result<Fsp<Ty>> {
  if t1.kind != t2.kind {
    return Err(Report::Semantic(SemanticKind::TypeMismatch(
      t2.span,
      t1.to_string(),
      t2.to_string(),
    )));
  }

  Ok(t1.into())
}

#[inline]
const fn make_ty_void(span: Span) -> Ty {
  Ty::new(TyKind::Void, span)
}

#[inline]
const fn make_ty_bool(span: Span) -> Ty {
  Ty::new(TyKind::Bool, span)
}

#[inline]
const fn make_ty_int(span: Span) -> Ty {
  Ty::new(TyKind::Int, span)
}

#[inline]
const fn make_ty_real(span: Span) -> Ty {
  Ty::new(TyKind::Real, span)
}

#[inline]
const fn make_ty_str(span: Span) -> Ty {
  Ty::new(TyKind::Str, span)
}

#[inline]
const fn make_ty_array(ty: Fsp<Ty>, size: Option<i64>, span: Span) -> Ty {
  Ty::new(TyKind::Array(ty, size), span)
}
