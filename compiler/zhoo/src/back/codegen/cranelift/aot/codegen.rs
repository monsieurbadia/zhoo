use super::translator::Translator;

use crate::front::analyzer::builtins::{io_builtins, sys_builtins, Builtin};

use crate::back::codegen::cranelift::interface::{
  CompiledFunction, DataContextBuilder, VariableBuilder,
};

use crate::front::parser::tree::ast::{
  Ext, Fun, Program, Prototype, ReturnTy, Stmt, StmtKind,
};

use crate::front::parser::tree::ty::AsTy;

use crate::util::constant::{
  ENTRY_POINT, PATH_LIBRARY, PATH_LIBRARY_CORE, PATH_OUTPUT_DIRECTORY,
};

use crate::util::pack;

use codegen::ir::{ArgumentPurpose, GlobalValue};
use cranelift::prelude::{Block as CBlock, *};
use cranelift_codegen::settings::Flags;
use cranelift_codegen::{settings, Context};
use cranelift_module::{FuncId, Linkage, Module};
use cranelift_object::{ObjectBuilder, ObjectModule};
use cranelift_preopt::optimize;

use std::collections::HashMap;

pub type BuildResult = Result<Box<dyn FnOnce()>, String>;

pub fn generate(program: &Program) -> Codegen {
  Codegen::new(program).generate()
}

pub struct Codegen<'a> {
  builder_context: FunctionBuilderContext,
  module: ObjectModule,
  program: &'a Program,
  blocks: Vec<CBlock>,
  ctx: Context,
  ir: String,
  funs: HashMap<String, CompiledFunction>,
  globals: HashMap<String, GlobalValue>,
  data_ctx_builder: DataContextBuilder,
  variable_builder: VariableBuilder,
}

// todo:
// option
//  - output ir: bool
//  - option level: string

impl<'a> Codegen<'a> {
  fn new(program: &'a Program) -> Self {
    let mut flag_builder = settings::builder();

    flag_builder
      .set("opt_level", "speed_and_size")
      .expect("set optlevel");

    let isa_builder = cranelift_native::builder().unwrap();
    let isa = isa_builder.finish(Flags::new(flag_builder)).unwrap();

    let object_builder = ObjectBuilder::new(
      isa,
      "zhoo".to_string(),
      cranelift_module::default_libcall_names(),
    )
    .unwrap();

    let module = ObjectModule::new(object_builder);

    let mut me = Self {
      ctx: module.make_context(),
      builder_context: FunctionBuilderContext::new(),
      module,
      program,
      blocks: vec![],
      ir: String::new(),
      funs: HashMap::new(),
      globals: HashMap::new(),
      data_ctx_builder: DataContextBuilder::default(),
      variable_builder: VariableBuilder::default(),
    };

    register_builtin(&mut me);

    me
  }

  fn generate(mut self) -> Self {
    for stmt in &self.program.stmts {
      self.generate_stmt(stmt);
    }

    self
  }

  fn generate_stmt(&mut self, stmt: &Stmt) {
    match &stmt.kind {
      StmtKind::Ext(ext) => self.generate_stmt_ext(ext, Linkage::Import),
      StmtKind::Fun(fun) => self.generate_stmt_fun(fun),
      _ => panic!("generate stmt"),
    }
  }

  fn generate_stmt_fun(&mut self, fun: &Fun) {
    self.generate_fun(fun);
  }

  fn generate_fun(&mut self, fun: &Fun) {
    let signature = &mut self.ctx.func.signature;
    let params = &fun.prototype.inputs;

    for _ in params {
      signature.params.push(AbiParam::new(types::I64));
    }

    signature
      .returns
      .push(AbiParam::new(fun.prototype.output.as_ty().into()));

    let vm_context = AbiParam::special(
      self.module.target_config().pointer_type(),
      ArgumentPurpose::VMContext,
    );

    signature.params.push(vm_context);

    let func_name = fun.prototype.pattern.to_string();

    let func_id = self
      .generate_prototype(&fun.prototype, Linkage::Export)
      .unwrap();

    let mut builder =
      FunctionBuilder::new(&mut self.ctx.func, &mut self.builder_context);

    let entry_block = builder.create_block();
    builder.append_block_params_for_function_params(entry_block);
    builder.switch_to_block(entry_block);
    builder.seal_block(entry_block);

    let mut vars = HashMap::new();

    for (i, input) in params.iter().enumerate() {
      let val = builder.block_params(entry_block)[i];

      let variable =
        self
          .variable_builder
          .create_var(&mut builder, val, types::I64);

      vars.insert(input.pattern.to_string(), variable);
    }

    if let Some(ref mut func) = self.funs.get_mut(&func_name) {
      func.defined = true;
    }

    let mut translator = Translator {
      builder,
      module: &mut self.module,
      funs: &self.funs,
      globals: &mut self.globals,
      vars,
      program: self.program,
      ty: types::I64,
      blocks: &mut self.blocks,
      data_ctx_builder: &mut self.data_ctx_builder,
      variable_builder: &mut self.variable_builder,
    };

    let return_value = match translator.translate(&fun.body) {
      Ok(value) => value,
      Err(_) => {
        translator.builder.finalize();
        self.funs.remove(&func_name);
        return; // todo: error
      }
    };

    translator.builder.ins().return_(&[return_value]);
    translator.builder.finalize();

    optimize(&mut self.ctx, self.module.isa()).unwrap();

    self.ir = self.ctx.func.display().to_string();

    self.module.define_function(func_id, &mut self.ctx).unwrap();
    self.module.clear_context(&mut self.ctx);
  }

  fn generate_stmt_ext(&mut self, ext: &Ext, linkage: Linkage) {
    if self.generate_prototype(&ext.prototype, linkage).is_ok() {}
  }

  fn generate_prototype(
    &mut self,
    prototype: &Prototype,
    linkage: Linkage,
  ) -> Result<FuncId, String> {
    let func_name = &prototype.pattern.to_string();
    let params = &prototype.inputs;

    match self.funs.get(func_name) {
      Some(func) => {
        if func.defined {
          return Err(format!("redefinition of function: {func_name}"));
        }

        if func.input_len != params.len() {
          return Err(format!(
            "`{}`: redefinition of function's parameters different, {}(before) vs {}(after)",
            func_name,
            func.input_len,
            params.len()
          ));
        }

        Ok(func.id)
      }
      None => {
        let mut signature = self.module.make_signature();

        for _ in params.iter() {
          signature.params.push(AbiParam::new(types::I64));
        }

        if let ReturnTy::Ty(_ty) = &prototype.output {
          signature.returns.push(AbiParam::new(types::I64));
        } else {
          signature.returns.push(AbiParam::new(types::I64));
        }

        let id =
          match self.module.declare_function(func_name, linkage, &signature) {
            Ok(id) => id,
            Err(e) => return Err(format!("{e}")),
          };

        self.funs.insert(
          func_name.to_string(),
          CompiledFunction::new(id, false, params.len()),
        );

        Ok(id)
      }
    }
  }

  pub fn build(self, output_ir: bool) -> BuildResult {
    let object = self.module.finish();
    let bytes = object.emit().unwrap();

    Ok(Box::new(move || {
      let path_object_file = format!("{PATH_OUTPUT_DIRECTORY}/{ENTRY_POINT}.o");

      let path_core_lib = format!("{PATH_LIBRARY}/{PATH_LIBRARY_CORE}");
      let path_exe_file = format!("{PATH_OUTPUT_DIRECTORY}/{ENTRY_POINT}");

      pack::make_dir(PATH_OUTPUT_DIRECTORY);
      pack::make_file(&path_object_file, &bytes);

      pack::make_exe_with_link(
        &path_object_file,
        &path_core_lib,
        &path_exe_file,
      );

      if output_ir {
        println!("\n{}", self.ir);
      }
    }))
  }

  fn register_builtin(&mut self, builtin: Builtin) -> FuncId {
    let inputs = builtin
      .proto
      .0
      .iter()
      .enumerate()
      .map(|(x, input)| (format!("_input{x}"), input.clone()))
      .collect::<Vec<_>>();

    for _ in inputs.iter() {
      let abi_param = AbiParam::new(self.module.target_config().pointer_type());
      self.ctx.func.signature.params.push(abi_param);
    }

    let abi_param = AbiParam::new(self.module.target_config().pointer_type());
    self.ctx.func.signature.returns.push(abi_param);

    let func_id = self
      .module
      .declare_function(
        &builtin.name,
        Linkage::Import,
        &self.ctx.func.signature,
      )
      .unwrap();

    self.funs.insert(
      builtin.name.to_string(),
      CompiledFunction::new(func_id, false, inputs.len()),
    );

    self.module.clear_context(&mut self.ctx);

    func_id
  }
}

fn register_builtin(codegen: &mut Codegen) {
  for builtin in sys_builtins() {
    codegen.register_builtin(builtin);
  }

  for builtin in io_builtins() {
    codegen.register_builtin(builtin);
  }
}
