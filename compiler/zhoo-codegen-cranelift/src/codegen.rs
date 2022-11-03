use super::interface::{
  CompiledFunction, DataBuilder, TypeBuilder, VariableBuilder,
};

use super::translator::Translator;

use zhoo_analyzer::builtins::{c_builtins, io_builtins, sys_builtins, Builtin};

use zhoo_ast::ast::{
  AsTy, Ext, Fun, Program, Prototype, ReturnTy, Stmt, StmtKind,
};

use zhoo_helper::constant::{
  COMPILER_NAME, ENTRY_POINT, PATH_LIBRARY, PATH_LIBRARY_CORE,
  PATH_OUTPUT_DIRECTORY,
};

use zhoo_helper::pack;

use cranelift::prelude::{
  types, AbiParam, Block as CBlock, Configurable, FunctionBuilder,
  FunctionBuilderContext, InstBuilder, Variable,
};

use cranelift_codegen::ir::GlobalValue;
use cranelift_codegen::settings::Flags;
use cranelift_codegen::{settings, Context};
use cranelift_module::{FuncId, Linkage, Module};
use cranelift_object::{ObjectBuilder, ObjectModule};
use cranelift_preopt::optimize;
use fxhash::FxHashMap;

pub type BuildResult = Result<Box<dyn FnOnce()>, String>;

pub fn generate(program: &Program) -> Codegen {
  Codegen::new().generate(program)
}

pub struct Codegen {
  function_builder_context: FunctionBuilderContext,
  module: ObjectModule,
  blocks: Vec<CBlock>,
  context: Context,
  ir: String,
  funs: FxHashMap<String, CompiledFunction>,
  globals: FxHashMap<String, GlobalValue>,
  vars: FxHashMap<String, Variable>,
  data_builder: DataBuilder,
  variable_builder: VariableBuilder,
}

impl Codegen {
  fn new() -> Self {
    let mut flag_builder = settings::builder();

    flags_settings(&mut flag_builder);

    let isa_builder = cranelift_native::builder().unwrap();
    let isa = isa_builder.finish(Flags::new(flag_builder)).unwrap();

    let object_builder = ObjectBuilder::new(
      isa,
      String::from(COMPILER_NAME),
      cranelift_module::default_libcall_names(),
    )
    .unwrap();

    let module = ObjectModule::new(object_builder);

    let mut me = Self {
      context: module.make_context(),
      function_builder_context: FunctionBuilderContext::new(),
      module,
      blocks: vec![],
      ir: String::new(),
      funs: FxHashMap::default(),
      globals: FxHashMap::default(),
      vars: FxHashMap::default(),
      data_builder: DataBuilder::default(),
      variable_builder: VariableBuilder::default(),
    };

    register_builtins(&mut me);

    me
  }

  fn generate(mut self, program: &Program) -> Self {
    for stmt in &program.stmts {
      self.generate_stmt(stmt);
    }

    self
  }

  fn generate_stmt(&mut self, stmt: &Stmt) {
    match &stmt.kind {
      StmtKind::Ext(ext) => self.generate_stmt_ext(ext),
      StmtKind::Fun(fun) => self.generate_stmt_fun(fun),
      _ => unimplemented!(),
    }
  }

  fn generate_stmt_ext(&mut self, ext: &Ext) {
    let _ = self.generate_prototype(&ext.prototype, Linkage::Import);
  }

  fn generate_prototype(
    &mut self,
    prototype: &Prototype,
    linkage: Linkage,
  ) -> Result<FuncId, String> {
    let fun_name = &prototype.name.to_string();
    let inputs = &prototype.inputs;
    let inputs_len = inputs.len();

    match self.funs.get(fun_name) {
      Some(compiled_function) => {
        if compiled_function.is_defined {
          return Err(format!("redefinition of function: {fun_name}"));
        }

        if compiled_function.inputs_len != inputs_len {
          return Err(format!(
            "`{}`: redefinition of function's parameters different, {}(before) vs {}(after)",
            fun_name,
            compiled_function.inputs_len,
            inputs_len
          ));
        }

        Ok(compiled_function.id)
      }
      None => {
        let mut signature = self.module.make_signature();

        for input in inputs.iter() {
          let clif_type = TypeBuilder::from(&mut self.module, &input.ty);

          signature.params.push(AbiParam::new(clif_type));
        }

        if let ReturnTy::Ty(ty) = &prototype.output {
          let clif_type = TypeBuilder::from(&mut self.module, ty);

          signature.returns.push(AbiParam::new(clif_type));
        } else {
          signature.returns.push(AbiParam::new(types::I64));
        }

        let func_id =
          match self.module.declare_function(fun_name, linkage, &signature) {
            Ok(func_id) => func_id,
            Err(error) => return Err(format!("{error}")),
          };

        self.funs.insert(
          String::from(fun_name),
          CompiledFunction::new(func_id, false, inputs_len),
        );

        Ok(func_id)
      }
    }
  }

  fn generate_stmt_fun(&mut self, fun: &Fun) {
    self.generate_fun(fun);
  }

  fn generate_fun(&mut self, fun: &Fun) {
    let inputs = &fun.prototype.inputs;
    let signature = &mut self.context.func.signature;

    for input in inputs {
      let clif_type = TypeBuilder::from(&mut self.module, &input.ty);

      signature.params.push(AbiParam::new(clif_type));
    }

    let clif_type =
      TypeBuilder::from(&mut self.module, &fun.prototype.output.as_ty());

    signature.returns.push(AbiParam::new(clif_type));

    let func_name = fun.prototype.name.to_string();

    let func_id = self
      .generate_prototype(&fun.prototype, Linkage::Export)
      .unwrap();

    let mut builder = FunctionBuilder::new(
      &mut self.context.func,
      &mut self.function_builder_context,
    );

    for builtin in c_builtins() {
      register_builtin_c(
        &mut self.module,
        &mut builder,
        &mut self.funs,
        builtin,
      );
    }

    let entry_block = builder.create_block();
    builder.append_block_params_for_function_params(entry_block);
    builder.switch_to_block(entry_block);
    builder.seal_block(entry_block);

    for (i, input) in inputs.iter().enumerate() {
      let value = builder.block_params(entry_block)[i];

      let variable =
        self
          .variable_builder
          .create_variable(&mut builder, value, types::I64);

      self.vars.insert(input.pattern.to_string(), variable);
    }

    if let Some(ref mut compiled_function) = self.funs.get_mut(&func_name) {
      compiled_function.is_defined = true;
    }

    let mut translator = Translator {
      builder,
      module: &mut self.module,
      funs: &mut self.funs,
      globals: &mut self.globals,
      vars: &mut self.vars,
      ty: types::I64,
      blocks: &mut self.blocks,
      data_builder: &mut self.data_builder,
      variable_builder: &mut self.variable_builder,
    };

    let return_value = match translator.translate(&fun.body) {
      Ok(value) => value,
      Err(_) => {
        translator.builder.finalize();
        self.funs.remove(&func_name);

        return; // todo (?): error
      }
    };

    translator.builder.ins().return_(&[return_value]);
    translator.builder.finalize();
    optimize(&mut self.context, self.module.isa()).unwrap();

    self.ir = self.context.func.display().to_string();

    self
      .module
      .define_function(func_id, &mut self.context)
      .unwrap();

    self.module.clear_context(&mut self.context);
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

  fn register_builtin(&mut self, builtin: Builtin) {
    let inputs = builtin
      .proto
      .0
      .iter()
      .enumerate()
      .map(|(x, input)| (format!("_input{x}"), input))
      .collect::<Vec<_>>();

    for _ in inputs.iter() {
      let clif_type = self.module.target_config().pointer_type();

      self
        .context
        .func
        .signature
        .params
        .push(AbiParam::new(clif_type));
    }

    let clif_type = self.module.target_config().pointer_type();

    self
      .context
      .func
      .signature
      .returns
      .push(AbiParam::new(clif_type));

    let func_id = self
      .module
      .declare_function(
        &builtin.name,
        Linkage::Import,
        &self.context.func.signature,
      )
      .unwrap();

    self.funs.insert(
      builtin.name,
      CompiledFunction::new(func_id, false, inputs.len()),
    );

    self.module.clear_context(&mut self.context);
  }
}

// @see https://docs.rs/cranelift/latest/cranelift/prelude/settings/struct.Flags.html
fn flags_settings(flag_builder: &mut settings::Builder) {
  flag_builder
    .set("opt_level", "speed_and_size")
    .expect("set optlevel");
}

fn register_builtins(codegen: &mut Codegen) {
  let builtins = vec![io_builtins(), sys_builtins()];

  for builtin in builtins.into_iter().flatten() {
    codegen.register_builtin(builtin);
  }
}

// @see https://github.com/bytecodealliance/wasmtime/blob/main/cranelift/object/tests/basic.rs#L179-L185
fn register_builtin_c(
  module: &mut ObjectModule,
  builder: &mut FunctionBuilder,
  funs: &mut FxHashMap<String, CompiledFunction>,
  builtin: Builtin,
) {
  let mut signature = module.make_signature();

  for input in builtin.proto.0 {
    let clif_type = TypeBuilder::from(module, &input);

    signature.params.push(AbiParam::new(clif_type));
  }

  let clif_type = module.target_config().pointer_type();

  signature.returns.push(AbiParam::new(clif_type));

  let func_id = module
    .declare_function(&builtin.name, Linkage::Export, &signature)
    .unwrap_or_else(|_| panic!("declare {} function", builtin.name));

  funs.insert(builtin.name, CompiledFunction::new(func_id, false, 1));
  module.declare_func_in_func(func_id, builder.func);
}
