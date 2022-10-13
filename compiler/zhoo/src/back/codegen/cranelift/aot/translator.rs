use crate::back::codegen::cranelift::allocator;

use crate::back::codegen::cranelift::interface::{
  CompiledFunction, DataContextBuilder, VariableBuilder,
};

use crate::front::parser::tree::ast::*;
use crate::front::parser::tree::PBox;
use crate::util::error::{GenerateKind, Report};

use cranelift::prelude::{
  types, Block as CBlock, EntityRef, FunctionBuilder, GlobalValueData, Imm64,
  InstBuilder, IntCC, Value, Variable,
};

use cranelift_codegen::ir::immediates::Offset32;
use cranelift_codegen::ir::{GlobalValue, MemFlags};
use cranelift_module::Module;
use cranelift_object::ObjectModule;

use std::collections::HashMap;

pub struct Translator<'a> {
  pub builder: FunctionBuilder<'a>,
  pub module: &'a mut ObjectModule,
  pub funs: &'a HashMap<String, CompiledFunction>,
  pub globals: &'a mut HashMap<String, GlobalValue>,
  pub vars: HashMap<String, Variable>,
  pub program: &'a Program,
  pub ty: types::Type,
  pub blocks: &'a mut Vec<CBlock>,
  pub variable_builder: &'a mut VariableBuilder,
  pub data_ctx_builder: &'a mut DataContextBuilder,
}

impl<'a> Translator<'a> {
  pub fn translate(&mut self, block: &Block) -> Result<Value, String> {
    let mut value = Value::new(0);

    for expr in &block.exprs {
      value = self.translate_expr(expr);
    }

    Ok(value)
  }

  fn translate_stmt(&mut self, stmt: &Stmt) -> Value {
    match &stmt.kind {
      StmtKind::Val(decl) => self.translate_stmt_val(decl),
      StmtKind::Struct(struct_def) => self.translate_stmt_struct(struct_def),
      _ => panic!("tmp translate:stmt"),
    }
  }

  fn translate_stmt_val(&mut self, decl: &Decl) -> Value {
    self.translate_decl(decl)
  }

  fn translate_decl(&mut self, decl: &Decl) -> Value {
    let value = self.translate_expr(&decl.value);

    let var =
      self
        .variable_builder
        .create_var(&mut self.builder, value, types::I64);

    self.vars.insert(decl.pattern.to_string(), var);

    value
  }

  fn translate_stmt_struct(&mut self, struct_def: &Struct) -> Value {
    match &struct_def.kind {
      StructKind::Init => self.translate_stmt_struct_init(),
      StructKind::Decl(fields) => self.translate_stmt_struct_decl(fields),
      StructKind::Tuple(fields) => self.translate_stmt_struct_tuple(fields),
    }
  }

  fn translate_stmt_struct_init(&mut self) -> Value {
    todo!()
  }

  fn translate_stmt_struct_decl(
    &mut self,
    _fields: &[PBox<StructDeclField>],
  ) -> Value {
    todo!()
  }

  fn translate_stmt_struct_tuple(
    &mut self,
    _fields: &[PBox<StructTupleField>],
  ) -> Value {
    todo!()
  }

  fn translate_expr(&mut self, expr: &Expr) -> Value {
    match &expr.kind {
      ExprKind::Stmt(stmt) => self.translate_expr_stmt(stmt),
      ExprKind::Decl(decl) => self.translate_expr_decl(decl),
      ExprKind::Lit(lit) => self.translate_expr_lit(lit),
      ExprKind::Identifier(s) => self.translate_expr_id(s),
      ExprKind::Call(callee, args) => self.translate_expr_call(callee, args),
      ExprKind::UnOp(op, rhs) => self.translate_expr_un_op(op, rhs),
      ExprKind::BinOp(lhs, op, rhs) => self.translate_expr_bin_op(lhs, op, rhs),
      ExprKind::Return(value) => self.translate_expr_return(value),
      ExprKind::Assign(id, op, rhs) => self.translate_expr_assign(id, op, rhs),
      ExprKind::AssignOp(lhs, op, rhs) => {
        self.translate_expr_assign_op(lhs, op, rhs)
      }
      ExprKind::Loop(body) => self.translate_expr_loop(body),
      ExprKind::While(condition, body) => {
        self.translate_expr_while(condition, body)
      }
      ExprKind::Break(value) => self.translate_expr_break(expr, value),
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
      ExprKind::Index(indexed, index) => {
        self.translate_expr_index(indexed, index)
      }
      _ => todo!("tmp translate:expr => {}", expr),
    }
  }

  fn translate_expr_stmt(&mut self, stmt: &Stmt) -> Value {
    self.translate_stmt(stmt)
  }

  fn translate_expr_decl(&mut self, decl: &Decl) -> Value {
    self.translate_decl(decl)
  }

  fn translate_expr_lit(&mut self, lit: &Lit) -> Value {
    match &lit.kind {
      LitKind::Bool(boolean) => self.translate_expr_lit_bool(boolean),
      LitKind::Int(num) => self.translate_expr_lit_int(num),
      LitKind::Real(num) => self.translate_expr_lit_real(num),
      LitKind::Str(s) => self.translate_expr_lit_str(s),
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

  fn translate_expr_lit_str(&mut self, data: &String) -> Value {
    self.data_ctx_builder.create_data(
      &mut self.builder,
      self.module,
      self.globals,
      data,
    )
  }

  fn translate_expr_id(&mut self, name: &String) -> Value {
    if let Some(decl) = self.vars.get(&name.to_string()) {
      return self.builder.use_var(*decl);
    }

    if let Some(_fun) = self.funs.get(&name.to_string()) {
      todo!();
    }

    self.program.reporter.raise(Report::Generate(
      GenerateKind::IdentifierNotFound(name.to_string()),
    ))
  }

  fn translate_expr_call(
    &mut self,
    callee: &Expr,
    inputs: &[PBox<Expr>],
  ) -> Value {
    match self.funs.get(&callee.to_string()) {
      Some(func) => {
        if func.input_len != inputs.len() {
          // todo: not finished yet
          self.program.reporter.add_report(Report::Generate(
            GenerateKind::ArgumentsMismatch(callee.span),
          ))
        }

        let callee_ref =
          self.module.declare_func_in_func(func.id, self.builder.func);

        let inputs = inputs
          .iter()
          .map(|arg| self.translate_expr(arg))
          .collect::<Vec<_>>();

        let call_inst = self.builder.ins().call(callee_ref, &inputs);
        let call_results = self.builder.inst_results(call_inst);

        if call_results.is_empty() {
          return self.translate_expr_lit_int(&0);
        }

        call_results[0]
      }
      None => self.program.reporter.raise(Report::Generate(
        GenerateKind::CallFunctionNotFound(callee.span, callee.to_string()),
      )),
    }
  }

  fn translate_expr_un_op(&mut self, op: &UnOp, rhs: &Expr) -> Value {
    let rhs = self.translate_expr(rhs);

    match &op.node {
      UnOpKind::Neg => self.translate_expr_un_op_neg(rhs),
      UnOpKind::Not => self.translate_expr_un_op_not(rhs),
    }
  }

  fn translate_expr_un_op_neg(&mut self, rhs: Value) -> Value {
    self.builder.ins().ineg(rhs)
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
    let lhs = self.translate_expr(lhs);
    let rhs = self.translate_expr(rhs);

    match &op.node {
      BinOpKind::Add => self.translate_expr_bin_op_add(lhs, rhs),
      BinOpKind::Sub => self.translate_expr_bin_op_sub(lhs, rhs),
      BinOpKind::Mul => self.translate_expr_bin_op_mul(lhs, rhs),
      BinOpKind::Div => self.translate_expr_bin_op_div(lhs, rhs),
      BinOpKind::Rem => self.translate_expr_bin_op_rem(lhs, rhs),
      BinOpKind::Lt => self.translate_expr_bin_op_lt(lhs, rhs),
      BinOpKind::Gt => self.translate_expr_bin_op_gt(lhs, rhs),
      BinOpKind::Le => self.translate_expr_bin_op_le(lhs, rhs),
      BinOpKind::Ge => self.translate_expr_bin_op_ge(lhs, rhs),
      BinOpKind::Eq => self.translate_expr_bin_op_eq(lhs, rhs),
      BinOpKind::Ne => self.translate_expr_bin_op_ne(lhs, rhs),
      BinOpKind::Or => self.translate_expr_bin_op_or(lhs, rhs),
      BinOpKind::And => self.translate_expr_bin_op_and(lhs, rhs),
      BinOpKind::Shl => self.translate_expr_bin_op_shl(lhs, rhs),
      BinOpKind::Shr => self.translate_expr_bin_op_shr(lhs, rhs),
      BinOpKind::BitAnd => self.translate_expr_bin_op_bit_and(lhs, rhs),
      BinOpKind::BitXor => self.translate_expr_bin_op_bit_xor(lhs, rhs),
      BinOpKind::BitOr => self.translate_expr_bin_op_bit_or(lhs, rhs),
      _ => self.program.reporter.raise(Report::Generate(
        GenerateKind::InvalidBinOp(op.span, lhs.to_string(), rhs.to_string()),
      )),
    }
  }

  fn translate_expr_bin_op_add(&mut self, lhs: Value, rhs: Value) -> Value {
    self.builder.ins().iadd(lhs, rhs)
  }

  fn translate_expr_bin_op_sub(&mut self, lhs: Value, rhs: Value) -> Value {
    self.builder.ins().isub(lhs, rhs)
  }

  fn translate_expr_bin_op_mul(&mut self, lhs: Value, rhs: Value) -> Value {
    self.builder.ins().imul(lhs, rhs)
  }

  fn translate_expr_bin_op_div(&mut self, lhs: Value, rhs: Value) -> Value {
    self.builder.ins().sdiv(lhs, rhs)
  }

  fn translate_expr_bin_op_rem(&mut self, lhs: Value, rhs: Value) -> Value {
    self.builder.ins().srem(lhs, rhs)
  }

  fn translate_expr_bin_op_lt(&mut self, lhs: Value, rhs: Value) -> Value {
    let boolean = self.builder.ins().icmp(IntCC::SignedLessThan, lhs, rhs);

    self.builder.ins().bint(self.ty, boolean)
  }

  fn translate_expr_bin_op_gt(&mut self, lhs: Value, rhs: Value) -> Value {
    let boolean = self.builder.ins().icmp(IntCC::SignedGreaterThan, lhs, rhs);

    self.builder.ins().bint(self.ty, boolean)
  }

  fn translate_expr_bin_op_le(&mut self, lhs: Value, rhs: Value) -> Value {
    let boolean =
      self
        .builder
        .ins()
        .icmp(IntCC::SignedLessThanOrEqual, lhs, rhs);

    self.builder.ins().bint(self.ty, boolean)
  }

  fn translate_expr_bin_op_ge(&mut self, lhs: Value, rhs: Value) -> Value {
    let boolean =
      self
        .builder
        .ins()
        .icmp(IntCC::SignedGreaterThanOrEqual, lhs, rhs);

    self.builder.ins().bint(self.ty, boolean)
  }

  fn translate_expr_bin_op_eq(&mut self, lhs: Value, rhs: Value) -> Value {
    let boolean = self.builder.ins().icmp(IntCC::Equal, lhs, rhs);

    self.builder.ins().bint(self.ty, boolean)
  }

  fn translate_expr_bin_op_ne(&mut self, lhs: Value, rhs: Value) -> Value {
    let boolean = self.builder.ins().icmp(IntCC::NotEqual, lhs, rhs);

    self.builder.ins().bint(self.ty, boolean)
  }

  fn translate_expr_bin_op_or(&mut self, lhs: Value, rhs: Value) -> Value {
    self.builder.ins().srem(lhs, rhs)
  }

  fn translate_expr_bin_op_and(&mut self, lhs: Value, rhs: Value) -> Value {
    self.builder.ins().srem(lhs, rhs)
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
    _: &BinOp,
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
    let rhs = self.translate_expr(rhs);

    match &lhs.kind {
      ExprKind::Identifier(name) => {
        let var = *self.vars.get(&name.to_string()).unwrap();
        let lhs = self.translate_expr(lhs);

        let new_rhs = match &op.node {
          BinOpKind::Add => self.translate_expr_bin_op_add(lhs, rhs),
          BinOpKind::Sub => self.translate_expr_bin_op_sub(lhs, rhs),
          BinOpKind::Mul => self.translate_expr_bin_op_mul(lhs, rhs),
          BinOpKind::Div => self.translate_expr_bin_op_div(lhs, rhs),
          BinOpKind::Rem => self.translate_expr_bin_op_rem(lhs, rhs),
          BinOpKind::BitAnd => self.translate_expr_bin_op_bit_and(lhs, rhs),
          BinOpKind::BitXor => self.translate_expr_bin_op_bit_xor(lhs, rhs),
          BinOpKind::BitOr => self.translate_expr_bin_op_bit_or(lhs, rhs),
          _ => self.program.reporter.raise(Report::Generate(
            GenerateKind::InvalidBinOp(
              op.span,
              lhs.to_string(),
              rhs.to_string(),
            ),
          )),
        };

        self.builder.def_var(var, new_rhs);

        new_rhs
      }
      _ => unreachable!(), // fixme
    }
  }

  fn translate_expr_return(
    &mut self,
    maybe_expr: &Option<PBox<Expr>>,
  ) -> Value {
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

  fn translate_expr_break(
    &mut self,
    _: &Expr,
    maybe_expr: &Option<PBox<Expr>>,
  ) -> Value {
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
    let value = self.translate_expr_lit_int(&0);
    let end_block = *self.blocks.last().unwrap();

    self.builder.ins().jump(end_block, &[]);

    let new_block = self.builder.create_block();

    self.builder.seal_block(new_block);
    self.builder.switch_to_block(new_block);

    value
  }

  fn translate_expr_when(
    &mut self,
    condition: &PBox<Expr>,
    consequence: &PBox<Expr>,
    alternative: &PBox<Expr>,
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
    maybe_alternative: &Option<PBox<Expr>>,
  ) -> Value {
    self.translate_conditional(condition, consequence, maybe_alternative)
  }

  fn translate_conditional(
    &mut self,
    condition: &Expr,
    consequence: &Expr,
    maybe_alternative: &Option<PBox<Expr>>,
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
    _args: &[PBox<Expr>],
    _block_or_expr: &Expr,
  ) -> Value {
    todo!()
  }

  // fixme: [1]    46739 segmentation fault
  fn translate_expr_array(&mut self, elements: &[PBox<Expr>]) -> Value {
    let vm_context = self
      .builder
      .func
      .create_global_value(GlobalValueData::VMContext);

    let global_type = self.module.target_config().pointer_type();
    let array_bytes = elements.len() * global_type.bytes() as usize;
    let offset = Imm64::new(allocator::alloc(array_bytes));

    let global_value_data =
      self
        .builder
        .func
        .create_global_value(GlobalValueData::IAddImm {
          base: vm_context,
          offset,
          global_type,
        });

    let global_value = self
      .builder
      .ins()
      .global_value(global_type, global_value_data);

    for (x, element) in elements.iter().enumerate() {
      let value = self.translate_expr(element);
      let offset = global_type.bytes() * x as u32;

      self.builder.ins().store(
        MemFlags::new(),
        value,
        global_value,
        Offset32::new(offset as i32),
      );
    }

    global_value
  }

  fn translate_expr_index(&mut self, indexed: &Expr, index: &Expr) -> Value {
    let indexed = self.translate_expr(indexed);
    let index = self.translate_expr(index);
    let array_type = self.module.target_config().pointer_type();

    let offset = self
      .builder
      .ins()
      .imul_imm(index, array_type.bytes() as i64);

    let offset = self.builder.ins().iadd(indexed, offset);

    let value = self.builder.ins().load(
      array_type,
      MemFlags::new(),
      offset,
      Offset32::new(0),
    );

    value
  }
}
