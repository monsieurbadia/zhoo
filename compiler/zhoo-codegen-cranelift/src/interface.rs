use zhoo_ast::ast::{Ty, TyKind};
use zhoo_ast::ptr::Fsp;

use cranelift::prelude::{
  types, FunctionBuilder, InstBuilder, Value, Variable,
};

use cranelift_codegen::ir::GlobalValue;
use cranelift_module::{DataContext, FuncId, Linkage, Module};
use cranelift_object::ObjectModule;
use fxhash::FxHashMap;

pub(crate) struct CompiledFunction {
  pub id: FuncId,
  pub is_defined: bool,
  pub inputs_len: usize,
}

impl CompiledFunction {
  pub const fn new(id: FuncId, is_defined: bool, inputs_len: usize) -> Self {
    Self {
      id,
      is_defined,
      inputs_len,
    }
  }
}

#[derive(Default)]
pub(crate) struct VariableBuilder {
  pub index: u32,
}

impl VariableBuilder {
  pub fn create_variable(
    &mut self,
    builder: &mut FunctionBuilder,
    value: Value,
    ty: types::Type,
  ) -> Variable {
    let variable = Variable::from_u32(self.index);

    builder.declare_var(variable, ty);
    builder.def_var(variable, value);

    self.index += 1;

    variable
  }
}

#[derive(Default)]
pub(crate) struct DataBuilder {
  pub index: u32,
}

impl DataBuilder {
  pub fn create_data(
    &mut self,
    builder: &mut FunctionBuilder,
    module: &mut ObjectModule,
    globals: &mut FxHashMap<String, GlobalValue>,
    data: &String,
  ) -> Value {
    let data_id = match globals.get(data) {
      Some(data_id) => *data_id,
      None => {
        let data_name = format!("__data{}", self.index);

        let data_id = module
          .declare_data(&data_name, Linkage::Local, false, false)
          .unwrap();

        let mut data_context = DataContext::new();

        data_context.define(data.as_bytes().to_vec().into_boxed_slice());
        module.define_data(data_id, &data_context).unwrap();

        let data_id = module.declare_data_in_func(data_id, builder.func);

        globals.insert(data.to_string(), data_id);

        self.index += 1;

        data_id
      }
    };

    builder.ins().symbol_value(types::I64, data_id)
  }
}

pub(crate) struct TypeBuilder;

impl TypeBuilder {
  pub fn from(module: &mut ObjectModule, ty: &Fsp<Ty>) -> types::Type {
    match ty.kind {
      TyKind::Void => types::I64,
      TyKind::Bool => types::B1,
      TyKind::Int => types::I64,
      TyKind::Real => types::F64,
      _ => module.target_config().pointer_type(),
    }
  }
}
