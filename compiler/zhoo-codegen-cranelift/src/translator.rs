use super::interface::{CompiledFunction, DataBuilder, VariableBuilder};

use zhoo_ast::ast::{
  BinOp, BinOpKind, Block, Decl, Expr, ExprKind, Lit, LitKind, Stmt, StmtKind,
  UnOp, UnOpKind,
};

use zhoo_ast::ptr::Fsp;

use cranelift::prelude::{
  types, Block as CBlock, FloatCC, FunctionBuilder, InstBuilder, IntCC, Value,
  Variable,
};

use cranelift_codegen::ir::immediates::Offset32;
use cranelift_codegen::ir::{GlobalValue, StackSlot};
use cranelift_module::Module;
use cranelift_object::ObjectModule;
use fxhash::FxHashMap;

pub(crate) struct Translator<'a> {
  pub builder: FunctionBuilder<'a>,
  pub module: &'a mut ObjectModule,
  pub funs: &'a mut FxHashMap<String, CompiledFunction>,
  pub globals: &'a mut FxHashMap<String, GlobalValue>,
  pub vars: &'a mut FxHashMap<String, Variable>,
  pub ty: types::Type,
  pub blocks: &'a mut Vec<CBlock>,
  pub variable_builder: &'a mut VariableBuilder,
  pub data_builder: &'a mut DataBuilder,
}

impl<'a> Translator<'a> {
  pub fn translate(&mut self, block: &Block) -> Result<Value, String> {
    let mut value = self.translate_expr_lit_int(&0);

    for expr in &block.exprs {
      value = self.translate_expr(expr);
    }

    Ok(value)
  }

  fn translate_stmt(&mut self, stmt: &Stmt) -> Value {
    match &stmt.kind {
      StmtKind::Val(decl) => self.translate_stmt_val(decl),
      _ => unimplemented!(),
    }
  }

  fn translate_stmt_val(&mut self, decl: &Decl) -> Value {
    self.translate_decl(decl)
  }

  fn translate_decl(&mut self, decl: &Decl) -> Value {
    let value = self.translate_expr(&decl.value);

    let variable = self.variable_builder.create_variable(
      &mut self.builder,
      value,
      types::I64,
    );

    self.vars.insert(decl.pattern.to_string(), variable);

    value
  }

  fn translate_expr(&mut self, expr: &Expr) -> Value {
    match &expr.kind {
      ExprKind::Lit(lit) => self.translate_expr_lit(lit),
      ExprKind::Identifier(s) => self.translate_expr_id(s),
      ExprKind::Call(callee, args) => self.translate_expr_call(callee, args),
      ExprKind::UnOp(op, rhs) => self.translate_expr_un_op(op, rhs),
      ExprKind::BinOp(lhs, op, rhs) => self.translate_expr_bin_op(lhs, op, rhs),
      ExprKind::Decl(decl) => self.translate_expr_decl(decl),
      ExprKind::Assign(id, op, rhs) => self.translate_expr_assign(id, op, rhs),
      ExprKind::AssignOp(lhs, op, rhs) => {
        self.translate_expr_assign_op(lhs, op, rhs)
      }
      ExprKind::Loop(body) => self.translate_expr_loop(body),
      ExprKind::While(condition, body) => {
        self.translate_expr_while(condition, body)
      }
      ExprKind::Until(_condition, _body) => todo!(),
      ExprKind::Return(value) => self.translate_expr_return(value),
      ExprKind::Break(value) => self.translate_expr_break(value),
      ExprKind::Continue => self.translate_expr_continue(),
      ExprKind::Block(block) => self.translate_expr_block(block),
      ExprKind::When(condition, consequence, alternative) => {
        self.translate_expr_when(condition, consequence, alternative)
      }
      ExprKind::IfElse(condition, consequence, maybe_alternative) => {
        self.translate_expr_if_else(condition, consequence, maybe_alternative)
      }
      ExprKind::Lambda(args, block_or_expr) => {
        self.translate_expr_lambda(args, block_or_expr)
      }
      ExprKind::Array(elements) => self.translate_expr_array(elements),
      ExprKind::ArrayAccess(indexed, index) => {
        self.translate_expr_array_access(indexed, index)
      }
      ExprKind::Stmt(stmt) => self.translate_expr_stmt(stmt),
      _ => unimplemented!(),
    }
  }

  fn translate_expr_lit(&mut self, lit: &Lit) -> Value {
    match &lit.kind {
      LitKind::Bool(boolean) => self.translate_expr_lit_bool(boolean),
      LitKind::Int(int) => self.translate_expr_lit_int(int),
      LitKind::Real(real) => self.translate_expr_lit_real(real),
      LitKind::Str(string) => self.translate_expr_lit_str(string),
    }
  }

  fn translate_expr_lit_bool(&mut self, boolean: &bool) -> Value {
    self.builder.ins().bconst(types::B1, *boolean)
  }

  fn translate_expr_lit_int(&mut self, num: &i64) -> Value {
    self.builder.ins().iconst(types::I64, *num)
  }

  fn translate_expr_lit_real(&mut self, num: &f64) -> Value {
    self.builder.ins().f64const(*num)
  }

  // fixme #1
  fn translate_expr_lit_str(&mut self, data: &String) -> Value {
    self.data_builder.create_data(
      &mut self.builder,
      self.module,
      self.globals,
      data,
    )
  }

  fn translate_expr_id(&mut self, name: &String) -> Value {
    let Some(decl) = self.vars.get(&name.to_string()) else {
      panic!("{}", format!("🤖 the name `{name}` not found"))
    };

    self.builder.use_var(*decl)
  }

  fn translate_expr_call(
    &mut self,
    callee: &Expr,
    inputs: &[Fsp<Expr>],
  ) -> Value {
    match self.funs.get(&callee.to_string()) {
      Some(fun) => {
        let callee_ref =
          self.module.declare_func_in_func(fun.id, self.builder.func);

        let inputs = inputs
          .iter()
          .map(|arg| self.translate_expr(arg))
          .collect::<Vec<_>>();

        let call_instruction = self.builder.ins().call(callee_ref, &inputs);
        let call_results = self.builder.inst_results(call_instruction);

        if call_results.is_empty() {
          return self.translate_expr_lit_int(&0);
        }

        call_results[0]
      }
      None => panic!("{}", format!("🤖 this function `{callee}` do not exist")),
    }
  }

  fn translate_expr_un_op(&mut self, op: &UnOp, rhs: &Expr) -> Value {
    let rhs_new = self.translate_expr(rhs);

    match &op.node {
      UnOpKind::Neg => self.translate_expr_un_op_neg(&rhs.kind, rhs_new),
      UnOpKind::Not => self.translate_expr_un_op_not(rhs_new),
    }
  }

  fn translate_expr_un_op_neg(&mut self, kind: &ExprKind, rhs: Value) -> Value {
    let ExprKind::Lit(lit) = kind else { unreachable!() };

    match &lit.kind {
      LitKind::Int(_int) => self.builder.ins().ineg(rhs),
      LitKind::Real(_real) => self.builder.ins().fneg(rhs),
      _ => panic!("{}", format!("🤖 unexpected unary operation: {kind}")),
    }
  }

  fn translate_expr_un_op_not(&mut self, rhs: Value) -> Value {
    let value = self.builder.ins().icmp_imm(IntCC::Equal, rhs, 0);

    self.builder.ins().bint(self.ty, value)
  }

  fn translate_expr_bin_op(
    &mut self,
    lhs: &Expr,
    op: &BinOp,
    rhs: &Expr,
  ) -> Value {
    let kind = &rhs.kind;
    let lhs = self.translate_expr(lhs);
    let rhs = self.translate_expr(rhs);

    match &op.node {
      BinOpKind::Add => self.translate_expr_bin_op_add(kind, lhs, rhs),
      BinOpKind::Sub => self.translate_expr_bin_op_sub(kind, lhs, rhs),
      BinOpKind::Mul => self.translate_expr_bin_op_mul(kind, lhs, rhs),
      BinOpKind::Div => self.translate_expr_bin_op_div(kind, lhs, rhs),
      BinOpKind::Rem => self.translate_expr_bin_op_rem(lhs, rhs),
      BinOpKind::Lt => self.translate_expr_bin_op_lt(kind, lhs, rhs),
      BinOpKind::Gt => self.translate_expr_bin_op_gt(kind, lhs, rhs),
      BinOpKind::Le => self.translate_expr_bin_op_le(kind, lhs, rhs),
      BinOpKind::Ge => self.translate_expr_bin_op_ge(kind, lhs, rhs),
      BinOpKind::Eq => self.translate_expr_bin_op_eq(kind, lhs, rhs),
      BinOpKind::Ne => self.translate_expr_bin_op_ne(kind, lhs, rhs),
      BinOpKind::Or => self.translate_expr_bin_op_or(lhs, op, rhs),
      BinOpKind::And => self.translate_expr_bin_op_and(lhs, op, rhs),
      BinOpKind::Shl => self.translate_expr_bin_op_shl(lhs, rhs),
      BinOpKind::Shr => self.translate_expr_bin_op_shr(lhs, rhs),
      BinOpKind::BitAnd => self.translate_expr_bin_op_bit_and(lhs, rhs),
      BinOpKind::BitXor => self.translate_expr_bin_op_bit_xor(lhs, rhs),
      BinOpKind::BitOr => self.translate_expr_bin_op_bit_or(lhs, rhs),
      _ => panic!("{}", format!("🤖 unexpected binary operation: {op}")),
    }
  }

  fn translate_expr_bin_op_add(
    &mut self,
    kind: &ExprKind,
    lhs: Value,
    rhs: Value,
  ) -> Value {
    let ExprKind::Lit(lit) = kind else { unreachable!() };

    match &lit.kind {
      LitKind::Int(_int) => self.builder.ins().iadd(lhs, rhs),
      LitKind::Real(_real) => self.builder.ins().fadd(lhs, rhs),
      _ => panic!("{}", format!("🤖 unexpected binary operation: {kind}")),
    }
  }

  fn translate_expr_bin_op_sub(
    &mut self,
    kind: &ExprKind,
    lhs: Value,
    rhs: Value,
  ) -> Value {
    let ExprKind::Lit(lit) = kind else { unreachable!() };

    match &lit.kind {
      LitKind::Int(_int) => self.builder.ins().isub(lhs, rhs),
      LitKind::Real(_real) => self.builder.ins().fsub(lhs, rhs),
      _ => panic!("{}", format!("🤖 unexpected binary operation: {kind}")),
    }
  }

  fn translate_expr_bin_op_mul(
    &mut self,
    kind: &ExprKind,
    lhs: Value,
    rhs: Value,
  ) -> Value {
    let ExprKind::Lit(lit) = kind else { unreachable!() };

    match &lit.kind {
      LitKind::Int(_int) => self.builder.ins().imul(lhs, rhs),
      LitKind::Real(_real) => self.builder.ins().fmul(lhs, rhs),
      _ => panic!("{}", format!("🤖 unexpected binary operation: {kind}")),
    }
  }

  fn translate_expr_bin_op_div(
    &mut self,
    kind: &ExprKind,
    lhs: Value,
    rhs: Value,
  ) -> Value {
    let ExprKind::Lit(lit) = kind else { unreachable!() };

    match &lit.kind {
      LitKind::Int(_int) => self.builder.ins().sdiv(lhs, rhs),
      LitKind::Real(_real) => self.builder.ins().fdiv(lhs, rhs),
      _ => panic!("{}", format!("🤖 unexpected binary operation: {kind}")),
    }
  }

  fn translate_expr_bin_op_rem(&mut self, lhs: Value, rhs: Value) -> Value {
    self.builder.ins().srem(lhs, rhs)
  }

  fn translate_expr_bin_op_lt(
    &mut self,
    kind: &ExprKind,
    lhs: Value,
    rhs: Value,
  ) -> Value {
    let ExprKind::Lit(lit) = kind else { unreachable!() };

    let boolean = match &lit.kind {
      LitKind::Int(_int) => {
        self.builder.ins().icmp(IntCC::SignedLessThan, lhs, rhs)
      }
      LitKind::Real(_real) => {
        self.builder.ins().fcmp(FloatCC::LessThan, lhs, rhs)
      }
      _ => panic!("{}", format!("🤖 unexpected binary operation: {kind}")),
    };

    self.builder.ins().bint(self.ty, boolean)
  }

  fn translate_expr_bin_op_gt(
    &mut self,
    kind: &ExprKind,
    lhs: Value,
    rhs: Value,
  ) -> Value {
    let ExprKind::Lit(lit) = kind else { unreachable!() };

    let boolean = match &lit.kind {
      LitKind::Int(_int) => {
        self.builder.ins().icmp(IntCC::SignedGreaterThan, lhs, rhs)
      }
      LitKind::Real(_real) => {
        self.builder.ins().fcmp(FloatCC::GreaterThan, lhs, rhs)
      }
      _ => panic!("{}", format!("🤖 unexpected binary operation: {kind}")),
    };

    self.builder.ins().bint(self.ty, boolean)
  }

  fn translate_expr_bin_op_le(
    &mut self,
    kind: &ExprKind,
    lhs: Value,
    rhs: Value,
  ) -> Value {
    let ExprKind::Lit(lit) = kind else { unreachable!() };

    let boolean = match &lit.kind {
      LitKind::Int(_int) => {
        self
          .builder
          .ins()
          .icmp(IntCC::SignedLessThanOrEqual, lhs, rhs)
      }
      LitKind::Real(_real) => {
        self.builder.ins().fcmp(FloatCC::LessThanOrEqual, lhs, rhs)
      }
      _ => panic!("{}", format!("🤖 unexpected binary operation: {kind}")),
    };

    self.builder.ins().bint(self.ty, boolean)
  }

  fn translate_expr_bin_op_ge(
    &mut self,
    kind: &ExprKind,
    lhs: Value,
    rhs: Value,
  ) -> Value {
    let ExprKind::Lit(lit) = kind else { unreachable!() };

    let boolean = match &lit.kind {
      LitKind::Int(_int) => {
        self
          .builder
          .ins()
          .icmp(IntCC::SignedGreaterThanOrEqual, lhs, rhs)
      }
      LitKind::Real(_real) => {
        self
          .builder
          .ins()
          .fcmp(FloatCC::GreaterThanOrEqual, lhs, rhs)
      }
      _ => panic!("{}", format!("🤖 unexpected binary operation: {kind}")),
    };

    self.builder.ins().bint(self.ty, boolean)
  }

  fn translate_expr_bin_op_eq(
    &mut self,
    kind: &ExprKind,
    lhs: Value,
    rhs: Value,
  ) -> Value {
    let ExprKind::Lit(lit) = kind else { unreachable!() };

    let boolean = match &lit.kind {
      LitKind::Int(_int) => self.builder.ins().icmp(IntCC::Equal, lhs, rhs),
      LitKind::Real(_real) => self.builder.ins().fcmp(FloatCC::Equal, lhs, rhs),
      _ => panic!("{}", format!("🤖 unexpected binary operation: {kind}")),
    };

    self.builder.ins().bint(self.ty, boolean)
  }

  fn translate_expr_bin_op_ne(
    &mut self,
    kind: &ExprKind,
    lhs: Value,
    rhs: Value,
  ) -> Value {
    let ExprKind::Lit(lit) = kind else { unreachable!() };

    let boolean = match &lit.kind {
      LitKind::Int(_int) => self.builder.ins().icmp(IntCC::NotEqual, lhs, rhs),
      LitKind::Real(_real) => {
        self.builder.ins().fcmp(FloatCC::NotEqual, lhs, rhs)
      }
      _ => panic!("{}", format!("🤖 unexpected binary operation: {kind}")),
    };

    self.builder.ins().bint(self.ty, boolean)
  }

  fn translate_expr_bin_op_or(
    &mut self,
    lhs: Value,
    op: &BinOp,
    rhs: Value,
  ) -> Value {
    self.translate_bin_op_logical(lhs, op, rhs)
  }

  fn translate_expr_bin_op_and(
    &mut self,
    lhs: Value,
    op: &BinOp,
    rhs: Value,
  ) -> Value {
    self.translate_bin_op_logical(lhs, op, rhs)
  }

  fn translate_bin_op_logical(
    &mut self,
    lhs: Value,
    op: &BinOp,
    rhs: Value,
  ) -> Value {
    let body_block = self.builder.create_block();
    let merge_block = self.builder.create_block();

    self.builder.append_block_param(merge_block, self.ty);

    match op.node {
      BinOpKind::And => self.builder.ins().brnz(lhs, body_block, &[]),
      BinOpKind::Or => self.builder.ins().brz(lhs, body_block, &[]),
      _ => unreachable!(),
    };

    self.builder.ins().jump(merge_block, &[lhs]);
    self.builder.seal_block(body_block);
    self.builder.switch_to_block(body_block);
    self.builder.ins().jump(merge_block, &[rhs]);
    self.builder.seal_block(merge_block);
    self.builder.switch_to_block(merge_block);
    self.builder.block_params(merge_block)[0]
  }

  fn translate_expr_bin_op_shl(&mut self, lhs: Value, rhs: Value) -> Value {
    self.builder.ins().ishl(lhs, rhs)
  }

  fn translate_expr_bin_op_shr(&mut self, lhs: Value, rhs: Value) -> Value {
    self.builder.ins().sshr(lhs, rhs)
  }

  fn translate_expr_bin_op_bit_and(&mut self, lhs: Value, rhs: Value) -> Value {
    self.builder.ins().band(lhs, rhs)
  }

  fn translate_expr_bin_op_bit_xor(&mut self, lhs: Value, rhs: Value) -> Value {
    self.builder.ins().bxor(lhs, rhs)
  }

  fn translate_expr_bin_op_bit_or(&mut self, lhs: Value, rhs: Value) -> Value {
    self.builder.ins().bor(lhs, rhs)
  }

  fn translate_expr_assign(
    &mut self,
    lhs: &Expr,
    _op: &BinOp,
    rhs: &Expr,
  ) -> Value {
    let rhs = self.translate_expr(rhs);
    let variable = self.vars.get(&lhs.to_string()).unwrap();

    self.builder.def_var(*variable, rhs);

    rhs
  }

  fn translate_expr_assign_op(
    &mut self,
    lhs: &Expr,
    op: &BinOp,
    rhs: &Expr,
  ) -> Value {
    let kind = &rhs.kind;
    let rhs = self.translate_expr(rhs);

    match &lhs.kind {
      ExprKind::Identifier(name) => {
        let variable = *self.vars.get(&name.to_string()).unwrap();
        let lhs = self.translate_expr(lhs);

        let rhs = match &op.node {
          BinOpKind::Add => self.translate_expr_bin_op_add(kind, lhs, rhs),
          BinOpKind::Sub => self.translate_expr_bin_op_sub(kind, lhs, rhs),
          BinOpKind::Mul => self.translate_expr_bin_op_mul(kind, lhs, rhs),
          BinOpKind::Div => self.translate_expr_bin_op_div(kind, lhs, rhs),
          BinOpKind::Rem => self.translate_expr_bin_op_rem(lhs, rhs),
          BinOpKind::BitAnd => self.translate_expr_bin_op_bit_and(lhs, rhs),
          BinOpKind::BitXor => self.translate_expr_bin_op_bit_xor(lhs, rhs),
          BinOpKind::BitOr => self.translate_expr_bin_op_bit_or(lhs, rhs),
          _ => panic!("{}", format!("🤖 unexpected assign operation: {op}")),
        };

        self.builder.def_var(variable, rhs);

        rhs
      }
      _ => unreachable!(),
    }
  }

  fn translate_expr_decl(&mut self, decl: &Decl) -> Value {
    let value = self.translate_expr(&decl.value);

    let variable = self.variable_builder.create_variable(
      &mut self.builder,
      value,
      types::I64,
    );

    let variable_shadowed = self.vars.remove(&decl.pattern.to_string());

    self.vars.insert(decl.pattern.to_string(), variable);

    if let Some(variable) = variable_shadowed {
      self.vars.insert(decl.pattern.to_string(), variable);
    }

    value
  }

  fn translate_expr_block(&mut self, block: &Block) -> Value {
    let mut value = self.translate_expr_lit_int(&0);

    for expr in &block.exprs {
      value = self.translate_expr(expr);
    }

    value
  }

  fn translate_expr_loop(&mut self, body: &Block) -> Value {
    let body_block = self.builder.create_block();
    let end_block = self.builder.create_block();

    self.builder.ins().jump(body_block, &[]);
    self.builder.switch_to_block(body_block);
    self.blocks.push(end_block);
    self.builder.switch_to_block(body_block);

    for expr in &body.exprs {
      self.translate_expr(expr);
    }

    self.builder.ins().jump(body_block, &[]);
    self.blocks.pop();
    self.builder.seal_block(body_block);
    self.builder.seal_block(end_block);
    self.builder.switch_to_block(end_block);
    self.builder.ins().iconst(self.ty, 0)
  }

  fn translate_expr_while(&mut self, condition: &Expr, body: &Block) -> Value {
    let header_block = self.builder.create_block();
    let body_block = self.builder.create_block();
    let end_block = self.builder.create_block();

    self.builder.ins().jump(header_block, &[]);
    self.builder.switch_to_block(header_block);

    let condition_value = self.translate_expr(condition);

    self.builder.ins().brz(condition_value, end_block, &[]);
    self.builder.ins().jump(body_block, &[]);
    self.blocks.push(end_block);
    self.builder.seal_block(body_block);
    self.builder.switch_to_block(body_block);

    for expr in &body.exprs {
      self.translate_expr(expr);
    }

    self.builder.ins().jump(header_block, &[]);
    self.blocks.pop();
    self.builder.seal_block(header_block);
    self.builder.seal_block(end_block);
    self.builder.switch_to_block(end_block);
    self.builder.ins().iconst(self.ty, 0)
  }

  fn translate_expr_return(&mut self, maybe_expr: &Option<Fsp<Expr>>) -> Value {
    let mut value = self.translate_expr_lit_int(&0);

    if let Some(e) = maybe_expr {
      value = self.translate_expr(e);

      self.builder.ins().return_(&[value]);
    } else {
      self.builder.ins().return_(&[]);
    }

    let new_block = self.builder.create_block();

    self.builder.seal_block(new_block);
    self.builder.switch_to_block(new_block);

    value
  }

  fn translate_expr_break(&mut self, maybe_expr: &Option<Fsp<Expr>>) -> Value {
    let mut value = self.translate_expr_lit_int(&0);
    let end_block = *self.blocks.last().unwrap();

    if let Some(expr) = maybe_expr {
      value = self.translate_expr(expr);

      self.builder.ins().jump(end_block, &[value]);
    } else {
      self.builder.ins().jump(end_block, &[]);
    }

    let new_block = self.builder.create_block();

    self.builder.seal_block(new_block);
    self.builder.switch_to_block(new_block);

    value
  }

  fn translate_expr_continue(&mut self) -> Value {
    let end_block = *self.blocks.last().unwrap();

    self.builder.ins().jump(end_block, &[]);

    let new_block = self.builder.create_block();

    self.builder.seal_block(new_block);
    self.builder.switch_to_block(new_block);
    self.translate_expr_lit_int(&0)
  }

  fn translate_expr_when(
    &mut self,
    condition: &Fsp<Expr>,
    consequence: &Fsp<Expr>,
    alternative: &Fsp<Expr>,
  ) -> Value {
    self.translate_conditional(
      condition,
      consequence,
      &Some(alternative.clone()),
    )
  }

  fn translate_expr_if_else(
    &mut self,
    condition: &Expr,
    consequence: &Expr,
    maybe_alternative: &Option<Fsp<Expr>>,
  ) -> Value {
    self.translate_conditional(condition, consequence, maybe_alternative)
  }

  fn translate_conditional(
    &mut self,
    condition: &Expr,
    consequence: &Expr,
    maybe_alternative: &Option<Fsp<Expr>>,
  ) -> Value {
    let cond_block = self.builder.create_block();
    let cons_block = self.builder.create_block();
    let merge_block = self.builder.create_block();

    self.builder.append_block_param(merge_block, types::I64);

    let condition = self.translate_expr(condition);

    self.builder.ins().brz(condition, cons_block, &[]);
    self.builder.ins().jump(cond_block, &[]);

    self.builder.switch_to_block(cond_block);
    self.builder.seal_block(cond_block);

    let consequence = self.translate_expr(consequence);

    self.builder.ins().jump(merge_block, &[consequence]);

    self.builder.switch_to_block(cons_block);
    self.builder.seal_block(cons_block);

    let mut alternative = self.translate_expr_lit_int(&0);

    if let Some(alt) = &maybe_alternative {
      alternative = self.translate_expr(alt);
    };

    self.builder.ins().jump(merge_block, &[alternative]);

    self.builder.switch_to_block(merge_block);
    self.builder.seal_block(merge_block);

    self.builder.block_params(merge_block)[0]
  }

  fn translate_expr_lambda(
    &mut self,
    _args: &[Fsp<Expr>],
    _block_or_expr: &Expr,
  ) -> Value {
    todo!()
  }

  // todo: nyf! not yet finished
  fn translate_expr_array(&mut self, elements: &[Fsp<Expr>]) -> Value {
    for (x, element) in elements.iter().enumerate() {
      let element_value = self.translate_expr(element);
      let offset = types::I64.bytes() as i32 * x as i32;

      self.builder.ins().stack_store(
        element_value,
        StackSlot::from_u32(x as u32),
        Offset32::new(offset),
      );
    }

    self.translate_expr_lit_int(&0)
  }

  fn translate_expr_array_access(
    &mut self,
    _indexed: &Expr,
    _index: &Expr,
  ) -> Value {
    todo!()
  }

  fn translate_expr_stmt(&mut self, stmt: &Stmt) -> Value {
    self.translate_stmt(stmt)
  }
}
