use crate::front::analyzer::context::Context;

use crate::front::parser::tree::ast::{
  BinOp, BinOpKind, Block, Decl, Expr, ExprKind, Fun, Lit, LitKind,
  PatternKind, Program, Prototype, Stmt, StmtKind, UnOp, UnOpKind,
};

use crate::front::parser::tree::ty::{AsTy, Ty};
use crate::front::parser::tree::PBox;
use crate::util::error::{Report, Result, SemanticKind};
use crate::util::span::Span;

// fixme #1
//
// too many raise, i should rather use the result type to return an error.
// errors will be stored in a vector

pub fn check(program: &Program) -> Result<()> {
  let mut context = Context::new(program);

  for stmt in &context.program.stmts {
    check_stmt(&mut context, stmt);
  }

  Ok(())
}

fn check_stmt(context: &mut Context, stmt: &Stmt) {
  match &stmt.kind {
    StmtKind::Val(decl) => check_stmt_decl(context, decl),
    StmtKind::Fun(fun) => check_stmt_fun(context, fun),
    _ => {}
  }
}

fn check_stmt_decl(context: &mut Context, decl: &Decl) {
  check_decl(context, decl);
}

fn check_stmt_fun(context: &mut Context, fun: &Fun) {
  match context.scope_map.set_fun(
    fun.prototype.pattern.to_string(),
    (fun.prototype.as_inputs_tys(), fun.prototype.as_ty()),
  ) {
    Ok(_) => {
      context.scope_map.enter_scope();
      check_prototype(context, &fun.prototype);
      check_block(context, &fun.body);
      context.scope_map.exit_scope();
    }
    Err(_error) => todo!(),
  }
}

fn check_prototype(context: &mut Context, prototype: &Prototype) {
  for input in &prototype.inputs {
    if context
      .scope_map
      .set_decl(input.pattern.to_string(), input.ty.to_owned())
      .is_err()
    {
      context.program.reporter.add_report(Report::Semantic(
        SemanticKind::NameClash(input.span, input.to_string()),
      ))
    }
  }

  context.return_ty = prototype.as_ty();
}

fn check_block(context: &mut Context, block: &Block) {
  for expr in &block.exprs {
    check_expr(context, expr);
  }
}

fn check_expr(context: &mut Context, expr: &Expr) -> PBox<Ty> {
  match &expr.kind {
    ExprKind::Stmt(stmt) => check_expr_stmt(context, stmt),
    ExprKind::Decl(decl) => check_expr_decl(context, decl),
    ExprKind::Lit(lit) => check_expr_lit(lit),
    ExprKind::Identifier(identifier) => {
      check_expr_identifier(context, identifier)
    }
    ExprKind::Call(callee, args) => check_expr_call(context, callee, args),
    ExprKind::UnOp(op, rhs) => check_expr_un_op(context, op, rhs),
    ExprKind::BinOp(lhs, op, rhs) => check_expr_bin_op(context, lhs, op, rhs),
    ExprKind::Assign(lhs, op, rhs) => check_expr_assign(context, lhs, op, rhs),
    ExprKind::AssignOp(lhs, op, rhs) => {
      check_expr_assign_op(context, lhs, op, rhs)
    }
    ExprKind::Return(maybe_expr) => {
      check_expr_return(context, maybe_expr, expr.span)
    }
    ExprKind::Block(body) => check_expr_block(context, body),
    ExprKind::Loop(body) => check_expr_loop(context, body),
    ExprKind::While(condition, body) => {
      check_expr_while(context, condition, body)
    }
    ExprKind::Break(maybe_expr) => check_expr_break(context, maybe_expr, expr),
    ExprKind::Continue => check_expr_continue(context, expr),
    ExprKind::When(condition, consequence, alternative) => {
      check_expr_when(context, condition, consequence, alternative)
    }
    ExprKind::IfElse(condition, consequence, maybe_alternative) => {
      check_expr_if_else(context, condition, consequence, maybe_alternative)
    }
    ExprKind::Lambda(args, block_or_expr) => {
      check_expr_lambda(context, args, block_or_expr)
    }
    _ => panic!("tmp error for `check:expr`"), // fixme #1
  }
}

fn check_expr_stmt(context: &mut Context, stmt: &Stmt) -> PBox<Ty> {
  check_stmt(context, stmt);
  Ty::with_void(stmt.span).into()
}

fn check_expr_decl(context: &mut Context, decl: &Decl) -> PBox<Ty> {
  check_decl(context, decl)
}

fn check_decl(context: &mut Context, decl: &Decl) -> PBox<Ty> {
  let name = &decl.pattern;

  let ty = if let Some(ty) = &decl.ty {
    ty.clone()
  } else {
    Ty::INFER.into()
  };

  let Ok(_) = context.scope_map.set_decl(name.to_string(), ty) else {
    // fixme #1
    context.program.reporter.raise(
      Report::Semantic(SemanticKind::NameClash(name.span, name.to_string())),
    );
  };

  let name = match &name.kind {
    PatternKind::Identifier(identifier) => identifier,
    _ => panic!("not good at all"), // fixme #1
  };

  let t1 = check_expr(context, name);
  let t2 = check_expr(context, &decl.value);

  unify_tys(context, &t1, &t2);
  Ty::with_void(decl.span).into()
}

fn check_expr_lit(lit: &Lit) -> PBox<Ty> {
  match &lit.kind {
    LitKind::Bool(_) => check_expr_lit_bool(lit.span),
    LitKind::Int(_) => check_expr_lit_int(lit.span),
    LitKind::Real(_) => check_expr_lit_real(lit.span),
    LitKind::Str(_) => check_expr_lit_str(lit.span),
  }
}

fn check_expr_lit_bool(span: Span) -> PBox<Ty> {
  Ty::with_bool(span).into()
}

fn check_expr_lit_int(span: Span) -> PBox<Ty> {
  Ty::with_int(span).into()
}

fn check_expr_lit_real(span: Span) -> PBox<Ty> {
  Ty::with_real(span).into()
}

fn check_expr_lit_str(span: Span) -> PBox<Ty> {
  Ty::with_str(span).into()
}

fn check_expr_identifier(context: &mut Context, identifier: &str) -> PBox<Ty> {
  if let Some(ty) = context.scope_map.decl(identifier) {
    ty.clone()
  } else if let Some(ty) = context.scope_map.fun(identifier) {
    ty.1.clone()
  } else {
    panic!("tmp error for `check:expr`"); // fixme #1
  }
}

fn check_expr_call(
  context: &mut Context,
  callee: &Expr,
  inputs: &Vec<PBox<Expr>>,
) -> PBox<Ty> {
  let (fun_input_tys, fun_return_ty) =
    match context.scope_map.fun(&callee.to_string()) {
      Some(fun_ty) => fun_ty,
      None => panic!("calling not defined function"), // fixme #1
    };

  if inputs.len() != fun_input_tys.len() {
    let expected_inputs = fun_input_tys
      .iter()
      .map(|input| format!("`{}`", input))
      .collect::<Vec<_>>()
      .join(", ");

    let should_be = format!("{}({})", callee, expected_inputs);

    context.program.reporter.add_report(Report::Semantic(
      SemanticKind::WrongInputCount(
        callee.span,
        expected_inputs,
        fun_input_tys.len(),
        inputs.len(),
        should_be,
      ),
    ))
  }

  for (x, input) in inputs.iter().enumerate() {
    if x < fun_input_tys.len() {
      ensure_expr_ty(&mut context.clone(), input, &fun_input_tys[x]);
    }
  }

  ensure_expr_ty(&mut context.clone(), callee, fun_return_ty);
  fun_return_ty.clone()
}

fn check_expr_un_op(context: &mut Context, op: &UnOp, rhs: &Expr) -> PBox<Ty> {
  let t1 = check_expr(context, rhs);

  match &op.node {
    UnOpKind::Neg => {
      if !t1.is_numeric() {
        context.program.reporter.add_report(Report::Semantic(
          SemanticKind::TypeMismatch(
            Span::merge(&t1.span, &op.span),
            Ty::INT.to_string(),
            t1.to_string(),
          ),
        ));
      }

      Ty::with_int(Span::merge(&op.span, &rhs.span)).into()
    }
    UnOpKind::Not => {
      if !t1.is_boolean() {
        context.program.reporter.add_report(Report::Semantic(
          SemanticKind::TypeMismatch(
            Span::merge(&t1.span, &op.span),
            Ty::BOOL.to_string(),
            t1.to_string(),
          ),
        ));
      }

      Ty::with_bool(Span::merge(&op.span, &rhs.span)).into()
    }
  }
}

// todo: ugly stuff, this will be improve later
fn check_expr_bin_op(
  context: &mut Context,
  lhs: &Expr,
  op: &BinOp,
  rhs: &Expr,
) -> PBox<Ty> {
  let t1 = check_expr(context, lhs);
  let t2 = check_expr(context, rhs);

  match &op.node {
    BinOpKind::Lt | BinOpKind::Le | BinOpKind::Gt | BinOpKind::Ge => {
      if !t1.kind.is_int() || !t2.kind.is_int() {
        // fixme #1
        context.program.reporter.raise(Report::Semantic(
          SemanticKind::TypeMismatch(op.span, t1.to_string(), t2.to_string()),
        ));
      }

      Ty::with_bool(Span::merge(&lhs.span, &rhs.span)).into()
    }
    BinOpKind::And | BinOpKind::Or => {
      if t1.kind != t2.kind {
        // fixme #1
        context.program.reporter.raise(Report::Semantic(
          SemanticKind::TypeMismatch(op.span, t1.to_string(), t2.to_string()),
        ));
      }

      Ty::with_bool(Span::merge(&lhs.span, &rhs.span)).into()
    }
    BinOpKind::Eq | BinOpKind::Ne => {
      if t1.kind != t2.kind {
        // fixme #1
        context.program.reporter.raise(Report::Semantic(
          SemanticKind::TypeMismatch(op.span, t1.to_string(), t2.to_string()),
        ));
      }

      Ty::with_bool(Span::merge(&lhs.span, &rhs.span)).into()
    }
    _ => {
      if t1.kind != t2.kind {
        // fixme #1
        context.program.reporter.raise(Report::Semantic(
          SemanticKind::TypeMismatch(op.span, t1.to_string(), t2.to_string()),
        ));
      }

      Ty::with_int(Span::merge(&lhs.span, &rhs.span)).into()
    }
  }
}

fn check_expr_assign(
  context: &mut Context,
  lhs: &Expr,
  _: &BinOp,
  rhs: &Expr,
) -> PBox<Ty> {
  let t1 = check_expr(context, lhs);

  ensure_expr_ty(context, rhs, &t1);
  Ty::with_void(Span::merge(&lhs.span, &rhs.span)).into()
}

fn check_expr_assign_op(
  context: &mut Context,
  lhs: &Expr,
  op: &BinOp,
  rhs: &Expr,
) -> PBox<Ty> {
  let t1 = check_expr(context, lhs);
  let t2 = check_expr(context, rhs);

  if !op.node.is_assign_op() {
    // fixme #1
    context.program.reporter.raise(Report::Semantic(
      SemanticKind::TypeMismatch(op.span, t1.to_string(), t2.to_string()),
    ));
  }

  expect_equality(context, &t1, &t2);
  Ty::with_void(Span::merge(&lhs.span, &rhs.span)).into()
}

fn check_expr_return(
  context: &mut Context,
  maybe_expr: &Option<PBox<Expr>>,
  return_span: Span,
) -> PBox<Ty> {
  if let Some(expr) = maybe_expr {
    let t1 = check_expr(context, expr);

    expect_equality(context, &t1, &context.return_ty.clone());

    return t1;
  };

  Ty::with_void(return_span).into()
}

fn check_expr_block(context: &mut Context, body: &Block) -> PBox<Ty> {
  let mut t1 = Ty::with_void(body.span).into();

  for expr in &body.exprs {
    t1 = check_expr(context, expr);
  }

  t1
}

fn check_expr_loop(context: &mut Context, body: &Block) -> PBox<Ty> {
  context.loops += 1;
  check_block(context, body);
  context.loops -= 1;

  Ty::with_void(body.span).into()
}

fn check_expr_while(
  context: &mut Context,
  condition: &Expr,
  body: &Block,
) -> PBox<Ty> {
  ensure_expr_ty(context, condition, &Ty::with_bool(condition.span));
  context.loops += 1;
  check_block(context, body);
  context.loops -= 1;

  Ty::with_void(body.span).into()
}

fn check_expr_break(
  context: &mut Context,
  maybe_expr: &Option<PBox<Expr>>,
  origin: &Expr,
) -> PBox<Ty> {
  if context.loops == 0 {
    context.program.reporter.add_report(Report::Semantic(
      SemanticKind::OutOfLoop(origin.span, origin.to_string()),
    ));
  }

  if let Some(expr) = maybe_expr {
    let t1 = check_expr(context, expr);

    expect_equality(context, &t1, &context.return_ty.clone());

    return t1;
  }

  Ty::with_void(origin.span).into()
}

fn check_expr_continue(context: &mut Context, origin: &Expr) -> PBox<Ty> {
  if context.loops == 0 {
    context.program.reporter.add_report(Report::Semantic(
      SemanticKind::OutOfLoop(origin.span, origin.to_string()),
    ));
  }

  Ty::with_void(origin.span).into()
}

fn check_expr_when(
  context: &mut Context,
  condition: &Expr,
  consequence: &Expr,
  alternative: &Expr,
) -> PBox<Ty> {
  let t1 = check_expr(context, condition);
  let t2 = check_expr(context, consequence);
  let t3 = check_expr(context, alternative);
  let boolean = Ty::with_bool(condition.span);

  expect_equality(context, &t1, &boolean);
  unify_tys(context, &t2, &t3)
}

fn check_expr_if_else(
  context: &mut Context,
  condition: &Expr,
  consequence: &Expr,
  maybe_alternative: &Option<PBox<Expr>>,
) -> PBox<Ty> {
  let t1 = check_expr(context, condition);
  let t2 = check_expr(context, consequence);
  let Some(alternative) = maybe_alternative else { return t2; };
  let t3 = check_expr(context, alternative);

  if !t1.is_boolean() {
    let boolean = Ty::with_bool(condition.span);

    // fixme #1
    context.program.reporter.raise(Report::Semantic(
      SemanticKind::TypeMismatch(
        Span::merge(&t1.span, &boolean.span),
        boolean.to_string(),
        t1.to_string(),
      ),
    ));
  }

  expect_equality(context, &t2, &t3);

  t2
}

fn check_expr_lambda(
  _context: &mut Context,
  _args: &Vec<PBox<Expr>>,
  _block_or_expr: &Expr,
) -> PBox<Ty> {
  todo!()
}

fn ensure_expr_ty(context: &mut Context, expr: &Expr, t1: &Ty) -> bool {
  let t2 = check_expr(context, expr);

  expect_equality(context, t1, &t2)
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

fn unify_tys(context: &mut Context, t1: &Ty, t2: &Ty) -> PBox<Ty> {
  if t1.kind != t2.kind {
    // fixme #1
    context.program.reporter.raise(Report::Semantic(
      SemanticKind::TypeMismatch(
        Span::merge(&t1.span, &t2.span),
        t1.to_string(),
        t2.to_string(),
      ),
    ));
  }

  t1.into()
}
